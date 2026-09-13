use arcstone_execution_boundary::{
    execute_request, issue_authorization, AuthorizationState, EvidenceContext, ExecutionRequest,
    ExperimentPaths,
};
use arcstone_execution_boundary::authorization::load_authorization;
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    if let Err(e) = run() {
        eprintln!("ERROR: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(usage());
    }

    match args[1].as_str() {
        "init" => {
            let root = required_arg(&args, "--root")?;
            ExperimentPaths::new(root).init()?;
            println!("initialized");
        }
        "issue" => {
            let root = PathBuf::from(required_arg(&args, "--root")?);
            let auth_id = required_arg(&args, "--auth-id")?;
            let action = required_arg(&args, "--action")?;
            let resource_id = required_arg(&args, "--resource-id")?;
            let payload_hex = required_arg(&args, "--payload-hex")?;
            let payload =
                hex::decode(payload_hex).map_err(|e| format!("invalid --payload-hex: {e}"))?;

            ExperimentPaths::new(&root).init()?;
            let record =
                issue_authorization(&root, auth_id, action, resource_id, &payload)?;
            println!(
                "{}",
                serde_json::to_string_pretty(&record)
                    .map_err(|e| format!("encode output failed: {e}"))?
            );
        }
        "execute" => {
            let root = PathBuf::from(required_arg(&args, "--root")?);
            let request_path = PathBuf::from(required_arg(&args, "--request")?);
            let run_id = required_arg(&args, "--run-id")?;
            let producer_label = optional_arg(&args, "--producer").map(str::to_string);
            let core_observation = optional_arg(&args, "--core-observation").map(str::to_string);

            let request_bytes = fs::read(&request_path)
                .map_err(|e| format!("failed reading request {}: {e}", request_path.display()))?;
            let request: ExecutionRequest = serde_json::from_slice(&request_bytes)
                .map_err(|e| format!("malformed request JSON: {e}"))?;

            let result = execute_request(
                &ExperimentPaths::new(root),
                &request,
                run_id,
                EvidenceContext {
                    producer_label,
                    core_observation,
                },
            )?;

            println!(
                "{}",
                serde_json::to_string_pretty(&result)
                    .map_err(|e| format!("encode output failed: {e}"))?
            );
        }
        "inspect" => {
            let root = PathBuf::from(required_arg(&args, "--root")?);
            let auth_id = required_arg(&args, "--auth-id")?;
            let (state, record) = load_authorization(&root, auth_id)?;
            println!("state={:?}", state);
            match record {
                Some(r) => println!(
                    "{}",
                    serde_json::to_string_pretty(&r)
                        .map_err(|e| format!("encode output failed: {e}"))?
                ),
                None => {
                    if state == AuthorizationState::Absent {
                        println!("authorization=absent");
                    } else {
                        println!("authorization=unreadable");
                    }
                }
            }
        }
        _ => return Err(usage()),
    }

    Ok(())
}

fn required_arg<'a>(args: &'a [String], name: &str) -> Result<&'a str, String> {
    optional_arg(args, name).ok_or_else(|| format!("missing argument {name}\n\n{}", usage()))
}

fn optional_arg<'a>(args: &'a [String], name: &str) -> Option<&'a str> {
    args.iter()
        .position(|v| v == name)
        .and_then(|i| args.get(i + 1))
        .map(String::as_str)
}

fn usage() -> String {
    r#"Usage:
  arcstone-exec init --root <dir>
  arcstone-exec issue --root <dir> --auth-id <id> --action <action> --resource-id <id> --payload-hex <hex>
  arcstone-exec execute --root <dir> --request <request.json> --run-id <id> [--producer <label>] [--core-observation <text>]
  arcstone-exec inspect --root <dir> --auth-id <id>
"#
    .to_string()
}
