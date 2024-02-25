use alloy_sol_types::sol;

mod constants;
mod error;
mod token;
mod token_client;
mod token_id;
mod token_service;

pub mod stores;

pub use constants::*;
pub use error::Error;
pub use token::Token;
pub use token_client::TokenClient;
pub use token_id::TokenId;
pub use token_service::TokenService;

sol!(ERC20Contract, "abi/erc20.json");
