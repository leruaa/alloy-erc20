use std::{collections::HashMap, sync::Arc};

use crate::{token_id::TokenId, Token};

use super::TokenStore;

#[derive(Debug, Default)]
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
}
