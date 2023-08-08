use dotenv::dotenv;
use ethers::prelude::*;
use std::env;

pub async fn get_wallet() -> eyre::Result<LocalWallet> {
    dotenv().ok();
    let private_key: String = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set");
    let res = private_key
        .as_str()
        .parse::<LocalWallet>()
        .unwrap()
        .with_chain_id(Chain::PolygonMumbai);

    Ok(res)
}
