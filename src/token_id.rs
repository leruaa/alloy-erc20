use std::{
    fmt::{Display, Formatter},
    hash::Hash,
};

use alloy::primitives::Address;

/// A token identifier.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum TokenId {
    /// Identify a token by its symbol
    Symbol(String),
    /// Identify a token by its address
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

impl From<Address> for TokenId {
    fn from(value: Address) -> Self {
        TokenId::Address(value)
    }
}
