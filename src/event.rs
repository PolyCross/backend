use ethers::{
    prelude::EthEvent,
    types::{Address, U256},
};

#[derive(Clone, Debug, EthEvent)]
pub struct BridgeIn {
    #[ethevent(indexed)]
    pub operator: Address,
    #[ethevent(indexed)]
    pub token: Address,
    pub amount: U256,
}

#[derive(Clone, Debug, EthEvent)]
pub struct BridgeSwapIn {
    pub path: Vec<Address>,
    pub amountin: U256,
    #[ethevent(indexed)]
    pub receiver: Address
}