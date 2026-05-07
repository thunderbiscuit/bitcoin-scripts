#!/usr/bin/swift sh

import BitcoinDevKit  // https://github.com/bitcoindevkit/bdk-swift.git == 0.32.1

let db = DatabaseConfig.memory

do {
    let descriptor = try Descriptor.init(descriptor: "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)", network: Network.regtest)
    let electrum = ElectrumConfig(url: "tcp://127.0.0.1:60401", socks5: nil, retry: 5, timeout: nil, stopGap: 10, validateDomain: false)
    let blockchainConfig = BlockchainConfig.electrum(config: electrum)
    let blockchain = try Blockchain(config: blockchainConfig)
    let wallet = try Wallet(descriptor: descriptor, changeDescriptor: nil, network: Network.regtest, databaseConfig: db)
    try wallet.sync(
        blockchain: blockchain, 
        progress: nil
    )
    
    let balance = try wallet.getBalance().total
    print("Descriptor balance is \(balance) satoshis")

} catch let error {
    print(error)
}
