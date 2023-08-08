use std::env;

use dotenv::dotenv;
use ethers::prelude::*;

pub async fn get_sepolia_client() -> eyre::Result<Provider<Ws>> {
    dotenv().ok();
    let sepolia: String = env::var("SEPOLIA_RPC_URL").expect("SEPOLIA_RPC_URL must be set");
    let rpc_url: &str = sepolia.as_str();

    let res = Provider::<Ws>::connect(rpc_url).await.unwrap();
    Ok(res)
}

pub async fn get_mumbai_provider() -> eyre::Result<Provider<Http>> {
    dotenv().ok();
    let mumbai: String = env::var("MUMBAI_RPC_URL").expect("MUMBAI_RPC_URL must be set");
    let rpc_url: &str = mumbai.as_str();

    let res = Provider::<Http>::try_from(rpc_url).unwrap();
    Ok(res)
}
