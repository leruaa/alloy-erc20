use alloy_sol_types::sol;

mod constants;
mod error;
mod token;
mod token_id;

pub mod clients;
pub mod stores;
pub mod util;

pub use constants::*;
pub use error::Error;
pub use token::Token;
pub use token_id::TokenId;

sol!(ERC20Contract, "abi/erc20.json");
