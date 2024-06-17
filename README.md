# ERC20

ERC20 is a Rust libary relying on [Alloy] allowing to interact with ERC-20
contracts.

[Alloy]: https://github.com/alloy-rs/alloy

## Installation

Add `erc20-rs` to your `Cargo.toml`.

```toml
erc20-rs = { git = "https://github.com/leruaa/erc20-rs" }
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

## Getting started

```rust
let provider = ProviderBuilder::new().on_http("https://...".parse()?);

// Just retrieve a token from its address
let dai = provider
    .retrieve_token(address!("6B175474E89094C44Da98b954EedeAC495271d0F"))
    .await?;

// Compute a balance as a BigDecimal from a U256
let balance = dai.get_balance(U256::from(1000000000000_u64));
```