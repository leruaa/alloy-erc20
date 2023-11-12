use ethers::prelude::abigen;

mod constants;
mod error;
mod token;
mod token_store;

pub use constants::mainnet;
pub use error::Error;
pub use token::Token;
pub use token_store::{TokenId, TokenStore};

abigen!(ERC20Contract, "abi/erc20.json");
