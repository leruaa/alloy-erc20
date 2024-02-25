use alloy_sol_types::sol;

mod cachable_token_client;
mod constants;
mod error;
mod token;
mod token_client;
mod token_id;

pub mod stores;

pub use cachable_token_client::CachableTokenClient;
pub use constants::*;
pub use error::Error;
pub use token::Token;
pub use token_client::TokenClient;
pub use token_id::TokenId;

sol!(ERC20Contract, "abi/erc20.json");
