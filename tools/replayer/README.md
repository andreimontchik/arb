# Config
Use the [Replayer Local configuration](../../config/mainnet/replayer.sandbox.json) as reference.

# Build 
`cargo build -p replayer`

# Test
`cargo test -p replayer`

# Run
1. Generate message file by using the [Solana Ledger Tool](https://github.com/andreimontchik/research/wiki/Solana-Validator-Ledger#replay-historical-transactions) with the [MessagePersister](../plugin/src/transfer/message_persister.rs#L43) Geyser plugin.
1. Prepare the configuration file.
1. `cd ~/work/`
1. `. ~/work/src/research/bin/replayer.sh <CONFIG_FILE> <MESSAGE_FILE>`
