use std::collections::hash_map::Entry;

use alloy::primitives::Address;
use alloy::{network::Network, providers::Provider, transports::Transport};
use async_trait::async_trait;
use bigdecimal::BigDecimal;

use crate::{
    error::InternalError, stores::TokenStore, Erc20Contract::Erc20ContractInstance, Error, Token,
    TokenId,
};

#[async_trait]
pub trait Erc20ProviderExt<T, N>: Provider<T, N> + Sized
where
    T: Transport + Clone,
    N: Network,
{
    async fn retrieve_token(&self, address: Address) -> Result<Token, Error> {
        let instance = Erc20ContractInstance::new(address, self);

        let symbol = instance
            .symbol()
            .call()
            .await
            .map_err(|err| Error::new(TokenId::Address(address), err))?;

        let decimals = instance
            .decimals()
            .call()
            .await
            .map_err(|err| Error::new(TokenId::Address(address), err))?;

        let token = Token::new(address, symbol._0, decimals._0);

        Ok(token)
    }

    async fn get_token<'a, S>(&'a self, id: TokenId, store: &'a mut S) -> Result<&Token, Error>
    where
        S: TokenStore + Send,
    {
        let chain_id = self
            .get_chain_id()
            .await
            .map_err(|err| Error::new(id.clone(), err))?;

        match store.entry(chain_id as u8, id.clone()) {
            Entry::Occupied(e) => Ok(e.into_mut()),
            Entry::Vacant(e) => match &id {
                TokenId::Symbol(_) => Err(Error::new(id, InternalError::NotInStore)),
                TokenId::Address(a) => {
                    let token = self.retrieve_token(*a).await?;

                    Ok(e.insert(token))
                }
            },
        }
    }

    async fn balance_of(&self, token: Address, address: Address) -> Result<BigDecimal, Error> {
        let instance = Erc20ContractInstance::new(token, self);

        let result = instance
            .balanceOf(address)
            .call()
            .await
            .map_err(|err| Error::new(TokenId::Address(address), err))?;

        let token = self.retrieve_token(token).await?;

        let balance = token.get_balance(result.balance);

        Ok(balance)
    }
}

#[async_trait]
impl<P, T, N> Erc20ProviderExt<T, N> for P
where
    P: Provider<T, N>,
    T: Transport + Clone,
    N: Network,
{
}
