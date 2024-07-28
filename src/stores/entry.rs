use crate::{Token, TokenId};

use super::TokenStore;

/// A view into a single entry in a map, which may either be vacant or occupied.
#[derive(Debug)]
pub enum Entry<'a, S>
where
    S: TokenStore<'a>,
{
    /// An occupied entry.
    Occupied(OccupiedEntry<'a>),

    /// A vacant entry.
    Vacant(VacantEntry<'a, S>),
}

impl<'a, S> Entry<'a, S>
where
    S: TokenStore<'a>,
{
    /// Creates a new [`Entry`].
    pub fn new(chain_id: u8, id: TokenId, store: &'a mut S) -> Self {
        if store.contains(chain_id, id.clone()) {
            Self::Occupied(OccupiedEntry::new(chain_id, id, store))
        } else {
            Self::Vacant(VacantEntry::new(chain_id, id, store))
        }
    }
}

#[derive(Debug)]
pub struct OccupiedEntry<'a> {
    value: &'a mut Token,
}

impl<'a> OccupiedEntry<'a> {
    pub fn new<S>(chain_id: u8, id: TokenId, store: &'a mut S) -> Self
    where
        S: TokenStore<'a>,
    {
        Self {
            value: store.get_mut(chain_id, id).unwrap(),
        }
    }

    /// Gets a reference to the value in the entry.
    pub fn get(&self) -> &Token {
        self.value
    }

    /// Gets a mutable reference to the value in the entry.
    pub fn get_mut(&mut self) -> &mut Token {
        self.value
    }

    /// Converts the `OccupiedEntry` into a mutable reference to the value in the entry
    /// with a lifetime bound to the map itself.
    pub fn into_mut(self) -> &'a mut Token {
        self.value
    }
}

#[derive(Debug)]
pub struct VacantEntry<'a, S>
where
    S: TokenStore<'a>,
{
    chain_id: u8,
    id: TokenId,
    store: &'a mut S,
}

impl<'a, S> VacantEntry<'a, S>
where
    S: TokenStore<'a>,
{
    pub fn new(chain_id: u8, id: TokenId, store: &'a mut S) -> Self {
        Self {
            chain_id,
            id,
            store,
        }
    }

    /// Gets a reference to the [`TokenId`] that would be used when inserting a value
    /// through the [`VacantEntry`].
    pub const fn id(&self) -> &TokenId {
        &self.id
    }

    /// Gets the chain id that would be used when inserting a value through
    /// the [`VacantEntry`]
    pub const fn chain_id(&self) -> u8 {
        self.chain_id
    }

    pub fn insert(self, token: Token) -> &'a mut Token {
        self.store.insert(self.chain_id, token);
        self.store.get_mut(self.chain_id, self.id.clone()).unwrap()
    }
}
