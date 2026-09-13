use arcstone_execution_boundary::{
    execute_request, issue_authorization, Decision, DenyReason, EvidenceContext, ExecutionRequest,
    ExperimentPaths, ACTION_WRITE_PROTECTED_FILE, RESOURCE_EFFECT_LOG,
};
use tempfile::tempdir;

#[test]
fn t6_replay_is_denied() {
    let dir = tempdir().unwrap();
    let paths = ExperimentPaths::new(dir.path());
    paths.init().unwrap();

    let payload = b"ONE";
    issue_authorization(
        dir.path(),
        "AUTH-R",
        ACTION_WRITE_PROTECTED_FILE,
        RESOURCE_EFFECT_LOG,
        payload,
    )
    .unwrap();

    let req = ExecutionRequest {
        authorization_id: "AUTH-R".into(),
        action: ACTION_WRITE_PROTECTED_FILE.into(),
        resource_id: RESOURCE_EFFECT_LOG.into(),
        payload_hex: hex::encode(payload),
    };

    let first = execute_request(&paths, &req, "t6-first", EvidenceContext::default()).unwrap();
    assert_eq!(first.decision, Decision::Allow);

    let second = execute_request(&paths, &req, "t6-replay", EvidenceContext::default()).unwrap();
    assert_eq!(second.decision, Decision::Deny);
    assert_eq!(second.deny_reason, Some(DenyReason::ConsumedAuthorization));
}
