use crate::core::{Algorithm, Target, hasher::*};
use rand::Rng;

/// generate demo targets from known passwords
pub fn generate_demo_targets(passwords: &[String], algorithms: &[Algorithm]) -> Vec<Target> {
    let mut targets = Vec::new();
    let mut rng = rand::thread_rng();
    
    for (idx, password) in passwords.iter().enumerate() {
        for algo in algorithms {
            let hasher = create_hasher(*algo);
            
            // optionally add salt for some targets
            let use_salt = rng.gen_bool(0.3);
            let salt = if use_salt {
                format!("salt{}", idx)
            } else {
                String::new()
            };
            
            let hash = if salt.is_empty() {
                hasher.hash(password.as_bytes())
            } else {
                hasher.hash_with_salt(password.as_bytes(), salt.as_bytes())
            };
            
            targets.push(Target {
                id: format!("demo{}_{}", idx, algo),
                username: format!("user{}", idx),
                algorithm: *algo,
                hash: hex::encode(hash),
                salt,
            });
        }
    }
    
    targets
}