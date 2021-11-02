use blake2::{Blake2s, Digest};
use blake2::digest::FixedOutput;

use crate::traits::Hashable;
use crate::types::{AccountId, Balance, Hash, Timestamp};

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