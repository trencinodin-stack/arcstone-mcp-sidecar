use arcstone_execution_boundary::{
    execute_request, issue_authorization, Decision, EvidenceContext, ExecutionRequest,
    ExperimentPaths, ACTION_WRITE_PROTECTED_FILE, RESOURCE_EFFECT_LOG,
};
use tempfile::tempdir;

fn make_req(id: &str, payload: &[u8]) -> ExecutionRequest {
    ExecutionRequest {
        authorization_id: id.into(),
        action: ACTION_WRITE_PROTECTED_FILE.into(),
        resource_id: RESOURCE_EFFECT_LOG.into(),
        payload_hex: hex::encode(payload),
    }
}

#[test]
fn t11_identity_metadata_does_not_change_decision() {
    let dir_a = tempdir().unwrap();
    let dir_b = tempdir().unwrap();

    for (dir, label) in [(&dir_a, "producer-a"), (&dir_b, "producer-b")] {
        let paths = ExperimentPaths::new(dir.path());
        paths.init().unwrap();
        issue_authorization(
            dir.path(),
            "AUTH-ID",
            ACTION_WRITE_PROTECTED_FILE,
            RESOURCE_EFFECT_LOG,
            b"SAME",
        )
        .unwrap();

        let result = execute_request(
            &paths,
            &make_req("AUTH-ID", b"SAME"),
            label,
            EvidenceContext {
                producer_label: Some(label.into()),
                core_observation: None,
            },
        )
        .unwrap();

        assert_eq!(result.decision, Decision::Allow);
    }
}

#[test]
fn t15_invalid_attempt_does_not_consume_valid_authorization() {
    let dir = tempdir().unwrap();
    let paths = ExperimentPaths::new(dir.path());
    paths.init().unwrap();

    issue_authorization(
        dir.path(),
        "AUTH-T15",
        ACTION_WRITE_PROTECTED_FILE,
        RESOURCE_EFFECT_LOG,
        b"RIGHT",
    )
    .unwrap();

    let bad = execute_request(
        &paths,
        &make_req("AUTH-T15", b"WRONG"),
        "t15-bad",
        EvidenceContext::default(),
    )
    .unwrap();
    assert_eq!(bad.decision, Decision::Deny);

    let good = execute_request(
        &paths,
        &make_req("AUTH-T15", b"RIGHT"),
        "t15-good",
        EvidenceContext::default(),
    )
    .unwrap();
    assert_eq!(good.decision, Decision::Allow);
}
