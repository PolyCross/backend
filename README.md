# Introduction

This is the back end of **PolyCross** which is a Rust project.

It achieves the monitoring of smart contract events and the automatic invocation of related functions.

## Installation

First, clone the repo to localhost:

```bash
git clone https://github.com/PolyCross/backend.git
```

Second, install the dependencies:

```bash
cargo build
```

Then, run the server:

```bash
cargo run
```

## Acknowledgements

- Thanks to crate [ethers-rs](https://docs.rs/ethers/latest/ethers/) for enabling interaction with the EVM blockchain.