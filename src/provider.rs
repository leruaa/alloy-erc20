use alloy::{
    network::Network, primitives::Address, providers::Provider, sol, transports::Transport,
};
use async_trait::async_trait;
use bigdecimal::BigDecimal;

use crate::{error::InternalError, stores::TokenStore, Entry, Error, Token, TokenId};

sol!(
    #[sol(rpc)]
    Erc20Contract,
    "abi/erc20.json"
);

/// Extends Alloy [`Provider`] trait with ERC-20 related features.
#[async_trait]
pub trait Erc20ProviderExt<T, N>: Provider<T, N> + Sized
where
    T: Transport + Clone,
    N: Network,
{
    /// Retrieves a token by querying its ERC-20 contract.
    async fn retrieve_token(&self, address: Address) -> Result<Token, Error> {
        let instance = Erc20Contract::Erc20ContractInstance::new(address, self);

        let symbol = instance
            .symbol()
            .call()
            .await
            .map_err(|err| Error::new(address.into(), err))?;

        let decimals = instance
            .decimals()
            .call()
            .await
            .map_err(|err| Error::new(address.into(), err))?;

        let token = Token::new(address, symbol._0, decimals._0);

        Ok(token)
    }

    /// Returns a token from the given store if present, otherwise retrieves
    /// it from its ERC-20 contract and update the store.
    async fn get_token<'a, Id, S>(&'a self, id: Id, store: &'a mut S) -> Result<&'a Token, Error>
    where
        S: TokenStore<'a> + Send,
        Id: Into<TokenId> + Send,
    {
        let id: TokenId = id.into();
        let chain_id = self
            .get_chain_id()
            .await
            .map_err(|err| Error::new(id.clone(), err))?;

        match store.entry(chain_id as u8, id.clone()) {
            Entry::Occupied(e) => Ok(e.into_mut()),
            Entry::Vacant(e) => {
                let token = match id.clone() {
                    TokenId::Address(address) => self.retrieve_token(address).await,
                    TokenId::Symbol(symbol) => {
                        Err(Error::new(id, InternalError::NotInStore(symbol)))
                    }
                }?;

                Ok(e.insert(token))
            }
        }
    }

    /// Retrieves the given address balance from the given token contract.
    async fn balance_of(&self, token: Address, address: Address) -> Result<BigDecimal, Error> {
        let instance = Erc20Contract::Erc20ContractInstance::new(token, self);

        let result = instance
            .balanceOf(address)
            .call()
            .await
            .map_err(|err| Error::new(address.into(), err))?;

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
