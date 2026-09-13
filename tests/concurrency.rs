use arcstone_execution_boundary::{
    execute_request, issue_authorization, Decision, EvidenceContext, ExecutionRequest,
    ExperimentPaths, ACTION_WRITE_PROTECTED_FILE, RESOURCE_EFFECT_LOG,
};
use std::sync::{Arc, Barrier};
use std::thread;
use tempfile::tempdir;

#[test]
fn t17_double_submit_has_at_most_one_allow() {
    let dir = tempdir().unwrap();
    let root = dir.path().to_path_buf();
    let paths = ExperimentPaths::new(&root);
    paths.init().unwrap();

    issue_authorization(
        &root,
        "AUTH-RACE",
        ACTION_WRITE_PROTECTED_FILE,
        RESOURCE_EFFECT_LOG,
        b"RACE",
    )
    .unwrap();

    let req = ExecutionRequest {
        authorization_id: "AUTH-RACE".into(),
        action: ACTION_WRITE_PROTECTED_FILE.into(),
        resource_id: RESOURCE_EFFECT_LOG.into(),
        payload_hex: hex::encode(b"RACE"),
    };

    let barrier = Arc::new(Barrier::new(3));
    let mut handles = Vec::new();

    for i in 0..2 {
        let barrier = barrier.clone();
        let req = req.clone();
        let root = root.clone();

        handles.push(thread::spawn(move || {
            barrier.wait();
            execute_request(
                &ExperimentPaths::new(root),
                &req,
                &format!("t17-{i}"),
                EvidenceContext::default(),
            )
            .unwrap()
        }));
    }

    barrier.wait();

    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    let allows = results
        .iter()
        .filter(|r| r.decision == Decision::Allow)
        .count();

    assert_eq!(allows, 1);
}
