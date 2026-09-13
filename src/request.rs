use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const ACTION_WRITE_PROTECTED_FILE: &str = "WRITE_PROTECTED_FILE";
pub const RESOURCE_EFFECT_LOG: &str = "EFFECT_LOG";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExecutionRequest {
    pub authorization_id: String,
    pub action: String,
    pub resource_id: String,
    pub payload_hex: String,
}

impl ExecutionRequest {
    pub fn payload_bytes(&self) -> Result<Vec<u8>, String> {
        hex::decode(&self.payload_hex).map_err(|e| format!("invalid payload_hex: {e}"))
    }

    pub fn payload_sha256(&self) -> Result<String, String> {
        Ok(sha256_hex(&self.payload_bytes()?))
    }

    pub fn request_sha256(&self) -> Result<String, String> {
        let encoded =
            serde_json::to_vec(self).map_err(|e| format!("request serialization failed: {e}"))?;
        Ok(sha256_hex(&encoded))
    }
}

pub fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hex::encode(hasher.finalize())
}
