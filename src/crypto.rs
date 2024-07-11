extern crate sha2;
use sha2::{Digest, Sha256};

pub type Hash = [u8; 32];

/// Concatenates two hashes and returns the hash of the result.
pub fn hash_combined(a: &Hash, b: &Hash) -> Hash {
    let mut hasher = Sha256::new();
    hasher.update(a);
    hasher.update(b);
    hasher.finalize().into()
}

/// Returns the hash digest of a single data slice.
pub fn hash_value<T: AsRef<[u8]>>(data: T) -> Hash {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}
