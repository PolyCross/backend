use std::{
    fs::OpenOptions,
    io::{BufReader, Read},
};

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

pub async fn get_contract_address(network: &str) -> eyre::Result<Address> {
    let mut buf = BufReader::new(OpenOptions::new().read(true).open("config.toml")?);
    let mut conf = String::new();
    buf.read_to_string(&mut conf)?;

    let config: Conf = toml::from_str(&conf)?;

    let address = match network {
        "sepolia" => config.contract_address.sepolia.parse::<Address>().unwrap(),
        "mumbai" => config.contract_address.mumbai.parse::<Address>().unwrap(),
        _ => return Err(eyre::eyre!("Invalid network")),
    };

    Ok(address)
}
