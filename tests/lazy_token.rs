use alloy::{primitives::address, providers::ProviderBuilder};
use alloy_erc20::LazyToken;
use dotenv::dotenv;
use std::env;

#[tokio::test]
async fn test_lazy_token() {
    dotenv().ok();
    let eth_rpc = env::var("ETH_RPC").unwrap();
    let provider = ProviderBuilder::new().on_http(eth_rpc.parse().unwrap());

    let dai = LazyToken::new(
        address!("6B175474E89094C44Da98b954EedeAC495271d0F"),
        provider,
    );

    let name = dai.name().await.unwrap();

    assert_eq!(name, "Dai Stablecoin")
}
