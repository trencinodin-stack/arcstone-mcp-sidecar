use arcstone_execution_boundary::{
    execute_request, issue_authorization, Decision, DenyReason, EvidenceContext, ExecutionRequest,
    ExperimentPaths, ACTION_WRITE_PROTECTED_FILE, RESOURCE_EFFECT_LOG,
};
use tempfile::tempdir;

fn req(id: &str) -> ExecutionRequest {
    ExecutionRequest {
        authorization_id: id.into(),
        action: ACTION_WRITE_PROTECTED_FILE.into(),
        resource_id: RESOURCE_EFFECT_LOG.into(),
        payload_hex: hex::encode(b"CORE-SEPARATION"),
    }
}

#[test]
fn t12_favorable_core_observation_without_authorization_is_denied() {
    let dir = tempdir().unwrap();
    let paths = ExperimentPaths::new(dir.path());

    let result = execute_request(
        &paths,
        &req("NO-AUTH"),
        "t12",
        EvidenceContext {
            producer_label: Some("test-producer".into()),
            core_observation: Some("PASS".into()),
        },
    )
    .unwrap();

    assert_eq!(result.decision, Decision::Deny);
    assert_eq!(result.deny_reason, Some(DenyReason::AbsentAuthorization));
}

#[test]
fn t13_valid_authorization_with_core_omitted_can_allow() {
    let dir = tempdir().unwrap();
    let paths = ExperimentPaths::new(dir.path());
    paths.init().unwrap();

    issue_authorization(
        dir.path(),
        "AUTH-T13",
        ACTION_WRITE_PROTECTED_FILE,
        RESOURCE_EFFECT_LOG,
        b"CORE-SEPARATION",
    )
    .unwrap();

    let result = execute_request(
        &paths,
        &req("AUTH-T13"),
        "t13",
        EvidenceContext {
            producer_label: Some("test-producer".into()),
            core_observation: None,
        },
    )
    .unwrap();

    assert_eq!(result.decision, Decision::Allow);
}
