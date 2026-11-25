# celo-contracts

`KoraRegistry` is the Solidity contract that backs Kora App on the Celo network.

## Prerequisites

- [Node.js](https://nodejs.org/) 20.x (LTS, matches CI; see `.nvmrc` and `engines` in `package.json`)
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

| Variable           | Required for                               | Default |
|--------------------|--------------------------------------------|---------|
| `PRIVATE_KEY`      | Signing transactions on `alfajores`/`celo` | none    |
| `CELOSCAN_API_KEY` | Verifying contracts on Celoscan            | none    |

`scripts/register-pet.js` additionally reads these variables (set them in the
shell or `.env`):

| Variable           | Required | Default      | Validation |
|--------------------|----------|--------------|------------|
| `CONTRACT_ADDRESS` | yes      | none         | Script aborts if unset |
| `PET_NAME`         | no       | `Rex`        | Must be non-empty (not whitespace) |
| `PET_SPECIES`      | no       | `Dog`        | Must be non-empty (not whitespace) |
| `PET_BREED`        | no       | `Labrador`   | Must be non-empty (not whitespace) |
| `PET_BIRTHDAY`     | no       | `2020-01-01` | Must be non-empty (not whitespace) |

Complete example:

```bash
CONTRACT_ADDRESS=0xDeployedAddress \
PET_NAME=Rex PET_SPECIES=Dog PET_BREED=Labrador PET_BIRTHDAY=2020-01-01 \
npx hardhat run scripts/register-pet.js --network alfajores
```

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

### Persistent local node

The in-process `hardhat` network is ephemeral: every `npx hardhat run` creates
a fresh chain, so an address deployed in one invocation does not exist in the
next. To deploy and then register, start a persistent node and use the
`localhost` network:

```bash
# Terminal 1: keep this running
npx hardhat node

# Terminal 2: deploy; the address is printed (also shown in the node's logs)
npx hardhat run scripts/deploy.js --network localhost

# Register against that address
CONTRACT_ADDRESS=0xAddressPrintedByDeploy npx hardhat run scripts/register-pet.js --network localhost
```

Restarting the node resets the chain, so redeploy and use the new address.

## Networks

| Network    | Chain ID | RPC URL                                   |
|------------|----------|--------------------------------------------|
| `alfajores`| 44787    | https://alfajores-forno.celo-testnet.org    |
| `celo`     | 42220    | https://forno.celo.org                      |
