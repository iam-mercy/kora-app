# celo-contracts

`KoraRegistry` is the Solidity contract that backs Kora App on the Celo network.

## Prerequisites

- [Node.js](https://nodejs.org/) 18.x or 20.x (LTS)
- npm (bundled with Node.js)

## Setup

```bash
cd celo-contracts
npm install
npx hardhat compile
```

## Environment variables

Create a `.env` file in `celo-contracts/` (never commit this file):

```bash
PRIVATE_KEY=your_wallet_private_key
CELOSCAN_API_KEY=your_celoscan_api_key
```

| Variable            | Required for                          |
|---------------------|----------------------------------------|
| `PRIVATE_KEY`        | Signing transactions on `alfajores`/`celo` |
| `CELOSCAN_API_KEY`   | Verifying contracts on Celoscan       |

## Running tests

```bash
npx hardhat test
```

## Dependency audit policy

CI runs `npm audit` against the committed `package-lock.json` in a dedicated
`audit` job (see `.github/workflows/celo-contracts.yml`). The gate is
reproducible: it audits the lockfile, not a fresh resolution.

### Severity thresholds

| Dependency scope | Command | Fails on |
|------------------|---------|----------|
| Production       | `npm audit --omit=dev --audit-level=high` | any `high` or `critical` advisory |
| Development      | `npm audit --audit-level=critical`       | any `critical` advisory |

Production advisories are held to a stricter bar because they ship in the
deployed contract tooling; development-only advisories (Hardhat, test
harnesses) only block on `critical`.

Run the same checks locally:

```bash
npm audit --omit=dev --audit-level=high
npm audit --audit-level=critical
```

### Accepted advisories

Advisories that cannot be fixed immediately must be explicitly accepted, never
silently ignored. To accept an advisory:

1. Add an entry to `audit-allowlist.json` in this directory with the advisory
   `id`, the affected `package`, a `justification`, and an `expires` date
   (`YYYY-MM-DD`).
2. Reference the tracking issue in the `justification`.
3. The audit job fails if an allowlisted advisory has passed its `expires`
   date, so accepted risk is re-reviewed on a bounded schedule.

Allowlist entries are reviewed on expiry; remove the entry once the dependency
is upgraded or the advisory is withdrawn.

## Scripts

### `scripts/deploy.js`

Deploys `KoraRegistry` and prints its address.

```bash
# Local network (no env vars needed)
npx hardhat run scripts/deploy.js --network hardhat

# Celo Alfajores testnet
npx hardhat run scripts/deploy.js --network alfajores

# Celo mainnet
npx hardhat run scripts/deploy.js --network celo
```

### `scripts/register-pet.js`

Registers a sample pet against an already-deployed `KoraRegistry`. Requires
`CONTRACT_ADDRESS` to be set to the address printed by `deploy.js`.

```bash
CONTRACT_ADDRESS=0xDeployedAddress npx hardhat run scripts/register-pet.js --network alfajores
```

## Networks

| Network    | Chain ID | RPC URL                                   |
|------------|----------|--------------------------------------------|
| `alfajores`| 44787    | https://alfajores-forno.celo-testnet.org    |
| `celo`     | 42220    | https://forno.celo.org                      |
