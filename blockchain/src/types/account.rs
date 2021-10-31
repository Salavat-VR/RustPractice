use crate::types::Balance;

#[derive(Debug)]
pub struct Account {
    account_type: AccountType,
    balance: Balance,
}

#[derive(Debug)]
pub enum AccountType {
    User,
    Contract,
}
