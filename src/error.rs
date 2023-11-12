use std::fmt::Display;

use thiserror::Error;

use crate::TokenId;

#[derive(Error, Debug)]
pub struct Error {
    pub token: TokenId,
    pub source: anyhow::Error,
}

impl Error {
    pub fn new<E: Into<anyhow::Error>>(token: TokenId, source: E) -> Self {
        Self {
            token,
            source: source.into(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Token {:?}: {}", self.token, self.source)
    }
}

#[derive(Error, Debug)]
#[error("The token is not present in store")]
pub struct NotInStoreError;
