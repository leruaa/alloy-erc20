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

## Getting started

### Basic usage

```rust
let provider = ProviderBuilder::new().on_http("https://...".parse()?);

// Just retrieve a token from its address
let dai = provider
    .retrieve_token(address!("6B175474E89094C44Da98b954EedeAC495271d0F"))
    .await?;

// Compute a balance as a BigDecimal from a U256
let balance = dai.get_balance(U256::from(1000000000000_u64));
```

### Store

```rust
let provider = ProviderBuilder::new().on_http("https://...".parse()?);

let store = BasicTokenStore::new();

// Just retrieve a token from its address, and add it to the store
let dai = provider
    .get_token(address!("6B175474E89094C44Da98b954EedeAC495271d0F").into(), &store)
    .await?;

// ...

// Now the token can be retrieved from its symbol or address from the store
let dai = store.get(1, TokenId::Symbol("DAI".to_string())).unwrap();

// Compute a balance as a BigDecimal from a U256
let balance = dai.get_balance(U256::from(1000000000000_u64));
```

### Lazy
```rust
let provider = ProviderBuilder::new().on_http("https://...".parse()?);

let dai = LazyToken::new(address!("6B175474E89094C44Da98b954EedeAC495271d0F"), provider);

// lazily query the network for the decimals and cache the result
let total_supply = dai.decimals()?;
```