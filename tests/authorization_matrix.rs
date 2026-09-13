use arcstone_execution_boundary::{
    execute_request, issue_authorization, Decision, DenyReason, EvidenceContext, ExecutionRequest,
    ExperimentPaths, ACTION_WRITE_PROTECTED_FILE, RESOURCE_EFFECT_LOG,
};
use tempfile::tempdir;

fn request(auth: &str, action: &str, resource: &str, payload: &[u8]) -> ExecutionRequest {
    ExecutionRequest {
        authorization_id: auth.into(),
        action: action.into(),
        resource_id: resource.into(),
        payload_hex: hex::encode(payload),
    }
}

#[test]
fn t0_valid_exact_authorization_succeeds_once() {
    let dir = tempdir().unwrap();
    let paths = ExperimentPaths::new(dir.path());
    paths.init().unwrap();

    issue_authorization(
        dir.path(),
        "AUTH-001",
        ACTION_WRITE_PROTECTED_FILE,
        RESOURCE_EFFECT_LOG,
        b"HELLO",
    )
    .unwrap();

    let result = execute_request(
        &paths,
        &request("AUTH-001", ACTION_WRITE_PROTECTED_FILE, RESOURCE_EFFECT_LOG, b"HELLO"),
        "t0",
        EvidenceContext::default(),
    )
    .unwrap();

    assert_eq!(result.decision, Decision::Allow);
    assert!(result.effect_present_after);
}

#[test]
fn t1_absent_authorization_denied() {
    let dir = tempdir().unwrap();
    let paths = ExperimentPaths::new(dir.path());

    let result = execute_request(
        &paths,
        &request("MISSING", ACTION_WRITE_PROTECTED_FILE, RESOURCE_EFFECT_LOG, b"HELLO"),
        "t1",
        EvidenceContext::default(),
    )
    .unwrap();

    assert_eq!(result.decision, Decision::Deny);
    assert_eq!(result.deny_reason, Some(DenyReason::AbsentAuthorization));
    assert!(!result.effect_present_after);
}

#[test]
fn t3_payload_mutation_denied_without_consuming() {
    let dir = tempdir().unwrap();
    let paths = ExperimentPaths::new(dir.path());
    paths.init().unwrap();

    issue_authorization(
        dir.path(),
        "AUTH-003",
        ACTION_WRITE_PROTECTED_FILE,
        RESOURCE_EFFECT_LOG,
        b"HELLO",
    )
    .unwrap();

    let result = execute_request(
        &paths,
        &request("AUTH-003", ACTION_WRITE_PROTECTED_FILE, RESOURCE_EFFECT_LOG, b"HELLX"),
        "t3",
        EvidenceContext::default(),
    )
    .unwrap();

    assert_eq!(result.decision, Decision::Deny);
    assert_eq!(result.deny_reason, Some(DenyReason::PayloadMismatch));
}

#[test]
fn t4_resource_substitution_denied() {
    let dir = tempdir().unwrap();
    let paths = ExperimentPaths::new(dir.path());
    paths.init().unwrap();

    issue_authorization(
        dir.path(),
        "AUTH-004",
        ACTION_WRITE_PROTECTED_FILE,
        RESOURCE_EFFECT_LOG,
        b"HELLO",
    )
    .unwrap();

    let result = execute_request(
        &paths,
        &request("AUTH-004", ACTION_WRITE_PROTECTED_FILE, "OTHER_RESOURCE", b"HELLO"),
        "t4",
        EvidenceContext::default(),
    )
    .unwrap();

    assert_eq!(result.decision, Decision::Deny);
    assert_eq!(result.deny_reason, Some(DenyReason::ResourceMismatch));
}

#[test]
fn t5_action_substitution_denied() {
    let dir = tempdir().unwrap();
    let paths = ExperimentPaths::new(dir.path());
    paths.init().unwrap();

    issue_authorization(
        dir.path(),
        "AUTH-005",
        ACTION_WRITE_PROTECTED_FILE,
        RESOURCE_EFFECT_LOG,
        b"HELLO",
    )
    .unwrap();

    let result = execute_request(
        &paths,
        &request("AUTH-005", "DELETE_PROTECTED_FILE", RESOURCE_EFFECT_LOG, b"HELLO"),
        "t5",
        EvidenceContext::default(),
    )
    .unwrap();

    assert_eq!(result.decision, Decision::Deny);
    assert_eq!(result.deny_reason, Some(DenyReason::ActionMismatch));
}
