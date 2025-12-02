//! Hash functions (SHA256, Keccak, RIPEMD160, Blake2b)

use blake2::{Blake2b512, Digest as Blake2Digest};
use ripemd::Ripemd160;
use sha2::{Digest, Sha256};
use tiny_keccak::{Hasher, Keccak};

/// Compute SHA256 hash
pub fn sha256(data: &[u8]) -> [u8; 32] {
    Sha256::digest(data).into()
}

/// Compute double SHA256 hash (SHA256(SHA256(data)))
pub fn double_sha256(data: &[u8]) -> [u8; 32] {
    sha256(&sha256(data))
}

/// Compute Keccak-256 hash
pub fn keccak256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak::v256();
    hasher.update(data);
    let mut hash = [0u8; 32];
    hasher.finalize(&mut hash);
    hash
}

/// Compute hash160: RIPEMD160(SHA256(data))
pub fn hash160(data: &[u8]) -> [u8; 20] {
    let sha256_hash = sha256(data);
    Ripemd160::digest(sha256_hash).into()
}

/// Compute Blake2b-512 hash and return first 32 bytes
/// Used for Substrate secp256k1 account ID derivation
pub fn blake2b_256(data: &[u8]) -> [u8; 32] {
    let hash = Blake2b512::digest(data);
    let mut result = [0u8; 32];
    result.copy_from_slice(&hash[..32]);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256() {
        let data = b"hello world";
        let hash = sha256(data);
        assert_eq!(hash.len(), 32);
        // Verify it's deterministic
        let hash2 = sha256(data);
        assert_eq!(hash, hash2);
    }

    #[test]
    fn test_sha256_empty() {
        let data = b"";
        let hash = sha256(data);
        assert_eq!(hash.len(), 32);
    }

    #[test]
    fn test_double_sha256() {
        let data = b"hello world";
        let hash = double_sha256(data);
        assert_eq!(hash.len(), 32);
        // Double SHA256 should be different from single SHA256
        let single_hash = sha256(data);
        assert_ne!(hash, single_hash);
    }

    #[test]
    fn test_double_sha256_deterministic() {
        let data = b"test data";
        let hash1 = double_sha256(data);
        let hash2 = double_sha256(data);
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_keccak256() {
        let data = b"hello world";
        let hash = keccak256(data);
        assert_eq!(hash.len(), 32);
        // Verify it's deterministic
        let hash2 = keccak256(data);
        assert_eq!(hash, hash2);
    }

    #[test]
    fn test_keccak256_different_from_sha256() {
        let data = b"test";
        let keccak_hash = keccak256(data);
        let sha256_hash = sha256(data);
        // Keccak and SHA256 should produce different hashes
        assert_ne!(keccak_hash, sha256_hash);
    }

    #[test]
    fn test_hash160() {
        let data = b"hello world";
        let hash = hash160(data);
        assert_eq!(hash.len(), 20);
        // Verify it's deterministic
        let hash2 = hash160(data);
        assert_eq!(hash, hash2);
    }

    #[test]
    fn test_hash160_different_from_sha256() {
        let data = b"test";
        let hash160_result = hash160(data);
        let sha256_hash = sha256(data);
        // hash160 is 20 bytes, SHA256 is 32 bytes
        assert_ne!(hash160_result.len(), sha256_hash.len());
    }

    #[test]
    fn test_hash160_empty() {
        let data = b"";
        let hash = hash160(data);
        assert_eq!(hash.len(), 20);
    }

    #[test]
    fn test_sha256_large_data() {
        let data = vec![0u8; 1000];
        let hash = sha256(&data);
        assert_eq!(hash.len(), 32);
    }

    #[test]
    fn test_keccak256_empty() {
        let data = b"";
        let hash = keccak256(data);
        assert_eq!(hash.len(), 32);
    }
}
