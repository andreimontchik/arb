#!/usr/bin/env bash

export RUST_LOG=debug

~/src/research/target/debug/publisher "$@"
