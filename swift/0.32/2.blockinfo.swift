#!/usr/bin/swift sh

import BitcoinDevKit  // https://github.com/bitcoindevkit/bdk-swift.git == 0.32.1

let db = DatabaseConfig.memory

do {
    let blockchainConfig = BlockchainConfig.electrum(
        config: ElectrumConfig(
            url: "tcp://127.0.0.1:60401",
            socks5: nil,
            retry: 5,
            timeout: nil,
            stopGap: 10,
            validateDomain: false
        )
    )
    
    let blockchain = try Blockchain(config: blockchainConfig)
    let blockHeight: UInt32 = try blockchain.getHeight()
    let blockHash: String = try blockchain.getBlockHash(height: blockHeight)
    
    print("Latest block is block \(blockHeight) with hash \(blockHash)")
} catch {
    print(error)
}
