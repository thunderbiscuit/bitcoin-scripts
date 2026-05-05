#!/usr/bin/swift sh

import BitcoinDevKit  // https://github.com/bitcoindevkit/bdk-swift.git == 0.29.0

/*
 * Create extended keys using custom derivation paths.
*/

do {
    
    // {
    //   "fingerprint": "9122d9e0",
    //   "mnemonic": "fire alter tide over object advance diamond pond region select tone pole",
    //   "xprv": "tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B"
    // }

    let bip32RootKey: DescriptorSecretKey = try DescriptorSecretKey.fromString(
        secretKey: "tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B"
    )

    // The DescriptorSecretKey.derive() method allows you to derive an extended key for a given
    // node in the derivation tree (for example to create an xpub for a particular account)
    let veryCustomDerivationPath: DerivationPath = try DerivationPath(path: "m/800h/2020h/1234h/77/42")
    let xprv: DescriptorSecretKey = try bip32RootKey.derive(path: veryCustomDerivationPath)
    print("The extended key on path m/800h/2020h/1234h/77/42 is \(xprv.asString())")

    // The DescriptorSecretKey.extend() method allows you to extend a key to any given path.
    // The method will also automatically apply the wildcard (*) to your DerivationPath
    let extendCustomPath: DerivationPath = try DerivationPath(path: "m/2/2/42")
    let keysAtVeryCustomLocation: DescriptorSecretKey = try xprv.extend(path: extendCustomPath)
    print("Keys will be derived at location \(keysAtVeryCustomLocation.asString())")
    
} catch {
    print(error)
}
