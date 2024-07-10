//! Simple Merkle Tree data structure.
//!
//! Provides a Merkle Tree data structure built from a data slice.
//! The structure's abstraction is able to be easily navigated in a recursive way,
//! and it's able to provide inclusion proofs as Merkle Paths.

pub mod crypto;
pub mod merkle_node;
pub mod merkle_tree;
