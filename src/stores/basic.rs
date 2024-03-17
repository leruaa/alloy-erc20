use std::{collections::HashMap, sync::Arc};

use alloy_primitives::Address;

use crate::{token_id::TokenId, Token};

use super::TokenStore;

#[derive(Debug, Default, Clone)]
pub struct BasicTokenStore {
    tokens: HashMap<(u8, TokenId), Arc<Token>>,
}

impl BasicTokenStore {
    pub fn new() -> Self {
        Self {
            tokens: HashMap::new(),
        }
    }
}

impl TokenStore for BasicTokenStore {
    fn insert(&mut self, chain_id: u8, token: Arc<Token>) {
        self.tokens
            .insert((chain_id, TokenId::Address(token.address)), token.clone());
        self.tokens
            .insert((chain_id, TokenId::Symbol(token.symbol.to_string())), token);
    }

    fn contains(&self, chain_id: u8, id: TokenId) -> bool {
        self.tokens.contains_key(&(chain_id, id))
    }

    fn get(&self, chain_id: u8, id: TokenId) -> Option<Arc<Token>> {
        self.tokens.get(&(chain_id, id.clone())).cloned()
    }

    fn symbols(&self, chain_id: Option<u8>) -> impl Iterator<Item = String> {
        self.tokens
            .keys()
            .filter_map(move |(token_chain_id, id)| match (id, chain_id) {
                (TokenId::Symbol(id), Some(chain_id)) if token_chain_id == &chain_id => {
                    Some(id.clone())
                }
                (TokenId::Symbol(id), None) => Some(id.clone()),
                _ => None,
            })
    }

    fn addresses(&self, chain_id: Option<u8>) -> impl Iterator<Item = Address> {
        self.tokens
            .keys()
            .filter_map(move |(token_chain_id, id)| match (id, chain_id) {
                (TokenId::Address(id), Some(chain_id)) if token_chain_id == &chain_id => Some(*id),
                (TokenId::Address(id), None) => Some(*id),
                _ => None,
            })
    }
}
