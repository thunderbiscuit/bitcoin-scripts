/**
 * bdk-jvm 0.29.2
 *
 * Create extended keys using custom derivation paths.
 */

@file:DependsOn("org.bitcoindevkit:bdk-jvm:0.29.2")

import org.bitcoindevkit.*

// {
//   "fingerprint": "9122d9e0",
//   "mnemonic": "fire alter tide over object advance diamond pond region select tone pole",
//   "xprv": "tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B"
// }

val bip32RootKey: DescriptorSecretKey = DescriptorSecretKey.fromString(
    "tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B"
)

// The DescriptorSecretKey.derive() method allows you to derive an extended key for a given
// node in the derivation tree (for example to create an xpub for a particular account)
val veryCustomDerivationPath: DerivationPath = DerivationPath("m/800h/2020h/1234h/77/42")
val xprv: DescriptorSecretKey = bip32RootKey.derive(veryCustomDerivationPath)
println("The extended key on path m/800h/2020h/1234h/77/42 is ${xprv.asString()}")

// The DescriptorSecretKey.extend() method allows you to extend a key to any given path.
// The method will also automatically apply the wildcard (*) to your DerivationPath
val extendCustomPath: DerivationPath = DerivationPath("m/2/2/42")
val keysAtVeryCustomLocation: DescriptorSecretKey = xprv.extend(extendCustomPath)
println("Keys will be derived at location ${keysAtVeryCustomLocation.asString()}")
