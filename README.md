# alloy-erc20

ERC20 is a Rust libary relying on [Alloy] allowing to interact with ERC-20
contracts.

[Alloy]: https://github.com/alloy-rs/alloy

## Installation

Add `alloy-erc20` to your `Cargo.toml`.

```toml
alloy-erc20 = "1.2"
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
* A `LazyTokenSigner` struct for executing write operations like `transfer`,
  `approve`, and `transferFrom` with a signer-capable provider.

## Testing

This library includes comprehensive integration tests using [testcontainers-modules] and [Anvil].

The integration tests:

* Run completely standalone without external RPC dependencies
* Deploy a mock ERC-20 contract to a blank Anvil dev chain
* Test all `LazyToken` and `LazyTokenSigner` operations
* Complete in approximately 1 second
* Require only Docker to be installed

To run the integration tests:

```bash
cargo test --test integration_tests
```

[testcontainers-modules]: https://github.com/testcontainers/testcontainers-modules
[Anvil]: https://book.getfoundry.sh/reference/anvil/
