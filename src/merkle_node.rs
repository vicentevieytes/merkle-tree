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
    /// children's hashes and a reference to each child node.
    pub fn combine(merkle_left: &MerkleNode, merkle_right: &MerkleNode) -> Self {
        let hash = hash_combined(&merkle_left.hash, &merkle_right.hash);

        MerkleNode {
            hash,
            left: Some(Box::new(merkle_left.clone())),
            right: Some(Box::new(merkle_right.clone())),
        }
    }
    /// Returns the Root MerkleNode of a Merkle Tree made from a &\[u8\].
    pub fn root_node_from(data: &[u8]) -> MerkleNode {
        // Construct the tree one level at a time.
        // First level: constructing leaf MerkleNodes from data.
        let mut current_level = Self::create_leaves(&data);

        // Construct middle levels and finally root by combinig lower levels.
        while current_level.len() > 1 {
            current_level = Self::next_merkle_level(current_level);
        }

        current_level[0].clone()
    }
    pub fn get_hash(&self) -> Vec<u8> {
        self.hash.clone()
    }

    pub fn right(&self) -> Option<&MerkleNode> {
        self.right.as_deref()
    }

    pub fn left(&self) -> Option<&MerkleNode> {
        self.left.as_deref()
    }

    /// Return even length vector of leaf MerkleNodes from &\[u8\]
    fn create_leaves(data: &[u8]) -> Vec<MerkleNode> {
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

    /// From a vec<MerkleNode>, iterate by pairs
    /// and create the next level by concatenating the hashes and hashing again
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
