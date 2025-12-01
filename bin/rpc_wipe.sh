#!/usr/bin/env bash
set +ex

confirm() {
    read -r -p "Are you sure? [y/N] " response
    case "$response" in
        [yY][eE][sS]|[yY]) 
            true
            ;;
        *)
            false
            ;;
    esac
}


if confirm; then
	echo "Wiping the Validator Ledger directories..."
	rm -rf /mnt/ledger/rocksdb
	rm -rf /mnt/ledger/banking_trace

	rm -rf /mnt/accounts/run
	rm -rf /mnt/accounts/snapshot

	rm -rf ~/data/accounts_hash_cache
	rm -rf ~/data/accounts_index
	rm -rf ~/data/snapshots/*
        echo "Completed."
else
	echo "Cancelled."
fi
