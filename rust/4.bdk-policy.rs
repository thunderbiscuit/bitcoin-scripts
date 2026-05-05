#!/usr/bin/env rust-script

//! bdk 0.26.0
//!
//! Print the BDK policy for a non-trivial descriptor.
//!
//! ```cargo
//! [dependencies]
//! bdk = "0.26.0"
//! ```

#![allow(unused_doc_comments)]

use bdk::descriptor::IntoWalletDescriptor;
use bdk::signer::SignersContainer;
use bdk::bitcoin::Network;
use bdk::descriptor::ExtractPolicy;
use bdk::bitcoin::secp256k1::Secp256k1;
use bdk::descriptor::policy::BuildSatisfaction;

let secp = Secp256k1::new();

// The form of this descriptor is wsh(multi(2, <privkey>, <pubkey>))
let desc = "wsh(multi(2,tprv8ZgxMBicQKsPdpkqS7Eair4YxjcuuvDPNYmKX3sCniCf16tHEVrjjiSXEkFRnUH77yXc6ZcwHHcLNfjdi5qUvw3VDfgYiH5mNsj5izuiu2N/1/*,tpubD6NzVbkrYhZ4XHndKkuB8FifXm8r5FQHwrN6oZuWCz13qb93rtgKvD4PQsqC4HP4yhV3tA2fqr2RbY5mNXfM7RxXUoeABoDtsFUq2zJq6YK/1/*))";

// Use the descriptor string to derive the full descriptor and a keymap.
// The wallet descriptor can be used to create a new bdk::Wallet.
// While the `keymap` can be used to create a `SignerContainer`.
//
// The `SignerContainer` can sign for `PSBT`s.
// a bdk::Wallet internally uses these to handle transaction signing.
// But they can be used as independent tools also.
let (wallet_desc, keymap) = desc.into_wallet_descriptor(&secp, Network::Testnet).unwrap();

println!("Example Descriptor for policy analysis : {}", wallet_desc);

// Create the signer with the keymap and descriptor.
let signers_container = SignersContainer::build(keymap, &wallet_desc, &secp);

// Extract the Policy from the given descriptor and signer.
// Note that Policy is a wallet specific structure. It depends on the the descriptor, and
// what the concerned wallet with a given signer can sign for.
let policy = wallet_desc
    .extract_policy(&signers_container, BuildSatisfaction::None, &secp)?
    .expect("We expect a policy");

println!("Derived Policy for the descriptor {:#?}", policy);
