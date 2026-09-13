use crate::actuator::{FileActuator, ProtectedActuator};
use crate::authorization::{
    claim_authorization, load_authorization, AuthorizationState,
};
use crate::evidence::{write_evidence, ActuationOutcome, EvidenceContext, ExecutionEvidence};
use crate::request::{
    sha256_hex, ExecutionRequest, ACTION_WRITE_PROTECTED_FILE, RESOURCE_EFFECT_LOG,
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct ExperimentPaths {
    pub root: PathBuf,
}

impl ExperimentPaths {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn protected_target(&self) -> PathBuf {
        self.root.join("protected").join("effect.bin")
    }

    pub fn init(&self) -> Result<(), String> {
        fs::create_dir_all(self.root.join("auth").join("issued"))
            .map_err(|e| format!("failed creating issued directory: {e}"))?;
        fs::create_dir_all(self.root.join("auth").join("consumed"))
            .map_err(|e| format!("failed creating consumed directory: {e}"))?;
        fs::create_dir_all(self.root.join("protected"))
            .map_err(|e| format!("failed creating protected directory: {e}"))?;
        fs::create_dir_all(self.root.join("evidence"))
            .map_err(|e| format!("failed creating evidence directory: {e}"))?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Decision {
    Allow,
    Deny,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DenyReason {
    AbsentAuthorization,
    InvalidAuthorization,
    ConsumedAuthorization,
    ActionMismatch,
    ResourceMismatch,
    PayloadMismatch,
    MalformedRequest,
    ClaimFailed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExecutionResult {
    pub decision: Decision,
    pub deny_reason: Option<DenyReason>,
    pub authorization_state_before: AuthorizationState,
    pub authorization_state_after: AuthorizationState,
    pub actuation: ActuationOutcome,
    pub effect_present_after: bool,
    pub effect_sha256_after: Option<String>,
}

pub fn execute_request(
    paths: &ExperimentPaths,
    request: &ExecutionRequest,
    run_id: &str,
    context: EvidenceContext,
) -> Result<ExecutionResult, String> {
    let mut actuator = FileActuator;
    execute_request_with_actuator(paths, request, run_id, context, &mut actuator, true)
}

fn execute_request_with_actuator<A: ProtectedActuator>(
    paths: &ExperimentPaths,
    request: &ExecutionRequest,
    run_id: &str,
    context: EvidenceContext,
    actuator: &mut A,
    write_run_evidence: bool,
) -> Result<ExecutionResult, String> {
    paths.init()?;

    let payload = match request.payload_bytes() {
        Ok(v) => v,
        Err(_) => {
            return finish_denied(
                paths,
                request,
                run_id,
                context,
                AuthorizationState::Invalid,
                DenyReason::MalformedRequest,
                write_run_evidence,
            );
        }
    };

    let request_sha = request.request_sha256()?;
    let payload_sha = sha256_hex(&payload);

    let (state_before, record) =
        load_authorization(&paths.root, &request.authorization_id)?;

    let record = match state_before {
        AuthorizationState::Absent => {
            return finish_denied(
                paths,
                request,
                run_id,
                context,
                state_before,
                DenyReason::AbsentAuthorization,
                write_run_evidence,
            );
        }
        AuthorizationState::Invalid => {
            return finish_denied(
                paths,
                request,
                run_id,
                context,
                state_before,
                DenyReason::InvalidAuthorization,
                write_run_evidence,
            );
        }
        AuthorizationState::Consumed => {
            return finish_denied(
                paths,
                request,
                run_id,
                context,
                state_before,
                DenyReason::ConsumedAuthorization,
                write_run_evidence,
            );
        }
        AuthorizationState::Issued => record.ok_or_else(|| {
            "authorization state ISSUED without authorization record".to_string()
        })?,
    };

    if record.action != request.action {
        return finish_denied(
            paths,
            request,
            run_id,
            context,
            state_before,
            DenyReason::ActionMismatch,
            write_run_evidence,
        );
    }

    if record.resource_id != request.resource_id
        || request.resource_id != RESOURCE_EFFECT_LOG
        || request.action != ACTION_WRITE_PROTECTED_FILE
    {
        return finish_denied(
            paths,
            request,
            run_id,
            context,
            state_before,
            DenyReason::ResourceMismatch,
            write_run_evidence,
        );
    }

    if record.payload_sha256 != payload_sha {
        return finish_denied(
            paths,
            request,
            run_id,
            context,
            state_before,
            DenyReason::PayloadMismatch,
            write_run_evidence,
        );
    }

    if claim_authorization(&paths.root, &request.authorization_id, &request_sha).is_err() {
        return finish_denied(
            paths,
            request,
            run_id,
            context,
            AuthorizationState::Consumed,
            DenyReason::ClaimFailed,
            write_run_evidence,
        );
    }

    let target = paths.protected_target();
    let actuation = match actuator.actuate(&target, &payload) {
        Ok(()) => ActuationOutcome::Succeeded,
        Err(_) => ActuationOutcome::Failed,
    };

    let state_after = AuthorizationState::Consumed;
    let (effect_present_after, effect_sha256_after) = effect_state(&target)?;

    let result = ExecutionResult {
        decision: Decision::Allow,
        deny_reason: None,
        authorization_state_before: state_before,
        authorization_state_after: state_after,
        actuation,
        effect_present_after,
        effect_sha256_after,
    };

    if write_run_evidence {
        let ev = ExecutionEvidence {
            schema: "arcstone-execution-boundary/run-v1".to_string(),
            run_id: run_id.to_string(),
            request_sha256: request_sha,
            authorization_id: request.authorization_id.clone(),
            authorization_state_before: result.authorization_state_before,
            authorization_state_after: result.authorization_state_after,
            decision: "ALLOW".to_string(),
            deny_reason: None,
            actuation: result.actuation.clone(),
            effect_present_after: result.effect_present_after,
            effect_sha256_after: result.effect_sha256_after.clone(),
            producer_label: context.producer_label,
            core_observation: context.core_observation,
        };
        write_evidence(&paths.root, &ev)?;
    }

    Ok(result)
}

fn finish_denied(
    paths: &ExperimentPaths,
    request: &ExecutionRequest,
    run_id: &str,
    context: EvidenceContext,
    state_before: AuthorizationState,
    reason: DenyReason,
    write_run_evidence: bool,
) -> Result<ExecutionResult, String> {
    let target = paths.protected_target();
    let (effect_present_after, effect_sha256_after) = effect_state(&target)?;

    let result = ExecutionResult {
        decision: Decision::Deny,
        deny_reason: Some(reason.clone()),
        authorization_state_before: state_before,
        authorization_state_after: state_before,
        actuation: ActuationOutcome::NotAttempted,
        effect_present_after,
        effect_sha256_after,
    };

    if write_run_evidence {
        let request_sha = request
            .request_sha256()
            .unwrap_or_else(|_| "MALFORMED".to_string());

        let ev = ExecutionEvidence {
            schema: "arcstone-execution-boundary/run-v1".to_string(),
            run_id: run_id.to_string(),
            request_sha256: request_sha,
            authorization_id: request.authorization_id.clone(),
            authorization_state_before: result.authorization_state_before,
            authorization_state_after: result.authorization_state_after,
            decision: "DENY".to_string(),
            deny_reason: Some(format!("{reason:?}")),
            actuation: result.actuation.clone(),
            effect_present_after: result.effect_present_after,
            effect_sha256_after: result.effect_sha256_after.clone(),
            producer_label: context.producer_label,
            core_observation: context.core_observation,
        };
        write_evidence(&paths.root, &ev)?;
    }

    Ok(result)
}

fn effect_state(target: &Path) -> Result<(bool, Option<String>), String> {
    if !target.exists() {
        return Ok((false, None));
    }
    let bytes = fs::read(target)
        .map_err(|e| format!("failed reading protected effect {}: {e}", target.display()))?;
    Ok((true, Some(sha256_hex(&bytes))))
}

#[cfg(test)]
mod internal_tests {
    use super::*;
    use crate::actuator::FailingActuator;
    use crate::issuer::issue_authorization;
    use tempfile::tempdir;

    #[test]
    fn authorized_but_failed_actuator_consumes_authorization() {
        let dir = tempdir().unwrap();
        let paths = ExperimentPaths::new(dir.path());
        paths.init().unwrap();

        let payload = b"HELLO";
        issue_authorization(
            dir.path(),
            "AUTH-FAIL",
            ACTION_WRITE_PROTECTED_FILE,
            RESOURCE_EFFECT_LOG,
            payload,
        )
        .unwrap();

        let req = ExecutionRequest {
            authorization_id: "AUTH-FAIL".into(),
            action: ACTION_WRITE_PROTECTED_FILE.into(),
            resource_id: RESOURCE_EFFECT_LOG.into(),
            payload_hex: hex::encode(payload),
        };

        let mut actuator = FailingActuator;
        let result = execute_request_with_actuator(
            &paths,
            &req,
            "t16",
            EvidenceContext::default(),
            &mut actuator,
            false,
        )
        .unwrap();

        assert_eq!(result.decision, Decision::Allow);
        assert_eq!(result.authorization_state_after, AuthorizationState::Consumed);
        assert_eq!(result.actuation, ActuationOutcome::Failed);
        assert!(!result.effect_present_after);
    }
}
