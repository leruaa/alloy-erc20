use alloy::primitives::{address, U256};
use alloy::providers::ProviderBuilder;
use alloy_erc20::{BasicTokenStore, Erc20ProviderExt, TokenId, TokenStore};
use dotenv::dotenv;
use std::env;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    dotenv().ok();

    let eth_rpc = env::var("ETH_RPC").unwrap();
    let provider = ProviderBuilder::new().on_http(eth_rpc.parse().unwrap());

    let mut store = BasicTokenStore::new();

    // Just retrieve a token from its address, and add it to the store
    let _dai = provider
        .get_token(
            address!("6B175474E89094C44Da98b954EedeAC495271d0F"),
            &mut store,
        )
        .await
        .unwrap();

    // ...

    // Now the token can be retrieved from its symbol or address from the store
    let dai = store.get(1, TokenId::Symbol("DAI".to_string())).unwrap();

    // Compute a balance as a BigDecimal from a U256
    let balance = dai.get_balance(U256::from(1000000000000_u64));

    println!("Balance: {balance}")
}
