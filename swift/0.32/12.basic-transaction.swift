#!/usr/bin/swift sh

import BitcoinDevKit  // https://github.com/bitcoindevkit/bdk-swift.git == 0.32.1

/*
 * Create a transaction with an OP_RETURN output.
*/

do {

    let memoryDatabaseConfig = DatabaseConfig.memory

    let electrumUrl = "tcp://127.0.0.1:60401"
    let blockchainConfig = BlockchainConfig.electrum(
        config: ElectrumConfig(
            url: electrumUrl,
            socks5: nil,
            retry: 5,
            timeout: nil,
            stopGap: 100,
            validateDomain: false
        )
    )
    
    let descriptor = try Descriptor(descriptor: "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)", network: .regtest)

    let wallet = try Wallet(descriptor: descriptor, changeDescriptor: nil, network: Network.regtest, databaseConfig: memoryDatabaseConfig)

    let blockchain = try Blockchain(config: blockchainConfig)

    try wallet.sync(blockchain: blockchain, progress: nil)

    let balance = try wallet.getBalance()

    if balance.total == 0 {
        print("Wallet balance is 0, please add funds!")
    }

    let faucetAddress = "bcrt1qw508d6qejxtdg4y5r3zarvary0c5xw7kygt080"

    let txBuilderResult = try TxBuilder()
        .addRecipient(script: Address(address: faucetAddress).scriptPubkey(), amount: 1000)
        .feeRate(satPerVbyte: 2.0)
        .finish(wallet: wallet)
        
    print("Transaction details: \(txBuilderResult.transactionDetails)")
    try wallet.sign(psbt: txBuilderResult.psbt, signOptions: nil)
    let tx = txBuilderResult.psbt.extractTx()
    // blockchain.broadcast(transaction: tx)
    
} catch {
    print(error)
}
