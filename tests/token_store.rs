use alloy_primitives::address;
use alloy_providers::provider::Provider;
use alloy_transport::BoxTransport;
use alloy_transport_http::Http;
use dotenv::dotenv;
use erc20::TokenStore;
use std::sync::Arc;

#[tokio::test]
async fn test_token_store() {
    dotenv().ok();

    let transport = Http::<reqwest::Client>::new(
        "https://eth-mainnet.g.alchemy.com/v2/7xeGs22pDCMlsMC5ZiasB0XfC41g0F9z"
            .parse()
            .unwrap(),
    );
    let transport = BoxTransport::new(transport);
    let provider = Provider::new(transport);
    let token_store = TokenStore::new(1, Arc::new(provider));

    let dai = token_store
        .get(erc20::TokenId::Address(address!(
            "6B175474E89094C44Da98b954EedeAC495271d0F"
        )))
        .await
        .unwrap();

    assert_eq!(dai.symbol, "DAI");
}
