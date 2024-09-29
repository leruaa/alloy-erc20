use std::fmt::{self, Debug};

use alloy::primitives::Address;

use crate::TokenId;

use super::TokenStore;

/// A store iterator.
pub struct StoreIter<'a, S> {
    chain_id: u64,
    store: &'a S,
    addresses: Vec<Address>,
    index: usize,
}

impl<'a, S> StoreIter<'a, S>
where
    S: TokenStore<'a>,
{
    /// Creates a new store iterator.
    pub fn new(store: &'a S, chain_id: u64) -> Self {
        Self {
            chain_id,
            store,
            addresses: store.addresses(Some(chain_id)),
            index: 0,
        }
    }
}

impl<'a, S: Debug> Debug for StoreIter<'a, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StoreIter")
            .field("chain_id", &self.chain_id)
            .field("store", &self.store)
            .finish()
    }
}

impl<'a, S> Iterator for StoreIter<'a, S>
where
    S: TokenStore<'a>,
{
    type Item = S::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let item = match self.addresses.get(self.index) {
            Some(current_address) => self
                .store
                .get(self.chain_id, TokenId::Address(*current_address)),
            None => None,
        };

        self.index += 1;

        item
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        mainnet::{USDC, WBTC, WETH},
        stores::BasicTokenStore,
        TokenStore,
    };

    #[test]
    fn test_iter() {
        let mut store = BasicTokenStore::new();

        store.insert(1, WETH.clone());
        store.insert(1, WBTC.clone());
        store.insert(1, USDC.clone());

        let symbols = store.iter(1).map(|t| t.symbol.as_str()).collect::<Vec<_>>();

        assert!(symbols.contains(&"WETH"));
        assert!(symbols.contains(&"WBTC"));
        assert!(symbols.contains(&"USDC"));
    }
}
