use alloy::primitives::address;
use alloy::providers::ProviderBuilder;
use dotenv::dotenv;
use erc20::Erc20Provider;
use std::{env, sync::Arc};

#[tokio::test]
async fn test_retrieve_token() {
    dotenv().ok();
    let eth_rpc = env::var("ETH_RPC").unwrap();
    let provider = ProviderBuilder::new()
        .on_http(eth_rpc.parse().unwrap())
        .unwrap();
    let erc_provider = Erc20Provider::new(Arc::new(provider), 1);

    let dai = erc_provider
        .retrieve_token(address!("6B175474E89094C44Da98b954EedeAC495271d0F").into())
        .await
        .unwrap();

    assert_eq!(dai.symbol, "DAI");
}
