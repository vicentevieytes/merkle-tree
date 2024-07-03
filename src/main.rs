extern crate sha2;

use sha2::{Digest, Sha256};

fn hash_data(data: &[u8]) {
    let mut data_vector = vec![];
    for &block in data.iter() {
        let mut hasher = Sha256::new();
        hasher.update(&[block]);
        let hashed_data_block = hasher.finalize().to_vec();
        data_vector.push(hashed_data_block);
    }

    println!("{:?}", data_vector[0].len())
}

fn main() {
    hash_data(&[1, 2, 3, 4])
}
