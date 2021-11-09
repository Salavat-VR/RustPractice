extern crate ed25519;
extern crate ed25519_dalek;
extern crate rand;




use ed25519_dalek::Keypair;
use rand::rngs::OsRng;

use crate::types::Balance;


#[derive(Debug)]
pub struct Account {
    account_type: AccountType,
    pub(crate) balance: Balance,
    // we are verifying by public key and signing by private one
    pub(crate) private_key: ed25519_dalek::Keypair,
    pub(crate) public_key: ed25519_dalek::PublicKey,
}

#[derive(Debug, Clone)]
pub enum AccountType {
    User,
    Contract,
}

impl Account {
    pub fn new(account_type: AccountType) -> Account {

        let mut csprng = OsRng{};
        // creating each pair of private & public key for account whilst creating it
        let signing_key = Keypair::generate(&mut csprng);


        Account {
            account_type,
            balance: 0,
            private_key: signing_key.copy(),
            public_key: signing_key.public,
        }
    }

    pub fn get_balance(&self) -> Balance {
        self.balance
    }

    pub fn set_balance(&mut self, balance: u128) {
        self.balance = balance;
    }




}
