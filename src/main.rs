extern crate sha2;

use sha2::{Digest, Sha256};

#[derive(Clone)]
struct MerkleNode {
    hash: Vec<u8>,
    left: Option<Box<MerkleNode>>,
    right: Option<Box<MerkleNode>>,
}

impl MerkleNode {
    fn new_leaf(data_block: &[u8]) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(data_block);
        let hash = hasher.finalize().to_vec();
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

fn new_merkle_tree(data: &[u8]) {
    let mut data_vector = vec![];
    for &block in data.iter() {
        data_vector.push(MerkleNode::new_leaf(&[block]));
    }
    if data_vector.len() % 2 == 1 {
        let last_element = data_vector[data_vector.len() - 1].clone();
        data_vector.push(last_element);
    }
}

fn main() {}
