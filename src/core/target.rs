use serde::{Deserialize, Serialize};
use crate::core::Algorithm;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Target {
    pub id: String,
    pub username: String,
    #[serde(rename = "hash_algo")]
    pub algorithm: Algorithm,
    #[serde(rename = "hash_hex")]
    pub hash: String,
    #[serde(default)]
    pub salt: String,
}

impl Target {
    /// check if a computed hash matches this target
    pub fn matches(&self, computed_hash: &[u8]) -> bool {
        let target_bytes = hex::decode(&self.hash).unwrap_or_default();
        computed_hash == target_bytes.as_slice()
    }
    
    /// get salt as bytes
    pub fn salt_bytes(&self) -> Vec<u8> {
        if self.salt.is_empty() {
            vec![]
        } else {
            self.salt.as_bytes().to_vec()
        }
    }
}

#[derive(Debug, Clone)]
pub struct TargetMatch {
    pub target_id: String,
    pub username: String,
    pub password: Vec<u8>,
    pub algorithm: Algorithm,
    pub guesses_tried: u64,
    pub time_seconds: f64,
}

impl TargetMatch {
    pub fn password_string(&self) -> String {
        String::from_utf8_lossy(&self.password).to_string()
    }
}