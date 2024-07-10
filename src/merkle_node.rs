use crate::crypto::hash_combined;
use crate::crypto::hash_value;

/// The MerkleNode data structure represents the root of a binary MerkleTree
/// A MerkleNode without any children is called a "leaf".
/// A MerkleNode is composed of it's hash value and a reference to each of it's children.
#[derive(Clone, Debug)]
pub struct MerkleNode {
    hash: Vec<u8>,
    left: Option<Box<MerkleNode>>,
    right: Option<Box<MerkleNode>>,
}

impl MerkleNode {
    /// Create a new instance without any children
    pub fn new_leaf(data_block: &[u8]) -> Self {
        let hash = hash_value(data_block);
        MerkleNode {
            hash,
            left: None,
            right: None,
        }
    }
    /// Create a new instance from two child nodes. Store the hash of the concatenation of the two
    /// children's hashes and a reference to each child node.k
    pub fn combine(merkle_left: &MerkleNode, merkle_right: &MerkleNode) -> Self {
        let hash = hash_combined(&merkle_left.hash, &merkle_right.hash);

        MerkleNode {
            hash,
            left: Some(Box::new(merkle_left.clone())),
            right: Some(Box::new(merkle_right.clone())),
        }
    }

    pub fn get_hash(&self) -> Vec<u8> {
        self.hash.clone()
    }
}
