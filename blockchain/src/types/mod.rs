// какой будет API - то что "покажем" юзерам
pub use account::{Account, AccountType};
pub use block::Block;
pub use blockchain::Blockchain;
pub use chain::Chain;
pub use transaction::{Transaction, TransactionData};

mod account;
mod block;
mod blockchain;
mod chain;
mod transaction;

pub type Hash = String;
pub type TransactionTimestamp = u128;
pub type AccountId = String;
pub type Balance = u128;
pub type Error = String;
