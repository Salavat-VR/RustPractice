use crate::types::Balance;

#[derive(Debug, Clone)]
pub struct Account {
    account_type: AccountType,
    pub(crate) balance: Balance,
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

    pub fn get_balance(&self) -> Balance {
        self.balance
    }

    pub fn set_balance(&mut self, balance: u128) {
        self.balance = balance;
    }
}
