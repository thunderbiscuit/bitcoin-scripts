# /// script
# requires-python = "==3.12.*"
# dependencies = [
#   "bdkpython==0.32.1",
# ]
# ///

# Query an Electrum server for the testnet blockchain height and latest block hash.

from bdkpython import BlockchainConfig
from bdkpython import ElectrumConfig
from bdkpython import Blockchain

blockchain_config = BlockchainConfig.ELECTRUM(
    ElectrumConfig(
        url="tcp://127.0.0.1:60401",
        socks5=None,
        retry=5,
        timeout=None,
        stop_gap=100,
        validate_domain=True,
    )
)
blockchain = Blockchain(blockchain_config)

block_height = blockchain.get_height()
block_hash = blockchain.get_block_hash(block_height)

print(f"Latest block height is {block_height} with hash {block_hash}")
