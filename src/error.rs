use std::fmt::Display;

use crate::TokenId;

/// Token related error.
#[derive(thiserror::Error, Debug)]
pub struct Error {
    /// The error token.
    pub token: TokenId,
    /// The error details.
    pub source: InternalError,
}

impl Error {
    /// Creates a new [`Error`]
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

/// Token related possible errors
#[derive(thiserror::Error, Debug)]
pub enum InternalError {
    #[error("The token {0} is not present in the store")]
    NotInStore(String),
    #[error("Failed to query token: {0}")]
    Transport(#[from] alloy::transports::TransportError),
    #[error("Contract error: {0}")]
    Contract(#[from] alloy::contract::Error),
    #[error("Failed to decode token: {0}")]
    Sol(#[from] alloy::sol_types::Error),
}
