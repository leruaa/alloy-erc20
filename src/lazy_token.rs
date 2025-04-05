use std::{
    fmt::Debug,
    future::{ready, IntoFuture},
};

use alloy::{
    contract::Error,
    network::Network,
    primitives::{Address, U256},
    providers::Provider,
    transports::Transport,
};
use async_once_cell::OnceCell;
use bigdecimal::{
    num_bigint::{BigInt, Sign},
    BigDecimal,
};
use futures::TryFutureExt;

use crate::provider::Erc20Contract;

#[derive(Debug)]
/// A token with an embedded contract instance that lazily query the
/// blockchain.
pub struct LazyToken<P, T, N> {
    name: OnceCell<String>,
    symbol: OnceCell<String>,
    decimals: OnceCell<u8>,
    instance: Erc20Contract::Erc20ContractInstance<T, P, N>,
}

impl<P, T, N> LazyToken<P, T, N>
where
    P: Provider<T, N>,
    T: Transport + Clone,
    N: Network,
{
    /// Creates a new [`LazyToken`].
    pub const fn new(address: Address, provider: P) -> Self {
        Self {
            name: OnceCell::new(),
            symbol: OnceCell::new(),
            decimals: OnceCell::new(),
            instance: Erc20Contract::Erc20ContractInstance::new(address, provider),
        }
    }

    /// Returns the token contract address.
    pub const fn address(&self) -> &Address {
        self.instance.address()
    }

    /// Returns the name of the token.
    pub async fn name(&self) -> Result<&String, Error> {
        self.name
            .get_or_try_init(
                self.instance
                    .name()
                    .call()
                    .into_future()
                    .and_then(|r| ready(Ok(r._0))),
            )
            .await
    }

    /// Returns the symbol of the token.
    pub async fn symbol(&self) -> Result<&String, Error> {
        self.symbol
            .get_or_try_init(
                self.instance
                    .symbol()
                    .call()
                    .into_future()
                    .and_then(|r| ready(Ok(r._0))),
            )
            .await
    }

    /// Returns the decimals places of the token.
    pub async fn decimals(&self) -> Result<&u8, Error> {
        self.decimals
            .get_or_try_init(
                self.instance
                    .decimals()
                    .call()
                    .into_future()
                    .and_then(|r| ready(Ok(r._0))),
            )
            .await
    }

    /// Returns the amount of tokens in existence.
    pub async fn total_supply(&self) -> Result<U256, Error> {
        self.instance
            .totalSupply()
            .call()
            .into_future()
            .and_then(|r| ready(Ok(r._0)))
            .await
    }

    /// Returns the value of tokens owned by `account`.
    pub async fn balance_of(&self, account: Address) -> Result<U256, Error> {
        self.instance
            .balanceOf(account)
            .call()
            .into_future()
            .and_then(|r| ready(Ok(r.balance)))
            .await
    }

    /// Returns the remaining number of tokens that `spender` will be
    /// allowed to spend on behalf of `owner`.
    pub async fn allowance(&self, owner: Address, spender: Address) -> Result<U256, Error> {
        self.instance
            .allowance(owner, spender)
            .call()
            .into_future()
            .and_then(|r| ready(Ok(r._0)))
            .await
    }

    /// Gets the token balance as a [`BigDecimal`]
    pub async fn get_balance(&self, amount: U256) -> Result<BigDecimal, Error> {
        let decimals = self.decimals().await?;

        let balance = BigDecimal::from((
            BigInt::from_bytes_be(Sign::Plus, &amount.to_be_bytes::<{ U256::BYTES }>()),
            *decimals as i64,
        ));

        Ok(balance)
    }
}
