use std::sync::Arc;

use crate::{Token, TokenId};

mod basic;

pub use basic::BasicTokenStore;

pub trait TokenStore {
    fn insert(&self, chain_id: u8, token: Arc<Token>);
    fn contains(&self, chain_id: u8, id: TokenId) -> bool;
    fn get(&self, chain_id: u8, id: TokenId) -> Option<Arc<Token>>;
}
