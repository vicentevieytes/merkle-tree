extern crate sha2;

use sha2::{Digest, Sha256};

#[derive(Clone, Debug)]
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

fn new_merkle_tree(data: &[u8]) -> MerkleNode {
    // Construct the tree one level at a time.
    // First level: constructing leaf MerkleNodes from data.
    let mut current_level: Vec<MerkleNode> = vec![];
    for &block in data.iter() {
        current_level.push(MerkleNode::new_leaf(&[block]));
    }
    if current_level.len() % 2 == 1 {
        let last_element = current_level[current_level.len() - 1].clone();
        current_level.push(last_element);
    }
    // Construct middle levels and finally root by combinig lower levels.
    let mut next_level: Vec<MerkleNode> = vec![];
    while current_level.len() > 1 {
        let mut i = 0;
        while i < current_level.len() {
            let left_node = &current_level[i];
            let right_node = &current_level[i + 1];
            let new_node = MerkleNode::combine(left_node, right_node);
            next_level.push(new_node);
            i += 2;
        }
        // Make the next level of even length to repeat the process,
        // only if it is not the root level.
        if (next_level.len() != 1) && (next_level.len() % 2 == 1) {
            let last_element = next_level[next_level.len() - 1].clone();
            next_level.push(last_element);
        }
        current_level = next_level.clone();
        next_level = vec![];
    }
    return current_level[0].clone();
}

fn main() {
    let merkle_tree = new_merkle_tree(&[1, 2, 3, 4, 5]);
    println!("{:?}", merkle_tree.hash);
}
