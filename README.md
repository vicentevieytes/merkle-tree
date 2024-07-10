![license](https://img.shields.io/github/license/vicentevieytes/merkle-tree)

# Merkle Tree
Rust crate implementing a Merkle Tree data structure. 

A Merkle Tree is a data strucuture which provides efficient integrity verification for some stream of data. The data is divided into N blocks and a cryptographic hash hash_N for each block is computed, these are the "leaves" of tree.

Leaves are then grouped by two like this: `(leaf_0 leaf_1), (leaf_2, leaf_3) ... (leaf_N-1, leaf_N)`.

If the ammount of blocks (or leaves) is odd, then the last leaf doesn't have a pair so it's paired with itself `(leaf_N, leaf_N)`.

Then, the next level of the tree is constructed by computing `next_1 = hash(hash_I || hash_J) ... next_N/2 = (hash_N-1 || hash_N)`

This process is repeated for every level until you get one last hash, the root of the merkle tree.

![image](https://github.com/vicentevieytes/merkle-tree/assets/73846744/cc18346a-775f-44ad-b38f-442445301d9c)
Image from [wikimedia](https://upload.wikimedia.org/wikipedia/commons/9/95/Hash_Tree.svg)

## Example
```rust
let data = &["hola", "como", "estas", "bro", "pana"];
let tree = MerkleTree::new(data);
println!("Your MerkleTree root: {}", hex::encode(merkle_tree.get_root_node().get_hash()));
```
```
Your MerkleTree root: 4fc38c5af479e7f303421cbd1cf88b6f020dd984068c7945a95d2353789310ab
```

# Data integrity verification

If we receive a data stream from a server, and the root of the Merkle Tree computed from this data, it's easy to verify the data integrity by constructing the tree from the data and comparing the obtained root with the one provieded. To figure out which of the blocks have damaged data, one can ask for the hashes utilized to compute each hash, tracing the ones that don't match you can reach the damaged data blocks.

# Providing data inclusion proofs

Given a value of data and it's position on the data, a Merkle Tree provides a way to proove that this value is in that position.

By following the path from the root to the leaf, and providing the value of the hash of each sibling node utilized through that path, if you trust in the the root value of the tree you can verify the inclusion of the element by reconstructing the root. The proover could only possibly know the value of each pre-image needed to calculate every hash and finally the root hash if that data block is actually a part of the tree.

![image](https://github.com/vicentevieytes/merkle-tree/assets/73846744/79244787-8286-475c-9062-4d6c0ff1fd2c)

Image taken from [research gate](https://www.researchgate.net/figure/A-Merkle-Path-used-to-prove-inclusion-of-a-data-element_fig6_361381853).

In this image, to proove the inclusion of H\_k, the proof would be \[H\_l, H\_ij, H_\mnop, H\_abcdefgh\], if the verifier obtains the root of the tree by accumulating the result of concatenating and hashing accordingly each of these values, then it can be sure of the inclusion of the data it asked for.

## Example

```rust
use hex::encode;
use merkle_tree::merkle_tree::*;

fn main() {
    let merkle_tree = MerkleTree::new(&["hola", "como", "estas", "bro", "pana"]);
    println!("Your Merkle Tree root: {}", encode(merkle_tree.get_root_node().get_hash()));
    if let Some(proof) = merkle_tree.inclusion_proof(1, "como") {
        println!(
            "Inclusion proof: {:?}",
            proof.iter().map(|elem| encode(elem)).collect::<Vec<_>>()
        );

        println!(
            "Proof verified: {}",
            verify_proof(1, "como", proof, merkle_tree.get_root_node().get_hash())
        );
    }
}
```
```
Your Merkle Tree root: 4fc38c5af479e7f303421cbd1cf88b6f020dd984068c7945a95d2353789310ab
Inclusion proof: ["b221d9dbb083a7f33428d7c2a3c3198ae925614d70210e28716ccaa7cd4ddb79", "281915a418ffe2d7b6f1a02f519349c5272e3eaccfba828c968cb160ae110c47", "d600a1c363ac8f1ad9bceaaee4be42ee7e9eb3443f652b3da14c6da432b0c1e2"]
Proof verified: true
```
