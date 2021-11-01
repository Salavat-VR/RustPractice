use std::collections::HashMap;

use crate::types::{Account, AccountId, Block, Chain, Transaction};

#[derive(Debug)]
pub struct Blockchain {
    blocks: Chain<Block>,
    account: HashMap<AccountId, Account>,
    transaction_pool: Vec<Transaction>,
}
