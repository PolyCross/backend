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
