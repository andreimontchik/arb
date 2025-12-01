#!/usr/bin/env bash
# set -ex

export RUST_LOG=solana=info,solana_metrics=warn,solana_runtime::bank=warn,plugin=info,common=info
# export SOLANA_METRICS_CONFIG="host=https://metrics.solana.com:8086,db=mainnet-beta,u=mainnet-beta_write,p=password"

GEYSER_PLUGIN_CONFIG="/home/sol/src/research/config/mainnet/plugin.arb.json"
ledger_dir=/mnt/ledger

cd ~

# Backup old log
timestamp=$(date +"%Y-%m-%dT%H%M%S")
log_file="log/rpc.log"
log_archive="${log_file}.$timestamp"
if [ -e "$log_file" ]; then
    echo "Moving $log_file to $log_archive"
    mv "$log_file" "$log_archive"
fi

args=(
  --identity ~/.keys/rpc.json
  --entrypoint entrypoint.mainnet-beta.solana.com:8001
  --entrypoint entrypoint2.mainnet-beta.solana.com:8001
  --entrypoint entrypoint3.mainnet-beta.solana.com:8001
  --entrypoint entrypoint4.mainnet-beta.solana.com:8001
  --entrypoint entrypoint5.mainnet-beta.solana.com:8001
  --known-validator 7Np41oeYqPefeNQEHSv1UDhYrehxin3NStELsSKCT4K2
  --known-validator GdnSyH3YtwcxFvQrVVJMm1JhTS4QVX7MFsX56uJLUfiZ
  --known-validator DE1bawNcRJB9rVm3buyMVfr8mBEoyyu73NBovf2oXJsJ
  --known-validator CakcnaRDHka2gXyfbEd2d3xsvkJkqsLw2akB3zsN1D2S
  --no-voting
  --only-known-rpc
  --private-rpc
  --rpc-port 8899
  --rpc-threads 1
  --dynamic-port-range 8000-10000
  --tpu-disable-quic
  --expected-genesis-hash 5eykt4UsFv8P8NJdTREpY1vzqKqZKvdpKuc147dw2N9d
  --no-genesis-fetch
  --wal-recovery-mode skip_any_corrupted_record
  --log ~/log/rpc.log
  --ledger "$ledger_dir"
  --accounts-hash-cache-path ~/data/accounts_hash_cache
  --accounts-index-path ~/data/accounts_index
  --accounts /mnt/accounts
  --snapshots ~/data/snapshots
  --use-snapshot-archives-at-startup when-newest
  --no-snapshot-fetch
  --no-incremental-snapshots
  --incremental-snapshot-interval-slots 0
)

if [[ -n $GEYSER_PLUGIN_CONFIG ]]; then
  args+=(--geyser-plugin-config "$GEYSER_PLUGIN_CONFIG")
fi

# echo "${args[@]}"
# exec solana-validator "${args[@]}"
nohup solana-validator "${args[@]}" > "log/rpc.out" 2>&1 &

