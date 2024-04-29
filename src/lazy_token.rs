use std::{fmt::Debug, future::ready, marker::PhantomData};

use alloy::{
    contract::Error,
    network::Network,
    primitives::{Address, U256},
    providers::Provider,
    transports::Transport,
};
use async_once_cell::OnceCell;
use futures::TryFutureExt;

use crate::provider::Erc20Contract;

#[derive(Debug)]
/// A token with an embedded contract instance that lazily query the
/// blockchain.
pub struct LazyToken<P, T, N> {
    name: OnceCell<String>,
    symbol: OnceCell<String>,
    decimals: OnceCell<u8>,
    total_supply: OnceCell<U256>,
    instance: Erc20Contract::Erc20ContractInstance<T, P, N>,
    phantom: PhantomData<(T, N)>,
}

impl<P, T, N> LazyToken<P, T, N>
where
    P: Provider<T, N>,
    T: Transport + Clone,
    N: Network,
{
    /// Creates a new [`LazyToken`].
    pub fn new(address: Address, provider: P) -> Self {
        Self {
            name: OnceCell::new(),
            symbol: OnceCell::new(),
            decimals: OnceCell::new(),
            total_supply: OnceCell::new(),
            instance: Erc20Contract::Erc20ContractInstance::new(address, provider),
            phantom: PhantomData,
        }
    }

    /// Returns the token contract address.
    pub const fn address(&self) -> &Address {
        self.instance.address()
    }

    /// Returns the name of the token.
    pub async fn name(&self) -> Result<&String, Error> {
        self.name
            .get_or_try_init(self.instance.name().call().and_then(|r| ready(Ok(r._0))))
            .await
    }

    /// Returns the symbol of the token.
    pub async fn symbol(&self) -> Result<&String, Error> {
        self.symbol
            .get_or_try_init(self.instance.symbol().call().and_then(|r| ready(Ok(r._0))))
            .await
    }

    /// Returns the decimals places of the token.
    pub async fn decimals(&self) -> Result<&u8, Error> {
        self.decimals
            .get_or_try_init(
                self.instance
                    .decimals()
                    .call()
                    .and_then(|r| ready(Ok(r._0))),
            )
            .await
    }

    /// Returns the amount of tokens in existence.
    pub async fn total_supply(&self) -> Result<&U256, Error> {
        self.total_supply
            .get_or_try_init(
                self.instance
                    .totalSupply()
                    .call()
                    .and_then(|r| ready(Ok(r._0))),
            )
            .await
    }

    /// Returns the value of tokens owned by `account`.
    pub async fn balance_of(&self, account: Address) -> Result<U256, Error> {
        self.instance
            .balanceOf(account)
            .call()
            .and_then(|r| ready(Ok(r.balance)))
            .await
    }

    /// Returns the remaining number of tokens that `spender` will be
    /// allowed to spend on behalf of `owner`.
    pub async fn allowance(&self, owner: Address, spender: Address) -> Result<U256, Error> {
        self.instance
            .allowance(owner, spender)
            .call()
            .and_then(|r| ready(Ok(r._0)))
            .await
    }
}
