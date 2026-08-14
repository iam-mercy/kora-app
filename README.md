# Kora App

Kora App is a blockchain-based pet registry and veterinary records platform.
It gives pet owners, veterinarians, and insurers a shared, tamper-evident
source of truth for pet identity, ownership history, medical records, and
care events — enforced on-chain instead of trusted to a single database.

This repository is a monorepo: it holds both on-chain implementations of the
registry, plus the docs that describe how they work together.

## What's inside

- **`stellar-contracts/`** — the primary implementation: a Soroban (Rust)
  smart contract on Stellar, plus a nested pet transfer/adoption contract.
- **`celo-contracts/`** — a parallel Solidity implementation of the registry
  (`KoraRegistry.sol`) targeting the Celo network via Hardhat.
- **`docs/`** — architecture notes, the smart contract API reference, error
  codes, environment variables, and the OpenAPI spec for the companion 2FA
  backend service.

## Core capabilities

- Pet registration, profiles, and ownership transfer/adoption flows
- Veterinarian registration and verification
- Medical records, vaccinations, lab results, and attachments
- Emergency contacts and emergency access
- Insurance, grooming, nutrition, activity, and behavior tracking
- Multisig admin, upgrade proposals, and audit logging

## Quick Start

### Stellar contracts

```bash
cd stellar-contracts
cargo test
```

### Celo contracts

```bash
cd celo-contracts
npm install
npx hardhat compile
```

## Documentation

- [Architecture](docs/architecture.md)
- [Development](docs/development.md)
- [API Overview](docs/api.md)
- [Error Codes](docs/error-codes.md)
- [Security Policy](SECURITY.md)
- [Contributing](CONTRIBUTING.md)

## License

MIT
