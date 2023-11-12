use once_cell::sync::Lazy;

use crate::Token;

pub static ETH: Lazy<Token> = Lazy::<Token>::new(|| {
    let address = "0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE"
        .parse()
        .unwrap();

    Token::new(address, "ETH", 18)
});

pub static WETH: Lazy<Token> = Lazy::<Token>::new(|| {
    let address = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"
        .parse()
        .unwrap();

    Token::new(address, "WETH", 18)
});

pub static USDT: Lazy<Token> = Lazy::<Token>::new(|| {
    let address = "0xdAC17F958D2ee523a2206206994597C13D831ec7"
        .parse()
        .unwrap();

    Token::new(address, "USDT", 6)
});
