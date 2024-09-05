The sandbox snapshots were originally generated based on some Sui mainnet transactions.
However, we can no longer support them due to significant code changes (rebate, renaming, balance, etc.).
We need to generate new ones using the SandboxPersist command (for this, we require a non-system transactions to replay).

Main.rs generates a snapshot based on a mock transaction (package publishing). This should be sufficient to make the tests pass, at least until we have specific transactions we want to replay on the testnet/mainnet.


The sandbox snapshots are generated according to specific transactions we want to replay based on the testnet/mainnet. 
Currently, they only replay locally.