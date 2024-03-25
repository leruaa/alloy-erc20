use std::fmt::Display;

use thiserror::Error;

use crate::TokenId;

#[derive(Error, Debug)]
pub struct Error {
    pub token: TokenId,
    pub source: InternalError,
}

impl Error {
    pub fn new<E: Into<InternalError>>(token: TokenId, source: E) -> Self {
        Self {
            token,
            source: source.into(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Token {}: {}", self.token, self.source)
    }
}

#[derive(Error, Debug)]
pub enum InternalError {
    #[error("The token is not present in store")]
    NotInStore,
    #[error("Failed to query token: {0}")]
    Transport(#[from] alloy::transports::TransportError),
    #[error("Contract error: {0}")]
    Contract(#[from] alloy::contract::Error),
    #[error("Failed to decode token: {0}")]
    Sol(#[from] alloy_sol_types::Error),
}
