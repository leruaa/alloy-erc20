use std::ops::Deref;

use alloy::primitives::Address;

use crate::{Token, TokenId};

use super::{Entry, StoreIter};

/// A [`Token`] store
pub trait TokenStore<'a>: Sized {
    /// a reference to a token.
    type Item: Deref<Target = Token>;

    /// Returns the value corresponding to the given id.
    fn get(&'a self, chain_id: u64, id: TokenId) -> Option<Self::Item>;

    /// Returns the mutable value corresponding to the given id.
    fn get_mut(&mut self, chain_id: u64, id: TokenId) -> Option<&mut Token>;

    /// Inserts a token into the store.
    fn insert(&mut self, chain_id: u64, token: Token);

    /// Returns `true` if the store contains a value for the specified `id`.
    fn contains(&self, chain_id: u64, id: TokenId) -> bool;

    /// Returns the symbols from all the tokens in the store.
    fn symbols(&'a self, chain_id: Option<u64>) -> Vec<String>;

    /// Returns the addresses from all the tokens in the store.
    fn addresses(&'a self, chain_id: Option<u64>) -> Vec<Address>;

    /// Gets the entry for the given token id.
    fn entry(&'a mut self, chain_id: u64, id: TokenId) -> Entry<'a, Self> {
        Entry::new(chain_id, id, self)
    }

    /// Returns an iterator over the store's tokens.
    fn iter(&'a self, chain_id: u64) -> StoreIter<'a, Self> {
        StoreIter::new(self, chain_id)
    }

    #[cfg(feature = "known-tokens")]
    /// Insert a few well known token to the store.
    fn insert_known_tokens(&mut self, chain_id: u64) {
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
