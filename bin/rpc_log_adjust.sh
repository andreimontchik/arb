#!/usr/bin/env bash
# set +ex


solana-validator --ledger /mnt/ledger set-log-filter "\
solana=info,\
solana_metrics=warn,\
solana_core=info,\
solana_core::replay_stage=warn,\
solana_core::heaviest_subtree_fork_choice=warn,\
solana_core::repair=info,\
solana_core::repair::cluster_slot_state_verifier=warn,\
solana_core::consensus=warn,\
solana_core::window_service=warn,\
solana_runtime=info,\
solana_runtime::bank=warn,\
solana_poh=warn,\
solana_streamer=warn,\
solana_gossip=info,\
solana_gossip::cluster_info=warn,\
solana_accounts_db=warn,\
solana_rpc=info,\
geyser_md_plugin=info,\
plugin=info,\
common=info"
