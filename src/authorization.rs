use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AuthorizationRecord {
    pub authorization_id: String,
    pub action: String,
    pub resource_id: String,
    pub payload_sha256: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AuthorizationState {
    Absent,
    Invalid,
    Issued,
    Consumed,
}

pub fn issued_path(root: &Path, auth_id: &str) -> PathBuf {
    root.join("auth")
        .join("issued")
        .join(format!("{auth_id}.json"))
}

pub fn claim_path(root: &Path, auth_id: &str) -> PathBuf {
    root.join("auth")
        .join("consumed")
        .join(format!("{auth_id}.claim"))
}

pub fn load_authorization(
    root: &Path,
    auth_id: &str,
) -> Result<(AuthorizationState, Option<AuthorizationRecord>), String> {
    let issued = issued_path(root, auth_id);
    if !issued.exists() {
        return Ok((AuthorizationState::Absent, None));
    }

    let bytes = fs::read(&issued)
        .map_err(|e| format!("failed to read authorization {}: {e}", issued.display()))?;

    let record: AuthorizationRecord = match serde_json::from_slice(&bytes) {
        Ok(v) => v,
        Err(_) => return Ok((AuthorizationState::Invalid, None)),
    };

    if record.authorization_id != auth_id {
        return Ok((AuthorizationState::Invalid, Some(record)));
    }

    if claim_path(root, auth_id).exists() {
        Ok((AuthorizationState::Consumed, Some(record)))
    } else {
        Ok((AuthorizationState::Issued, Some(record)))
    }
}

pub fn claim_authorization(
    root: &Path,
    auth_id: &str,
    request_sha256: &str,
) -> Result<(), String> {
    let path = claim_path(root, auth_id);
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)
        .map_err(|e| format!("authorization claim failed for {}: {e}", path.display()))?;

    writeln!(file, "authorization_id={auth_id}")
        .map_err(|e| format!("failed writing claim marker: {e}"))?;
    writeln!(file, "request_sha256={request_sha256}")
        .map_err(|e| format!("failed writing claim marker: {e}"))?;
    file.sync_all()
        .map_err(|e| format!("failed syncing claim marker: {e}"))?;

    Ok(())
}
