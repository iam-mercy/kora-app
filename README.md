# Kora App Contracts

Smart contracts for Kora App's on-chain pet registry: ownership, veterinary access,
medical records, and adoption/transfer flows.

## Repository Layout

```text
Kora-App-Contracts/
├── stellar-contracts/       # Soroban smart contract crate
│   ├── src/lib.rs           # Main contract
│   ├── src/test_*.rs        # Test modules
│   └── contracts/
│       └── pet-transfer-adoption/
├── celo-contracts/          # Solidity registry (Hardhat, Celo network)
│   └── contracts/
│       └── KoraRegistry.sol
└── docs/
    ├── architecture.md
    ├── development.md
    ├── api.md
    └── error-codes.md
```

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
