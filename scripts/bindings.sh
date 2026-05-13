#!/bin/bash
set -e

echo "=== Generating Go Bindings ==="

# Read contract IDs from deploy output or config
if [ ! -f "deploy/testnet.toml" ]; then
    echo "ERROR: deploy/testnet.toml not found. Run deploy first."
    exit 1
fi

BINDINGS_DIR="bindings"

echo "Generating bindings to: $BINDINGS_DIR/"

# For each contract, generate Go bindings
# In a production setup, this would use soroban-cli or a custom tool
# to read the contract spec and generate typed Go clients.

echo "Bindings generation requires soroban-cli >= 22.0"
echo "Run: soroban contract bindings --wasm <contract.wasm> --output-dir bindings/"

# Placeholder for now — actual generation depends on soroban-cli version
echo "See PLANS.md Phase 1 for manual binding generation steps."
