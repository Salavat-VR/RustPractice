use blake2::digest::FixedOutput;
use blake2::{Blake2s, Digest};

use crate::traits::{Hashable, WorldState};
use crate::types::{AccountId, AccountType, Balance, Error, Hash, Timestamp};

#[derive(Debug)]
pub struct Transaction {
    nonce: u128,
    from: Option<AccountId>,
    data: TransactionData,
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
