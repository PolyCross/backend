use ethers::prelude::Address;

pub enum Network {
    Sepolia,
    Mumbai
}

pub fn get_addr(net: Network) -> Address {
    let sepolia_addr = "0x9fE146cFfa1dBfff382fFf19f56C55F44DA2baA5";
    let sepolia_addr: Address = sepolia_addr.parse::<Address>().unwrap();

    let mumbai_addr = "0x9fE146cFfa1dBfff382fFf19f56C55F44DA2baA5";
    let mumbai_addr: Address = mumbai_addr.parse::<Address>().unwrap();

    match net {
        Network::Sepolia => return sepolia_addr,
        Network::Mumbai => return mumbai_addr,
    };
}
