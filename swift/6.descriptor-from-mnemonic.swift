#!/usr/bin/swift sh

import BitcoinDevKit  // https://github.com/bitcoindevkit/bdk-swift.git == 0.29.0

do {
    let mnemonic: Mnemonic = try Mnemonic.fromString(mnemonic: "fire alter tide over object advance diamond pond region select tone pole")
    let bip32RootKey: DescriptorSecretKey = DescriptorSecretKey(
        network: .testnet,
        mnemonic: mnemonic,
        password: ""
    )

    // The extend method will automatically apply the wildcard (*) to your path,
    // i.e the following will generate the typical testnet BIP84 external wallet path m/84h/1h/0h/0/*
    let bip84ExternalPath: DerivationPath = try DerivationPath(path: "m/84h/1h/0h/0")
    let externalExtendedKey: String = try bip32RootKey.extend(path: bip84ExternalPath).asString()

    // To create the descriptor you'll need to use this extended key in a descriptor script expression, i.e. wpkh(), wsh(), tr(), etc.
    // Here we create a BIP84-compatible descriptor, so P2PWKH
    let descriptor = "wpkh(\(externalExtendedKey))"

    print("The descriptor is \(descriptor)") // wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)
    
} catch {
    print(error)
}
