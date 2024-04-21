#![doc = include_str!("../README.md")]
#![warn(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    unreachable_pub,
    clippy::missing_const_for_fn,
    rustdoc::all
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]

use alloy::sol;

mod constants;
pub use constants::*;

mod provider;
pub use provider::Erc20ProviderExt;

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
