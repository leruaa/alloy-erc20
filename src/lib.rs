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
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

mod constants;
pub use constants::*;

mod provider;
pub use provider::Erc20ProviderExt;

mod error;
pub use error::Error;

mod token;
pub use token::Token;

mod lazy_token;
pub use lazy_token::LazyToken;

mod token_id;
pub use token_id::TokenId;

mod stores;
pub use stores::{BasicTokenStore, Entry, StoreIter, TokenStore};

#[cfg(feature = "lru-store")]
pub use stores::LruTokenStore;
