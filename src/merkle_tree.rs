use crate::crypto::hash_combined;
use crate::crypto::hash_value;
use crate::crypto::Hash;
use crate::merkle_node::MerkleNode;

use std::f64;

/// The MerkleTree data structure wraps a tree represented as a MerkleNode
/// and also keeps a copy of the data it's constructed out of.
/// It can provide inclusion proofs that a certain leaf exists in the tree.
#[derive(Clone, Debug)]
pub struct MerkleTree {
    root: MerkleNode,
    data: Vec<Vec<u8>>,
}

impl MerkleTree {
    /// Create a new MerkleTree from a &\[u8\]
    pub fn new<T: AsRef<[u8]>>(data: &[T]) -> Self {
        MerkleTree {
            root: MerkleNode::root_node_from(data),
            data: data.iter().map(|item| item.as_ref().to_vec()).collect(),
        }
    }

    pub fn root_hash(&self) -> Hash {
        self.root.get_hash()
    }

    /// Given a value and it's position on the data, returns a cryptographic inclusion proof.
    pub fn inclusion_proof<T: AsRef<[u8]>>(&self, index: usize, value: T) -> Option<Vec<Hash>> {
        if self.data.get(index) != Some(&value.as_ref().to_vec()) {
            return None;
        } else {
            let tree_height = self.get_tree_height();
            return Some(Self::merkle_proof(&self.root, index, tree_height));
        }
    }

    /// Recursive helper function to generate the inclusion proof:
    /// The proof that an element is in the tree is the proof that an element is
    /// on one of the sub-trees + the hash of that subtree's sibling.
    fn merkle_proof(root_node: &MerkleNode, index: usize, tree_height: usize) -> Vec<Hash> {
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

/// Verifies a Merkle proof that a certain leaf node has a specific value.
///
/// This function checks whether the provided data exists at the given index in the Merkle tree
/// with the specified root hash. It does so by constructing the Merkle path starting from the
/// hash of the provided data and processing each result by concatenating the next value of the
/// proof and taking the hash from that concatenation. If the final computed hash matches the
/// provided root hash, it proves that the data exists at the provided index in the Merkle tree.
///
/// # Arguments
///
/// * `index` - The index of the leaf node in the Merkle tree.
/// * `data` - The value of the data at the leaf node.
/// * `proof` - A vector of vectors of bytes representing the Merkle proof (hashes of sibling nodes).
/// * `root_hash` - A vector of bytes representing the root hash of the Merkle tree.
///
/// # Returns
///
/// A boolean value indicating whether the proof is valid. Returns `true` if the proof is valid,
/// `false` otherwise.
pub fn verify_proof<T: AsRef<[u8]>>(
    index: usize,
    data: T,
    proof: Vec<Hash>,
    root_hash: Hash,
) -> bool {
    let mut computed_hash = hash_value(data);
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
