extern crate ed25519;
extern crate ed25519_dalek;
extern crate rand;


// use ed25519_dalek::Keypair;

use crate::types::Balance;

#[derive(Debug,Clone)]
pub struct Account {
    account_type: AccountType,
    pub(crate) balance: Balance,
    //pub(crate) private_key: ed25519_dalek::SecretKey,
    //pub(crate) public_key: ed25519_dalek::PublicKey,
}

#[derive(Debug, Clone)]
pub enum AccountType {
    User,
    Contract,
}

impl Account {
    pub fn new(account_type: AccountType) -> Account {

        //let key_pair = Keypair::generate(&mut rand::rngs::OsRng{});


        Account {
            account_type,
            balance: 0,
            //private_key:  key_pair.secret,
            //public_key: key_pair.public
        }
    }

    pub fn get_balance(&self) -> Balance {
        self.balance
    }

    pub fn set_balance(&mut self, balance: u128) {
        self.balance = balance;
    }




}
