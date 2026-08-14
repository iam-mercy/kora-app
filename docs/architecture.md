# Architecture

## Repository Layout

```text
Kora-App-Contracts/
├── README.md
├── SECURITY.md
├── CONTRIBUTING.md
├── CHANGELOG.md
├── docs/
│   ├── architecture.md
│   ├── development.md
│   ├── api.md
│   └── error-codes.md
├── stellar-contracts/
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs
│   │   └── test_*.rs
│   └── contracts/
│       └── pet-transfer-adoption/
└── celo-contracts/
    ├── package.json
    ├── hardhat.config.js
    └── contracts/
        └── KoraRegistry.sol
```

## Components

### `stellar-contracts`

The main Soroban contract crate. It contains the primary Kora App smart contract, including:

- pet registration and ownership
- veterinary access control
- medical records, vaccinations, and attachments
- emergency data and consent flows
- activity, grooming, and insurance features
- multisig admin and upgrade flows

The nested `contracts/pet-transfer-adoption` package is a smaller ownership-transfer contract with its own tests.

### `celo-contracts`

A parallel Solidity implementation (`KoraRegistry.sol`) targeting the Celo network via Hardhat:

- vet, pet, and medical record registry
- pausable emergency-stop

## Verification Status

As of this cleanup:

- `cd stellar-contracts && cargo test` passes
