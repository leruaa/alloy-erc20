use alloy::primitives::address;
use once_cell::sync::Lazy;

use crate::Token;

/// Ether.
pub static ETH: Lazy<Token> = Lazy::<Token>::new(|| {
    Token::new(
        address!("EeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE"),
        String::from("ETH"),
        18,
    )
});

/// Wrapped Ether.
pub static WETH: Lazy<Token> = Lazy::<Token>::new(|| {
    Token::new(
        address!("C02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"),
        String::from("WETH"),
        18,
    )
});

/// Wrapped Bitcoin.
pub static WBTC: Lazy<Token> = Lazy::<Token>::new(|| {
    Token::new(
        address!("2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599"),
        String::from("WBTC"),
        8,
    )
});

/// Circle USD.
pub static USDC: Lazy<Token> = Lazy::<Token>::new(|| {
    Token::new(
        address!("A0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"),
        String::from("USDC"),
        6,
    )
});

/// Tether USD.
pub static USDT: Lazy<Token> = Lazy::<Token>::new(|| {
    Token::new(
        address!("dAC17F958D2ee523a2206206994597C13D831ec7"),
        String::from("USDT"),
        6,
    )
});

/// Dai stablecoin.
pub static DAI: Lazy<Token> = Lazy::<Token>::new(|| {
    Token::new(
        address!("6B175474E89094C44Da98b954EedeAC495271d0F"),
        String::from("DAI"),
        18,
    )
});
