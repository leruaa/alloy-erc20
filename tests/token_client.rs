use alloy_network::Ethereum;
use alloy_primitives::address;
use alloy_provider::ProviderBuilder;
use alloy_rpc_client::RpcClient;
use dotenv::dotenv;
use erc20::clients::TokenClient;
use std::{env, sync::Arc};

#[tokio::test]
async fn test_retrieve_token() {
    dotenv().ok();
    let eth_rpc = env::var("ETH_RPC").unwrap();
    let rpc_client = RpcClient::builder().reqwest_http(eth_rpc.parse().unwrap());
    let provider = ProviderBuilder::<_, Ethereum>::new().on_client(rpc_client);
    let token_client = TokenClient::new(Arc::new(provider));

    let dai = token_client
        .retrieve_token(address!("6B175474E89094C44Da98b954EedeAC495271d0F"))
        .await
        .unwrap();

    assert_eq!(dai.symbol, "DAI");
}
