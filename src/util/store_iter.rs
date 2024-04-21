use alloy::primitives::Address;

use crate::{stores::TokenStore, Token, TokenId};

/// A store iterator.
#[derive(Debug)]
pub struct StoreIter<'a, S> {
    chain_id: u8,
    store: &'a S,
    addresses: Vec<Address>,
    current_index: usize,
}

impl<'a, S> StoreIter<'a, S>
where
    S: TokenStore,
{
    /// Creates a new store iterator.
    pub fn new(store: &'a S, chain_id: u8) -> Self {
        let addresses = store.addresses(Some(chain_id)).collect();
        Self {
            chain_id,
            store,
            addresses,
            current_index: 0,
        }
    }
}

impl<'a, S> Iterator for StoreIter<'a, S>
where
    S: TokenStore,
{
    type Item = &'a Token;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self
            .addresses
            .get(self.current_index)
            .and_then(|a| self.store.get(self.chain_id, TokenId::Address(*a)));

        self.current_index += 1;

        token
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        mainnet::{USDC, WBTC, WETH},
        stores::{BasicTokenStore, TokenStore},
    };

    #[test]
    fn test_iter() {
        let mut store = BasicTokenStore::new();

        store.insert(1, WETH.clone());
        store.insert(1, WBTC.clone());
        store.insert(1, USDC.clone());

        let mut iter = store.iter(1);

        assert_eq!(iter.next().unwrap().symbol, "WETH");
        assert_eq!(iter.next().unwrap().symbol, "WBTC");
        assert_eq!(iter.next().unwrap().symbol, "USDC");
    }
}
