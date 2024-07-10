use hex::encode;
use merkle_tree::merkle_tree::*;

fn main() {
    let merkle_tree = MerkleTree::new(&["hola", "como", "estas"]);
    println!("{}", encode(merkle_tree.get_root_node().get_hash()));
    let proof = merkle_tree.inclusion_proof(1, "como");
    println!("{:?}", proof);
    println!(
        "{}",
        verify_proof(
            1,
            "como",
            proof.expect("aaaa"),
            merkle_tree.get_root_node().get_hash()
        )
    );
}
