#!/usr/bin/swift sh

import BitcoinDevKit  // https://github.com/bitcoindevkit/bdk-swift.git == 0.29.0

do {
    let descriptor = try Descriptor(
        descriptor: "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)",
        network: .testnet
    )

    let electrumUrl = "ssl://electrum.blockstream.info:60002"
    let blockchainConfig = BlockchainConfig.electrum(config: ElectrumConfig(url: electrumUrl, socks5: nil, retry: 5, timeout: nil, stopGap: 100, validateDomain: true))
    let blockchain = try Blockchain(config: blockchainConfig)
    let databaseConfig = DatabaseConfig.memory

    let wallet = try Wallet(descriptor: descriptor, changeDescriptor: nil, network: .testnet, databaseConfig: databaseConfig)

    try wallet.sync(blockchain: blockchain, progress: nil)

    let balance = try wallet.getBalance().total
    print("Wallet descriptor is \(descriptor.asStringPrivate())")
    print("The wallet balance is \(balance)\n")

    let txList: [TransactionDetails] = try wallet.listTransactions(includeRaw: false)
    txList.forEach { item in
        print("\(item) \n")
    }
    
} catch {
    print(error)
}
