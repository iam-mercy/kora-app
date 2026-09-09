# Development Notes

## Soroban Gas Profiling

Budget measurements are taken in unit tests with `env.cost_estimate().budget()` after fixture setup, so setup cost is excluded and only the target call is tracked.

### Top 10 Profiled Functions

| Function | Baseline Instructions | Optimized Instructions | Memory Bound | Notes |
|---|---:|---:|---:|---|
| `get_behavior_by_type` | 17,500,000 | 14,000,000 | 2,500,000 | Avoids building an intermediate full history vector before filtering. |
| `get_activity_stats` | 15,000,000 | 12,000,000 | 2,500,000 | Iterates indexed storage directly instead of materializing full history. |
| `get_consent_history_page` | 12,000,000 | 9,000,000 | 2,000,000 | Reads only requested page indices instead of all consent history. |
| `get_activity_summary` | 15,000,000 | 12,000,000 | 2,500,000 | Shares the direct indexed iteration pattern. |
| `get_behavior_history` | 11,000,000 | 11,000,000 | 2,500,000 | Baseline retained. |
| `get_activity_history` | 10,500,000 | 10,500,000 | 2,500,000 | Baseline retained. |
| `get_pet_insurance_claims` | 10,000,000 | 10,000,000 | 2,500,000 | Baseline retained. |
| `get_training_milestones` | 9,500,000 | 9,500,000 | 2,500,000 | Baseline retained. |
| `get_active_consents` | 9,000,000 | 9,000,000 | 2,000,000 | Baseline retained. |
| `get_vet_reviews` | 8,500,000 | 8,500,000 | 2,000,000 | Baseline retained. |

### Regression Bounds

`gas_profile_tests` in `stellar-contracts/src/lib.rs` asserts that optimized instruction and memory costs remain under the documented optimized bounds for `get_behavior_by_type`, `get_activity_stats`, and `get_consent_history_page`.

# Development

## Prerequisites

- Rust toolchain
- Stellar CLI for contract work
- Node.js + npm for `celo-contracts` (Hardhat)

## Common Commands

### Stellar contracts

```bash
cd stellar-contracts
cargo fmt
cargo test
```

### Celo contracts

```bash
cd celo-contracts
npx hardhat test
```

## Notes

- The repo contains a Rust Soroban crate and a separate Hardhat/Solidity project; build and test them separately.

## Wasm Size Audit

The transfer/adoption contract is audited with `twiggy` after building the
Soroban Wasm target:

```bash
cd stellar-contracts/contracts/pet-transfer-adoption
cargo build --release --target wasm32-unknown-unknown
twiggy top -n 12 target/wasm32-unknown-unknown/release/pet_transfer_adoption.wasm
```

Measured reduction after enabling a contract-local size release profile
(`opt-level = "z"`, LTO, single codegen unit, stripped symbols, abort panics):

| Build | Wasm bytes |
|---|---:|
| Baseline release profile | 49,332 |
| Size-tuned release profile | 41,245 |
| Reduction | 8,087 bytes (16.39%) |

Top `twiggy` contributors in the optimized artifact:

| Contributor | Bytes | Share |
|---|---:|---:|
| custom section `contractspecv0` | 8,344 | 20.23% |
| `data[0]` | 1,953 | 4.74% |
| largest code body `code[95]` | 1,359 | 3.29% |

The root `stellar-contracts` crate also produces a release Wasm artifact.
It is a workspace member, so the build output lands in the workspace-root
`target/` directory. `soroban-sdk` 28 requires the build to go through
`stellar contract build` (stellar-cli >= v25.2.0), which targets
`wasm32v1-none` and runs the spec-shaking strip; a plain `cargo build` for a
wasm target now hard-errors.

```bash
cd stellar-contracts
stellar contract build --optimize=false
wasm-opt -Oz --mvp-features \
  ../target/wasm32v1-none/release/kora_stellar.wasm \
  -o ../target/wasm32v1-none/release/kora_stellar.optimized.wasm
twiggy top -n 12 ../target/wasm32v1-none/release/kora_stellar.optimized.wasm
```

The build emits `target/wasm32v1-none/release/kora_stellar.wasm` (relative to
the repo root; ~253,600 bytes), and the `wasm-opt` pass brings it to
~219,161 bytes.

> **Not deployable yet.** That is still ~88 KB over Soroban's 131,072-byte
> `contract_max_size_bytes` cap, so the contract cannot be uploaded to any live
> network until it is split. See
> [issue #2](https://github.com/iam-mercy/kora-app/issues/2).
