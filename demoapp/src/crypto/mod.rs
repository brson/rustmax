//! Cryptographic utilities using blake3 and sha2.

use rustmax::prelude::*;
use rustmax::blake3;
use rustmax::sha2::{Sha256, Sha512, Digest};

/// Hash data using BLAKE3 (fast, secure).
pub fn blake3_hash(data: &[u8]) -> String {
    blake3::hash(data).to_hex().to_string()
}

/// Hash a string using BLAKE3.
pub fn blake3_hash_str(s: &str) -> String {
    blake3_hash(s.as_bytes())
}

/// Hash data using SHA-256.
pub fn sha256_hash(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

/// Hash a string using SHA-256.
pub fn sha256_hash_str(s: &str) -> String {
    sha256_hash(s.as_bytes())
}

/// Hash data using SHA-512.
pub fn sha512_hash(data: &[u8]) -> String {
    let mut hasher = Sha512::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

/// Hash a string using SHA-512.
pub fn sha512_hash_str(s: &str) -> String {
    sha512_hash(s.as_bytes())
}

/// Hash a file using BLAKE3.
pub fn hash_file(path: &std::path::Path) -> crate::Result<String> {
    let data = std::fs::read(path)?;
    Ok(blake3_hash(&data))
}

/// Hash a file using SHA-256.
pub fn hash_file_sha256(path: &std::path::Path) -> crate::Result<String> {
    let data = std::fs::read(path)?;
    Ok(sha256_hash(&data))
}

/// Compute a short hash prefix for display.
pub fn short_hash(hash: &str, len: usize) -> &str {
    if hash.len() > len {
        &hash[..len]
    } else {
        hash
    }
}

/// Verify a hash matches expected value.
pub fn verify_hash(data: &[u8], expected: &str) -> bool {
    blake3_hash(data) == expected
}

/// Verify a SHA-256 hash matches expected value.
pub fn verify_sha256(data: &[u8], expected: &str) -> bool {
    sha256_hash(data) == expected
}

/// Hash algorithm selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HashAlgorithm {
    Blake3,
    Sha256,
    Sha512,
}

impl HashAlgorithm {
    /// Hash data using the selected algorithm.
    pub fn hash(&self, data: &[u8]) -> String {
        match self {
            HashAlgorithm::Blake3 => blake3_hash(data),
            HashAlgorithm::Sha256 => sha256_hash(data),
            HashAlgorithm::Sha512 => sha512_hash(data),
        }
    }

    /// Get the hash length in characters.
    pub fn hash_len(&self) -> usize {
        match self {
            HashAlgorithm::Blake3 => 64,
            HashAlgorithm::Sha256 => 64,
            HashAlgorithm::Sha512 => 128,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blake3_hash() {
        let hash = blake3_hash_str("hello");
        assert_eq!(hash.len(), 64);
        // BLAKE3 is deterministic.
        assert_eq!(blake3_hash_str("hello"), blake3_hash_str("hello"));
        assert_ne!(blake3_hash_str("hello"), blake3_hash_str("world"));
    }

    #[test]
    fn test_sha256_hash() {
        let hash = sha256_hash_str("hello");
        assert_eq!(hash.len(), 64);
        // SHA-256 of "hello" is well-known.
        assert_eq!(
            hash,
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
    }

    #[test]
    fn test_sha512_hash() {
        let hash = sha512_hash_str("hello");
        assert_eq!(hash.len(), 128);
    }

    #[test]
    fn test_short_hash() {
        let hash = "abcdef1234567890";
        assert_eq!(short_hash(hash, 8), "abcdef12");
        assert_eq!(short_hash(hash, 4), "abcd");
    }

    #[test]
    fn test_verify_hash() {
        let data = b"test data";
        let hash = blake3_hash(data);
        assert!(verify_hash(data, &hash));
        assert!(!verify_hash(b"wrong data", &hash));
    }

    #[test]
    fn test_verify_sha256() {
        let data = b"test data";
        let hash = sha256_hash(data);
        assert!(verify_sha256(data, &hash));
        assert!(!verify_sha256(b"wrong data", &hash));
    }

    #[test]
    fn test_hash_algorithm() {
        let data = b"test";
        let b3 = HashAlgorithm::Blake3.hash(data);
        let s256 = HashAlgorithm::Sha256.hash(data);
        let s512 = HashAlgorithm::Sha512.hash(data);

        assert_eq!(b3.len(), HashAlgorithm::Blake3.hash_len());
        assert_eq!(s256.len(), HashAlgorithm::Sha256.hash_len());
        assert_eq!(s512.len(), HashAlgorithm::Sha512.hash_len());
    }
}
