use alloy_primitives::address;
use alloy_providers::provider::Provider;
use alloy_rpc_client::RpcClient;
use dotenv::dotenv;
use erc20::TokenStore;
use std::{env, sync::Arc};

#[tokio::test]
async fn test_token_store() {
    dotenv().ok();
    let eth_rpc = env::var("ETH_RPC").unwrap();

    let provider = Provider::new_with_client(
        RpcClient::builder()
            .reqwest_http(eth_rpc.parse().unwrap())
            .boxed(),
    );
    let token_store = TokenStore::new(1, Arc::new(provider));

    let dai = token_store
        .get(erc20::TokenId::Address(address!(
            "6B175474E89094C44Da98b954EedeAC495271d0F"
        )))
        .await
        .unwrap();

    assert_eq!(dai.symbol, "DAI");
}
