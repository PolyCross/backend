use ethers::prelude::abigen;

abigen!(
    Bridge,
    r#"[
        event BridgeIn(address indexed operator, address indexed token, uint256 amount)
        function bridgeOut(address token, uint256 amount, address to) public
    ]"#,
);
