use std::collections::HashMap;

use crate::types::{Account, AccountId, Block, Transaction};

#[derive(Debug)]
pub struct Blockchain {
    blocks: Vec<Block>,
    account: HashMap<AccountId, Account>,
    transaction_pool: Vec<Transaction>,
}
