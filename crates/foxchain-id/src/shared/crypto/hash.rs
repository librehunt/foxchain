//! Hash functions (SHA256, Keccak, RIPEMD160, Blake2b)

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
