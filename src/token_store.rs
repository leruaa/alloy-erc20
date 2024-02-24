use std::{collections::HashMap, sync::Arc};

use parking_lot::RwLock;

use crate::{token_id::TokenId, Token};

#[derive(Debug, Default)]
pub struct TokenStore {
    tokens: RwLock<HashMap<(u8, TokenId), Arc<Token>>>,
}

impl TokenStore {
    pub fn new() -> Self {
        Self {
            tokens: RwLock::new(HashMap::new()),
        }
    }

    #[cfg(feature = "known-tokens")]
    pub fn insert_known_tokens(&self, chain_id: u64) {
        todo!()
    }

    pub fn insert(&self, chain_id: u8, token: Arc<Token>) {
        let mut tokens = self.tokens.write();

        tokens.insert((chain_id, TokenId::Address(token.address)), token.clone());
        tokens.insert((chain_id, TokenId::Symbol(token.symbol.clone())), token);
    }

    pub fn contains(&self, chain_id: u8, id: TokenId) -> bool {
        self.tokens.read().contains_key(&(chain_id, id))
    }

    pub fn get(&self, chain_id: u8, id: TokenId) -> Option<Arc<Token>> {
        self.tokens.read().get(&(chain_id, id.clone())).cloned()
    }
}
