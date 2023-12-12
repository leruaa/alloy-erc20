use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("The token is not present in store")]
    NotInStoreError,
    #[error("{0}")]
    Ethers(String),
}
