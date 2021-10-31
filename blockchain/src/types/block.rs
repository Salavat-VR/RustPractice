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



}

#[cfg(test)]
mod tests {
    use crate::types::TransactionData;
    use super::*;

    #[test]
    fn block_creation() {
        let mut block = Block::new(None);
        block.set_nonce(2);

        let mut tx = Transaction::new(TransactionData::CreateAccount("Dmytro    ".to_string()), None);


        block.add_transaction(tx);

        dbg!(block);
    }
}
