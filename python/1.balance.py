# bdkpython 1.0.0a11
# 
# Query an Electrum server for the total balance of a wallet.

import bdkpython as bdk
from utilities import check_bdk_version

check_bdk_version(original_major_version=1, original_minor_version=0)

descriptor = bdk.Descriptor(
    "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)",
    bdk.Network.SIGNET,
)
change_descriptor = bdk.Descriptor(
    "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/1/*)",
    bdk.Network.SIGNET
)

wallet = bdk.Wallet.new_no_persist(
    descriptor=descriptor,
    change_descriptor=change_descriptor,
    network=bdk.Network.SIGNET,
)

electrum_client = bdk.ElectrumClient("ssl://mempool.space:60602")
balance = wallet.get_balance()
print(f"Wallet balance is: {balance.total.to_sat()}")

full_scan_request = wallet.start_full_scan()
update = electrum_client.full_scan(
    full_scan_request=full_scan_request,
    stop_gap=10,
    batch_size=10,
    fetch_prev_txouts=False
)
wallet.apply_update(update)

new_balance = wallet.get_balance()
print(f"Wallet balance after sync is: {new_balance.total.to_sat()}")
