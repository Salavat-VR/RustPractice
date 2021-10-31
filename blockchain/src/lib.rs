use std::collections::HashMap;

pub struct Block {
    nonce: u128,
    hash: Hash,
    prev_hash: Hash,
    transactions: Vec<Transaction>,
}

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

pub struct Blockchain {
    blocks: Vec<Block>,
    account: HashMap<AccountId, Account>,
    transaction_pool: Vec<Transaction>,
}

pub struct Account {
    account_type: AccountType,
    balance: Balance,
}

pub enum AccountType {
    User,
    Contract,
}


pub type Hash = String;
pub type Timestamp = u128;
pub type AccountId = String;
pub type Balance = u128;
