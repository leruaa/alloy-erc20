use std::collections::hash_map::Entry;

use crate::{util::StoreIter, Token, TokenId};

mod basic;

use alloy::primitives::Address;
pub use basic::BasicTokenStore;

/// A [`Token`] store
pub trait TokenStore: Sized {
    /// Inserts a token into the store.
    fn insert(&mut self, chain_id: u8, token: Token);

    /// Returns `true` if the store contains a value for the specified `id`.
    fn contains(&self, chain_id: u8, id: TokenId) -> bool;

    /// Returns the value corresponding to the given id.
    fn get(&self, chain_id: u8, id: TokenId) -> Option<&Token>;
    fn symbols(&self, chain_id: Option<u8>) -> impl Iterator<Item = String>;
    fn addresses(&self, chain_id: Option<u8>) -> impl Iterator<Item = Address>;
    fn entry(&mut self, chain_id: u8, id: TokenId) -> Entry<(u8, TokenId), Token>;

    fn iter(&self, chain_id: u8) -> StoreIter<Self> {
        StoreIter::new(self, chain_id)
    }

    #[cfg(feature = "known-tokens")]
    fn insert_known_tokens(&mut self, chain_id: u8) {
        use crate::{arbitrum, mainnet};

        if chain_id == 1 {
            self.insert(chain_id, mainnet::WETH.to_owned());
            self.insert(chain_id, mainnet::WBTC.to_owned());
            self.insert(chain_id, mainnet::USDC.to_owned());
            self.insert(chain_id, mainnet::USDT.to_owned());
            self.insert(chain_id, mainnet::DAI.to_owned());
        } else if chain_id == 42161 {
            // Arbitrum
            self.insert(chain_id, arbitrum::WETH.to_owned());
            self.insert(chain_id, arbitrum::USDC.to_owned());
            self.insert(chain_id, arbitrum::USDT.to_owned());
        }
    }
}
