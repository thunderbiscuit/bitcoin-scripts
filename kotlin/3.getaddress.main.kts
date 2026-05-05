/**
 * bdk-jvm 1.0.0-alpha.11
 *
 * Get an unused address from a wallet.
 */

@file:DependsOn("org.bitcoindevkit:bdk-jvm:1.0.0-alpha.11")

import org.bitcoindevkit.Descriptor
import org.bitcoindevkit.Wallet
import org.bitcoindevkit.ElectrumClient
import org.bitcoindevkit.FullScanRequest
import org.bitcoindevkit.Network
import org.bitcoindevkit.KeychainKind
import org.bitcoindevkit.AddressInfo

val descriptor = Descriptor("wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)", Network.SIGNET)
val changeDescriptor = Descriptor("wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/1/*)", Network.SIGNET)

val wallet: Wallet = Wallet.newNoPersist(descriptor, changeDescriptor, Network.SIGNET)
val electrumClient: ElectrumClient = ElectrumClient("ssl://mempool.space:60602")
val fullScanRequest: FullScanRequest = wallet.startFullScan()
val update = electrumClient.fullScan(fullScanRequest, 10uL, 10uL, false)
wallet.applyUpdate(update)

val lastUnusedAddress: AddressInfo = wallet.revealNextAddress(KeychainKind.EXTERNAL)
println("The last unused address for this wallet is ${lastUnusedAddress.address.asString()} at index ${lastUnusedAddress.index}")
