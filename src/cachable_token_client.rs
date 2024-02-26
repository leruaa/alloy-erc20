use std::sync::Arc;

use parking_lot::RwLock;

use crate::{
    error::InternalError,
    stores::{BasicTokenStore, TokenStore},
    Error, Token, TokenClient, TokenId,
};

pub struct CachableTokenClient<S = BasicTokenStore> {
    inner: TokenClient,
    chain_id: u8,
    store: RwLock<S>,
}

impl<S> CachableTokenClient<S> {
    pub fn new(inner: TokenClient, chain_id: u8, store: S) -> Self {
        Self {
            inner,
            chain_id,
            store: RwLock::new(store),
        }
    }
}

impl<S: TokenStore> CachableTokenClient<S> {
    pub async fn retrieve_token(&self, id: TokenId) -> Result<Arc<Token>, Error> {
        if let Some(token) = self.store.read().get(self.chain_id, id.clone()) {
            return Ok(token);
        }

        match id {
            TokenId::Symbol(_) => Err(Error::new(id, InternalError::NotInStore)),
            TokenId::Address(a) => match self.inner.retrieve_token(a).await {
                Ok(token) => {
                    let token = Arc::new(token);
                    self.store.write().insert(self.chain_id, token.clone());
                    Ok(token)
                }
                Err(err) => Err(err),
            },
        }
    }
}
