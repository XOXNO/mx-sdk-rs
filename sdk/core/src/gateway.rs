mod gateway_account;
mod gateway_block;
mod gateway_network;
mod gateway_proxy;
mod gateway_tx;
mod gateway_tx_retrieve;
mod gateway_chain_simulator;

pub use gateway_proxy::GatewayProxy;

pub const MAINNET_GATEWAY: &str = "https://gateway.multiversx.com";
pub const TESTNET_GATEWAY: &str = "https://testnet-gateway.multiversx.com";
pub const DEVNET_GATEWAY: &str = "https://devnet-gateway.multiversx.com";
pub const SIMULATOR_GATEWAY: &str = "http://localhost:8085";

// MetachainShardId will be used to identify a shard ID as metachain
pub const METACHAIN_SHARD_ID: u32 = 0xFFFFFFFF;
