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

    const TARGET: &str = "0000000000000000000cfecf0000000000000000000000000000000000000000";

    let mut block = Block::new(bc.get_last_block_hash());
    let tx_create_account =
        Transaction::new(TransactionData::CreateAccount(generate_account_id()), None);
    block.add_transaction(tx_create_account);


    // it takes some time to execute
    while block.hash().to_string() >= TARGET.to_string() {
        block.set_nonce(rand::thread_rng().gen());
    }



    let block_clone = block.clone();

    assert!(bc.append_block(block).is_ok());

    block_clone

    /*

    засекаемя, затраченое на весь блок / 10*2016 и находим коефициент регуляции для таргета =k

    next_target = k * previous_target

   есть наш таргет = "-----------------------------------------------"

   перебираем nonce (рандомно) что бы хеш был < target     => только тогда дрбавляем


   # 2. Work out the ratio of the actual time against the expected time
    actual = last - first     # 1157929 (number of seconds between first and last block)
    expected = 2016 * 10 * 60 # 1209600 (number of seconds expected between 2016 blocks)
    ratio = actual.to_f / expected.to_f

    # 3. Limit the adjustment by a factor of 4 (to prevent massive changes from one target to the next)
    ratio = 0.25 if ratio < 0.25
    ratio = 4 if ratio > 4

    */



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
