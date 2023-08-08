use dotenv::dotenv;
use ethers::prelude::*;
use std::env;

pub fn get_wallet(net: Chain) -> LocalWallet {
    dotenv().ok();
    let private_key: String = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set");
    private_key
        .as_str()
        .parse::<LocalWallet>()
        .unwrap()
        .with_chain_id(net)
}
