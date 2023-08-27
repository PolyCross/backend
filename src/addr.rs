use ethers::prelude::Address;

pub enum Network {
    Sepolia,
    Mumbai
}

pub fn get_bridge_addr(net: Network) -> Address {
    let sepolia_addr = "0xfad4E0022f540B313BD8A77137E2f51e031810F1";
    let sepolia_addr: Address = sepolia_addr.parse::<Address>().unwrap();

    let mumbai_addr = "0xfc99C6B880D548f9172EdDe156B9d285F5027dD6";
    let mumbai_addr: Address = mumbai_addr.parse::<Address>().unwrap();

    match net {
        Network::Sepolia => return sepolia_addr,
        Network::Mumbai => return mumbai_addr,
    };
}

pub fn get_bridge_swap_addr(net: Network) -> Address {
    let sepolia_addr = "0x651611eB218D30Ab736c8548F7DB5F5F0b35Fa91";
    let sepolia_addr: Address = sepolia_addr.parse::<Address>().unwrap();

    let mumbai_addr = "0x9652B3320550CeEe00BcA395060fad3419A94DC5";
    let mumbai_addr: Address = mumbai_addr.parse::<Address>().unwrap();

    match net {
        Network::Sepolia => return sepolia_addr,
        Network::Mumbai => return mumbai_addr,
    };
}
