use thiserror::Error;

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
        write!(f, "Token {:?}: {}", self.token, self.source)
    }
}

#[derive(Error, Debug)]
pub enum InternalError {
    #[error("The token is not present in store")]
    NotInStore,
    #[error(transparent)]
    Transport(#[from] alloy_transport::TransportError),
    #[error(transparent)]
    Sol(#[from] alloy_sol_types::Error),
}
