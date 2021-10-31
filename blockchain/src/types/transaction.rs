use crate::types::{AccountId, Balance, Hash, Timestamp};

#[derive(Debug)]
pub struct Transaction {
    nonce: u128,
    from: Option<AccountId>,
    data: TransactionData,
    signature: Option<Hash>,
    timestamp: Timestamp,
}

#[derive(Debug)]
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

