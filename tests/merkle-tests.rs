use merkle_tree::crypto::*;
use merkle_tree::merkle::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_even_merkle_tree() {
        let data = &[1, 2, 3, 4];
        let tree = MerkleTree::new(data);

        let left_combined = hash_combined(&hash_value(&[1]), &hash_value(&[2]));
        let right_combined = hash_combined(&hash_value(&[3]), &hash_value(&[4]));
        assert_eq!(
            hash_combined(&left_combined, &right_combined),
            tree.get_root_node().get_hash()
        )
    }

    #[test]
    fn test_from_odd_two_level_merkle_tree() {
        let data = &[1, 2, 3, 4, 5];
        let tree = MerkleTree::new(data);

        let left_combined = hash_combined(&hash_value(&[1]), &hash_value(&[2]));
        let middle_combined = hash_combined(&hash_value(&[3]), &hash_value(&[4]));
        let right_combined = hash_combined(&hash_value(&[5]), &hash_value(&[5]));

        let level_2_left = hash_combined(&left_combined, &middle_combined);
        let level_2_right = hash_combined(&right_combined, &right_combined);
        assert_eq!(
            hash_combined(&level_2_left, &level_2_right),
            tree.get_root_node().get_hash()
        )
    }

    #[test]
    fn test_generate_proof_correctly() {
        let data = &[1, 2, 3, 4, 5];
        let tree = MerkleTree::new(data);

        let left_combined = hash_combined(&hash_value(&[1]), &hash_value(&[2]));
        let middle_combined = hash_combined(&hash_value(&[3]), &hash_value(&[4]));
        let right_combined = hash_combined(&hash_value(&[5]), &hash_value(&[5]));

        let _level_2_left = hash_combined(&left_combined, &middle_combined);
        let level_2_right = hash_combined(&right_combined, &right_combined);

        //proof that the value 3 is at index 2
        let proof = tree.inclusion_proof(2, 3);
        assert_eq!(
            proof.expect("Proof is None"),
            vec![hash_value(&[4]), left_combined, level_2_right]
        )
    }

    #[test]
    fn test_verify_proof_correctly() {
        let data = &[1, 2, 3, 4, 5];
        let tree = MerkleTree::new(data);
        let proof = tree.inclusion_proof(2, 3);
        assert!(verify_proof(
            2,
            3,
            proof.expect("Proof is None"),
            tree.get_root_node().get_hash()
        ));
    }

    #[test]
    fn test_verify_incorrect_proof_returns_false() {
        let data = &[1, 2, 3, 4, 5];
        let tree = MerkleTree::new(data);
        let proof = tree.inclusion_proof(2, 3);
        assert_eq!(
            verify_proof(
                3,
                2,
                proof.expect("Proof is None"),
                tree.get_root_node().get_hash()
            ),
            false
        );
    }
}
