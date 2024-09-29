use std::num::NonZeroUsize;

use alloy::primitives::Address;
use lru::LruCache;
use parking_lot::{MappedRwLockWriteGuard, RwLock, RwLockWriteGuard};

use crate::{Token, TokenId};

use super::TokenStore;

/// A basic [`TokenStore`] implementation.
#[derive(Debug)]
pub struct LruTokenStore {
    tokens: RwLock<LruCache<(u64, TokenId), Token>>,
}

impl LruTokenStore {
    /// Creates a new [`LruTokenStore`]
    pub fn new(cap: NonZeroUsize) -> Self {
        Self {
            tokens: RwLock::new(LruCache::new(cap)),
        }
    }
}

impl<'a> TokenStore<'a> for LruTokenStore {
    type Item = MappedRwLockWriteGuard<'a, Token>;

    fn get(&'a self, chain_id: u64, id: TokenId) -> Option<Self::Item> {
        RwLockWriteGuard::try_map(self.tokens.write(), |tokens| {
            tokens.get_mut(&(chain_id, id))
        })
        .ok()
    }

    fn get_mut(&mut self, chain_id: u64, id: TokenId) -> Option<&mut Token> {
        self.tokens.get_mut().get_mut(&(chain_id, id))
    }

    fn insert(&mut self, chain_id: u64, token: Token) {
        let mut tokens = self.tokens.write();

        tokens.put((chain_id, TokenId::Address(token.address)), token.clone());
        tokens.put((chain_id, TokenId::Symbol(token.symbol.to_string())), token);
    }

    fn contains(&self, chain_id: u64, id: TokenId) -> bool {
        let tokens = self.tokens.read();

        tokens.contains(&(chain_id, id))
    }

    fn symbols(&'a self, chain_id: Option<u64>) -> Vec<String> {
        let tokens = self.tokens.read();

        tokens
            .iter()
            .filter_map(move |((token_chain_id, id), _)| match (id, chain_id) {
                (TokenId::Symbol(id), Some(chain_id)) if token_chain_id == &chain_id => {
                    Some(id.clone())
                }
                (TokenId::Symbol(id), None) => Some(id.clone()),
                _ => None,
            })
            .collect::<Vec<_>>()
    }

    fn addresses(&'a self, chain_id: Option<u64>) -> Vec<Address> {
        let tokens = self.tokens.read();

        tokens
            .iter()
            .filter_map(move |((token_chain_id, id), _)| match (id, chain_id) {
                (TokenId::Address(id), Some(chain_id)) if token_chain_id == &chain_id => Some(*id),
                (TokenId::Address(id), None) => Some(*id),
                _ => None,
            })
            .collect::<Vec<_>>()
    }
}
