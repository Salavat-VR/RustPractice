pub use account::{Account, AccountType};
pub use block::Block;
pub use blockchain::Blockchain;
pub use transaction::{Transaction, TransactionData};

mod account;
mod block;
mod blockchain;
mod transaction;

pub type Hash = String;
pub type Timestamp = u128;
pub type AccountId = String;
pub type Balance = u128;