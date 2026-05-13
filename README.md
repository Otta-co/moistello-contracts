# Moistello Contracts

Stellar/Soroban smart contracts for decentralized rotating savings and credit associations (ROSCAs).

## Contracts

| Contract | WASM | Functions | Purpose |
|---|---|---|---|
| `circle-factory` | 22 KB | 4 | Deploy new circle instances, manage registry |
| `circle` | 49 KB | 13 | Core engine — join, contribute, payout (4 types), penalties |
| `reputation-registry` | 19 KB | 3 | MoiScore (0-1000) on-chain reputation |
| `governance-token` | 20 KB | 11 | MOI governance token (SEP-41) |
| `treasury` | 17 KB | 5 | Protocol fee collection |
| `common` | 608 B | — | Shared: VRF, math, access control, pause |

## Quick Start

```bash
# Build all contracts
cargo build --target wasm32v1-none --release
make optimize

# Run tests
cargo test

# Deploy to testnet (requires soroban-cli)
make deploy-testnet

# Generate Go bindings
make bindings
```

## Testnet

- Network: Stellar Testnet
- RPC: https://soroban-testnet.stellar.org
- Horizon: https://horizon-testnet.stellar.org

## Security

- All mutating functions require caller authorization
- Emergency pause/unpause via common library
- Access control: admin, organizer, member roles
- No `unwrap()` in production code
- Safe math via `checked_add/sub/mul/div`
- Typed error enums with numeric codes

## License

Apache 2.0
