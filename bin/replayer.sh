#!/usr/bin/env bash

export RUST_LOG=solana=info,solana_metrics=warn,plugin=info

~/work/src/research/target/debug/replayer "$@"
