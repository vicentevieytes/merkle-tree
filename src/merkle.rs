extern crate sha2;

use sha2::{Digest, Sha256};

#[derive(Clone, Debug)]
pub struct MerkleNode {
    hash: Vec<u8>,
    left: Option<Box<MerkleNode>>,
    right: Option<Box<MerkleNode>>,
}

impl MerkleNode {
    pub fn new_leaf(data_block: &[u8]) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(data_block);
        let hash = hasher.finalize().to_vec();

        MerkleNode {
            hash,
            left: None,
            right: None,
        }
    }

    pub fn combine(merkle_left: &MerkleNode, merkle_right: &MerkleNode) -> Self {
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

pub struct MerkleTree {
    root: MerkleNode,
    data: Vec<u8>,
}

impl MerkleTree {
    pub fn new(data: &[u8]) -> Self {
        MerkleTree {
            root: Self::create_from(data),
            data: data.to_vec(),
        }
    }

    // Return the Root MerkleNode from data.
    fn create_from(data: &[u8]) -> MerkleNode {
        // Construct the tree one level at a time.
        // First level: constructing leaf MerkleNodes from data.
        let mut current_level = Self::get_leaves(&data);

        // Construct middle levels and finally root by combinig lower levels.
        while current_level.len() > 1 {
            current_level = Self::next_merkle_level(current_level);
        }

        current_level[0].clone()
    }

    // Return even length vector of leaf MerkleNodes from &[u8]
    fn get_leaves(data: &[u8]) -> Vec<MerkleNode> {
        let mut leaves: Vec<MerkleNode> = vec![];
        for &block in data.iter() {
            leaves.push(MerkleNode::new_leaf(&[block]));
        }
        // Leaves must be of even length
        if leaves.len() % 2 == 1 {
            let last_element = leaves[leaves.len() - 1].clone();
            leaves.push(last_element);
        }
        leaves
    }

    /// Returns the root node of the tree as a MerkleNode struct.

    pub fn get_root(&self) -> MerkleNode {
        self.root.clone()
    }
    // From a vec<MerkleNode>, iterate by pairs
    // and create the next level by concatenating the hashes and hashing again
    fn next_merkle_level(current_level: Vec<MerkleNode>) -> Vec<MerkleNode> {
        let mut next_level: Vec<MerkleNode> = vec![];
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
        next_level
    }
}
