use std::collections::HashMap;

use crate::traits::Hashable;
use crate::types::{Account, AccountId, Block, Chain, Error, Hash, Transaction};

#[derive(Debug, Default)]
pub struct Blockchain {
    blocks: Chain<Block>,
    account: HashMap<AccountId, Account>,
    transaction_pool: Vec<Transaction>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain {
            ..Default::default()
        }
    }

    pub fn len(&self) -> usize {
        self.blocks.len()
    }

    pub fn append_block(&mut self, block: Block) -> Result<(), Error> {
        self.blocks.append(block);
        Ok(())
    }

    pub fn get_last_block_hash(&self) -> Option<Hash> {
        self.blocks.head().map(|block| { block.hash() })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bc() {
        let mut bc = Blockchain::new();
        let mut genesis_block = Block::new(None);
        genesis_block.set_nonce(2);

        let mut first_block = Block::new(Some(genesis_block.hash()));
        first_block.set_nonce(2);

        bc.append_block(genesis_block);
        bc.append_block(first_block);


        dbg!(bc);
    }
}