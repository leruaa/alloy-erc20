use std::collections::HashMap;

use alloy::primitives::Address;

use crate::{token_id::TokenId, Token};

use super::TokenStore;

/// A basic [`TokenStore`] implementation.
#[derive(Debug, Default, Clone)]
pub struct BasicTokenStore {
    tokens: HashMap<(u64, TokenId), Token>,
}

impl BasicTokenStore {
    /// Creates a new [`BasicTokenStore`]
    pub fn new() -> Self {
        Self {
            tokens: HashMap::new(),
        }
    }
}

impl<'a> TokenStore<'a> for BasicTokenStore {
    type Item = &'a Token;

    fn get(&'a self, chain_id: u64, id: TokenId) -> Option<Self::Item> {
        self.tokens.get(&(chain_id, id.clone()))
    }

    fn get_mut(&mut self, chain_id: u64, id: TokenId) -> Option<&mut Token> {
        self.tokens.get_mut(&(chain_id, id.clone()))
    }

    fn insert(&mut self, chain_id: u64, token: Token) {
        self.tokens
            .insert((chain_id, TokenId::Address(token.address)), token.clone());
        self.tokens
            .insert((chain_id, TokenId::Symbol(token.symbol.to_string())), token);
    }

    fn contains(&self, chain_id: u64, id: TokenId) -> bool {
        self.tokens.contains_key(&(chain_id, id))
    }

    fn symbols(&'a self, chain_id: Option<u64>) -> Vec<String> {
        self.tokens
            .keys()
            .filter_map(move |(token_chain_id, id)| match (id, chain_id) {
                (TokenId::Symbol(id), Some(chain_id)) if token_chain_id == &chain_id => {
                    Some(id.clone())
                }
                (TokenId::Symbol(id), None) => Some(id.clone()),
                _ => None,
            })
            .collect()
    }

    fn addresses(&'a self, chain_id: Option<u64>) -> Vec<Address> {
        self.tokens
            .keys()
            .filter_map(move |(token_chain_id, id)| match (id, chain_id) {
                (TokenId::Address(id), Some(chain_id)) if token_chain_id == &chain_id => Some(*id),
                (TokenId::Address(id), None) => Some(*id),
                _ => None,
            })
            .collect()
    }
}
