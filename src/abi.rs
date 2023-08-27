use ethers::prelude::abigen;

abigen!(
    Bridge,
    r#"[
        event BridgeIn(address indexed operator, address indexed token, uint256 amount)
        function bridgeOut(address token, uint256 amount, address to) public
    ]"#,
);

abigen!(
    BridgeSwap,
    r#"[
        event BridgeSwapIn(address[] path, uint256 amountIn, address indexed receiver)
        function swapOut(uint256 amountIn, address[] calldata path, address to) public returns (uint256[] memory amounts)
    ]"#,
);