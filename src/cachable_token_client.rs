use std::sync::Arc;

use parking_lot::RwLock;

use crate::{error::InternalError, stores::TokenStore, Error, Token, TokenClient, TokenId};

pub struct CachableTokenClient {
    inner: TokenClient,
    store: RwLock<Box<dyn TokenStore>>,
}

impl CachableTokenClient {
    pub fn new<S: TokenStore + 'static>(inner: TokenClient, store: S) -> Self {
        Self {
            inner,
            store: RwLock::new(Box::new(store)),
        }
    }

    pub async fn retrieve_token(&self, chain_id: u8, id: TokenId) -> Result<Arc<Token>, Error> {
        if let Some(token) = self.store.read().get(chain_id, id.clone()) {
            return Ok(token);
        }

        match id {
            TokenId::Symbol(_) => Err(Error::new(id, InternalError::NotInStore)),
            TokenId::Address(a) => match self.inner.retrieve_token(a).await {
                Ok(token) => {
                    let token = Arc::new(token);
                    self.store.write().insert(chain_id, token.clone());
                    Ok(token)
                }
                Err(err) => Err(err),
            },
        }
    }
}
