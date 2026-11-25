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

## Proptest regression seeds

Proptest writes minimized failing inputs to `proptest-regressions/` next to the
crate that ran the test. Those files are generated artifacts, so the default
ignore rules keep them out of version control. To preserve a discovered failure
as tracked regression evidence, promote it into the curated location:

1. Reproduce the failure locally with `cargo test` and confirm the minimized
   seed file appears under `proptest-regressions/` (for example
   `stellar-contracts/proptest-regressions/<test>.txt`).
2. Copy the seed file into the tracked curated directory, preserving the
   relative path used by the test:

   ```bash
   mkdir -p proptest-regressions/curated
   cp stellar-contracts/proptest-regressions/<test>.txt \
      proptest-regressions/curated/<test>.txt
   ```

3. Commit the curated seed together with the fix or the regression test that
   exercises it. Only files under `proptest-regressions/curated/` are tracked;
   ad-hoc and generated regression files elsewhere remain ignored.

Curated seeds are reviewed like any other test fixture: keep them minimal, name
them after the test they belong to, and remove them once the underlying case is
covered by an explicit unit test.

## Contract initialization

Set `CONTRACT_ID` and use valid Soroban address arguments. The address that
submits an initialization call must authorize the transaction.

### Single administrator

```bash
stellar contract invoke --id "$CONTRACT_ID" --source "$ADMIN_SECRET" --network testnet -- init_admin --admin "$ADMIN_ADDRESS"
```

`init_admin` accepts exactly one argument: `admin: Address`.

### Multisig administrators

The invoker must appear in the admin list, and `threshold` must be at least one
and no greater than the number of admins.

```bash
stellar contract invoke --id "$CONTRACT_ID" --source "$ADMIN_SECRET" --network testnet -- init_multisig --invoker "$ADMIN_ADDRESS" --admins '["'$ADMIN_ADDRESS'", "'$SECOND_ADMIN_ADDRESS'"]' --threshold 2
```

For a three-phase bootstrap, propose the same configuration and then have the
required admins confirm it:

```bash
stellar contract invoke --id "$CONTRACT_ID" --source "$ADMIN_SECRET" --network testnet -- propose_init --admins '["'$ADMIN_ADDRESS'", "'$SECOND_ADMIN_ADDRESS'"]' --threshold 2
stellar contract invoke --id "$CONTRACT_ID" --source "$ADMIN_SECRET" --network testnet -- confirm_init --confirmer "$ADMIN_ADDRESS"
```

The contract signatures are `propose_init(admins: Vec<Address>, threshold:
u32)`, `confirm_init(confirmer: Address)`, `init_admin(admin: Address)`, and
`init_multisig(invoker: Address, admins: Vec<Address>, threshold: u32)`. Do not
invoke `propose_init` without `admins` and `threshold`.

## Wasm Size Audit

The transfer/adoption contract is audited with `twiggy` after building the
Soroban Wasm target. Like the root `stellar-contracts` crate, it depends on
`soroban-sdk` 28, so the artifact must come from `stellar contract build`
(stellar-cli >= v25.2.0), which targets `wasm32v1-none`; a plain `cargo
build` for a wasm target now hard-errors:

```bash
cd stellar-contracts/contracts/pet-transfer-adoption
stellar contract build --optimize=false
twiggy top -n 12 ../../../target/wasm32v1-none/release/pet_transfer_adoption.wasm
```

Measured reduction after enabling a contract-local size release profile
(`opt-level = "z"`, LTO, single codegen unit, stripped symbols, abort panics)
(historical measurement, taken against the then-current
`cargo build --target wasm32-unknown-unknown` output; re-measure against the
current `stellar contract build` artifact before relying on these numbers):

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
