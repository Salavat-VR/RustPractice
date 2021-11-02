use crate::types::Balance;

#[derive(Debug, Clone)]
pub struct Account {
    account_type: AccountType,
    balance: Balance,
}

#[derive(Debug, Clone)]
pub enum AccountType {
    User,
    Contract,
}

impl Account {
    pub fn new(account_type: AccountType) -> Account {
        Account {
            account_type,
            balance: 0,
        }
    }
}

