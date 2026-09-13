use crate::authorization::{issued_path, AuthorizationRecord};
use crate::request::sha256_hex;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

pub fn issue_authorization(
    root: &Path,
    auth_id: &str,
    action: &str,
    resource_id: &str,
    payload: &[u8],
) -> Result<AuthorizationRecord, String> {
    fs::create_dir_all(root.join("auth").join("issued"))
        .map_err(|e| format!("failed to create issued directory: {e}"))?;
    fs::create_dir_all(root.join("auth").join("consumed"))
        .map_err(|e| format!("failed to create consumed directory: {e}"))?;

    let record = AuthorizationRecord {
        authorization_id: auth_id.to_string(),
        action: action.to_string(),
        resource_id: resource_id.to_string(),
        payload_sha256: sha256_hex(payload),
    };

    let path = issued_path(root, auth_id);
    let bytes =
        serde_json::to_vec_pretty(&record).map_err(|e| format!("authorization encode failed: {e}"))?;

    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)
        .map_err(|e| format!("authorization already exists or cannot be created: {e}"))?;
    file.write_all(&bytes)
        .map_err(|e| format!("failed writing authorization: {e}"))?;
    file.sync_all()
        .map_err(|e| format!("failed syncing authorization: {e}"))?;

    Ok(record)
}
