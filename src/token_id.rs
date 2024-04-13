use std::{
    fmt::{Display, Formatter},
    hash::Hash,
};

use alloy::primitives::Address;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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

impl From<Address> for TokenId {
    fn from(value: Address) -> Self {
        TokenId::Address(value)
    }
}
