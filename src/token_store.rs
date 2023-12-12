use std::{
    collections::{hash_map::Entry, HashMap},
    sync::Arc,
};

use async_recursion::async_recursion;
use ethers::{
    providers::{Http, Provider},
    types::Address,
};
use once_cell::sync::Lazy;
use tokio::sync::{Mutex, RwLock};

use crate::{ERC20Contract, Error, Token};

#[derive(Clone, Debug)]
pub enum TokenId {
    Symbol(String),
    Address(Address),
}

#[derive(Debug)]
pub struct TokenStore {
    provider: Arc<Provider<Http>>,
    by_symbol: Mutex<HashMap<String, Arc<Token>>>,
    by_addresses: RwLock<HashMap<Address, Arc<Token>>>,
    known_addresses: Lazy<HashMap<String, Lazy<Address>>>,
}

impl TokenStore {
    pub fn new(chain_id: u64, provider: Arc<Provider<Http>>) -> Self {
        Self {
            provider,
            by_symbol: Mutex::new(HashMap::new()),
            by_addresses: RwLock::new(HashMap::new()),
            known_addresses: match chain_id {
                1 => Lazy::new(|| {
                    HashMap::from([
                        (
                            String::from("WETH"),
                            Lazy::<Address>::new(|| {
                                "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"
                                    .parse()
                                    .unwrap()
                            }),
                        ),
                        (
                            String::from("WBTC"),
                            Lazy::<Address>::new(|| {
                                "0x2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599"
                                    .parse()
                                    .unwrap()
                            }),
                        ),
                        (
                            String::from("USDC"),
                            Lazy::<Address>::new(|| {
                                "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"
                                    .parse()
                                    .unwrap()
                            }),
                        ),
                        (
                            String::from("USDT"),
                            Lazy::<Address>::new(|| {
                                "0xdAC17F958D2ee523a2206206994597C13D831ec7"
                                    .parse()
                                    .unwrap()
                            }),
                        ),
                        (
                            String::from("DAI"),
                            Lazy::<Address>::new(|| {
                                "0x6B175474E89094C44Da98b954EedeAC495271d0F"
                                    .parse()
                                    .unwrap()
                            }),
                        ),
                        (
                            String::from("CRV"),
                            Lazy::<Address>::new(|| {
                                "0xD533a949740bb3306d119CC777fa900bA034cd52"
                                    .parse()
                                    .unwrap()
                            }),
                        ),
                    ])
                }),
                137 => Lazy::new(|| {
                    HashMap::from([
                        (
                            String::from("USDC"),
                            Lazy::<Address>::new(|| {
                                "0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174"
                                    .parse()
                                    .unwrap()
                            }),
                        ),
                        (
                            String::from("USDT"),
                            Lazy::<Address>::new(|| {
                                "0xc2132D05D31c914a87C6611C10748AEb04B58e8F"
                                    .parse()
                                    .unwrap()
                            }),
                        ),
                    ])
                }),
                _ => Lazy::new(HashMap::new),
            },
        }
    }

    #[async_recursion]
    pub async fn get(&self, id: TokenId) -> Result<Arc<Token>, Error> {
        match id.clone() {
            TokenId::Symbol(symbol) => {
                let mut by_symbol = self.by_symbol.lock().await;
                let entry = by_symbol.entry(symbol.clone());

                match entry {
                    Entry::Vacant(_) => match self.known_addresses.get(&symbol) {
                        Some(a) => self.get(TokenId::Address(**a)).await,
                        None => Err(Error::NotInStoreError),
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

                let erc20 = ERC20Contract::new(address, self.provider.clone());
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
                    let symbol = erc20
                        .symbol()
                        .call()
                        .await
                        .map_err(|err| Error::Ethers(err.to_string()))?;
                    let decimals = erc20
                        .decimals()
                        .call()
                        .await
                        .map_err(|err| Error::Ethers(err.to_string()))?;

                    let token = Arc::new(Token::new(address, &symbol, decimals));
                    let mut by_addresses = self.by_addresses.write().await;
                    by_addresses.insert(address, token.clone());
                    Ok(token)
                }
            }
        }
    }
}
