use alloy_primitives::address;
use once_cell::sync::Lazy;

use crate::Token;

pub static ETH: Lazy<Token> = Lazy::<Token>::new(|| {
    Token::new(
        address!("EeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE"),
        String::from("ETH"),
        18,
    )
});

pub static WETH: Lazy<Token> = Lazy::<Token>::new(|| {
    Token::new(
        address!("C02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"),
        String::from("WETH"),
        18,
    )
});

pub static USDT: Lazy<Token> = Lazy::<Token>::new(|| {
    Token::new(
        address!("dAC17F958D2ee523a2206206994597C13D831ec7"),
        String::from("USDT"),
        6,
    )
});
