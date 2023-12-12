
use bigdecimal::{
    num_bigint::{BigInt, Sign},
    BigDecimal,
};
use ethers::types::{Address, U256};

#[derive(Debug, Clone)]
pub struct Token {
    pub address: Address,
    pub symbol: String,
    pub decimals: u8,
}

impl Token {
    pub fn new(address: Address, symbol: &str, decimals: u8) -> Self {
        Self {
            address,
            symbol: String::from(symbol),
            decimals,
        }
    }

    pub fn get_balance(&self, amount: U256) -> BigDecimal {
        let mut bytes = [0; 32];

        amount.to_big_endian(&mut bytes);

        BigDecimal::from((
            BigInt::from_bytes_be(Sign::Plus, &bytes),
            self.decimals as i64,
        ))
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.symbol == other.symbol
    }
}
