use alloy::sol;

mod constants;
pub use constants::*;

mod provider;
pub use provider::Erc20Provider;

mod error;
pub use error::Error;

mod token;
pub use token::Token;

mod token_id;
pub use token_id::TokenId;

pub mod stores;
pub mod util;

sol!(
    #[sol(rpc)]
    Erc20Contract,
    "abi/erc20.json"
);
