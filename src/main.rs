mod abi;
use abi::{Bridge, BridgeSwap};
mod event;
use event::{BridgeIn, BridgeSwapIn};
mod addr;
use addr::{get_bridge_addr, get_bridge_swap_addr, Network};
mod client;
mod wallet;
use wallet::get_wallet;

use ethers::prelude::*;
use std::sync::Arc;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    bridge().await?;
    bridge_swap().await?;

    Ok(())
}

async fn bridge() -> eyre::Result<()> {
    let sepolia_client = client::get_sepolia_client().await;

    let sepolia_address: Address = get_bridge_addr(Network::Sepolia);

    let sepolia_contract = Bridge::new(sepolia_address, Arc::new(sepolia_client));

    // Connect to the mumbai client and get the contract address from the configuration
    let mumbai_provider = client::get_mumbai_provider();

    let mumbai_address: Address = get_bridge_addr(Network::Mumbai);

    // Create a client with the wallet as the signer middleware
    let wallet = get_wallet(Chain::PolygonMumbai);
    let client = SignerMiddleware::new(mumbai_provider, wallet);

    // Create a contract instance from the address and client with relevant ABI
    let mumbai_contract = Bridge::new(mumbai_address, Arc::new(client));

    let events = sepolia_contract.event::<BridgeIn>();
    
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

        println!("Bridge Successfully");
    }

    Ok(())
}

async fn bridge_swap() -> eyre::Result<()> {
    let sepolia_client = client::get_sepolia_client().await;

    let sepolia_address: Address = get_bridge_swap_addr(Network::Sepolia);

    let sepolia_contract = BridgeSwap::new(sepolia_address, Arc::new(sepolia_client));

    // Connect to the mumbai client and get the contract address from the configuration
    let mumbai_provider = client::get_mumbai_provider();

    let mumbai_address: Address = get_bridge_swap_addr(Network::Mumbai);

    // Create a client with the wallet as the signer middleware
    let wallet = get_wallet(Chain::PolygonMumbai);
    let client = SignerMiddleware::new(mumbai_provider, wallet);

    // Create a contract instance from the address and client with relevant ABI
    let mumbai_contract = BridgeSwap::new(mumbai_address, Arc::new(client));

    let events = sepolia_contract.event::<BridgeSwapIn>();
    
    let mut event_stream = events.subscribe().await?;

    while let Some(Ok(event)) = event_stream.next().await {
        println!("Bridge Swap Event: {event:?}");

        let path = event.path;
        let amount_in = event.amountin;
        let receiver = event.receiver;

        let _tx = mumbai_contract
            .swap_out(amount_in, path, receiver)
            .send()
            .await?
            .log_msg("Pending hash")
            .await?;

        println!("Bridge Swap Successfully");
    }

    Ok(())
}