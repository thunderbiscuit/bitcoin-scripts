#!/usr/bin/swift sh

import BitcoinDevKit  // https://github.com/bitcoindevkit/bdk-swift.git == 0.29.0

do {
    let descriptor = try Descriptor.init(descriptor:"wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)", network: .testnet)

    let electrumUrl = "ssl://electrum.blockstream.info:60002"
    
    let blockchainConfig = BlockchainConfig.electrum(
        config: ElectrumConfig(
            url: electrumUrl,
            socks5: nil,
            retry: 5,
            timeout: nil,
            stopGap: 100,
            validateDomain: true
        )
    )
    
    let blockchain = try Blockchain(config: blockchainConfig)
    let databaseConfig = DatabaseConfig.memory
    
    let wallet = try Wallet(descriptor: descriptor, changeDescriptor: nil, network: Network.testnet, databaseConfig: databaseConfig)
    
    try wallet.sync(blockchain: blockchain, progress: nil)
    
    let unspentUtxos: [LocalUtxo] = try wallet.listUnspent()
    print("There are \(unspentUtxos.count) number of outputs in this wallet")
    
    unspentUtxos.forEach { item in
        print(item)
    }

} catch {
    print(error)
}
