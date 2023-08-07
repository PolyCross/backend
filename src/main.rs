mod abi;
use abi::Bridge;
mod event;
use event::BridgeIn;

use std::{
    env,
    fs::OpenOptions,
    io::{BufReader, Read},
    sync::Arc,
};

use dotenv::dotenv;
use ethers::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Net {
    pub sepolia: String,
    pub mumbai: String,
}

#[derive(Deserialize)]
pub struct Conf {
    pub contract_address: Net,
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let mut buf = BufReader::new(OpenOptions::new().read(true).open("config.toml")?);
    let mut conf = String::new();
    buf.read_to_string(&mut conf).unwrap();

    let config: Conf = toml::from_str(&conf).unwrap();

    let eth_client = get_eth_client().await;
    let eth_address = config.contract_address.sepolia.parse::<Address>().unwrap();

    let eth_contract = Bridge::new(eth_address, Arc::new(eth_client));

    let events = eth_contract.event::<BridgeIn>();
    let mut event_stream = events.subscribe_with_meta().await?;

    // Connect to the Matic client and get the contract address from the configuration
    let matic_provider = get_matic_provider().await;
    let matic_address: Address = config.contract_address.mumbai.parse::<Address>().unwrap();

    // Create a client with the wallet as the signer middleware
    let wallet = get_wallet().await;
    let client = SignerMiddleware::new(matic_provider, wallet);

    // Create a contract instance from the address and client with relevant ABI
    let matic_contract = Bridge::new(matic_address, Arc::new(client));

    while let Some(Ok((event, _))) = event_stream.next().await {
        println!("Bridge Event: {event:?}");

        let to = event.operator;
        let token = event.token;
        let amount = event.amount;

        let _tx = matic_contract
            .bridge_out(token, amount, to)
            .send()
            .await?
            .log_msg("Pending hash")
            .await?;
    }

    Ok(())
}

async fn get_eth_client() -> Provider<Ws> {
    dotenv().ok();
    let sepolia: String = env::var("SEPOLIA_RPC_URL").expect("SEPOLIA_RPC_URL must be set");
    let rpc_url: &str = sepolia.as_str();

    Provider::<Ws>::connect(rpc_url).await.unwrap()
}

async fn get_matic_provider() -> Provider<Http> {
    dotenv().ok();
    let mumbai: String = env::var("MUMBAI_RPC_URL").expect("MUMBAI_RPC_URL must be set");
    let rpc_url: &str = mumbai.as_str();

    Provider::<Http>::try_from(rpc_url).unwrap()
}

async fn get_wallet() -> LocalWallet {
    let private_key: String = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set");
    private_key
        .as_str()
        .parse::<LocalWallet>()
        .unwrap()
        .with_chain_id(Chain::PolygonMumbai)
}
