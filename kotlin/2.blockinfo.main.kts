/**
 * bdk-jvm 0.29.2
 *
 * Query an Electrum server for the testnet blockchain height and latest block hash.
 */

@file:DependsOn("org.bitcoindevkit:bdk-jvm:0.29.2")

import org.bitcoindevkit.*

val memoryDatabaseConfig = DatabaseConfig.Memory

val blockchainConfig = BlockchainConfig.Electrum(
    ElectrumConfig(
        // url = "ssl://electrum.blockstream.info:50002", // mainnet
        url = "ssl://electrum.blockstream.info:60002",    // testnet
        socks5 = null,
        retry = 5u,
        timeout = null,
        stopGap = 200u,
        validateDomain = true,
    )
)

val blockchain = Blockchain(blockchainConfig)
val blockHeight: UInt = blockchain.getHeight()
val blockHash: String = blockchain.getBlockHash(blockHeight)

println("Latest block is block $blockHeight with hash $blockHash")
