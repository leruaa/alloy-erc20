use alloy::primitives::address;
use once_cell::sync::Lazy;

use crate::Token;

/// Wrapped Ether.
pub static WETH: Lazy<Token> = Lazy::<Token>::new(|| {
    Token::new(
        address!("82aF49447D8a07e3bd95BD0d56f35241523fBab1"),
        String::from("WETH"),
        18,
    )
});

/// Circle USD.
pub static USDC: Lazy<Token> = Lazy::<Token>::new(|| {
    Token::new(
        address!("af88d065e77c8cC2239327C5EDb3A432268e5831"),
        String::from("USDC"),
        6,
    )
});

/// Tether USD.
pub static USDT: Lazy<Token> = Lazy::<Token>::new(|| {
    Token::new(
        address!("Fd086bC7CD5C481DCC9C85ebE478A1C0b69FCbb9"),
        String::from("USDT"),
        6,
    )
});
