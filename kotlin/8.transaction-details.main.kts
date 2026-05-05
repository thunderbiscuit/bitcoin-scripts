/**
 * bdk-jvm 0.29.2
 *
 * Print transaction details for each transaction the wallet has completed.
 */

@file:DependsOn("org.bitcoindevkit:bdk-jvm:0.29.2")

import org.bitcoindevkit.*

val descriptor: Descriptor = Descriptor(
    descriptor = "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)",
    network = Network.TESTNET
)

val electrumUrl = "ssl://electrum.blockstream.info:60002"
val blockchainConfig = BlockchainConfig.Electrum(ElectrumConfig(electrumUrl, null, 5u, null, 100u, true))
val blockchain = Blockchain(blockchainConfig)
val databaseConfig = DatabaseConfig.Memory

val wallet = Wallet(descriptor, null, Network.TESTNET, databaseConfig)

wallet.sync(blockchain, null)

val balance = wallet.getBalance().total
println("Wallet descriptor is ${descriptor.asStringPrivate()}")
println("The wallet balance is $balance\n")

val txList: List<TransactionDetails> = wallet.listTransactions(includeRaw = false)
txList.forEach {
    println("$it \n")
}
