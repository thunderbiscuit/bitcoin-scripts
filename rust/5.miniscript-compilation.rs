#!/usr/bin/env rust-script

//! bdk 0.26.0
//!
//! Build a BDK wallet starting from a miniscript policy.
//!
//! ```cargo
//! [dependencies]
//! bdk = { version = "0.26.0", features = ["compiler"] }
//! serde_json = { version = "1.0" }
//! ```

#![allow(unused_doc_comments)]

use std::str::FromStr;
use bdk::bitcoin::Network;
use bdk::miniscript::policy::Concrete;
use bdk::miniscript::Descriptor;
use bdk::database::memory::MemoryDatabase;
use bdk::wallet::AddressIndex;
use bdk::{KeychainKind, Wallet};

// We start with a generic miniscript policy string
let policy_str = "or(10@thresh(4,pk(029ffbe722b147f3035c87cb1c60b9a5947dd49c774cc31e94773478711a929ac0),pk(025f05815e3a1a8a83bfbb03ce016c9a2ee31066b98f567f6227df1d76ec4bd143),pk(025625f41e4a065efc06d5019cbbd56fe8c07595af1231e7cbc03fafb87ebb71ec),pk(02a27c8b850a00f67da3499b60562673dcf5fdfb82b7e17652a7ac54416812aefd),pk(03e618ec5f384d6e19ca9ebdb8e2119e5bef978285076828ce054e55c4daf473e2)),1@and(older(4209713),thresh(2,pk(03deae92101c790b12653231439f27b8897264125ecb2f46f48278603102573165),pk(033841045a531e1adf9910a6ec279589a90b3b8a904ee64ffd692bd08a8996c1aa),pk(02aebf2d10b040eb936a6f02f44ee82f8b34f5c1ccb20ff3949c2b28206b7c1068))))";
println!("Compiling policy:\n{}", policy_str);

// Parse the string as a Concrete type miniscript policy.
let policy = Concrete::<String>::from_str(policy_str).unwrap();

// Create a `wsh` type descriptor from the policy.
// `policy.compile()` returns the resulting miniscript from the policy.
let descriptor = Descriptor::new_wsh(policy.compile().unwrap()).unwrap();

println!("Compiled into following Descriptor: \n{}", descriptor);

let database = MemoryDatabase::new();

// Create a new wallet from this descriptor
let wallet = Wallet::new(&format!("{}", descriptor), None, Network::Regtest, database)?;

println!(
    "First derived address from the descriptor: \n{}",
    wallet.get_address(AddressIndex::New)?
);

// BDK also has it's own `Policy` structure to represent the spending condition in a more
// human readable json format.
let spending_policy = wallet.policies(KeychainKind::External)?;
println!("The BDK spending policy: \n{}", serde_json::to_string_pretty(&spending_policy)?);
