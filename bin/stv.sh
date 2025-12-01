#!/usr/bin/env bash

pushd ~/work

export RUST_LOG=solana=info,solana_metrics=warn,plugin=debug
rm -f test-ledger/validator.log
solana-test-validator --geyser-plugin-config ~/work/src/research/config/localnet/plugin.mp.json

popd