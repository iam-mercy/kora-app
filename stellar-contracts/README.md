# Stellar Contracts

This crate contains the main Soroban smart contract for Kora App, plus the nested transfer/adoption contract package under `contracts/pet-transfer-adoption/`.

## Commands

```bash
cargo fmt
cargo test
```

To build a release artifact (soroban-sdk 28 targets `wasm32v1-none` and requires
`stellar contract build` from stellar-cli >= v25.2.0):

```bash
stellar contract build --optimize=false
wasm-opt -Oz --mvp-features \
  ../target/wasm32v1-none/release/kora_stellar.wasm \
  -o ../target/wasm32v1-none/release/kora_stellar.optimized.wasm
```

> The optimized artifact is ~219 KB, which is over Soroban's 128 KiB on-chain
> contract-size cap — it is not deployable to a live network until the contract
> is split. See [issue #2](https://github.com/iam-mercy/kora-app/issues/2).

## Notes

- The main contract source lives in `src/lib.rs`.
- The contract test suite is split across focused `src/test_*.rs` modules.
- High-level repository docs now live in the root `docs/` directory.
