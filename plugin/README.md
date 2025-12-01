# Config
* Create the Geyser plugin configuration file. [Source](src/config.rs#L10) in the Configuration location. 
* Create [Orca](src/config.rs#L29), [Raydium](src/config.rs#L53) and Processor configuration files. Currently supported processor implementations are [MessagePersister](src/transfer/message_persister.rs#L43) and [ArbitrageController](src/arbitrage/arbitrage_controller.rs#L19).

# Build 
`cargo build -p plugin`

# Test
`cargo test -p plugin`

# Run
* Localnet: `. ~/work/src/research/bin/stv.sh`
