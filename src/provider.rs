use std::{fmt::Debug, marker::PhantomData, sync::Arc};

use alloy::primitives::Address;
use alloy::{network::Network, providers::Provider, transports::Transport};
use bigdecimal::BigDecimal;
use parking_lot::RwLock;

use crate::{
    error::InternalError,
    stores::{BasicTokenStore, TokenStore},
    util::StoreIter,
    Erc20Contract::Erc20ContractInstance,
    Error, Token, TokenId,
};

#[derive(Debug, Clone)]
pub struct Erc20Provider<P, N, T, S> {
    provider: P,
    chain_id: u8,
    store: Arc<RwLock<S>>,
    phantom: PhantomData<(N, T)>,
}

impl<P, N, T> Erc20Provider<P, N, T, BasicTokenStore> {
    pub fn new(provider: P, chain_id: u8) -> Self {
        Self {
            provider,
            chain_id,
            store: Arc::new(RwLock::new(BasicTokenStore::new())),
            phantom: PhantomData,
        }
    }
}

impl<P, N, T, S> Erc20Provider<P, N, T, S> {
    pub fn with_store(provider: P, chain_id: u8, store: S) -> Self {
        Self {
            provider,
            chain_id,
            store: Arc::new(RwLock::new(store)),
            phantom: PhantomData,
        }
    }
}

impl<P, N, T, S> Erc20Provider<P, N, T, S>
where
    P: Provider<T, N>,
    T: Transport + Clone,
    N: Network,
    S: TokenStore,
{
    pub async fn retrieve_token(&self, id: TokenId) -> Result<Arc<Token>, Error> {
        if let Some(token) = self.store.read().get(self.chain_id, id.clone()) {
            return Ok(token);
        }

        match id {
            TokenId::Symbol(_) => Err(Error::new(id, InternalError::NotInStore)),
            TokenId::Address(a) => {
                let instance = Erc20ContractInstance::new(a, &self.provider);

                let symbol = instance
                    .symbol()
                    .call()
                    .await
                    .map_err(|err| Error::new(id.clone(), err))?;

                let decimals = instance
                    .decimals()
                    .call()
                    .await
                    .map_err(|err| Error::new(id.clone(), err))?;
                let token = Token::new(a, symbol._0, decimals._0);

                let token = Arc::new(token);

                self.store.write().insert(self.chain_id, token.clone());

                Ok(token)
            }
        }
    }

    pub async fn balance_of(&self, token: Address, address: Address) -> Result<BigDecimal, Error> {
        let instance = Erc20ContractInstance::new(token, &self.provider);

        let result = instance
            .balanceOf(address)
            .call()
            .await
            .map_err(|err| Error::new(TokenId::Address(address), err))?;

        let token = self.retrieve_token(TokenId::Address(token)).await?;

        let balance = token.get_balance(result.balance);

        Ok(balance)
    }

    pub fn iter(&self) -> StoreIter<RwLock<S>> {
        StoreIter::from_lock(self.store.as_ref(), self.chain_id)
    }
}
