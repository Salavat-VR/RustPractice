use blake2::{Blake2s, Digest};
use rand::Rng;

use crate::types::AccountId;

pub fn generate_account_id() -> AccountId {
    let mut rng = rand::thread_rng();
    let seed: u128 = rng.gen();
    hex::encode(Blake2s::digest(&seed.to_be_bytes()))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate() {

        println!("{}",generate_account_id())

    }
}