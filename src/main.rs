mod abi;
use abi::Bridge;
mod event;
use event::BridgeIn;
mod client;
mod get_address;

use std::{env, sync::Arc};

use dotenv::dotenv;
use ethers::prelude::*;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let sepolia_client = client::get_sepolia_client().await.unwrap();

    let sepolia_address = get_address::get_contract_address("sepolia").await.unwrap();

    let sepolia_contract = Bridge::new(sepolia_address, Arc::new(sepolia_client));

    // Connect to the mumbai client and get the contract address from the configuration
    let mumbai_provider = client::get_mumbai_provider().await.unwrap();

    let mumbai_address = get_address::get_contract_address("mumbai").await.unwrap();

    // Create a client with the wallet as the signer middleware
    let wallet = get_wallet().await;
    let client = SignerMiddleware::new(mumbai_provider, wallet);

    // Create a contract instance from the address and client with relevant ABI
    let mumbai_contract = Bridge::new(mumbai_address, Arc::new(client));

    let events = sepolia_contract.event::<BridgeIn>();
    // let mut event_stream = events.subscribe_with_meta().await?;
    let mut event_stream = events.subscribe().await?;

    while let Some(Ok(event)) = event_stream.next().await {
        println!("Bridge Event: {event:?}");

        let to = event.operator;
        let token = event.token;
        let amount = event.amount;

        let _tx = mumbai_contract
            .bridge_out(token, amount, to)
            .send()
            .await?
            .log_msg("Pending hash")
            .await?;

        println!("Finish one event");
    }

    Ok(())
}

async fn get_wallet() -> LocalWallet {
    dotenv().ok();
    let private_key: String = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set");
    private_key
        .as_str()
        .parse::<LocalWallet>()
        .unwrap()
        .with_chain_id(Chain::PolygonMumbai)
}
