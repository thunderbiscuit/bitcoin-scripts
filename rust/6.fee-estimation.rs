#!/usr/bin/env rust-script

//! bdk 0.26.0
//! 
//! Estimate the fee rate required to confirm a transaction in a given target of blocks.
//! 
//! ```cargo
//! [dependencies]
//! bdk = "0.26.0"
//! ```

#![allow(unused_doc_comments)]

use bdk::blockchain::Blockchain;
use bdk::blockchain::ElectrumBlockchain;
use bdk::electrum_client::Client;

let client = Client::new("ssl://electrum.blockstream.info:60002").unwrap();
let blockchain = ElectrumBlockchain::from(client);
let num_blocks = 3;
let fee_estimate = blockchain.estimate_fee(num_blocks);

println!("The estimated fee for {} is {}", num_blocks, fee_estimate?.as_sat_per_vb());
