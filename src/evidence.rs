use crate::authorization::AuthorizationState;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EvidenceContext {
    pub producer_label: Option<String>,
    pub core_observation: Option<String>,
}

impl Default for EvidenceContext {
    fn default() -> Self {
        Self {
            producer_label: None,
            core_observation: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ActuationOutcome {
    NotAttempted,
    Succeeded,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExecutionEvidence {
    pub schema: String,
    pub run_id: String,
    pub request_sha256: String,
    pub authorization_id: String,
    pub authorization_state_before: AuthorizationState,
    pub authorization_state_after: AuthorizationState,
    pub decision: String,
    pub deny_reason: Option<String>,
    pub actuation: ActuationOutcome,
    pub effect_present_after: bool,
    pub effect_sha256_after: Option<String>,
    pub producer_label: Option<String>,
    pub core_observation: Option<String>,
}

pub fn write_evidence(root: &Path, evidence: &ExecutionEvidence) -> Result<(), String> {
    let dir = root.join("evidence").join(&evidence.run_id);
    fs::create_dir_all(&dir).map_err(|e| format!("failed creating evidence directory: {e}"))?;

    let path = dir.join("summary.json");
    let bytes =
        serde_json::to_vec_pretty(evidence).map_err(|e| format!("evidence encode failed: {e}"))?;

    fs::write(&path, bytes).map_err(|e| format!("failed writing evidence {}: {e}", path.display()))
}
