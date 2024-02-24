use std::sync::Arc;

use alloy_primitives::Address;
use alloy_providers::provider::{Provider, TempProvider};
use alloy_rpc_types::{CallInput, CallRequest};
use alloy_sol_types::SolCall;
use alloy_transport::BoxTransport;

use crate::{ERC20Contract, Error, Token, TokenId};

pub struct TokenClient {
    provider: Arc<Provider<BoxTransport>>,
}

impl TokenClient {
    pub fn new(provider: Arc<Provider<BoxTransport>>) -> Self {
        Self { provider }
    }

    pub async fn retrieve_token(&self, address: Address) -> Result<Token, Error> {
        let symbol = self.symbol(address).await?;
        let decimals = self.decimals(address).await?;

        let token = Token::new(address, &symbol, decimals);

        Ok(token)
    }

    async fn symbol(&self, address: Address) -> Result<String, Error> {
        let tx = CallRequest {
            to: Some(address),
            input: CallInput::new(ERC20Contract::symbolCall::new(()).abi_encode().into()),
            ..Default::default()
        };

        let result = self
            .provider
            .call(tx, None)
            .await
            .map_err(|err| Error::new(TokenId::Address(address), err))?;
        let decoded = ERC20Contract::symbolCall::abi_decode_returns(&result, true)
            .map_err(|err| Error::new(TokenId::Address(address), err))?;

        Ok(decoded._0)
    }

    async fn decimals(&self, address: Address) -> Result<u8, Error> {
        let tx = CallRequest {
            to: Some(address),
            input: CallInput::new(ERC20Contract::decimalsCall::new(()).abi_encode().into()),
            ..Default::default()
        };

        let result = self
            .provider
            .call(tx, None)
            .await
            .map_err(|err| Error::new(TokenId::Address(address), err))?;
        let decoded = ERC20Contract::decimalsCall::abi_decode_returns(&result, true)
            .map_err(|err| Error::new(TokenId::Address(address), err))?;

        Ok(decoded._0)
    }
}
