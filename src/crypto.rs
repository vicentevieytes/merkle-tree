extern crate sha2;
use sha2::{Digest, Sha256};

pub fn hash_combined(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(a);
    hasher.update(b);
    hasher.finalize().to_vec()
}

pub fn hash_value(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}
