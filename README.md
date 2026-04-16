# Arbitrage system capturing price differences across Solana liquidity providers

- The [Solana Validator Geyser plugin](plugin) processes LP updates, searches for arbitrage opportunities and saves them in memory mapped file.
- The [Trader](trader) application reads arbigrage data from memfile then creates and submits Solana transactions.
