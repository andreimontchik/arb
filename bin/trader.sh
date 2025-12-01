#!/usr/bin/env bash

export RUST_LOG=info

cd ~

# Backup old log
timestamp=$(date +"%Y-%m-%dT%H%M%S")
log_file="log/trader.log"
log_archive="${log_file}.$timestamp"
if [ -e "$log_file" ]; then
    echo "Moving $log_file to $log_archive"
    mv "$log_file" "$log_archive"
fi

nohup ~/src/research/target/debug/trader ~/src/research/config/mainnet/trader.json > "$log_file" 2>&1 &

pid2=$!
echo "Trader started with PID $pid2"

