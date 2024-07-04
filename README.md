![license](https://img.shields.io/github/license/vicentevieytes/merkle-tree)

# merkle-tree
Rust implementation of a Merkle Tree data structure. 

A Merkle Tree is a data strucuture which provides efficient integrity verification for some stream of data. The data is divided into N blocks and a cryptographic hash hash_N for each block is computed, these are the "leaves" of tree.

Leaves are then grouped by two like this: `(leaf_0 leaf_1), (leaf_2, leaf_3) ... (leaf_N-1, leaf_N)`.

If the ammount of blocks (or leaves) is odd, then the last leaf doesn't have a pair so it's paired with itself `(leaf_N, leaf_N)`.

Then, the next level of the tree is constructed by computing `next_1 = hash(hash_I || hash_J) ... next_N/2 = (hash_N-1 || hash_N)`

This process is repeated for every level until you get one last hash, the root of the merkle tree.

Example:
```
Data = [A, B, C, D, E]  

hash\_1 = hash(hash(A) || hash(B))
hash\_2 = hash(hash(C) || hash(D))
hash\_3 = hash(hash(E) || hash(E))

hash\_4 = hash(hash\_1 || hash\_2)
hash\_5 = hash(hash\_3 || hash\_3)

hash\_6 = hash(hash\_4 || hash\_5) <-- The root of the Merkle Tree
```

Given the stream of data, and the root of it's merkle tree, it's easy to verify the integrity of the received data by doing this entire computation. Because the tree is binary, verification takes O(log(n)) steps.

If the verification fails, that means at least one of the blocks of data is corrupted. To find which one has the defect is also easy and efficient by asking one by one for every individual pair of hashes used to compute every level of the tree that does not match with the hash calculated by the verifier.
