use arcstone_execution_boundary::{
    execute_request, Decision, DenyReason, EvidenceContext, ExecutionRequest, ExperimentPaths,
    ACTION_WRITE_PROTECTED_FILE, RESOURCE_EFFECT_LOG,
};
use std::fs;
use tempfile::tempdir;

#[test]
fn t2_invalid_authorization_record_is_denied() {
    let dir = tempdir().unwrap();
    let paths = ExperimentPaths::new(dir.path());
    paths.init().unwrap();

    fs::write(
        dir.path().join("auth").join("issued").join("AUTH-BAD.json"),
        b"{ definitely not valid json",
    )
    .unwrap();

    let req = ExecutionRequest {
        authorization_id: "AUTH-BAD".into(),
        action: ACTION_WRITE_PROTECTED_FILE.into(),
        resource_id: RESOURCE_EFFECT_LOG.into(),
        payload_hex: hex::encode(b"X"),
    };

    let result =
        execute_request(&paths, &req, "t2", EvidenceContext::default()).unwrap();

    assert_eq!(result.decision, Decision::Deny);
    assert_eq!(result.deny_reason, Some(DenyReason::InvalidAuthorization));
    assert!(!result.effect_present_after);
}
