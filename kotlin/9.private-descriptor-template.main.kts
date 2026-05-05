/**
 * bdk-jvm 0.29.2
 *
 * Create private and public descriptors using descriptor templates.
 */

@file:DependsOn("org.bitcoindevkit:bdk-jvm:0.29.2")

import org.bitcoindevkit.*

// {
//   "fingerprint": "9122d9e0",
//   "mnemonic": "fire alter tide over object advance diamond pond region select tone pole",
//   "xprv": "tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B"
// }

val descriptorSecretKey: DescriptorSecretKey = DescriptorSecretKey.fromString(
    "tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B"
)

val descriptor = Descriptor.newBip84(
    secretKey = descriptorSecretKey,
    keychain = KeychainKind.EXTERNAL,
    network = Network.TESTNET
)

println("The descriptor is ${descriptor.asStringPrivate()}")
// wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84'/1'/0'/0/*)#yl0cyza0

println("The public descriptor is ${descriptor.asString()}")
// wpkh([9122d9e0/84'/1'/0']tpubDCYVtmaSaDzTxcgvoP5AHZNbZKZzrvoNH9KARep88vESc6MxRqAp4LmePc2eeGX6XUxBcdhAmkthWTDqygPz2wLAyHWisD299Lkdrj5egY6/0/*)#zpaanzgu
