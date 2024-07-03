extern crate sha2;

use sha2::{Digest, Sha256};

#[derive(Clone)]
struct MerkleNode {
    hash: Vec<u8>,
    left: Option<Box<MerkleNode>>,
    right: Option<Box<MerkleNode>>,
}

impl MerkleNode {
    fn new(hash: Vec<u8>) -> Self {
        MerkleNode {
            hash,
            left: None,
            right: None,
        }
    }
    fn combine(merkle_left: &MerkleNode, merkle_right: &MerkleNode) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(&merkle_left.hash);
        hasher.update(&merkle_right.hash);
        let hash = hasher.finalize().to_vec();
        MerkleNode {
            hash,
            left: Some(Box::new(merkle_left.clone())),
            right: Some(Box::new(merkle_right.clone())),
        }
    }
}

fn hash_data_bytes(data: &[u8]) {
    let mut data_vector = vec![];
    for &block in data.iter() {
        let mut hasher = Sha256::new();
        hasher.update(&[block]);
        let hashed_data_block = hasher.finalize().to_vec();
        data_vector.push(hashed_data_block);
    }

    println!("{:?}", data_vector[0].len())
}

fn main() {
    hash_data_bytes(&[1, 2, 3, 4])
}
