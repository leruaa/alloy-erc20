# alloy-erc20

ERC20 is a Rust libary relying on [Alloy] allowing to interact with ERC-20
contracts.

[Alloy]: https://github.com/alloy-rs/alloy

## Installation

Add `alloy-erc20` to your `Cargo.toml`.

```toml
alloy-erc20 = "0.2"
```

## Features

* A basic `Token` struct and associated extensions methods on Alloy's
  `Provider`, allowing to retrieve token decimals, and compute balances
  as `BigDecimal` from `U256`.
* A `TokenStore` trait, and a `BasicTokenStore` impl, allowing to cache
  `Token`s in memory.
* A `LazyToken` struct, acting as a wrapper around Alloy contract instance,
  lazily retrieving `name`, `symbol`, `decimals` and `totalSupply` from the
  blockchain.
