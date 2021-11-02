use std::collections::hash_map::Entry;
use std::collections::HashMap;

use crate::traits::{Hashable, WorldState};
use crate::types::{Account, AccountId, AccountType, Block, Chain, Error, Hash, Transaction};

#[derive(Debug, Default)]
pub struct Blockchain {
    blocks: Chain<Block>,
    accounts: HashMap<AccountId, Account>,
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
        if !block.verify() {
            return Err("block has invalid hash".to_string());
        }

        let is_genesis = self.blocks.len() == 0;

        if block.transactions.len() == 0 {
            return Err("block has zero transactions".to_string());
        }

        let accounts_backup = self.accounts.clone();


        for tx in &block.transactions {
            let result = tx.execute(self, is_genesis);
            if let Err(error) = result {
                // бэкап транзакций в случае если случилась какая то ошибка
                self.accounts = accounts_backup;

                return Err(format!("tx didn't execute, something went wrong"));
            }
        }

        self.blocks.append(block);

        Ok(())
    }


    pub fn get_last_block_hash(&self) -> Option<Hash> {
        self.blocks.head().map(|block| { block.hash() })
    }
}


impl WorldState for Blockchain {
    fn create_account(&mut self, account_id: AccountId, account_type: AccountType) -> Result<(), Error> {
        match self.accounts.entry(account_id) {
            Entry::Occupied(_) => Err(format!("account with this account id already exists")),
            Entry::Vacant(v) => {
                v.insert(Account::new(account_type));
                Ok(())
            }
        }
    }

    fn get_account_by_id(&self, account_id: AccountId) -> Option<&Account> {
        self.accounts.get(&account_id)
    }

    fn get_account_by_id_mut(&mut self, account_id: AccountId) -> Option<&mut Account> {
        self.accounts.get_mut(&account_id)
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