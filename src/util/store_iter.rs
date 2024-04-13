use std::sync::Arc;

use alloy::primitives::Address;
use parking_lot::RwLock;

use crate::{stores::TokenStore, Token, TokenId};

pub struct StoreIter<'a, S> {
    chain_id: u8,
    store: &'a S,
    addresses: Vec<Address>,
    current_index: usize,
}

impl<'a, S> StoreIter<'a, S> {
    fn next<T: TokenStore>(&mut self, store: &T) -> Option<Arc<Token>> {
        let token = self
            .addresses
            .get(self.current_index)
            .and_then(|a| store.get(self.chain_id, TokenId::Address(*a)));

        self.current_index += 1;

        token
    }
}

impl<'a, S> StoreIter<'a, S>
where
    S: TokenStore,
{
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

impl<'a, S> StoreIter<'a, RwLock<S>>
where
    S: TokenStore,
{
    pub fn from_lock(store: &'a RwLock<S>, chain_id: u8) -> Self {
        let addresses = store.read().addresses(Some(chain_id)).collect();
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
    type Item = Arc<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next(self.store)
    }
}

impl<'a, S> Iterator for StoreIter<'a, RwLock<S>>
where
    S: TokenStore,
{
    type Item = Arc<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next::<S>(&self.store.read())
    }
}
