#!/usr/bin/swift sh

import BitcoinDevKit  // https://github.com/bitcoindevkit/bdk-swift.git == 0.29.0

class LogProgress : BitcoinDevKit.Progress {
    func update(progress: Float, message: String?) {
        print("Syncing...")
    }
}

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

    let progress = LogProgress()
    try wallet.sync(blockchain: blockchain, progress: progress)

    let balance = try wallet.getBalance()

    if balance.total == 0 {
        print("Wallet balance is 0, please add funds!")
    }

    let message = "Working with friends is a happy way to live."
    let messageBytes: [UInt8] = [UInt8](message.utf8)

    let faucetAddress = "tb1ql7w62elx9ucw4pj5lgw4l028hmuw80sndtntxt"

    let txBuilderResult = try TxBuilder()
        .addRecipient(script: Address(address: faucetAddress).scriptPubkey(), amount: 1000)
        .feeRate(satPerVbyte: 2.0)
        .addData(data: messageBytes)
        .finish(wallet: wallet)

    let opReturnTx = try wallet.sign(psbt: txBuilderResult.psbt, signOptions: nil)

    print("OP_RETURN transaction created: \(opReturnTx)")
    print("Transaction details: \(txBuilderResult.transactionDetails)")
    
} catch {
    print(error)
}
