use std::sync::Arc;

use alloy_network::{Network, TransactionBuilder};
use alloy_primitives::Address;
use alloy_provider::{Provider, RootProvider};
use alloy_sol_types::SolCall;
use alloy_transport::Transport;

use crate::{ERC20Contract, Error, Token, TokenId};

#[derive(Debug, Clone)]
pub struct TokenClient<N, T> {
    provider: Arc<RootProvider<N, T>>,
}

/// A client for quering ERC20 [`Token`] from the blockchain.
impl<N, T> TokenClient<N, T>
where
    N: Network,
    T: Transport + Clone,
{
    /// Create a new [`TokenClient`] with the given provider.
    pub fn new(provider: Arc<RootProvider<N, T>>) -> Self {
        Self { provider }
    }

    pub async fn retrieve_token(&self, address: Address) -> Result<Token, Error> {
        let symbol = self.symbol(address).await?;
        let decimals = self.decimals(address).await?;

        let token = Token::new(address, symbol, decimals);

        Ok(token)
    }

    async fn symbol(&self, address: Address) -> Result<String, Error> {
        let tx = N::TransactionRequest::default()
            .with_to(address.into())
            .with_input(ERC20Contract::symbolCall::new(()).abi_encode().into());

        let result = self
            .provider
            .call(&tx, None)
            .await
            .map_err(|err| Error::new(TokenId::Address(address), err))?;
        let decoded = ERC20Contract::symbolCall::abi_decode_returns(&result, true)
            .map_err(|err| Error::new(TokenId::Address(address), err))?;

        Ok(decoded._0)
    }

    async fn decimals(&self, address: Address) -> Result<u8, Error> {
        let tx = N::TransactionRequest::default()
            .with_to(address.into())
            .with_input(ERC20Contract::decimalsCall::new(()).abi_encode().into());

        let result = self
            .provider
            .call(&tx, None)
            .await
            .map_err(|err| Error::new(TokenId::Address(address), err))?;
        let decoded = ERC20Contract::decimalsCall::abi_decode_returns(&result, true)
            .map_err(|err| Error::new(TokenId::Address(address), err))?;

        Ok(decoded._0)
    }
}