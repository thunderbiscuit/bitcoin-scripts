/**
 * bdk-jvm 0.29.2
 *
 * Create a BIP84 descriptor starting from a 12-word mnemonic.
 */

@file:DependsOn("org.bitcoindevkit:bdk-jvm:0.29.2")

import org.bitcoindevkit.*

val mnemonic: Mnemonic = Mnemonic.fromString("fire alter tide over object advance diamond pond region select tone pole")
val bip32RootKey: DescriptorSecretKey = DescriptorSecretKey(
    network = Network.TESTNET,
    mnemonic = mnemonic,
    password = ""
)

// The extend method will automatically apply the wildcard (*) to your path,
// i.e the following will generate the typical testnet BIP84 external wallet path m/84h/1h/0h/0/*
val bip84ExternalPath: DerivationPath = DerivationPath("m/84h/1h/0h/0")
val externalExtendedKey: String = bip32RootKey.extend(bip84ExternalPath).asString()

// To create the descriptor you'll need to use this extended key in a descriptor script expression, i.e. wpkh(), wsh(), tr(), etc.
// Here we create a BIP84-compatible descriptor, so P2PWKH
val descriptor = "wpkh($externalExtendedKey)"

println("The descriptor is $descriptor") // wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)
