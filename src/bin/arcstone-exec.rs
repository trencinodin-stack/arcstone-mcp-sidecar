use arcstone_execution_boundary::authorization::load_authorization;
use arcstone_execution_boundary::{
    execute_request, issue_authorization, AuthorizationRecord, AuthorizationState, EvidenceContext,
    ExecutionRequest, ExperimentPaths,
};
use serde::Serialize;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    match run() {
        Ok(()) => {}
        Err(CliError::Help(text)) => {
            print!("{text}");
        }
        Err(CliError::Failure(message)) => {
            eprintln!("ERROR: {message}");
            std::process::exit(1);
        }
    }
}

enum CliError {
    Help(String),
    Failure(String),
}

#[derive(Serialize)]
struct InspectOutput {
    state: AuthorizationState,
    authorization: Option<AuthorizationRecord>,
}

struct ParsedOptions {
    values: HashMap<String, String>,
}

impl ParsedOptions {
    fn required(&self, name: &str) -> Result<&str, CliError> {
        self.values
            .get(name)
            .map(String::as_str)
            .ok_or_else(|| CliError::Failure(format!("missing argument {name}")))
    }

    fn optional(&self, name: &str) -> Option<&str> {
        self.values.get(name).map(String::as_str)
    }
}

impl From<String> for CliError {
    fn from(value: String) -> Self {
        Self::Failure(value)
    }
}

fn run() -> Result<(), CliError> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(CliError::Failure(usage()));
    }

    match args[1].as_str() {
        "--help" | "-h" => {
            return Err(CliError::Help(usage()));
        }
        "--version" | "-V" => {
            if args.len() != 2 {
                return Err(CliError::Failure(format!(
                    "unexpected argument after {}\n\n{}",
                    args[1],
                    usage()
                )));
            }

            println!("arcstone-exec {VERSION}");
            return Ok(());
        }
        _ => {}
    }

    let command = args[1].as_str();

    if !matches!(command, "init" | "issue" | "execute" | "inspect") {
        return Err(CliError::Failure(format!(
            "unknown command: {command}\n\n{}",
            usage()
        )));
    }

    if args[2..].iter().any(|arg| arg == "--help" || arg == "-h") {
        return Err(CliError::Help(command_usage(command)));
    }

    match command {
        "init" => {
            let options = parse_options(command, &args[2..], &["--root"], &["--root"])?;

            let root = options.required("--root")?;

            ExperimentPaths::new(root).init()?;
            println!("initialized");
        }

        "issue" => {
            let options = parse_options(
                command,
                &args[2..],
                &[
                    "--root",
                    "--auth-id",
                    "--action",
                    "--resource-id",
                    "--payload-hex",
                ],
                &[
                    "--root",
                    "--auth-id",
                    "--action",
                    "--resource-id",
                    "--payload-hex",
                ],
            )?;

            let root = PathBuf::from(options.required("--root")?);
            let auth_id = options.required("--auth-id")?;
            let action = options.required("--action")?;
            let resource_id = options.required("--resource-id")?;
            let payload_hex = options.required("--payload-hex")?;

            let payload =
                hex::decode(payload_hex).map_err(|e| format!("invalid --payload-hex: {e}"))?;

            ExperimentPaths::new(&root).init()?;

            let record = issue_authorization(&root, auth_id, action, resource_id, &payload)?;

            println!(
                "{}",
                serde_json::to_string_pretty(&record)
                    .map_err(|e| format!("encode output failed: {e}"))?
            );
        }

        "execute" => {
            let options = parse_options(
                command,
                &args[2..],
                &[
                    "--root",
                    "--request",
                    "--run-id",
                    "--producer",
                    "--core-observation",
                ],
                &["--root", "--request", "--run-id"],
            )?;

            let root = PathBuf::from(options.required("--root")?);
            let request_path = PathBuf::from(options.required("--request")?);
            let run_id = options.required("--run-id")?;

            let producer_label = options.optional("--producer").map(str::to_string);
            let core_observation = options.optional("--core-observation").map(str::to_string);

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
            let options = parse_options(
                command,
                &args[2..],
                &["--root", "--auth-id"],
                &["--root", "--auth-id"],
            )?;

            let root = PathBuf::from(options.required("--root")?);
            let auth_id = options.required("--auth-id")?;

            let (state, record) = load_authorization(&root, auth_id)?;

            let output = InspectOutput {
                state,
                authorization: record,
            };

            println!(
                "{}",
                serde_json::to_string_pretty(&output)
                    .map_err(|e| format!("encode output failed: {e}"))?
            );
        }

        _ => unreachable!(),
    }

    Ok(())
}

fn parse_options(
    command: &str,
    args: &[String],
    allowed: &[&str],
    required: &[&str],
) -> Result<ParsedOptions, CliError> {
    let mut values = HashMap::new();
    let mut index = 0;

    while index < args.len() {
        let option = args[index].as_str();

        if !option.starts_with('-') {
            return Err(CliError::Failure(format!(
                "unexpected positional argument: {option}\n\n{}",
                command_usage(command)
            )));
        }

        if !allowed.contains(&option) {
            return Err(CliError::Failure(format!(
                "unknown option: {option}\n\n{}",
                command_usage(command)
            )));
        }

        if values.contains_key(option) {
            return Err(CliError::Failure(format!(
                "duplicate option: {option}\n\n{}",
                command_usage(command)
            )));
        }

        let value = args.get(index + 1).ok_or_else(|| {
            CliError::Failure(format!(
                "missing value for {option}\n\n{}",
                command_usage(command)
            ))
        })?;

        if value.starts_with('-') {
            return Err(CliError::Failure(format!(
                "missing value for {option}\n\n{}",
                command_usage(command)
            )));
        }

        values.insert(option.to_string(), value.to_string());
        index += 2;
    }

    for name in required {
        if !values.contains_key(*name) {
            return Err(CliError::Failure(format!(
                "missing argument {name}\n\n{}",
                command_usage(command)
            )));
        }
    }

    Ok(ParsedOptions { values })
}

fn usage() -> String {
    format!(
        r#"Arcstone Execution Boundary

Usage:
  arcstone-exec <command> [options]

Commands:
  init      Initialize an experiment root
  issue     Issue a single-use authorization
  execute   Evaluate and execute a request
  inspect   Inspect authorization state

Options:
  -h, --help       Print help
  -V, --version    Print version

Version:
  {VERSION}
"#
    )
}

fn command_usage(command: &str) -> String {
    match command {
        "init" => r#"Usage:
  arcstone-exec init --root <dir>

Options:
  --root <dir>    Experiment root directory
  -h, --help      Print help
"#
        .to_string(),

        "issue" => r#"Usage:
  arcstone-exec issue --root <dir> --auth-id <id> --action <action> --resource-id <id> --payload-hex <hex>

Options:
  --root <dir>          Experiment root directory
  --auth-id <id>        Authorization identifier
  --action <action>     Authorized action
  --resource-id <id>    Authorized resource identifier
  --payload-hex <hex>   Payload encoded as hexadecimal
  -h, --help            Print help
"#
        .to_string(),

        "execute" => r#"Usage:
  arcstone-exec execute --root <dir> --request <request.json> --run-id <id> [--producer <label>] [--core-observation <text>]

Options:
  --root <dir>               Experiment root directory
  --request <request.json>   Execution request JSON
  --run-id <id>              Evidence run identifier
  --producer <label>         Optional producer metadata
  --core-observation <text>  Optional core-observation metadata
  -h, --help                 Print help
"#
        .to_string(),

        "inspect" => r#"Usage:
  arcstone-exec inspect --root <dir> --auth-id <id>

Options:
  --root <dir>      Experiment root directory
  --auth-id <id>   Authorization identifier
  -h, --help        Print help
"#
        .to_string(),

        _ => usage(),
    }
}
