use std::collections::hash_map::Entry;
use std::collections::HashMap;

use crate::traits::{Hashable, WorldState};
use crate::types::{Account, AccountId, AccountType, Block, Chain, Error, Hash, Transaction};

#[derive(Debug, Default)]
pub struct Blockchain {
    blocks: Chain<Block>,
    accounts: HashMap<AccountId, Account>,
    transaction_pool: Vec<Transaction>,
    pub(crate) target: Hash,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain {
            // first target, which will be adjusting in process
            target: "0000000000000000000cfecf0000000000000000000000000000000000000000".to_string(),
            ..Default::default()
        }
    }

    pub fn len(&self) -> usize {
        self.blocks.len()
    }
    pub fn adjust_target(&mut self, ratio: f32) {
        // converting the target which is hexadecimal number, then multiplying by ratio
        // and finally convert back to hexadecimal (String)
        self.target = format!("{:x}", u32::from_str_radix(&*self.target.to_owned(), 16).unwrap() * ratio as u32);
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
            if let Err(_error) = result {
                // бэкап транзакций в  случае если случилась какая то ошибка
                self.accounts = accounts_backup;

                return Err(format!("tx didn't execute, something went wrong"));
            }
        }

        self.blocks.append(block);

        Ok(())
    }

    pub fn get_last_block_hash(&self) -> Option<Hash> {
        self.blocks.head().map(|block| block.hash())
    }

    pub fn validate(&self) -> Result<(), Error> {
        let mut block_num = self.blocks.len();
        let mut prev_block_hash: Option<Hash> = None;

        for block in self.blocks.iter() {
            let is_genesis = block_num == 1;

            if !block.verify() {
                return Err(format!("block {} has invalid hash", block_num));
            }

            if !is_genesis && block.prev_hash.is_none() {
                return Err(format!("block {} doesnt have prev hash", block_num));
            }

            if is_genesis && block.prev_hash.is_some() {
                return Err("genesis block shouldn't have prev hash".to_string());
            }

            if block_num != self.blocks.len() {
                if let Some(prev_block_hash) = &prev_block_hash {
                    if prev_block_hash != &prev_block_hash.clone() {
                        return Err(format!(
                            "block {} prev_hash doesnt match block {} hash ",
                            block_num + 1,
                            block_num
                        ));
                    }
                }
            }

            prev_block_hash = block.prev_hash.clone();
            block_num -= 1;
        }

        Ok(())
    }
}

impl WorldState for Blockchain {
    fn create_account(
        &mut self,
        account_id: AccountId,
        account_type: AccountType,
    ) -> Result<(), Error> {
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
    use crate::types::TransactionData;
    use crate::utils::{append_block, append_block_with_tx};

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

    #[test]
    fn satoshi_test() {
        let _bc = Blockchain::new();

        let tx_cr_account =
            Transaction::new(TransactionData::CreateAccount("amI".to_string()), None);

        let tx_mint_initial_supply = Transaction::new(
            TransactionData::MintInitialSupply {
                to: "amI".to_string(),
                amount: 100,
            },
            None,
        );

        let mut block = Block::new(None);
        block.set_nonce(2);
        block.add_transaction(tx_cr_account);
        block.add_transaction(tx_mint_initial_supply);

        dbg!(block);
    }

    #[test]
    fn test_validation_process() {
        let bc = &mut Blockchain::new();

        let tx_create_account =
            Transaction::new(TransactionData::CreateAccount("satoshi".to_string()), None);
        let tx_mint_initial_supply = Transaction::new(
            TransactionData::MintInitialSupply {
                to: "satoshi".to_string(),
                amount: 100_000_000,
            },
            None,
        );
        assert!(
            append_block_with_tx(bc, 1, vec![tx_create_account, tx_mint_initial_supply]).is_ok()
        );

        append_block(bc);
        append_block(bc);

        let mut iter = bc.blocks.iter_mut();
        iter.next();
        iter.next();
        let block = iter.next().unwrap();
        block.transactions[1].data = TransactionData::MintInitialSupply {
            to: "satoshi".to_string(),
            amount: 100,
        };

        assert!(bc.validate().is_err());
    }
}
