// какой будет API - то что "покажем" юзерам
pub use account::{Account, AccountType};
pub use block::Block;
pub use blockchain::Blockchain;
pub use chain::Chain;
pub use transaction::{Transaction, TransactionData};
use crate::types::signature::{HelloSigner, HelloVerifier};

mod account;
mod block;
mod blockchain;
mod chain;
mod transaction;
mod signature;

pub type Hash = String;
pub type Timestamp = u128;
pub type AccountId = String;
pub type Balance = u128;
pub type Error = String;
pub type DalekHelloSigner = HelloSigner<ed25519_dalek::Keypair>;
pub type DalekHelloVerifier = HelloVerifier<ed25519_dalek::PublicKey>;