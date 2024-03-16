use std::sync::Arc;

use alloy_network::Network;
use alloy_primitives::Address;
use alloy_transport::Transport;
use bigdecimal::BigDecimal;
use parking_lot::{RwLock, RwLockReadGuard};

use crate::{
    error::InternalError,
    stores::{BasicTokenStore, TokenStore},
    Error, Token, TokenId,
};

use super::TokenClient;

#[derive(Debug, Clone)]
pub struct CachableTokenClient<N, T, S = BasicTokenStore> {
    inner: TokenClient<N, T>,
    chain_id: u8,
    store: Arc<RwLock<S>>,
}

impl<N, T, S> CachableTokenClient<N, T, S> {
    pub fn new(inner: TokenClient<N, T>, chain_id: u8, store: S) -> Self {
        Self {
            inner,
            chain_id,
            store: Arc::new(RwLock::new(store)),
        }
    }
}

impl<N, T, S> CachableTokenClient<N, T, S>
where
    N: Network,
    T: Transport + Clone,
    S: TokenStore,
{
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

    pub async fn balance_of(&self, token: Address, address: Address) -> Result<BigDecimal, Error> {
        let amount = self.inner.balance_of(token, address).await?;
        let token = self.retrieve_token(TokenId::Address(token)).await?;

        let balance = token.get_balance(amount);

        Ok(balance)
    }

    pub fn store(&self) -> RwLockReadGuard<S> {
        self.store.read()
    }
}
