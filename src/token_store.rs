use std::{
    collections::{hash_map::Entry, HashMap},
    fmt::{Display, Formatter},
    sync::Arc,
};

use alloy_primitives::{address, Address};
use alloy_providers::provider::{Provider, TempProvider};
use alloy_rpc_types::{CallInput, CallRequest};
use alloy_sol_types::SolCall;
use alloy_transport::BoxTransport;
use async_recursion::async_recursion;
use once_cell::sync::Lazy;
use tokio::sync::{Mutex, RwLock};

use crate::{error::InternalError, ERC20Contract, Error, Token};

#[derive(Clone, Debug)]
pub enum TokenId {
    Symbol(String),
    Address(Address),
}

impl Display for TokenId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenId::Symbol(s) => write!(f, "{}", s),
            TokenId::Address(a) => write!(f, "{}", a),
        }
    }
}

#[derive(Debug)]
pub struct TokenStore {
    provider: Arc<Provider<BoxTransport>>,
    by_symbol: Mutex<HashMap<String, Arc<Token>>>,
    by_addresses: RwLock<HashMap<Address, Arc<Token>>>,
    known_addresses: Lazy<HashMap<String, Address>>,
}

impl TokenStore {
    pub fn new(chain_id: u64, provider: Arc<Provider<BoxTransport>>) -> Self {
        Self {
            provider,
            by_symbol: Mutex::new(HashMap::new()),
            by_addresses: RwLock::new(HashMap::new()),
            known_addresses: match chain_id {
                1 => Lazy::new(|| {
                    HashMap::from([
                        (
                            String::from("WETH"),
                            address!("C02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"),
                        ),
                        (
                            String::from("WBTC"),
                            address!("2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599"),
                        ),
                        (
                            String::from("USDC"),
                            address!("A0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"),
                        ),
                        (
                            String::from("USDT"),
                            address!("dAC17F958D2ee523a2206206994597C13D831ec7"),
                        ),
                        (
                            String::from("DAI"),
                            address!("6B175474E89094C44Da98b954EedeAC495271d0F"),
                        ),
                        (
                            String::from("CRV"),
                            address!("D533a949740bb3306d119CC777fa900bA034cd52"),
                        ),
                    ])
                }),
                137 => Lazy::new(|| {
                    HashMap::from([
                        (
                            String::from("USDC"),
                            address!("2791Bca1f2de4661ED88A30C99A7a9449Aa84174"),
                        ),
                        (
                            String::from("USDT"),
                            address!("c2132D05D31c914a87C6611C10748AEb04B58e8F"),
                        ),
                    ])
                }),
                _ => Lazy::new(HashMap::new),
            },
        }
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

    #[async_recursion]
    pub async fn get(&self, id: TokenId) -> Result<Arc<Token>, Error> {
        match id.clone() {
            TokenId::Symbol(symbol) => {
                let mut by_symbol = self.by_symbol.lock().await;
                let entry = by_symbol.entry(symbol.clone());

                match entry {
                    Entry::Vacant(_) => match self.known_addresses.get(&symbol) {
                        Some(a) => self.get(TokenId::Address(*a)).await,
                        None => Err(Error::new(id, InternalError::NotInStore)),
                    },
                    Entry::Occupied(e) => Ok(e.get().clone()),
                }
            }
            TokenId::Address(address) => {
                let by_addresses = self.by_addresses.read().await;

                if let Some(token) = by_addresses.get(&address) {
                    return Ok(token.clone());
                }

                drop(by_addresses);

                if format!("{address:?}") == "0x9f8f72aa9304c8b593d555f12ef6589cc3a579a2" {
                    // symbol() doesn't work on MKR
                    let token = Arc::new(Token::new(address, "MKR", 18));
                    let mut by_addresses = self.by_addresses.write().await;
                    by_addresses.insert(address, token.clone());
                    Ok(token)
                } else if format!("{address:?}") == "0x89d24a6b4ccb1b6faa2625fe562bdd9a23260359" {
                    // symbol() doesn't work on SEI
                    let token = Arc::new(Token::new(address, "SEI", 18));
                    let mut by_addresses = self.by_addresses.write().await;
                    by_addresses.insert(address, token.clone());
                    Ok(token)
                } else {
                    let symbol = self.symbol(address).await?;
                    let decimals = self.decimals(address).await?;

                    let token = Arc::new(Token::new(address, &symbol, decimals));
                    let mut by_addresses = self.by_addresses.write().await;
                    by_addresses.insert(address, token.clone());
                    Ok(token)
                }
            }
        }
    }
}
