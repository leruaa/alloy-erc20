use alloy_primitives::{Address, U256};
use bigdecimal::{
    num_bigint::{BigInt, Sign},
    BigDecimal,
};

/// A token
#[derive(Debug, Clone)]
pub struct Token {
    pub address: Address,
    pub symbol: String,
    pub decimals: u8,
}

impl Token {
    pub const fn new(address: Address, symbol: String, decimals: u8) -> Self {
        Self {
            address,
            symbol,
            decimals,
        }
    }

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
