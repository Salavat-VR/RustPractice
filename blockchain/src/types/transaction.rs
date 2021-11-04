use std::ops::Deref;

use blake2::digest::FixedOutput;
use blake2::{Blake2s, Digest};

use crate::traits::{Hashable, WorldState};
use crate::types::{AccountId, AccountType, Balance, Error, Hash, Timestamp};

#[derive(Debug, Clone)]
pub struct Transaction {
    nonce: u128,
    pub(crate) from: Option<AccountId>,
    pub(crate) data: TransactionData,
    signature: Option<Hash>,
    timestamp: Timestamp,
}

#[derive(Debug, Clone)]
pub enum TransactionData {
    CreateAccount(AccountId),
    MintInitialSupply { to: AccountId, amount: Balance },
    Transfer { to: AccountId, amount: Balance },
}

impl Transaction {
    pub fn new(data: TransactionData, from: Option<AccountId>) -> Transaction {
        Transaction {
            nonce: 0,
            from,
            data,
            signature: None,
            timestamp: 0,
        }
    }

    pub fn execute<T: WorldState>(&self, state: &mut T, is_genesis: bool) -> Result<(), Error> {
        match &self.data {
            TransactionData::CreateAccount(account_id) => {
                state.create_account(account_id.clone(), AccountType::User)
            }

            TransactionData::MintInitialSupply { to, amount } => {
                if !is_genesis {
                    return Err(
                        "initial supply is allowed to execute only to genesis block".to_string()
                    );
                }
                match state.get_account_by_id_mut(to.clone()) {
                    Some(account) => {
                        account.balance += amount;
                        Ok(())
                    }
                    None => Err("there is no such account. check account id".to_string()),
                }
            }

            // TODO Task 1: Implement transfer transition function
            // 1. Check that receiver and sender accounts exist
            // 2. Check sender balance
            // 3. Change sender/receiver balances and save to state
            // 4. Test
            TransactionData::Transfer { to, amount } => {
                // checking the sender first bc it has to exist obviously,
                // if does => checking the receiver existence so that we can
                // do the transfer

                match &self.from {
                    None => Err("you can't send from None address".to_string()),

                    Some(sender) => {
                        let mut sender_balance =
                            state.get_account_by_id_mut(sender.clone()).unwrap().balance;
                        if sender_balance - amount < 0 {
                            return Err("insufficient funds".to_string());
                        } else {
                            match state.get_account_by_id_mut(to.clone()) {
                                None => Err("you are sending to non-existent account. Be careful"
                                    .to_string()),
                                Some(account) => {
                                    sender_balance -= amount;
                                    account.balance += amount;
                                    Ok(())
                                }
                            }
                        }
                    }
                }
            }
            _ => Err("unknown tx".to_string()),
        }
    }
}

impl Hashable for Transaction {
    fn hash(&self) -> Hash {
        let mut hasher = Blake2s::new();

        hasher.update(format!(
            "{:?}",
            (
                self.nonce,
                self.timestamp,
                self.from.clone(),
                self.data.clone()
            )
        ));

        hex::encode(hasher.finalize_fixed())
    }
}

#[cfg(test)]
mod tests {
    use crate::types::Blockchain;
    use crate::utils::append_block_with_tx;

    use super::*;

    #[test]
    fn error_sent_from_none_addrs() {
        let bc = &mut Blockchain::new();

        let tx_satoshi_account =
            Transaction::new(TransactionData::CreateAccount("satoshi".to_string()), None);

        let tx_other_account =
            Transaction::new(TransactionData::CreateAccount("salavat".to_string()), None);


        let tx_mint_initial_supply = Transaction::new(
            TransactionData::MintInitialSupply {
                to: "satoshi".to_string(),
                amount: 10_000,
            },
            None,
        );

        let transfer_tx = Transaction::new(
            TransactionData::Transfer {
                to: "salavat".to_string(),
                amount: 10_000,
            },
            None,
        );

        let transfer_tx = Transaction::new(
            TransactionData::Transfer {
                to: "salavat".to_string(),
                amount: 2_000,
            },
            Some("satoshi".to_string()),
        );

        assert!(append_block_with_tx(
            bc,
            2,
            vec![
                tx_satoshi_account,
                tx_other_account,
                tx_mint_initial_supply,
                transfer_tx,
            ],
        )
        .is_ok());

        dbg!(bc);

    }
}
