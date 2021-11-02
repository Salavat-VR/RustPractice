use std::collections::hash_map::Entry;
use std::collections::HashMap;

use crate::traits::{Hashable, WorldState};
use crate::types::{Account, AccountId, AccountType, Block, Chain, Error, Hash, Transaction};

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


impl WorldState for Blockchain {
    fn create_account(&mut self, account_id: AccountId, account_type: AccountType) -> Result<(), Error> {
        match self.account.entry(account_id) {
            Entry::Occupied(_) => Err(format!("account with this account id already exists")),
            Entry::Vacant(v) => {
                v.insert(Account::new(account_type));
                Ok(())
            }
        }
    }

    fn get_account_by_id(&self, account_id: AccountId) -> Option<&Account> {
        self.account.get(&account_id)
    }

    fn get_account_by_id_mut(&mut self, account_id: AccountId) -> Option<&mut Account> {
        self.account.get_mut(&account_id)
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