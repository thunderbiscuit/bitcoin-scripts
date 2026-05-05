#!/usr/bin/swift sh

import BitcoinDevKit  // https://github.com/bitcoindevkit/bdk-swift.git == 0.29.0

let db = DatabaseConfig.memory

do {
    let descriptor = try Descriptor.init(descriptor: "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)", network: Network.testnet)
    let electrum = ElectrumConfig(url: "ssl://electrum.blockstream.info:60002", socks5: nil, retry: 5, timeout: nil, stopGap: 10, validateDomain: true)
    let blockchainConfig = BlockchainConfig.electrum(config: electrum)
    let blockchain = try Blockchain(config: blockchainConfig)
    let wallet = try Wallet(descriptor: descriptor, changeDescriptor: nil, network: Network.testnet, databaseConfig: db)
    try wallet.sync(
        blockchain: blockchain, 
        progress: nil
    )
    
    let balance = try wallet.getBalance().total
    print("Descriptor balance is \(balance) satoshis")

} catch let error {
    print(error)
}
