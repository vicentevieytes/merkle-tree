use crate::crypto::hash_combined;
use crate::crypto::hash_value;
use crate::merkle_node::MerkleNode;

use std::f64;

/// The MerkleTree data structure wraps a tree represented as a MerkleNode
/// and also keeps a copy of the data it's constructed out of.
#[derive(Clone, Debug)]
pub struct MerkleTree {
    root: MerkleNode,
    data: Vec<u8>,
}

impl MerkleTree {
    /// Create a new MerkleTree from a &\[u8\]
    pub fn new(data: &[u8]) -> Self {
        MerkleTree {
            root: MerkleNode::root_node_from(data),
            data: data.to_vec(),
        }
    }

    /// Returns the root node of the tree as a MerkleNode struct.
    pub fn get_root_node(&self) -> &MerkleNode {
        &self.root
    }

    /// Given a value and it's position on the data, returns a cryptographic inclusion proof.
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
        match (root_node.left(), root_node.right()) {
            (Some(left), Some(right)) => {
                //Each level's size is divided in a power of 2 and the remainder on each level, depending
                //on which half the element we want to prove inclusion of is at we choose left or right children
                //of the current root and add that node's sibling to the proof.

                //let half_size = 1 << (tree_height -1)
                let half_size = 2_usize.pow((tree_height - 1) as u32);
                if index < half_size {
                    let mut proof = Self::merkle_proof(left, index, tree_height - 1);
                    proof.push(right.get_hash());
                    return proof;
                } else {
                    let mut proof = Self::merkle_proof(right, index - tree_height, tree_height - 1);
                    proof.push(left.get_hash());
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

/// Function to verify a Merkle proof that a certain leaf has a certain value
/// The verifier constructs the merkle path starting from the hash of the provided data,
/// and processing each resultant by concatenating the next value of the proof and taking the hash
/// from that concatenation.
/// If the result at the end is the root_hash provided, then it's proof that the data exists
/// at the provided index on the merkle tree with that root hash value.
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
