use crate::crypto::hash_combined;
use crate::crypto::hash_value;

use std::f64;

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

    pub fn get_hash_string(&self) -> String {
        String::from_utf8(self.hash.clone()).expect("Invalid utf8 bytes on hash")
    }
}

#[derive(Clone, Debug)]
pub struct MerkleTree {
    root: MerkleNode,
    data: Vec<u8>,
}

impl MerkleTree {
    /// Create a new MerkleTree from a &\[u8\]
    pub fn new(data: &[u8]) -> Self {
        MerkleTree {
            root: Self::create_root_node_from(data),
            data: data.to_vec(),
        }
    }

    /// Returns the root node of the tree as a MerkleNode struct.
    pub fn get_root_node(&self) -> MerkleNode {
        self.root.clone()
    }

    /// Returns the Root MerkleNode of a Merkle Tree made from a &\[u8\].
    fn create_root_node_from(data: &[u8]) -> MerkleNode {
        // Construct the tree one level at a time.
        // First level: constructing leaf MerkleNodes from data.
        let mut current_level = Self::create_leaves(&data);

        // Construct middle levels and finally root by combinig lower levels.
        while current_level.len() > 1 {
            current_level = Self::next_merkle_level(current_level);
        }

        current_level[0].clone()
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

    /// Given a value and it's position on the data, returns the sequence of hashes
    /// which concatenated and hashed with the original value and each resultant
    /// returns the Merkle Tree's root hash.
    pub fn inclusion_proof(&self, index: usize, value: u8) -> Option<Vec<Vec<u8>>> {
        if self.data.get(index) != Some(&value) {
            return None;
        } else {
            let tree_height = self.get_tree_height();
            return Some(Self::merkle_proof(
                &self.get_root_node(),
                index,
                tree_height,
            ));
        }
    }

    /// Recursive helper function to generate the inclusion proof:
    /// The proof that an element is in the tree is the proof that an element is
    /// on one of the sub-trees + the hash of that subtree's sibling.
    fn merkle_proof(root_node: &MerkleNode, index: usize, tree_height: usize) -> Vec<Vec<u8>> {
        match (&root_node.left, &root_node.right) {
            (Some(left), Some(right)) => {
                //Each level's size is divided in a power of 2 and the remainder on each level, depending
                //on which half the element we want to prove inclusion of is at we choose left or right children
                //of the current root and add that node's sibling to the proof.

                //let half_size = 1 << (tree_height -1)
                let half_size = 2_usize.pow((tree_height - 1) as u32);

                if index < half_size {
                    let mut proof = Self::merkle_proof(left, index, tree_height - 1);
                    proof.push(right.hash.clone());
                    return proof;
                } else {
                    let mut proof = Self::merkle_proof(right, index - tree_height, tree_height - 1);
                    proof.push(left.hash.clone());
                    return proof;
                }
            }
            _ => {
                vec![]
            }
        }
    }

    /// Returns the height of the tree. Because it's a full binary tree, the height is calculated
    /// by applying log2 to the ammount of leaves and ceiling the result.
    pub fn get_tree_height(&self) -> usize {
        (f64::from(self.data.len() as u32).log2().ceil()) as usize
    }
}

/// Function to verify a Merkle proof
pub fn verify_proof(index: usize, data: u8, proof: Vec<Vec<u8>>, root_hash: Vec<u8>) -> bool {
    let mut computed_hash = hash_value(&[data]);
    let mut current_index = index;
    for sibling_hash in proof.iter() {
        if current_index % 2 == 0 {
            computed_hash = hash_combined(&computed_hash, &sibling_hash);
        } else {
            computed_hash = hash_combined(&sibling_hash, &computed_hash);
        }
        current_index /= 2;
    }
    computed_hash == root_hash
}
