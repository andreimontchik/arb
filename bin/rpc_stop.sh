#!/usr/bin/env bash
set -ex

solana-validator --ledger /mnt/ledger exit --monitor --skip-new-snapshot-check
