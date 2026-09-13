use arcstone_execution_boundary::{
    execute_request, issue_authorization, Decision, DenyReason, EvidenceContext, ExecutionRequest,
    ExperimentPaths, ACTION_WRITE_PROTECTED_FILE, RESOURCE_EFFECT_LOG,
};
use arcstone_execution_boundary::authorization::load_authorization;
use tempfile::tempdir;

#[test]
fn t10_issued_authorization_alone_does_not_actuate() {
    let dir = tempdir().unwrap();
    let paths = ExperimentPaths::new(dir.path());
    paths.init().unwrap();

    issue_authorization(
        dir.path(),
        "AUTH-STALE",
        ACTION_WRITE_PROTECTED_FILE,
        RESOURCE_EFFECT_LOG,
        b"STALE",
    )
    .unwrap();

    assert!(!paths.protected_target().exists());
    let (state, _) = load_authorization(dir.path(), "AUTH-STALE").unwrap();
    assert_eq!(format!("{state:?}"), "Issued");
}

#[test]
fn t14_malformed_payload_hex_fails_closed() {
    let dir = tempdir().unwrap();
    let paths = ExperimentPaths::new(dir.path());
    paths.init().unwrap();

    issue_authorization(
        dir.path(),
        "AUTH-MAL",
        ACTION_WRITE_PROTECTED_FILE,
        RESOURCE_EFFECT_LOG,
        b"X",
    )
    .unwrap();

    let req = ExecutionRequest {
        authorization_id: "AUTH-MAL".into(),
        action: ACTION_WRITE_PROTECTED_FILE.into(),
        resource_id: RESOURCE_EFFECT_LOG.into(),
        payload_hex: "not-hex".into(),
    };

    let result =
        execute_request(&paths, &req, "t14", EvidenceContext::default()).unwrap();

    assert_eq!(result.decision, Decision::Deny);
    assert_eq!(result.deny_reason, Some(DenyReason::MalformedRequest));
    assert!(!result.effect_present_after);
}
