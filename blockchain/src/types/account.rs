use rand::rngs::OsRng;

use crate::types::{Balance, DalekHelloSigner, DalekHelloVerifier};

#[derive(Debug, Clone)]
pub struct Account {
    account_type: AccountType,
    pub(crate) balance: Balance,
    // we are verifying by public key and signing by private one
    pub(crate) public_key: DalekHelloVerifier,
    pub(crate) private_key: DalekHelloSigner,
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
            public_key: DalekHelloVerifier,
            private_key: d25519_dalek::Keypair::generate(&mut OsRng),
        }
    }

    pub fn get_balance(&self) -> Balance {
        self.balance
    }

    pub fn set_balance(&mut self, balance: u128) {
        self.balance = balance;
    }
}
