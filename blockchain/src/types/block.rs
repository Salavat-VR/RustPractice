use blake2::{Blake2s, Digest};
use blake2::digest::FixedOutput;

use crate::traits::Hashable;
use crate::types::{Hash, Transaction};

// реализуем дефолтные значения для нашей структуры
#[derive(Default, Debug)]
pub struct Block {
    nonce: u128,
    pub(crate) hash: Option<Hash>,
    prev_hash: Option<Hash>,
    transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(prev_hash: Option<Hash>) -> Block {
        let mut block = Block {
            prev_hash,
            // используем эти дефолтные значение для кокретных типов,
            // определенных нами
            ..Default::default()
        };

        block.update_hash();

        block
    }

    pub fn set_nonce(&mut self, nonce: u128) {
        self.nonce = nonce;
        self.update_hash();
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
        self.update_hash();
    }

    fn update_hash(&mut self) {
        self.hash = Some(self.hash())
    }

    pub fn verify(&self) -> bool {
        matches!(&self.hash, Some(hash) if hash == self.hash())
    }


}

impl Hashable for Block {
    fn hash(&self) -> Hash {
        let mut hasher = Blake2s::new();
        hasher.update(format!("{:?}", (self.prev_hash.clone(), self.nonce)).as_bytes());

        for tx in self.transactions.iter() {
            hasher.update(tx.hash())
        }

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

        let mut tx = Transaction::new(TransactionData::CreateAccount("Dmytro".to_string()), None);

        block.add_transaction(tx);

        dbg!(block);
    }

    #[test]
    fn test_hash() {
        let mut block = Block::new(None);
        block.set_nonce(2);

        let hash1 = block.hash();

        block.set_nonce(3);
        let hash2 = block.hash();

        assert_ne!(hash1, hash2)
    }
}
