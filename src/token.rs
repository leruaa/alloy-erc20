use alloy::primitives::{Address, U256};
use bigdecimal::{
    num_bigint::{BigInt, Sign},
    BigDecimal,
};

/// A token.
#[derive(Debug, Clone)]
pub struct Token {
    /// The token address.
    pub address: Address,
    /// The token symbol.
    pub symbol: String,
    /// The token decimals
    pub decimals: u8,
}

impl Token {
    /// Creates a new token.
    pub const fn new(address: Address, symbol: String, decimals: u8) -> Self {
        Self {
            address,
            symbol,
            decimals,
        }
    }

    /// Gets the token balance as a [`BigDecimal`]
    pub fn get_balance(&self, amount: U256) -> BigDecimal {
        BigDecimal::from((
            BigInt::from_bytes_be(Sign::Plus, &amount.to_be_bytes::<{ U256::BYTES }>()),
            self.decimals as i64,
        ))
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.symbol == other.symbol
    }
}
