# Build

# Configure
Use the [Trader Local configuration](../config/mainnet/trader.local.json) file as reference.

__Logging__
Trader uses [env_logger](https://docs.rs/env_logger/latest/env_logger/) imlementation of the standar [Rust logging](https://github.com/rust-lang/log) API. Logging level is set in the [startup script](../bin/trader.sh#L3).

# Run
1. Prepare the configuration file.
1. `research/bin/trader.sh <CONFIG_FILE>`. 
   Example: 
   ```
   ~/work/src/research/bin/trader.sh ~/work/src/research/config/mainnet/trader.local.json
   ```
