# bdkpython 0.26.0
#
# Get new addresses from a wallet.

import bdkpython as bdk
from utilities import check_bdk_version

check_bdk_version(original_major_version=0, original_minor_version=26)


descriptor = bdk.Descriptor(
    "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)",
    bdk.Network.TESTNET,
)
db_config = bdk.DatabaseConfig.MEMORY()

wallet = bdk.Wallet(
             descriptor=descriptor,
             change_descriptor=None,
             network=bdk.Network.TESTNET,
             database_config=db_config,
         )

# print "last unused" receive address
address_info_last_unused = wallet.get_address(bdk.AddressIndex.LAST_UNUSED)
address_last_unused = address_info_last_unused.address
index_last_unused = address_info_last_unused.index
print(f"Last unused P2PKH testnet address: {address_last_unused} at index {index_last_unused}")

# print new receive address
address_info_new = wallet.get_address(bdk.AddressIndex.NEW)
address_new = address_info_new.address
index_new = address_info_new.index
print(f"New P2PKH testnet address: {address_new} at index {index_new}")
