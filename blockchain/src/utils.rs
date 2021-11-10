use blake2::{Blake2s, Digest};
use rand::Rng;

use crate::traits::Hashable;
use crate::types::{AccountId, Block, Blockchain, Error, Transaction, TransactionData};

pub fn generate_account_id() -> AccountId {
    let mut rng = rand::thread_rng();
    let seed: u128 = rng.gen();
    hex::encode(Blake2s::digest(&seed.to_be_bytes()))
}

pub fn append_block(bc: &mut Blockchain) -> Block {
    // have time specified in seconds whilst create a block
    let mut block = Block::new(bc.get_last_block_hash());

    let tx_create_account =
        Transaction::new(TransactionData::CreateAccount(generate_account_id()), None);
    block.add_transaction(tx_create_account);

    // it takes some time to execute
    while block.hash().to_string() >= bc.target.to_string() {
        block.set_nonce(rand::thread_rng().gen());
    }

    // estimate time that target matching took

    let actual = time::now().to_timespec().sec - block.timestamp; // number of seconds between block
                                                                  // creating and guessing the nonce to match the target

    let expected = 1 * 60 * 10 as i64; // 1 epoch = 1 block = 10 min => number of seconds expected to create one block

    let mut ratio = actual / expected;

    // adjusting ratio
    if ratio > 4 {
        ratio = 4;
    } else if ratio < 0.25 as i64 {
        ratio = 0.25 as i64;
    }

    bc.adjust_target(ratio as f32);

    let block_clone = block.clone();

    assert!(bc.append_block(block).is_ok());

    block_clone

}

pub fn append_block_with_tx(
    bc: &mut Blockchain,
    nonce: u128,
    transactions: Vec<Transaction>,
) -> Result<(), Error> {
    let mut block = Block::new(bc.get_last_block_hash());
    block.set_nonce(nonce);

    for tx in transactions {
        block.add_transaction(tx);
    }

    bc.append_block(block)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate() {
        println!("{}", generate_account_id())
    }
}
