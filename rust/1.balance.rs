#!/usr/bin/env rust-script

//! bdk 0.26.0
//! 
//! Query an Electrum server for the balance of a wallet.
//! 
//! ```cargo
//! [dependencies]
//! bdk = "0.26.0"
//! ```

#![allow(unused_doc_comments)]

use bdk::{Wallet, SyncOptions};
use bdk::database::MemoryDatabase;
use bdk::blockchain::ElectrumBlockchain;
use bdk::electrum_client::Client;

let client = Client::new("ssl://electrum.blockstream.info:60002").unwrap();
let blockchain = ElectrumBlockchain::from(client);
let wallet = Wallet::new(
    "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)",
    Some("wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)"),
    bdk::bitcoin::Network::Testnet,
    MemoryDatabase::default(),
)?;

wallet.sync(&blockchain, SyncOptions::default()).unwrap();
let balance = wallet.get_balance().unwrap();

println!("Descriptor balance: {} SAT", balance);
