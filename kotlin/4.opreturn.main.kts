/**
 * bdk-jvm 0.29.2
 *
 * Create a transaction with an OP_RETURN output.
 */

@file:DependsOn("org.bitcoindevkit:bdk-jvm:0.29.2")

import org.bitcoindevkit.*

val memoryDatabaseConfig = DatabaseConfig.Memory
val blockchainConfig = BlockchainConfig.Electrum(
    ElectrumConfig(
        "ssl://electrum.blockstream.info:60002",
        null,
        5u,
        null,
        100u,
        true,
    )
)

val descriptor = Descriptor("wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)", Network.TESTNET)
val wallet = Wallet(descriptor, null, Network.TESTNET, memoryDatabaseConfig)
val blockchain = Blockchain(blockchainConfig)
    
wallet.sync(blockchain, null)
val balance = wallet.getBalance()
check(balance.total > 0uL) { 
     "Wallet balance is 0, please add funds!"
}

@OptIn(ExperimentalUnsignedTypes::class)
val message = "Working with friends is a happy way to live.".toByteArray(charset = Charsets.UTF_8).asUByteArray().toList()
val faucetAddress = "tb1ql7w62elx9ucw4pj5lgw4l028hmuw80sndtntxt"

val (psbt, txDetails) = TxBuilder()
        .addRecipient(Address(faucetAddress).scriptPubkey(), 1000u)
        .feeRate(2.0f)
        .addData(message)
        .finish(wallet)

println("OP_RETURN transaction created: ${wallet.sign(psbt, null)}")
println("Transaction details: $txDetails")
