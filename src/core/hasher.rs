use md5::{Md5, Digest as Md5Digest};
use sha1::{Sha1, Digest as Sha1Digest};
use sha2::{Sha256, Digest as Sha256Digest};
use serde::{Deserialize, Serialize};

use super::blitzhash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Algorithm {
    BlitzHash,  // custom ultra-fast hash (demo only - not cryptographically secure)
    Md5,
    Sha1,
    Sha256,
    Md4,  // for ntlm hashes
}

impl std::fmt::Display for Algorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Algorithm::BlitzHash => write!(f, "blitzhash"),
            Algorithm::Md5 => write!(f, "md5"),
            Algorithm::Sha1 => write!(f, "sha1"),
            Algorithm::Sha256 => write!(f, "sha256"),
            Algorithm::Md4 => write!(f, "md4"),
        }
    }
}

impl std::str::FromStr for Algorithm {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "blitzhash" | "blitz" => Ok(Algorithm::BlitzHash),
            "md5" => Ok(Algorithm::Md5),
            "sha1" => Ok(Algorithm::Sha1),
            "sha256" => Ok(Algorithm::Sha256),
            "md4" => Ok(Algorithm::Md4),
            _ => Err(format!("unknown algorithm: {}", s)),
        }
    }
}

pub trait Hasher: Send + Sync {
    fn hash(&self, input: &[u8]) -> Vec<u8>;
    fn hash_with_salt(&self, password: &[u8], salt: &[u8]) -> Vec<u8>;
    fn algorithm(&self) -> Algorithm;
}

// blitzhash hasher - custom ultra-fast algorithm
pub struct BlitzHasher;

impl Hasher for BlitzHasher {
    #[inline(always)]
    fn hash(&self, input: &[u8]) -> Vec<u8> {
        blitzhash::blitz_hash(0, input).to_vec()
    }
    
    #[inline(always)]
    fn hash_with_salt(&self, password: &[u8], salt: &[u8]) -> Vec<u8> {
        // combine salt and password
        let mut combined = Vec::with_capacity(salt.len() + password.len());
        combined.extend_from_slice(salt);
        combined.extend_from_slice(password);
        blitzhash::blitz_hash(0, &combined).to_vec()
    }
    
    fn algorithm(&self) -> Algorithm {
        Algorithm::BlitzHash
    }
}

// md5 hasher
pub struct Md5Hasher;

impl Hasher for Md5Hasher {
    #[inline(always)]
    fn hash(&self, input: &[u8]) -> Vec<u8> {
        let mut hasher = Md5::new();
        hasher.update(input);
        hasher.finalize().to_vec()
    }
    
    #[inline(always)]
    fn hash_with_salt(&self, password: &[u8], salt: &[u8]) -> Vec<u8> {
        let mut hasher = Md5::new();
        hasher.update(salt);
        hasher.update(password);
        hasher.finalize().to_vec()
    }
    
    fn algorithm(&self) -> Algorithm {
        Algorithm::Md5
    }
}

// sha1 hasher
pub struct Sha1Hasher;

impl Hasher for Sha1Hasher {
    #[inline(always)]
    fn hash(&self, input: &[u8]) -> Vec<u8> {
        let mut hasher = Sha1::new();
        hasher.update(input);
        hasher.finalize().to_vec()
    }
    
    #[inline(always)]
    fn hash_with_salt(&self, password: &[u8], salt: &[u8]) -> Vec<u8> {
        let mut hasher = Sha1::new();
        hasher.update(salt);
        hasher.update(password);
        hasher.finalize().to_vec()
    }
    
    fn algorithm(&self) -> Algorithm {
        Algorithm::Sha1
    }
}

// sha256 hasher
pub struct Sha256Hasher;

impl Hasher for Sha256Hasher {
    #[inline(always)]
    fn hash(&self, input: &[u8]) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(input);
        hasher.finalize().to_vec()
    }
    
    #[inline(always)]
    fn hash_with_salt(&self, password: &[u8], salt: &[u8]) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(salt);
        hasher.update(password);
        hasher.finalize().to_vec()
    }
    
    fn algorithm(&self) -> Algorithm {
        Algorithm::Sha256
    }
}

// md4 hasher (simplified - would need proper md4 crate in production)
pub struct Md4Hasher;

impl Hasher for Md4Hasher {
    #[inline(always)]
    fn hash(&self, input: &[u8]) -> Vec<u8> {
        // note: using md5 as placeholder - real implementation needs md4 crate
        let mut hasher = Md5::new();
        hasher.update(input);
        hasher.finalize().to_vec()
    }
    
    #[inline(always)]
    fn hash_with_salt(&self, password: &[u8], salt: &[u8]) -> Vec<u8> {
        let mut hasher = Md5::new();
        hasher.update(salt);
        hasher.update(password);
        hasher.finalize().to_vec()
    }
    
    fn algorithm(&self) -> Algorithm {
        Algorithm::Md4
    }
}

/// create hasher for algorithm
pub fn create_hasher(algorithm: Algorithm) -> Box<dyn Hasher> {
    match algorithm {
        Algorithm::BlitzHash => Box::new(BlitzHasher),
        Algorithm::Md5 => Box::new(Md5Hasher),
        Algorithm::Sha1 => Box::new(Sha1Hasher),
        Algorithm::Sha256 => Box::new(Sha256Hasher),
        Algorithm::Md4 => Box::new(Md4Hasher),
    }
}