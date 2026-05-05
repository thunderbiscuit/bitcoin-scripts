#!/usr/bin/env rust-script

//! Accepts a list of pubkeys and a miniscript policy in a json file. Spits out another 
//! json file with ton of useful information.
//! 
//! Input filename is passed in as the first arg. If no args are provided a default 
//! policy will be used. Check the DEFAULT_INPUT const below.
//! Output filename is passed in a as the second arg. Default: 'output.json'
//! 
//! input file example:
//! ```
//! {
//! 	"keys" : {
//! 		"pubkey1" : "[22739455/84'/1'/0']tpubDDEqu57tdMjDiQhEobb2P2X8G6XMH1Vrrq3yhJmNSJtRT5gLLzAsXpFiKLGHPTDREkkeFaAmuzaDkCF4Kj9iMJggXLb48QyBwwP9CK94iZa/0/*",
//! 		"pubkey2" : "[3a686ab9/84'/1'/0']tpubDDAf2xGr2RqMHQwJBaYqYDr4dA3pYtgM1aCw9PeHSoUEQd9RYPKcjvZW42QT2cvNHHxa74NYcfw3jbyfZGWWwFJNWYHqXRVkp32jG2q1UjB/0/*",
//! 		"pubkey3" : "[90e343cc/84'/1'/0'/0]tpubDFhuJBvU6ajt4LSTFjX3SRLhWzsZazEbVuL8eaa91dHSq4n8WUjHDUoGNNXSBWyufh83zsBQhkRwk8mmAERbGQQhgUM6Nm3S1c5h1GG5uf5/*",
//! 		"pubkey4" : "[44eeebb2/84'/1'/0'/0]tpubDFStCq8evTuo4BQSdXKvjmKvXMasypdr17WfnpT7zfPE45gAJfqhFVdjXf43MbqymqMWhLyLfjQkqwRKFPsPRJAGLjXX7DqeWWEocAwccMH/*",
//! 		"pubkey5" : "[df19b3b1/84'/1'/0'/0]tpubDEu2VoQEB3RyLRXzYvGR3YzVQypeF7iLUk9r4FqLM9NRcYVc5b4VfECrXq4np4s5tvSg6ZPGaSpEknjk8sKR2H5VJm9Bo9CcFPQ6ZXA7Yza/*"
//! 	},
//! 	"policy" : "and(pk($PUBKEY1),or(pk($PUBKEY2),or(after(10000),and(older(5),thresh(2,pk($PUBKEY3),pk($PUBKEY4),pk($PUBKEY5))))))"
//! }
//! ```
//! 
//! output file includes:
//! - current wallet balance (script connects to electrum.blockstream.info)
//! - transaction history
//! - external spend policies
//! - internal spend policies
//! - 10 addresses
//! - external descriptor
//! - internal descriptor
//! - human readable external spend policy structure (intended for wallet UI use)
//! 
//! ```cargo
//! [dependencies]
//! serde = "1.0"
//! serde_json = "1.0"
//! bdk = { version = "0.25.0", default-features = false, features = ["compiler", "electrum", "all-keys"]}
//! bitcoin = "0.29.2"
//! ```

extern crate serde_json;
extern crate serde;
extern crate bitcoin;

use std::env;
use std::fmt::Display;
use std::fs::File;
use std::str::FromStr;
use std::io::prelude::*;
use bdk::database::MemoryDatabase;
use bdk::electrum_client::{Client};
use bdk::miniscript::policy::Concrete;
use bdk::descriptor::{Descriptor, Miniscript, ExtendedDescriptor};
use bdk::descriptor::Segwitv0;
use bdk::{wallet::AddressIndex, Error, KeychainKind, Wallet, SyncOptions};
use bdk::blockchain::{ElectrumBlockchain};
use bdk::{Balance, TransactionDetails};
use bdk::descriptor::policy::{Policy, SatisfiableItem, PkOrF};
use serde::{Serialize, Deserialize};
use serde_json::{json};

const DEFAULT_INPUT: &str = "{\"keys\":{\"pubkey1\":\"[22739455/84'/1'/0']tpubDDEqu57tdMjDiQhEobb2P2X8G6XMH1Vrrq3yhJmNSJtRT5gLLzAsXpFiKLGHPTDREkkeFaAmuzaDkCF4Kj9iMJggXLb48QyBwwP9CK94iZa/0/*\",\"pubkey2\" : \"[3a686ab9/84'/1'/0']tpubDDAf2xGr2RqMHQwJBaYqYDr4dA3pYtgM1aCw9PeHSoUEQd9RYPKcjvZW42QT2cvNHHxa74NYcfw3jbyfZGWWwFJNWYHqXRVkp32jG2q1UjB/0/*\"},\"policy\" : \"and(pk($PUBKEY1),or(99@pk($PUBKEY2),after(5)))\"}";
const OUTPUT_FILE: &str = "output.json";
const NUM_ADDRESSES: u32 = 10;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut file_contents = String::from(DEFAULT_INPUT);
    let mut output_file = OUTPUT_FILE.to_string();
    let mut filename = String::from("default input");
    if env::args().len() > 1 {
        filename = env::args().nth(1).unwrap();
        file_contents = read_file(&filename).unwrap();
        if env::args().len() > 2 {
            output_file = env::args().nth(2).unwrap();
        }
    }

    let policy = get_policy(&file_contents, &filename).map_err(|err| err.to_string())?;
    
    let policy = Concrete::<String>::from_str(&policy)?;
    let segwit_policy: Miniscript<String, Segwitv0> = policy
        .compile()
        .map_err(|e| Error::Generic(e.to_string()))?;

    let descriptor = Descriptor::new_wsh(segwit_policy.clone()).unwrap().to_string();

    let client = Client::new("ssl://electrum.blockstream.info:60002")?;
    let blockchain = ElectrumBlockchain::from(client);
    let wallet = Wallet::new(
        &descriptor,
        Some(&descriptor),
        bitcoin::Network::Testnet,
        MemoryDatabase::default(),
    )?;

    wallet.sync(&blockchain, SyncOptions::default())?;

    generate_output_files(&wallet, output_file).expect("error generating output files");

    Ok(())
}

fn get_policy_descriptions(policy: &Policy, depth: u32) -> serde_json::Value {
    let children: Option<serde_json::Value> = match &policy.item {
        SatisfiableItem::Thresh {items, threshold: _} => {
            let mut policy_descriptions:Vec<serde_json::Value> = Vec::new();
            for item in items {
                policy_descriptions.push(get_policy_descriptions(&item, depth + 1));
            }
            Some(json!(policy_descriptions))
        }
        SatisfiableItem::Multisig {keys, threshold: _} => {
            let mut policy_descriptions:Vec<serde_json::Value> = Vec::new();

            for key in keys {
                policy_descriptions.push(json!(&display_key(key)));
            }
            Some(json!(policy_descriptions))
        }
        _ => None,
    };

    let mut json = json!({
        "description": description(&policy),
        "depth": depth,
        "policy_id": &policy.id,
    });

    // include child policies if they exist
    // there HAS to be a better way to add a field to an existing json object
    if children.is_some() {
        json = match json {
            serde_json::Value::Object(m) => {
                let mut m = m.clone();
                m.insert(String::from("items"), children.unwrap());
                serde_json::Value::Object(m)
            },
            v => v.clone(),
        };
    }

    json
}

fn get_policy(json: &str, filename: &str) -> Result<String, String> {
    // Parse the string of data into a JSON value.
    // TODO validate pubkey and policy inputs
    let json: serde_json::Value = serde_json::from_str(&json).map_err(|err| err.to_string())?;
    
    let keys = json["keys"].as_object().ok_or_else(|| format!("keys not found in {filename}"))?;
    let mut policy = json["policy"].as_str().ok_or_else(|| format!("`policy` not found in {filename}"))?.to_owned();
    
    for (key, value) in keys.iter() {
        let placeholder = "$".to_owned() + &key.to_uppercase();
        policy = policy.replace(&placeholder, value.as_str().expect("`keys` must be a map of strings"));
    }

    Ok(policy.to_owned())
}

fn read_file(filename: &str) -> Result<String, String> {
    // Open the file in read-only mode with buffer.
    let mut file = File::open(filename).map_err(|err| err.to_string())?;
    let mut data = String::new();
    file.read_to_string(&mut data).map_err(|err| err.to_string())?;
    Ok(data)
}

fn generate_output_files(wallet: &Wallet<MemoryDatabase>, output_file: String) -> Result<(), Box<dyn std::error::Error>> {
    // get addresses
    // Note: this code returns the same address every time unless you specify an extended key descriptor i.e. one that ends in \*
    // TODO distinguish and handle single key vs. extended key descriptors
    let mut addresses = Vec::new();
    (0..NUM_ADDRESSES).for_each(|_i: u32| {
        addresses.push(wallet.get_address(AddressIndex::New).expect("error retrieving next address").to_string())
    });

    let extern_policies = &wallet.policies(KeychainKind::External).expect("error retrieving external policies").unwrap();

    let json_output = Output {
        balance: &wallet.get_balance().expect("error retrieving balance"),
        transactions: &wallet.list_transactions(true).expect("error retrieving transactions"),
        extern_policies: &extern_policies.clone(),
        intern_policies: &wallet.policies(KeychainKind::Internal).expect("error retrieving internal policies").unwrap(),
        addresses: addresses,
        extern_descriptor: &wallet.public_descriptor(KeychainKind::External).expect("error retrieving external descriptor").unwrap(),
        intern_descriptor: &wallet.public_descriptor(KeychainKind::Internal).expect("error retrieving internal descriptor").unwrap(),
        policy_structure: &get_policy_descriptions(&extern_policies, 0),
    };

    let mut file = File::create(output_file)?;

    serde_json::to_writer_pretty(&mut file, &json_output)?;

    Ok(())
}

#[derive(Serialize)]
struct Output<'a> {
    balance: &'a Balance,
    transactions: &'a Vec<TransactionDetails>,
    extern_policies: &'a Policy,
    intern_policies: &'a Policy,
    addresses: Vec<String>,
    extern_descriptor: &'a ExtendedDescriptor,
    intern_descriptor: &'a ExtendedDescriptor,
    policy_structure: &'a serde_json::Value,
}

#[derive(Serialize, Deserialize)]
struct Addresses {
    addresses: Vec<String>,
}

pub fn description(policy: &Policy) -> String {
    match &policy.item {
        SatisfiableItem::EcdsaSignature(key) => format!("ECDSA Sig of {}", display_key(key)),
        SatisfiableItem::SchnorrSignature(key) => {
            format!("Schnorr Sig of {}", display_key(key))
        }
        SatisfiableItem::Sha256Preimage { hash } => {
            format!("SHA256 Preimage of {}", hash.to_string())
        }
        SatisfiableItem::Hash256Preimage { hash } => {
            format!("Double-SHA256 Preimage of {}", hash.to_string())
        }
        SatisfiableItem::Ripemd160Preimage { hash } => {
            format!("RIPEMD160 Preimage of {}", hash.to_string())
        }
        SatisfiableItem::Hash160Preimage { hash } => {
            format!("Double-RIPEMD160 Preimage of {}", hash.to_string())
        }
        SatisfiableItem::AbsoluteTimelock { value } => {
            format!("Absolute Timelock of {}", value.to_string())
        }
        SatisfiableItem::RelativeTimelock { value } => {
            format!("Relative Timelock of {}", value.to_string())
        }
        SatisfiableItem::Multisig { keys, threshold } => {
            format!("{} of {} MultiSig:", threshold, keys.len())
        }
        SatisfiableItem::Thresh { items, threshold } => {
            format!("{} of {} Threshold:", threshold, items.len())
        }
    }
}

pub struct DisplayKey<'a>(&'a PkOrF);

impl<'a> Display for DisplayKey<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", display_key(self.0))
    }
}

fn display_key(key: &PkOrF) -> String {
    // TODO: Use aliases
    match key {
        PkOrF::Pubkey(pk) => format!("<pk:{}>", pk.to_string()),
        PkOrF::XOnlyPubkey(pk) => format!("<xonly-pk:{}>", pk.to_string()),
        PkOrF::Fingerprint(f) => format!("<fingerprint:{}>", f.to_string()),
    }
}
