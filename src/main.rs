use merkle_tree::merkle::MerkleTree;

fn main() {
    let merkle_tree = MerkleTree::new(&[1, 2, 3, 4, 5]);
    println!("{:?}", merkle_tree.get_root());
}
