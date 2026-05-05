/**
 * bdk-jvm 0.29.2
 *
 * Create a public descriptor using descriptor templates.
 */

@file:DependsOn("org.bitcoindevkit:bdk-jvm:0.29.2")

import org.bitcoindevkit.*

// {
//   "fingerprint": "9122d9e0",
//   "mnemonic": "fire alter tide over object advance diamond pond region select tone pole",
//   "xprv": "tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B"
// }

// this is the extended public key derived at `xprv/84h/1h/0h`
val descriptorPublicKey: DescriptorPublicKey = DescriptorPublicKey.fromString("tpubDCYVtmaSaDzTxcgvoP5AHZNbZKZzrvoNH9KARep88vESc6MxRqAp4LmePc2eeGX6XUxBcdhAmkthWTDqygPz2wLAyHWisD299Lkdrj5egY6")

val descriptor = Descriptor.newBip84Public(
    publicKey = descriptorPublicKey,
    fingerprint = "9122d9e0",
    keychain = KeychainKind.EXTERNAL,
    network = Network.TESTNET
)

println("The descriptor is ${descriptor.asString()}")
// wpkh([9122d9e0/84'/1'/0']tpubDCYVtmaSaDzTxcgvoP5AHZNbZKZzrvoNH9KARep88vESc6MxRqAp4LmePc2eeGX6XUxBcdhAmkthWTDqygPz2wLAyHWisD299Lkdrj5egY6/0/*)#zpaanzgu
