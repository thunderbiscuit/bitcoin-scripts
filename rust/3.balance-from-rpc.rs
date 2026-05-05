#!/usr/bin/env rust-script

//! bdk 0.26.0
//! 
//! Query a full node through RPC for the balance of a descriptor.
//! 
//! ```cargo
//! [dependencies]
//! bdk = { version = "0.26.0", features = ["rpc"] }
//! ```

#![allow(unused_doc_comments)]

use bdk::bitcoin::Network;
use bdk::blockchain::{ConfigurableBlockchain, RpcBlockchain, RpcConfig};
use bdk::blockchain::rpc::{Auth, RpcSyncParams};
use bdk::database::MemoryDatabase;
use bdk::{SyncOptions, Wallet};

let rpc_sync_parameters = RpcSyncParams {
        start_script_count: 0,
        start_time: 0,
        force_start_time: false,
        poll_rate_sec: 30,
    };

let rpc_config = RpcConfig {
    url: "http://127.0.0.1:18443".to_string(),
    // auth: Auth::UserPass {
    //     username: "__cookie__".to_string(),
    //     password: "abcdefg".to_string(),
    // },
    // place a copy of your cookie file in the same directory as your script
    // or provide the path to your cookie file
    auth: Auth::Cookie {
        file: "./.cookie".to_string().parse().unwrap()
    },
    network: Network::Regtest,
    wallet_name: "scriptwallet".to_string(),
    sync_params: Some(rpc_sync_parameters),
};

let blockchain = RpcBlockchain::from_config(&rpc_config).unwrap();

let wallet = Wallet::new(
    "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)",
    None,
    Network::Regtest,
    MemoryDatabase::default(),
)?;

wallet.sync(&blockchain, SyncOptions::default()).unwrap();

println!("Descriptor balance: {} satoshis", wallet.get_balance().unwrap());
