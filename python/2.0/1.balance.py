# /// script
# dependencies = [
#   "bdkpython==2.3.1",
# ]
# ///

# Query an Electrum server for the total balance of a wallet.

from bdkpython import Descriptor
from bdkpython import Network
from bdkpython import Persister
from bdkpython import Wallet
from bdkpython import ElectrumClient

descriptor = Descriptor(
    "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)",
    Network.REGTEST,
)
change_descriptor = Descriptor(
    "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/1/*)",
    Network.REGTEST
)
db = Persister.new_in_memory()

wallet = Wallet(
    descriptor=descriptor,
    change_descriptor=change_descriptor,
    network=Network.REGTEST,
    persister=db,
)

electrum_client = ElectrumClient("tcp://127.0.0.1:60401")
balance = wallet.balance()
print(f"Wallet balance is: {balance.total.to_sat()}")

full_scan_request = wallet.start_full_scan().build()
update = electrum_client.full_scan(
    request=full_scan_request,
    stop_gap=10,
    batch_size=10,
    fetch_prev_txouts=False
)
wallet.apply_update(update)

new_balance = wallet.balance()
print(f"Wallet balance after sync is: {new_balance.total.to_sat()}")
