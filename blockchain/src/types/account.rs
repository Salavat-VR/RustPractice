extern crate rand;
extern crate rand_core;
extern crate ed25519_dalek;
extern crate ed25519;


use crate::types::Balance;
use self::rand_core::OsRng;

#[derive(Debug, Clone)]
pub struct Account {
    account_type: AccountType,
    pub(crate) balance: Balance,
    // we are verifying by public key and signing by private one
    pub(crate) private_key: ed25519_dalek::SecretKey,
    pub(crate) public_key: ed25519_dalek::PublicKey,
}

#[derive(Debug, Clone)]
pub enum AccountType {
    User,
    Contract,
}

impl Account {
    pub fn new(account_type: AccountType) -> Account {
        // creating each pair of private & public key for account whilst creating it
        let signing_key = ed25519_dalek::Keypair::generate(&mut OsRng);

        Account {
            account_type,
            balance: 0,
            public_key: signing_key.public,
            private_key: signing_key.secret,
        }
    }

    pub fn get_balance(&self) -> Balance {
        self.balance
    }

    pub fn set_balance(&mut self, balance: u128) {
        self.balance = balance;
    }
}
