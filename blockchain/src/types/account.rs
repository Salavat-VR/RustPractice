use crate::types::Balance;

pub struct Account {
    account_type: AccountType,
    balance: Balance,
}

pub enum AccountType {
    User,
    Contract,
}
