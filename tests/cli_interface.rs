use serde_json::Value;
use std::fs;
use std::process::Command;
use tempfile::tempdir;

fn bin() -> &'static str {
    env!("CARGO_BIN_EXE_arcstone-exec")
}

fn run(args: &[&str]) -> std::process::Output {
    Command::new(bin())
        .args(args)
        .output()
        .expect("failed to run arcstone-exec")
}

#[test]
fn help_and_version_succeed() {
    let help = run(&["--help"]);
    assert!(help.status.success());

    let help_stdout = String::from_utf8_lossy(&help.stdout);
    assert!(help_stdout.contains("Arcstone Execution Boundary"));
    assert!(help_stdout.contains("arcstone-exec <command> [options]"));

    let version = run(&["--version"]);
    assert!(version.status.success());

    let version_stdout = String::from_utf8_lossy(&version.stdout);
    assert_eq!(
        version_stdout.trim(),
        format!("arcstone-exec {}", env!("CARGO_PKG_VERSION"))
    );
}

#[test]
fn command_help_succeeds_without_execution() {
    let root = tempdir().unwrap();
    let root_arg = root.path().to_string_lossy().to_string();

    let output = run(&["init", "--root", &root_arg, "--help"]);

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("arcstone-exec init --root <dir>"));
}

#[test]
fn malformed_options_fail_deterministically() {
    let root = tempdir().unwrap();
    let root_arg = root.path().to_string_lossy().to_string();

    let unknown = run(&["init", "--root", &root_arg, "--bogus", "value"]);
    assert_eq!(unknown.status.code(), Some(1));
    assert!(
        String::from_utf8_lossy(&unknown.stderr).contains("unknown option: --bogus")
    );

    let missing_value = run(&["init", "--root", "--bogus"]);
    assert_eq!(missing_value.status.code(), Some(1));
    assert!(
        String::from_utf8_lossy(&missing_value.stderr).contains("missing value for --root")
    );

    let duplicate = run(&["init", "--root", &root_arg, "--root", "OTHER"]);
    assert_eq!(duplicate.status.code(), Some(1));
    assert!(
        String::from_utf8_lossy(&duplicate.stderr).contains("duplicate option: --root")
    );
}

#[test]
fn inspect_absent_emits_single_json_object() {
    let root = tempdir().unwrap();
    let root_arg = root.path().to_string_lossy().to_string();

    let output = run(&[
        "inspect",
        "--root",
        &root_arg,
        "--auth-id",
        "AUTH-ABSENT",
    ]);

    assert!(output.status.success());

    let value: Value = serde_json::from_slice(&output.stdout).unwrap();

    assert_eq!(value["state"], "ABSENT");
    assert!(value["authorization"].is_null());
}

#[test]
fn issue_then_inspect_emits_machine_readable_json() {
    let root = tempdir().unwrap();
    let root_arg = root.path().to_string_lossy().to_string();

    let issue = run(&[
        "issue",
        "--root",
        &root_arg,
        "--auth-id",
        "AUTH-INTERFACE-001",
        "--action",
        "WRITE_PROTECTED_FILE",
        "--resource-id",
        "EFFECT_LOG",
        "--payload-hex",
        "48454C4C4F",
    ]);

    assert!(issue.status.success());

    let issued: Value = serde_json::from_slice(&issue.stdout).unwrap();

    assert_eq!(issued["authorization_id"], "AUTH-INTERFACE-001");
    assert_eq!(issued["action"], "WRITE_PROTECTED_FILE");
    assert_eq!(issued["resource_id"], "EFFECT_LOG");

    let inspect = run(&[
        "inspect",
        "--root",
        &root_arg,
        "--auth-id",
        "AUTH-INTERFACE-001",
    ]);

    assert!(inspect.status.success());

    let inspected: Value = serde_json::from_slice(&inspect.stdout).unwrap();

    assert_eq!(inspected["state"], "ISSUED");
    assert_eq!(
        inspected["authorization"]["authorization_id"],
        "AUTH-INTERFACE-001"
    );
}

#[test]
fn valid_deny_is_protocol_success_with_zero_exit_code() {
    let root = tempdir().unwrap();
    let root_arg = root.path().to_string_lossy().to_string();

    let request_path = root.path().join("request.json");

    fs::write(
        &request_path,
        r#"{
  "authorization_id": "AUTH-MISSING",
  "action": "WRITE_PROTECTED_FILE",
  "resource_id": "EFFECT_LOG",
  "payload_hex": "48454C4C4F"
}"#,
    )
    .unwrap();

    let request_arg = request_path.to_string_lossy().to_string();

    let output = run(&[
        "execute",
        "--root",
        &root_arg,
        "--request",
        &request_arg,
        "--run-id",
        "INTERFACE-DENY-001",
    ]);

    assert!(
        output.status.success(),
        "valid DENY result must remain a successful CLI execution"
    );

    let value: Value = serde_json::from_slice(&output.stdout).unwrap();

    assert_eq!(value["decision"], "DENY");
    assert_eq!(value["deny_reason"], "ABSENT_AUTHORIZATION");
    assert_eq!(value["actuation"], "NOT_ATTEMPTED");
    assert_eq!(value["effect_present_after"], false);
}