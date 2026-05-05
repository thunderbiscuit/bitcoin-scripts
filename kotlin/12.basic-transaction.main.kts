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
        url = "ssl://electrum.blockstream.info:60002",
        socks5 = null,
        retry = 5u,
        timeout = null,
        stopGap = 100u,
        validateDomain = true,
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
val faucetAddress = "tb1ql7w62elx9ucw4pj5lgw4l028hmuw80sndtntxt"

val (psbt, txDetails) = TxBuilder()
    .addRecipient(
        script = Address(faucetAddress).scriptPubkey(),
        amount = 1000u
    )
    .feeRate(2.0f)
    .finish(wallet)

println("Transaction details: $txDetails")
wallet.sign(psbt, null)
val tx = psbt.extractTx()
// blockchain.broadcast(tx)
