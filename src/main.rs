use hex::encode;
use merkle_tree::merkle::*;

fn main() {
    let merkle_tree = MerkleTree::new(&[1, 2, 3, 4, 5]);
    println!("{}", encode(merkle_tree.get_root_node().get_hash()));
    let proof = merkle_tree.inclusion_proof(1, 2);
    println!("{:?}", proof);
    println!(
        "{}",
        verify_proof(
            1,
            2,
            proof.expect("aaaa"),
            merkle_tree.get_root_node().get_hash()
        )
    );
}
