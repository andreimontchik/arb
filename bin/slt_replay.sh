#!/usr/bin/env bash

export RUST_LOG=solana=info,solana_metrics=warn,solana_runtime::bank=warn,plugin=info,common=info

solana-ledger-tool verify \
--skip-verification \
--ledger /mnt/ledger \
--accounts /mnt/accounts \
--accounts-hash-cache-path ~/data/accounts_hash_cache \
--accounts-index-path ~/data/accounts_index \
--snapshot-archive-path ~/data/archives/644 \
--use-snapshot-archives-at-startup when-newest \
--geyser-plugin-config ~/src/research/config/mainnet/plugin.mp.json
