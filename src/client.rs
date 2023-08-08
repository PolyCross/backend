use std::env;

use dotenv::dotenv;
use ethers::prelude::*;

pub async fn get_sepolia_client() -> Provider<Ws> {
    dotenv().ok();
    let sepolia: String = env::var("SEPOLIA_RPC_URL").expect("SEPOLIA_RPC_URL must be set");
    let rpc_url: &str = sepolia.as_str();

    Provider::<Ws>::connect(rpc_url).await.unwrap()
}

pub fn get_mumbai_provider() -> Provider<Http> {
    dotenv().ok();
    let mumbai: String = env::var("MUMBAI_RPC_URL").expect("MUMBAI_RPC_URL must be set");
    let rpc_url: &str = mumbai.as_str();

    Provider::<Http>::try_from(rpc_url).unwrap()
}
