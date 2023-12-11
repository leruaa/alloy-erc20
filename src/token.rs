use ethers::types::{Address, U256};
use rust_decimal::Decimal;

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

    pub fn get_balance(&self, amount: U256) -> Decimal {
        Decimal::new(amount.to_string().parse().unwrap(), self.decimals as u32)
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.symbol == other.symbol
    }
}
