use alloy::primitives::address;
use alloy::providers::ProviderBuilder;
use alloy_erc20::LazyToken;
use dotenv::dotenv;
use std::env;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    dotenv().ok();

    let eth_rpc = env::var("ETH_RPC").unwrap();
    let provider = ProviderBuilder::new().on_http(eth_rpc.parse().unwrap());

    let dai = LazyToken::new(
        address!("6B175474E89094C44Da98b954EedeAC495271d0F"), // DAI
        provider,
    );

    // lazily query the network for the symbol and cache the result
    let symbol = dai.symbol().await.unwrap();

    // lazily query the network for the total supply and cache the result
    let total_supply = dai.total_supply().await.unwrap();

    println!(
        "{symbol} total supply: {}",
        dai.get_balance(total_supply).await.unwrap()
    )
}
