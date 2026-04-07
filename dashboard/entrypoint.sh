#!/bin/sh
set -e

# Default environment variables
export SOLANA_RPC_URL="${SOLANA_RPC_URL:-https://api.devnet.solana.com}"
export CONTENT_REGISTRY="${CONTENT_REGISTRY:-GokivDYuQXPZCWRkwMhdH2h91KpDQXBEmpgBgs55bnpH}"
export ANALYSIS_MARKETPLACE="${ANALYSIS_MARKETPLACE:-9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin}"
export MODEL_REGISTRY="${MODEL_REGISTRY:-Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS}"
export WORKER_REGISTRY="${WORKER_REGISTRY:-HmbTLCmaGvZhKnn1Zfa1JVnp7vkMV4DYVxPLWBVoN65L}"
export REWARDS="${REWARDS:-4uQeVj5tqViQh7yWWGStvkEG1Zmhx6uasJtWCJziofM}"

# Ensure index directory exists
mkdir -p /data/indexes

echo "Starting DFPN Dashboard"
echo "  RPC URL: $SOLANA_RPC_URL"
echo "  Content Registry: $CONTENT_REGISTRY"
echo "  Analysis Marketplace: $ANALYSIS_MARKETPLACE"
echo "  Model Registry: $MODEL_REGISTRY"
echo "  Worker Registry: $WORKER_REGISTRY"
echo "  Rewards: $REWARDS"

exec supervisord -c /etc/supervisord.conf
