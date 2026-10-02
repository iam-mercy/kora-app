# Architecture

## Repository Layout

```text
kora-app/
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

## Dual-Chain Implementation Matrix

The Stellar and Celo contracts are related implementations, not feature-for-
feature equivalents. Integrations must select behavior from the target chain.

| Capability | Stellar / Soroban | Celo / EVM |
|---|---|---|
| Ownership transfer | Two-step flow: the current owner proposes a new owner and the recipient accepts with `accept_pet_transfer`. | One-step `transferPet(petId, to)` by the current owner; the recipient does not accept. |
| Sensitive data | Pet sensitive fields use encrypted data with nonce and ciphertext; reads apply privacy and caller access checks. | `Pet` and `MedicalRecord` fields are Solidity strings stored directly on-chain; no field-level encryption is implemented. |
| Medical records | Verified vets add records; caller-aware reads and privacy/access grants govern retrieval. | Verified vets add records; per-pet arrays and public view functions expose records. |
| Administration | Supports single-admin initialization and multisig admin lists, thresholds, approvals, quorum, and proposals. | Constructor assigns one `admin`; administrative operations use `onlyAdmin`. |
| Emergency behavior | Emergency views include caller authorization and audited access; encrypted emergency data is decrypted by the contract. | OpenZeppelin `Pausable` lets the admin halt state-changing operations; it is not an emergency data-override equivalent. |
| Storage model | Soroban instance/persistent storage with contract types and encrypted payloads. | EVM mappings and dynamic arrays containing plaintext Solidity structs. |

### Parity roadmap

Feature parity would require a Celo privacy and authorization design, encrypted
payload/key handling, grant-aware reads, a multisig governance layer, and a
two-step transfer workflow. Until those are implemented and audited, bridges
and clients must preserve the differences above.
