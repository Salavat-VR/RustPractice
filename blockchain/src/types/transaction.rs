use crate::types::{AccountId, Balance, Hash, Timestamp};

pub struct Transaction {
    nonce: u128,
    from: AccountId,
    data: TransactionData,
    signature: Option<Hash>,
    timestamp: Timestamp,
}

pub enum TransactionData {
    CreateAccount(AccountId),
    MintInitialSupply { to: AccountId, amount: Balance },
    Transfer { to: AccountId, amount: Balance },
}