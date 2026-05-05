/**
 * bdk-jvm 1.0.0-alpha.11
 *
 * Query an Electrum server for the total balance of a wallet using a descriptor.
 */

@file:DependsOn("org.bitcoindevkit:bdk-jvm:1.0.0-alpha.11")

import org.bitcoindevkit.Descriptor
import org.bitcoindevkit.Wallet
import org.bitcoindevkit.ElectrumClient
import org.bitcoindevkit.FullScanRequest
import org.bitcoindevkit.Network
import org.bitcoindevkit.KeychainKind

val descriptor = Descriptor("wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)", Network.REGTEST)
val changeDescriptor = Descriptor("wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/1/*)", Network.REGTEST)

val wallet: Wallet = Wallet.newNoPersist(descriptor, changeDescriptor, Network.REGTEST)
val electrumClient: ElectrumClient = ElectrumClient("tcp://127.0.0.1:60401")
val fullScanRequest: FullScanRequest = wallet.startFullScan()
val update = electrumClient.fullScan(fullScanRequest, 10uL, 10uL, false)
wallet.applyUpdate(update)

val balanceSatoshi = wallet.getBalance().total.toSat()
println("Balance: $balanceSatoshi satoshi")

val address = wallet.revealNextAddress(KeychainKind.EXTERNAL)
println(address.address.asString())
