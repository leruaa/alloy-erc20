use std::sync::Arc;

use crate::{Token, TokenId};

mod basic;

pub use basic::BasicTokenStore;

pub trait TokenStore {
    fn insert(&mut self, chain_id: u8, token: Arc<Token>);
    fn contains(&self, chain_id: u8, id: TokenId) -> bool;
    fn get(&self, chain_id: u8, id: TokenId) -> Option<Arc<Token>>;

    #[cfg(feature = "known-tokens")]
    fn insert_known_tokens(&mut self, chain_id: u8) {
        use crate::mainnet;

        if chain_id == 1 {
            self.insert(chain_id, Arc::new(mainnet::WETH.to_owned()));
            self.insert(chain_id, Arc::new(mainnet::WBTC.to_owned()));
            self.insert(chain_id, Arc::new(mainnet::USDC.to_owned()));
            self.insert(chain_id, Arc::new(mainnet::USDT.to_owned()));
            self.insert(chain_id, Arc::new(mainnet::DAI.to_owned()));
        }
    }
}
