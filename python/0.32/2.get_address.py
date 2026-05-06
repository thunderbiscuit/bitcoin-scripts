# /// script
# requires-python = "==3.12.*"
# dependencies = [
#   "bdkpython==0.32.1",
# ]
# ///

# Get new addresses from a wallet.

from bdkpython import Network
from bdkpython import DatabaseConfig
from bdkpython import Wallet
from bdkpython import Descriptor
from bdkpython import AddressIndex


descriptor = Descriptor(
    "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)",
    Network.REGTEST,
)
db_config = DatabaseConfig.MEMORY()

wallet = Wallet(
             descriptor=descriptor,
             change_descriptor=None,
             network=Network.TESTNET,
             database_config=db_config,
         )

# Print "last unused" receive address
address_info_last_unused = wallet.get_address(AddressIndex.LAST_UNUSED())
address_last_unused = address_info_last_unused.address.as_string()
index_last_unused = address_info_last_unused.index
print(f"Last unused P2PKH address: {address_last_unused} at index {index_last_unused}")

# Print new receive address
address_info_new = wallet.get_address(AddressIndex.NEW())
address_new = address_info_new.address.as_string()
index_new = address_info_new.index
print(f"New P2PKH address: {address_new} at index {index_new}")
