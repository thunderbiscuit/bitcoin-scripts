#!/usr/bin/env rust-script

//! bdk 0.26.0
//! 
//! Create public descriptors using the descriptor templates.
//! 
//! ```cargo
//! [dependencies]
//! bdk = "0.26.0"
//! ```

#![allow(unused_doc_comments)]

use std::str::FromStr;
use bdk::bitcoin::util::bip32::{ExtendedPubKey, Fingerprint};
use bdk::bitcoin::Network;
use bdk::template::{Bip44Public, Bip49Public, Bip84Public, DescriptorTemplate};

let key = ExtendedPubKey::from_str("tpubDDDzQ31JkZB7VxUr9bjvBivDdqoFLrDPyLWtLapArAi51ftfmCb2DPxwLQzX65iNcXz1DGaVvyvo6JQ6rTU73r2gqdEo8uov9QKRb7nKCSU").unwrap();
let fingerprint = Fingerprint::from_str("c55b303f").unwrap();

let descriptor_bip44_public = Bip44Public(key.clone(), fingerprint, bdk::KeychainKind::External).build(Network::Testnet).unwrap();
println!("The testnet BIP44 public external descriptor is {}\n", descriptor_bip44_public.0);

let descriptor_bip49_public = Bip49Public(key.clone(), fingerprint, bdk::KeychainKind::External).build(Network::Testnet).unwrap();
println!("The testnet BIP49 public external descriptor is {}\n", descriptor_bip49_public.0);

let descriptor_bip84_public = Bip84Public(key.clone(), fingerprint, bdk::KeychainKind::External).build(Network::Testnet).unwrap();
println!("The testnet BIP84 public external descriptor is {}", descriptor_bip84_public.0);
