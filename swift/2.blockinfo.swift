#!/usr/bin/swift sh

import BitcoinDevKit  // https://github.com/bitcoindevkit/bdk-swift.git == 0.29.0

let db = DatabaseConfig.memory

do {
    let blockchainConfig = BlockchainConfig.electrum(
        config: ElectrumConfig(
            // url: "ssl://electrum.blockstream.info:50002",
            url: "ssl://electrum.blockstream.info:60002",
            socks5: nil,
            retry: 5,
            timeout: nil,
            stopGap: 10,
            validateDomain: true
        )
    )
    
    let blockchain = try Blockchain(config: blockchainConfig)
    let blockHeight: UInt32 = try blockchain.getHeight()
    let blockHash: String = try blockchain.getBlockHash(height: blockHeight)
    
    print("Latest block is block \(blockHeight) with hash \(blockHash)")
} catch {
    print(error)
}
