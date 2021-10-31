use blake2::{Blake2s, Digest};
use blake2::digest::FixedOutput;

use crate::types::{Hash, Transaction};

// реализуем дефолтные значения для нашей структуры
#[derive(Default, Debug)]
pub struct Block {
    nonce: u128,
    hash: Hash,
    prev_hash: Option<Hash>,
    transactions: Vec<Transaction>,
}


impl Block {
    pub fn new(prev_hash: Option<Hash>) -> Block {
        Block {
            prev_hash,
            // используем эти дефолтные значение для кокретных типов,
            // определенных нами
            ..Default::default()
        }
    }

    pub fn set_nonce(&mut self, nonce: u128) {
        self.nonce = nonce;
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
    }

    pub fn hash(&self) -> Hash {
        let mut hasher = Blake2s::new();

        hasher.update(format!("{:?}", (self.prev_hash.clone(), self.nonce)).as_bytes());

        hex::encode(hasher.finalize_fixed())
    }
}

#[cfg(test)]
mod tests {
    use crate::types::TransactionData;

    use super::*;

    #[test]
    #[ignore]
    fn block_creation() {
        let mut block = Block::new(None);
        block.set_nonce(2);

        let mut tx = Transaction::new(TransactionData::CreateAccount("Dmytro    ".to_string()), None);


        block.add_transaction(tx);

        dbg!(block);
    }


    #[test]
    fn test_hash() {
        let mut block = Block::new(None);
        block.set_nonce(2);

        let hash1 = block.hash();


        assert_eq!(hash1, String::from("96cc9f93fc5855cf0d4a31c095d7f77969e54f690dcad51c789f8a5499cba5da"))
    }
}
