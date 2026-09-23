# Drips Wave 9 — Kora App Engineering Backlog

### Repository
`iam-mercy/kora-app`

### Audit scope
A comprehensive, line-by-line engineering audit of the entire Kora App repository was conducted across all subsystems:
- **Celo Contracts**: Solidity implementation (`celo-contracts/contracts/KoraRegistry.sol`), deployment scripts, Hardhat configuration, and test suites (`celo-contracts/test/KoraRegistry.test.js`).
- **Stellar/Soroban Contracts**: Core registry, storage models, access controls, encryption, upgradeability, multisig governance, insurance, and breeding subsystems in `stellar-contracts/src/lib.rs` (12,221 lines), plus peripheral contracts in `stellar-contracts/contracts/pet-transfer-adoption/`.
- **Testing Infrastructure**: 71 test modules in `stellar-contracts/src/`, test harnesses, proptest fuzzers, and unit/integration coverage gaps.
- **CI/CD & Automation**: GitHub Actions workflow configurations (`stellar-contracts.yml`, `stellar.yml`, `celo-contracts.yml`, `pr-review-bot.yml`, `auto-merge.yml`).
- **API & Documentation**: Architecture guides, OpenAPI specifications, error registries, and developer guides in `docs/` (`architecture.md`, `api.md`, `openapi.yaml`, `error-codes.md`, `development.md`).

### Backlog purpose
This backlog establishes a prioritized, actionable queue of exactly 50 concrete engineering issues identified through codebase investigation. Each issue is grounded directly in existing repository files, line numbers, and symbols, defining precise problem statements, engineering rationale, proposed remediation paths, testable acceptance criteria, and dependency links.

### Summary
- **Total issues**: 50
- **Priority breakdown**:
  - **P0 (Critical / Blocking)**: 9
  - **P1 (High)**: 20
  - **P2 (Medium)**: 19
  - **P3 (Lower priority)**: 2
- **Category breakdown**:
  - **Smart Contract**: 8
  - **Access Control**: 6
  - **CI/CD**: 6
  - **Security**: 5
  - **Data Integrity**: 5
  - **Celo/Solidity**: 4
  - **Documentation**: 4
  - **Testing**: 3
  - **Stellar/Soroban**: 2
  - **Performance**: 2
  - **Upgradeability**: 2
  - **API/Backend**: 1
  - **Architecture**: 1
  - **Technical Debt**: 1

### Published GitHub Issues Index
All 50 engineering backlog issues have been published to GitHub in `iam-mercy/kora-app` with the `wave-9` label:

| Wave 9 # | GitHub Issue | Title |
|:---:|:---:|---|
| #1 | [#4](https://github.com/iam-mercy/kora-app/issues/4) | Resolve variable shadowing, duplicate declarations, and inverted state emission in `correctMedicalRecord` |
| #2 | [#5](https://github.com/iam-mercy/kora-app/issues/5) | Enforce `whenNotPaused` modifier on `correctMedicalRecord` in `KoraRegistry` |
| #3 | [#6](https://github.com/iam-mercy/kora-app/issues/6) | Implement two-step ownership transfer (`Ownable2Step`) for admin role in `KoraRegistry` |
| #4 | [#7](https://github.com/iam-mercy/kora-app/issues/7) | Enforce recipient consent pattern for `transferPet` to prevent forced pet transfers |
| #5 | [#8](https://github.com/iam-mercy/kora-app/issues/8) | Deduplicate redundant `correctMedicalRecord` test suites in Celo Hardhat tests |
| #6 | [#9](https://github.com/iam-mercy/kora-app/issues/9) | Upgrade `pet-transfer-adoption` contract to Soroban SDK v28 and resolve transitive `ed25519-dalek` compilation failure |
| #7 | [#10](https://github.com/iam-mercy/kora-app/issues/10) | Integrate `pet-transfer-adoption` into root workspace `Cargo.toml` and CI test matrices |
| #8 | [#11](https://github.com/iam-mercy/kora-app/issues/11) | Implement systematic TTL extension strategy across all persistent and instance storage |
| #9 | [#12](https://github.com/iam-mercy/kora-app/issues/12) | Migrate unbounded pet records and audit logs from instance storage to persistent storage |
| #10 | [#13](https://github.com/iam-mercy/kora-app/issues/13) | Implement `decrement_pet_storage` to restore user quota upon record purging and photo removal |
| #11 | [#14](https://github.com/iam-mercy/kora-app/issues/14) | Replace O(N*M) quadratic iteration in `compact_storage` with single-pass compaction |
| #12 | [#15](https://github.com/iam-mercy/kora-app/issues/15) | Restrict public unauthenticated medical record queries to pet owners and authorized veterinarians |
| #13 | [#16](https://github.com/iam-mercy/kora-app/issues/16) | Enforce caller authentication on `add_activity_record` |
| #14 | [#17](https://github.com/iam-mercy/kora-app/issues/17) | Enforce caller authorization and breed registry permissions on `set_pet_traits` |
| #15 | [#18](https://github.com/iam-mercy/kora-app/issues/18) | Validate sire and dam existence, gender compatibility, and caller identity in `add_breeding_record` |
| #16 | [#19](https://github.com/iam-mercy/kora-app/issues/19) | Fix COI (Coefficient of Inbreeding) calculation to account for parent-child inbreeding |
| #17 | [#20](https://github.com/iam-mercy/kora-app/issues/20) | Make `ActivityKey::IdempotencyWindow` configurable and implement `purge_expired_idempotency_keys` |
| #18 | [#21](https://github.com/iam-mercy/kora-app/issues/21) | Enforce `caller.require_auth()` in `get_emergency_info_with_reason` and `notify_emergency_contacts` to prevent audit forgery |
| #19 | [#22](https://github.com/iam-mercy/kora-app/issues/22) | Replace static public-key pseudo-encryption in `get_encryption_key` with secure user-derived key management |
| #20 | [#23](https://github.com/iam-mercy/kora-app/issues/23) | Enforce upgrade proposal timelock delay and multi-admin quorum in `execute_upgrade` |
| #21 | [#24](https://github.com/iam-mercy/kora-app/issues/24) | Correct `PreviousWasmHash` storage in `execute_upgrade` to prevent corrupted upgrade rollbacks |
| #22 | [#25](https://github.com/iam-mercy/kora-app/issues/25) | Prevent single-admin denial of service on multisig threshold proposals |
| #23 | [#26](https://github.com/iam-mercy/kora-app/issues/26) | Require multisig proposal threshold to modify `set_quorum_percent` and prevent zero-quorum bypass |
| #24 | [#27](https://github.com/iam-mercy/kora-app/issues/27) | Deprecate unauthenticated `propose_init` and resolve competing contract initialization entrypoints |
| #25 | [#28](https://github.com/iam-mercy/kora-app/issues/28) | Fix `new_owner` initialization in `register_pet` and implement pet transfer cancellation in `KoraContract` |
| #26 | [#29](https://github.com/iam-mercy/kora-app/issues/29) | Fix caller resolution in `get_pet_age`, `get_pet_age_with_lifespan`, and `get_pet_by_tag` for non-public pets |
| #27 | [#30](https://github.com/iam-mercy/kora-app/issues/30) | Synchronize error codes between `ContractError`, `KoraError`, and runtime error registry |
| #28 | [#31](https://github.com/iam-mercy/kora-app/issues/31) | Replace full linear scan in `matching_subscription_ids` with indexed subscription mapping |
| #29 | [#32](https://github.com/iam-mercy/kora-app/issues/32) | Fix pagination offset boundary check in `get_lab_results` |
| #30 | [#33](https://github.com/iam-mercy/kora-app/issues/33) | Re-link and fix 49 orphaned test modules in `stellar-contracts/src/` into the crate test harness |
| #31 | [#34](https://github.com/iam-mercy/kora-app/issues/34) | Deduplicate redundant CI workflows between `stellar-contracts.yml` and `stellar.yml` |
| #32 | [#35](https://github.com/iam-mercy/kora-app/issues/35) | Cache `stellar-cli` installation in GitHub Actions workflows to eliminate redundant compilation |
| #33 | [#36](https://github.com/iam-mercy/kora-app/issues/36) | Fix unsound auto-merge rule that bypasses pull request review approval in `auto-merge.yml` |
| #34 | [#37](https://github.com/iam-mercy/kora-app/issues/37) | Add test and build steps for `pet-transfer-adoption` contract to CI pipeline |
| #35 | [#38](https://github.com/iam-mercy/kora-app/issues/38) | Fix permission scopes and branch triggers in `pr-review-bot.yml` |
| #36 | [#39](https://github.com/iam-mercy/kora-app/issues/39) | Synchronize `docs/error-codes.md` with on-chain `ContractError` enum variants |
| #37 | [#40](https://github.com/iam-mercy/kora-app/issues/40) | Update `docs/api.md` and `docs/openapi.yaml` to document missing medical record privacy and authorization parameters |
| #38 | [#41](https://github.com/iam-mercy/kora-app/issues/41) | Update `docs/architecture.md` to document dual-chain differences between Stellar and Celo implementations |
| #39 | [#42](https://github.com/iam-mercy/kora-app/issues/42) | Correct contract initialization and multisig onboarding guides in `docs/development.md` |
| #40 | [#43](https://github.com/iam-mercy/kora-app/issues/43) | Align OpenAPI schema response structures for pet transfer and adoption records |
| #41 | [#44](https://github.com/iam-mercy/kora-app/issues/44) | Prevent duplicate pet microchip registration across different owners |
| #42 | [#45](https://github.com/iam-mercy/kora-app/issues/45) | Add expiration timestamp check to veterinarian license verification status |
| #43 | [#46](https://github.com/iam-mercy/kora-app/issues/46) | Implement insurance policy registration and claim payout status transition |
| #44 | [#47](https://github.com/iam-mercy/kora-app/issues/47) | Validate dosage, frequency, and prescribing veterinarian in `Medication` records |
| #45 | [#48](https://github.com/iam-mercy/kora-app/issues/48) | Implement caller authorization and privacy checks for pet photo queries and deletion |
| #46 | [#49](https://github.com/iam-mercy/kora-app/issues/49) | Prevent deactivated or unverified veterinarians from adding medical and vaccination records |
| #47 | [#50](https://github.com/iam-mercy/kora-app/issues/50) | Validate diet plan calorie targets and macronutrient percentage totals |
| #48 | [#51](https://github.com/iam-mercy/kora-app/issues/51) | Prevent re-initialization of insurance policy parameters and duplicate claim submissions |
| #49 | [#52](https://github.com/iam-mercy/kora-app/issues/52) | Add integration test suite for cross-contract calls between `KoraContract` and `pet-transfer-adoption` |
| #50 | [#53](https://github.com/iam-mercy/kora-app/issues/53) | Clean up unused dependencies and redundant profile configurations in `stellar-contracts` |

---

## #1: Resolve variable shadowing, duplicate declarations, and inverted state emission in `correctMedicalRecord`

### Category
Celo/Solidity

### Priority
P0

### Problem
In `celo-contracts/contracts/KoraRegistry.sol`, the function `correctMedicalRecord` contains severe variable shadowing, duplicate state variable and event declarations, and an inverted event parameter emission where the updated diagnosis is recorded as the original diagnosis.
1. `mapping(uint256 => uint256) private _recordIndex;` is declared twice at lines 91 and 97.
2. `event MedicalRecordCorrected` is declared twice with conflicting signatures: line 148 defines 8 parameters, while line 172 defines 9 parameters.
3. Within `correctMedicalRecord` (lines 480–536), `uint256 petId` is declared twice within the same function scope (first from `_recordPetId[recordId]`, then re-declared from `_recordPet[recordId]`).
4. `MedicalRecord storage rec` is declared twice in the function body (lines 498 and 509).
5. At lines 514–519, `rec.diagnosis = diagnosis;` mutates storage *before* line 522 retrieves `string memory origDiagnosis = rec.diagnosis;`. Consequently, `origDiagnosis` contains the newly written diagnosis, causing the subsequent event emission to record identical values for both old and new diagnoses.
6. The event `MedicalRecordCorrected` is emitted twice within the single function call (lines 503 and 524).

### Current implementation
- `celo-contracts/contracts/KoraRegistry.sol`:
  - Lines 91 & 97: Duplicate declaration of `_recordIndex`.
  - Lines 148–157 & 172–181: Conflicting duplicate declarations of `event MedicalRecordCorrected`.
  - Lines 480–536: `correctMedicalRecord(uint256 recordId, string calldata diagnosis, string calldata treatment, string calldata notes, string calldata reason)` contains duplicate variable allocations and premature storage overwrite before event logging.

### Why it matters
The duplicate event declarations and shadowed variables lead to compiler warnings or compilation failure depending on Solidity compiler settings. More critically, mutating `rec.diagnosis` before reading `origDiagnosis` completely corrupts on-chain auditability: indexers and downstream consumers receive an event indicating the diagnosis did not change, destroying veterinary dispute logs.

### Proposed work
- Remove duplicate `_recordIndex` mapping declaration.
- Unify `MedicalRecordCorrected` into a single canonical event definition with consistent parameter ordering.
- Refactor `correctMedicalRecord` to eliminate shadowed `petId` and `rec` variables.
- Cache original record values (`origDiagnosis`, `origTreatment`, `origNotes`) into memory *before* updating storage fields.
- Remove redundant duplicate event emission so exactly one event is emitted per correction.

### Acceptance criteria
- `KoraRegistry.sol` compiles cleanly without shadowing warnings.
- `MedicalRecordCorrected` is defined once and emitted exactly once per transaction.
- Emitted event correctly reflects `origDiagnosis` as the pre-mutation value and `newDiagnosis` as the updated value.
- State updates to `diagnosis`, `treatment`, `notes`, and audit history preserve historical integrity.

### Testing requirements
- Unit tests verifying `origDiagnosis != newDiagnosis` in emitted event logs.
- Hardhat tests verifying rejection when non-veterinarians attempt record correction.
- Regression tests ensuring medical record index mapping remains intact after correction.

### Dependencies
Dependencies: None

### Code references
- `celo-contracts/contracts/KoraRegistry.sol`: lines 91, 97, 148–181, 480–536
- `celo-contracts/test/KoraRegistry.test.js`: lines 551–575, 863–885

---

## #2: Enforce `whenNotPaused` modifier on `correctMedicalRecord` in `KoraRegistry`

### Category
Celo/Solidity

### Priority
P1

### Problem
In `celo-contracts/contracts/KoraRegistry.sol`, emergency pausing via `Pausable` is implemented across critical state-changing functions (`registerPet`, `addMedicalRecord`, `transferPet`, `addVaccination`). However, `correctMedicalRecord` (line 480) omits the `whenNotPaused` modifier.

### Current implementation
In `celo-contracts/contracts/KoraRegistry.sol`:
- `addMedicalRecord` at line 440 declares: `external whenNotPaused onlyVerifiedVet returns (uint256)`.
- `correctMedicalRecord` at line 480 declares: `external onlyVerifiedVet returns (bool)` (missing `whenNotPaused`).

### Why it matters
If the contract is paused due to an active exploit, veterinary credential compromise, or data corruption incident, an attacker holding a compromised vet key can continue to modify existing medical records while the rest of the contract is paused. Emergency pause semantics must be consistent across all state mutations.

### Proposed work
- Add `whenNotPaused` modifier to `correctMedicalRecord` in `celo-contracts/contracts/KoraRegistry.sol`.
- Ensure test coverage verifies that pausing the registry reverts calls to `correctMedicalRecord`.

### Acceptance criteria
- `correctMedicalRecord` cannot be executed when `paused()` is `true`.
- Invoking `correctMedicalRecord` while paused reverts with `Pausable: paused` (or custom pause error).
- Unpausing the contract restores normal correction functionality for verified vets.

### Testing requirements
- Hardhat test: Pause contract via admin account and verify `correctMedicalRecord` reverts.
- Unpause contract and verify `correctMedicalRecord` executes successfully.

### Dependencies
Depends on #1

### Code references
- `celo-contracts/contracts/KoraRegistry.sol`: lines 440, 480
- `celo-contracts/test/KoraRegistry.test.js`

---

## #3: Implement two-step ownership transfer (`Ownable2Step`) for admin role in `KoraRegistry`

### Category
Celo/Solidity

### Priority
P2

### Problem
In `celo-contracts/contracts/KoraRegistry.sol`, the `transferAdmin` function directly updates the contract administrator in a single transaction without two-step recipient confirmation.

### Current implementation
In `celo-contracts/contracts/KoraRegistry.sol` (lines 221–226):
```solidity
function transferAdmin(address newAdmin) external onlyAdmin {
    require(newAdmin != address(0), "KoraRegistry: new admin is zero address");
    address oldAdmin = admin;
    admin = newAdmin;
    emit AdminTransferred(oldAdmin, newAdmin);
}
```

### Why it matters
If the current admin provides a typoed address or transfers administration to an uninitialized contract or account without key recovery, administrative control is permanently and irreversibly lost. The OpenZeppelin `Ownable2Step` pattern mitigates this existential operational risk.

### Proposed work
- Introduce a pending admin state (`pendingAdmin`).
- Split admin transfer into `proposeAdmin(address newAdmin)` and `acceptAdmin()`.
- Emit `AdminTransferProposed` and `AdminTransferred` events accordingly.
- Restrict `acceptAdmin()` to `msg.sender == pendingAdmin`.

### Acceptance criteria
- Admin address is not updated upon initial proposal.
- Only the proposed pending admin can call `acceptAdmin()`.
- Zero-address checks are enforced on proposal.
- Calling `proposeAdmin` overwrites any prior pending proposals.

### Testing requirements
- Test successful two-step transfer: propose -> accept -> verify new admin.
- Test rejection when non-pending account attempts `acceptAdmin()`.
- Test proposing zero address reverts.

### Dependencies
Dependencies: None

### Code references
- `celo-contracts/contracts/KoraRegistry.sol`: lines 221–226
- `celo-contracts/test/KoraRegistry.test.js`

---

## #4: Enforce recipient consent pattern for `transferPet` to prevent forced pet transfers

### Category
Celo/Solidity

### Priority
P1

### Problem
In `celo-contracts/contracts/KoraRegistry.sol` (lines 364–382), `transferPet` executes an instantaneous transfer of pet ownership directly from `msg.sender` to `address to`. This contradicts the Stellar/Soroban implementation and industry security standards by omitting a 2-step offer/acceptance workflow.

### Current implementation
In `celo-contracts/contracts/KoraRegistry.sol`:
```solidity
function transferPet(uint256 petId, address to) external whenNotPaused {
    require(_pets[petId].owner == msg.sender, "KoraRegistry: not pet owner");
    require(to != address(0), "KoraRegistry: transfer to zero address");
    require(to != msg.sender, "KoraRegistry: transfer to current owner");
    _transferPet(petId, msg.sender, to);
}
```
In contrast, `stellar-contracts/src/lib.rs` (lines 4748, 5928–5960) requires a recipient to call `accept_pet_transfer`.

### Why it matters
An owner can unilaterally transfer legal/medical liability or unwanted pet records onto any arbitrary wallet address without the recipient's consent or knowledge. Furthermore, transfers to contracts unable to manage pet ownership permanently strand the pet record.

### Proposed work
- Update `KoraRegistry.sol` to record `pendingOwner` upon transfer initiation (`initiatePetTransfer`).
- Implement `acceptPetTransfer(uint256 petId)` requiring `msg.sender == _pets[petId].pendingOwner`.
- Implement `cancelPetTransfer(uint256 petId)` allowing the current owner to abort pending offers.

### Acceptance criteria
- Ownership does not change until `acceptPetTransfer` is invoked by the intended recipient.
- `cancelPetTransfer` clears `pendingOwner` and allows owner to retain custody.
- Events `PetTransferInitiated`, `PetTransferAccepted`, and `PetTransferCancelled` are emitted.

### Testing requirements
- Unit test: Owner initiates transfer, third party attempts acceptance (reverts), recipient accepts (succeeds).
- Unit test: Owner initiates transfer and cancels before acceptance.
- Cross-chain parity test verifying matching state transitions between Stellar and Celo.

### Dependencies
Dependencies: None

### Code references
- `celo-contracts/contracts/KoraRegistry.sol`: lines 364–382
- `stellar-contracts/src/lib.rs`: lines 5928–5960

---

## #5: Deduplicate redundant `correctMedicalRecord` test suites in Celo Hardhat tests

### Category
Testing

### Priority
P3

### Problem
In `celo-contracts/test/KoraRegistry.test.js`, two duplicate test blocks test `correctMedicalRecord` with identical setups and assertions:
1. Lines 551–575: `describe("correctMedicalRecord", ...)`
2. Lines 863–885: `describe("#920 — correctMedicalRecord", ...)`

### Current implementation
- `celo-contracts/test/KoraRegistry.test.js`:
  - Lines 551–575: Sets up a pet, logs a medical record, calls `correctMedicalRecord`, and asserts diagnosis change.
  - Lines 863–885: Re-executes the exact same fixture, asserts identical event and storage properties.

### Why it matters
Duplicate test suites inflate test run times, create maintenance debt, and mask the absence of tests for actual edge cases (such as non-vet callers, paused states, or mismatched pet IDs).

### Proposed work
- Remove duplicate test suite at lines 863–885.
- Expand lines 551–575 to cover authorization failure, non-existent record IDs, empty diagnosis validation, and paused state behavior.

### Acceptance criteria
- Exactly one `correctMedicalRecord` test suite exists in `KoraRegistry.test.js`.
- Test suite verifies positive correction, event parameters, non-vet rejection, and pause rejection.

### Testing requirements
- Execute `npm test` or `npx hardhat test` inside `celo-contracts/` and verify all tests pass with no duplicate block execution.

### Dependencies
Depends on #1, #2

### Code references
- `celo-contracts/test/KoraRegistry.test.js`: lines 551–575, 863–885

---

## #6: Upgrade `pet-transfer-adoption` contract to Soroban SDK v28 and resolve transitive `ed25519-dalek` compilation failure

### Category
Stellar/Soroban

### Priority
P0

### Problem
The `pet-transfer-adoption` peripheral contract (`stellar-contracts/contracts/pet-transfer-adoption/Cargo.toml`) is pinned to `soroban-sdk = "21.7.7"`, whereas the primary contract (`stellar-contracts/Cargo.toml`) was upgraded to `soroban-sdk = "=28.0.0-rc.1"`.
When building or testing `pet-transfer-adoption`, Cargo resolves `ed25519-dalek v3.0.0` as a transitive dependency through `soroban-env-host`, which breaks compilation:
`the trait bound ChaCha20Rng: ed25519_dalek::rand_core::CryptoRng is not satisfied`.

### Current implementation
- `stellar-contracts/contracts/pet-transfer-adoption/Cargo.toml`:
  - Line 12: `soroban-sdk = "21.7.7"`
  - Line 21: `ed25519-dalek = "2"` (placed only in `[dev-dependencies]`)
  - Line 31: `[workspace]` (declares isolated sub-workspace disconnected from root)

### Why it matters
The `pet-transfer-adoption` contract cannot be compiled or tested. Attempting `cargo check --tests` in that directory fails immediately with compilation errors. Furthermore, Protocol 28 deployment on Stellar network rejects contracts compiled against Protocol 21 SDKs due to missing `contractenvmetav0` metadata.

### Proposed work
- Upgrade `soroban-sdk` in `stellar-contracts/contracts/pet-transfer-adoption/Cargo.toml` to `=28.0.0-rc.1`.
- Align all Soroban dependencies with the root workspace.
- Remove outdated profile overrides and `ed25519-dalek` workarounds that conflict with modern SDK resolution.

### Acceptance criteria
- `cargo check --tests` and `cargo test` pass cleanly inside `stellar-contracts/contracts/pet-transfer-adoption/`.
- No compiler errors regarding `CryptoRng` or `ed25519_dalek`.
- Contract builds target Protocol 28 bytecode.

### Testing requirements
- Execute `cargo test --manifest-path stellar-contracts/contracts/pet-transfer-adoption/Cargo.toml`.
- Execute `stellar contract build` on the sub-crate.

### Dependencies
Dependencies: None

### Code references
- `stellar-contracts/contracts/pet-transfer-adoption/Cargo.toml`: lines 12–22
- `stellar-contracts/Cargo.toml`: lines 10–20

---

## #7: Integrate `pet-transfer-adoption` into root workspace `Cargo.toml` and CI test matrices

### Category
CI/CD

### Priority
P1

### Problem
The root `Cargo.toml` specifies only `members = ["stellar-contracts"]`. The `pet-transfer-adoption` contract (`stellar-contracts/contracts/pet-transfer-adoption/`) is omitted from the workspace. As a result, running `cargo check --all` or `cargo test --all` at root skips this contract entirely.

### Current implementation
In root `Cargo.toml`:
```toml
[workspace]
resolver = "2"
members = [
    "stellar-contracts",
]
```
`stellar-contracts/contracts/pet-transfer-adoption/` has its own isolated `[workspace]` declaration.

### Why it matters
Because the contract is invisible to root workspace tooling, CI workflows (`.github/workflows/stellar-contracts.yml`) never build or test it. Breaking changes in shared types or SDK versions go unnoticed until deployment.

### Proposed work
- Remove isolated `[workspace]` from `stellar-contracts/contracts/pet-transfer-adoption/Cargo.toml`.
- Add `"stellar-contracts/contracts/pet-transfer-adoption"` to `members` in root `Cargo.toml`.
- Update CI workflow jobs to test all workspace members.

### Acceptance criteria
- `cargo test --workspace` at repo root compiles and tests both `kora-stellar` and `pet_transfer_adoption`.
- Root `Cargo.lock` unifies dependency resolution across both crates.

### Testing requirements
- Run `cargo test --workspace` from the repository root.
- Validate CI workflow execution on pull requests touching either contract.

### Dependencies
Depends on #6

### Code references
- `/workspaces/kora-app/Cargo.toml`: lines 1–6
- `stellar-contracts/contracts/pet-transfer-adoption/Cargo.toml`: lines 1–32
- `.github/workflows/stellar-contracts.yml`: lines 24–40

---

## #8: Implement systematic TTL extension strategy across all persistent and instance storage

### Category
Stellar/Soroban

### Priority
P0

### Problem
Across the entire `stellar-contracts/src/lib.rs` file (12,221 lines), `env.storage().instance().extend_ttl(...)` and `env.storage().persistent().extend_ttl(...)` are never called (0 occurrences in the entire crate). In Soroban, storage entries have a finite time-to-live (TTL). If entries are not explicitly extended, they become archived/expired on ledger.

### Current implementation
In `stellar-contracts/src/lib.rs`:
- Storage writes occur via `env.storage().instance().set(...)` (128 locations) and `env.storage().persistent().set(...)` (4 locations).
- No TTL extension functions (`extend_ttl`, `extend_ttl_if_needed`) exist anywhere in reading or writing paths.

### Why it matters
All on-chain state—including pet registrations, medical histories, admin multisig configurations, and access grants—will expire and become inaccessible after the default ledger threshold (typically 30–60 days on live testnet/mainnet). The contract will brick once instance storage expires, preventing further invocations.

### Proposed work
- Define standardized TTL constants: `INSTANCE_BUMP_AMOUNT`, `INSTANCE_LIFETIME_THRESHOLD`, `PERSISTENT_BUMP_AMOUNT`, `PERSISTENT_LIFETIME_THRESHOLD`.
- Implement internal helper `bump_instance_ttl(&Env)` called on every contract entrypoint.
- Implement `bump_persistent_ttl(&Env, &DataKey)` invoked whenever individual records are accessed or mutated.
- Provide an administrative maintenance method allowing external keepers to bump critical contract entries.

### Acceptance criteria
- Every state read and write extends the TTL of accessed instance and persistent keys.
- Contract instance storage maintains TTL above threshold across all operations.
- Dedicated unit tests verify TTL extension on pet creation, retrieval, and medical logging.

### Testing requirements
- Unit tests using Soroban SDK `testutils::Ledger` simulating ledger advancement and asserting TTL bump behavior.
- Test that read-only queries also extend TTL as required by Soroban best practices.

### Dependencies
Dependencies: None

### Code references
- `stellar-contracts/src/lib.rs`: lines 3690–3780 (init), 4640–4760 (register_pet), 11470–11520 (add_medical_record)

---

## #9: Migrate unbounded pet records and audit logs from instance storage to persistent storage

### Category
Architecture

### Priority
P0

### Problem
In `stellar-contracts/src/lib.rs`, `env.storage().instance()` is used for almost all data models: `Pet`, `MedicalRecord`, `DietPlan`, `AuditLog`, `ActivityKey`, `Dispute`, and `Vaccination`. There are 128 instances of `.instance().set()` versus only 4 uses of `.persistent().set()`.

### Current implementation
In `stellar-contracts/src/lib.rs`:
- Line 4757: `env.storage().instance().set(&DataKey::Pet(pet_id), &pet);`
- Line 11514: `env.storage().instance().set(&MedicalKey::MedicalRecord(record_id), &record);`
- Line 7411: `env.storage().instance().set(&NutritionKey::DietPlan(diet_id), &plan);`
- Instance storage is bounded by Soroban protocol limits (~128 KiB total across all instance entries).

### Why it matters
Soroban instance storage loads all instance keys into memory during every invocation. Storing unbounded entities (such as every registered pet, photo hash, medical record, and activity log) in instance storage guarantees that the contract will exceed Soroban's instance storage byte limit and fuel caps as adoption grows, permanently blocking any transaction on the contract.

### Proposed work
- Migrate `DataKey::Pet`, `MedicalKey::MedicalRecord`, `NutritionKey::DietPlan`, and other per-entity records to `env.storage().persistent()`.
- Retain global administrative counters (`PetCount`, `AdminList`, `ContractConfig`) in instance storage.
- Implement data migration logic or compatibility accessors for existing test environments.

### Acceptance criteria
- Entity records are saved into and retrieved from persistent storage.
- Instance storage footprint remains static regardless of the number of registered pets or medical records.
- Contract successfully creates hundreds of records without exceeding instance size limits.

### Testing requirements
- Benchmark test simulating 100+ pet registrations and medical records, measuring instance storage size.
- Regression tests ensuring all getters correctly retrieve entities from persistent storage.

### Dependencies
Depends on #8

### Code references
- `stellar-contracts/src/lib.rs`: lines 4757, 5721, 7411, 10740, 11514

---

## #10: Implement `decrement_pet_storage` to restore user quota upon record purging and photo removal

### Category
Data Integrity

### Priority
P1

### Problem
`stellar-contracts/src/lib.rs` tracks storage consumption per pet via `PetStorageUsage(pet_id)` and increments it with `Self::increment_pet_storage(&env, pet_id)` (lines 4760, 5633, 11490). However, there is no corresponding `decrement_pet_storage` function. When photos are removed (`remove_pet_photo`, line 5700) or records are purged (`purge_deleted_records`, line 10450), the storage counter is never decreased.

### Current implementation
In `stellar-contracts/src/lib.rs`:
- Lines 4760, 5633, 7745, 11490: Call `Self::increment_pet_storage(&env, pet_id);`.
- Lines 5700–5725 (`remove_pet_photo`): Removes photo hash from `pet.photo_hashes` but never adjusts `PetStorageUsage`.
- Lines 10450–10480 (`purge_deleted_records`): Deletes tombstones but never adjusts `PetStorageUsage`.

### Why it matters
Users face a permanent, ratchet-style storage quota lock. Once a pet reaches the maximum allowed storage quota, deleting old photos or soft-deleted records does not release capacity, preventing owners from ever uploading new records or photos.

### Proposed work
- Implement private helper `decrement_pet_storage(env: &Env, pet_id: u64)` that decrements `PetStorageUsage(pet_id)` using `saturating_sub(1)`.
- Invoke `decrement_pet_storage` within `remove_pet_photo` and `purge_deleted_records`.
- Add public query `get_pet_storage_usage(env: Env, pet_id: u64) -> u32`.

### Acceptance criteria
- Removing a photo decreases the pet's recorded storage usage by 1.
- Purging soft-deleted records decreases storage usage according to the count of purged records.
- Decrementing storage never underflows below zero.

### Testing requirements
- Test: Add photo -> verify storage increments -> remove photo -> verify storage decrements to previous value.
- Test: Underflow protection when decrementing storage at 0.

### Dependencies
Dependencies: None

### Code references
- `stellar-contracts/src/lib.rs`: lines 5633, 5700–5725, 10450–10480

---

## #11: Replace O(N*M) quadratic iteration in `compact_storage` with single-pass compaction

### Category
Performance

### Priority
P2

### Problem
In `stellar-contracts/src/lib.rs` (lines 10488–10514), `compact_storage` re-indexes pet medical records when tombstones are cleared. For every deleted index slot `i`, it executes a nested loop shifting all subsequent records `j` from `i+1` down to `count`, reading and rewriting storage keys one-by-one.

### Current implementation
In `stellar-contracts/src/lib.rs` (lines 10488–10514):
```rust
for i in 1..=count {
    if is_deleted(i) {
        for j in (i + 1)..=count {
            let rec = env.storage().instance().get(&MedicalKey::MedicalRecord(j));
            env.storage().instance().set(&MedicalKey::MedicalRecord(j - 1), &rec);
        }
        count -= 1;
    }
}
```

### Why it matters
This nested shifting logic requires $O(N \times M)$ storage read and write operations. On Soroban, where CPU cycles and ledger write footprints directly consume transaction budget limits, compacting a list with even 20 deleted records exceeds the maximum execution budget, aborting the transaction and trapping deleted records permanently.

### Proposed work
- Implement a two-pointer single-pass compaction algorithm ($O(N)$).
- Maintain a `write_index` pointer starting at 1. Scan through `1..=count`; whenever an active (non-deleted) record is encountered, copy it to `write_index` (if `write_index != read_index`) and increment `write_index`.
- Delete remaining stale keys from `write_index..=old_count` and update the total record count.

### Acceptance criteria
- Compaction executes in a single linear pass over the records.
- Medical record IDs remain contiguous without gaps.
- Budget consumption scales linearly ($O(N)$) with record count.

### Testing requirements
- Benchmark test measuring CPU instructions during compaction of 50 records with interspersed deletions.
- Correctness test ensuring record integrity and associations match pre-compaction data.

### Dependencies
Depends on #9

### Code references
- `stellar-contracts/src/lib.rs`: lines 10488–10514

---

## #12: Restrict public unauthenticated medical record queries to pet owners and authorized veterinarians

### Category
Access Control

### Priority
P0

### Problem
Multiple medical record retrieval functions in `stellar-contracts/src/lib.rs` are completely unauthenticated and take no caller address:
- Line 2802: `get_medical_record(env: Env, record_id: u64) -> Option<MedicalRecord>`
- Line 8280: `search_medical_records(env: Env, pet_id: u64, ...)`
- Line 11568: `get_pet_medical_records(env: Env, pet_id: u64, ...)`
- Line 11895: `search_by_keyword(env: Env, pet_id: u64, ...)`
None of these functions verify whether the caller is the pet owner, an authorized veterinarian, or a granted emergency contact.

### Current implementation
In `stellar-contracts/src/lib.rs`:
```rust
pub fn get_medical_record(env: Env, record_id: u64) -> Option<MedicalRecord> {
    env.storage().instance().get(&MedicalKey::MedicalRecord(record_id))
}
```
No `caller: Address` argument is accepted, and no privacy checks against `pet.privacy_level` are performed.

### Why it matters
Anyone on the network can query sensitive medical diagnoses, treatments, medications, and clinical notes for any pet, regardless of whether the pet is marked `Restricted` or `Private`. This completely violates the privacy architecture described in `docs/architecture.md`.

### Proposed work
- Update signatures to accept `caller: Address` and call `caller.require_auth()`.
- Check if caller is pet owner, active verified vet with an active treatment relationship, or holding a valid temporary access grant.
- If pet privacy is `Public`, permit open reading; if `Restricted` or `Private`, enforce strict authorization.

### Acceptance criteria
- Non-owners and unverified callers are rejected with `ContractError::Unauthorized` when accessing restricted/private pet records.
- Pet owners and authorized vets retain full read access.
- Public pets remain readable according to privacy policies.

### Testing requirements
- Unit test: Unauthorized caller attempting `get_medical_record` on private pet reverts.
- Unit test: Owner successfully reads medical records.
- Unit test: Verified vet with active treatment relationship successfully reads records.

### Dependencies
Dependencies: None

### Code references
- `stellar-contracts/src/lib.rs`: lines 2802, 8280, 11568, 11895
- `docs/architecture.md`: Privacy & Access Control section

---

## #13: Enforce caller authentication on `add_activity_record`

### Category
Access Control

### Priority
P0

### Problem
In `stellar-contracts/src/lib.rs` (line 10742), `add_activity_record` accepts a `pet_id`, activity parameters, and an idempotency key, but never authenticates the caller. There is no `caller: Address` parameter or `require_auth()` invocation.

### Current implementation
In `stellar-contracts/src/lib.rs` (lines 10742–10760):
```rust
pub fn add_activity_record(
    env: Env,
    pet_id: u64,
    activity_type: ActivityType,
    duration_minutes: u32,
    distance_meters: u32,
    calories_burned: u32,
    timestamp: u64,
    notes: String,
    idempotency_key: Option<BytesN<32>>,
) -> u64 {
    // Fetches pet, performs idempotency check, but NO require_auth()
```

### Why it matters
Any arbitrary actor can inject fraudulent activity records into any pet's history. This pollutes health tracking analytics, triggers spurious insurance fitness discounts, and exhausts the pet's storage quota without the owner's knowledge or consent.

### Proposed work
- Add `caller: Address` parameter to `add_activity_record`.
- Call `caller.require_auth()`.
- Validate that `caller == pet.owner` or that `caller` holds a delegated caretaker grant.

### Acceptance criteria
- Calls to `add_activity_record` without valid signature revert.
- Callers who are neither the owner nor an authorized delegate are rejected with `ContractError::Unauthorized`.

### Testing requirements
- Test: Owner successfully adds activity record.
- Test: Random caller fails with authorization error.
- Test: Authorized caretaker adds activity record successfully.

### Dependencies
Dependencies: None

### Code references
- `stellar-contracts/src/lib.rs`: lines 10742–10760
- `stellar-contracts/src/test_activity.rs`

---

## #14: Enforce caller authorization and breed registry permissions on `set_pet_traits`

### Category
Access Control

### Priority
P1

### Problem
In `stellar-contracts/src/lib.rs` (lines 11116–11140), `set_pet_traits` allows updating physical and genetic traits (`coat_color`, `eye_color`, `size_category`, `genetic_markers`) for a pet. However, the function fails to call `require_auth()`, allowing anyone to overwrite pet genetics.

### Current implementation
In `stellar-contracts/src/lib.rs`:
```rust
pub fn set_pet_traits(
    env: Env,
    pet_id: u64,
    traits: PhysicalTraits,
) -> bool {
    let _pet: Pet = env
        .storage()
        .instance()
        .get(&DataKey::Pet(pet_id))
        .unwrap_or_else(|| env.panic_with_error(ContractError::PetNotFound));
    // No auth verification
    env.storage().instance().set(&BreedingKey::PetTraits(pet_id), &traits);
    true
}
```

### Why it matters
Genetic marker data is vital for breed registries, pedigree verification, and hereditary disease tracking. Lack of authorization allows malicious actors to manipulate genetic profiles, corrupting breeding integrity and inbreeding coefficient (COI) calculations across the entire registry.

### Proposed work
- Add `caller: Address` to `set_pet_traits` and call `caller.require_auth()`.
- Require `caller` to be either the pet owner or an authorized breed registry authority.
- Emit `PetTraitsUpdatedEvent` upon mutation.

### Acceptance criteria
- Only authorized callers can update physical traits.
- Unauthorized callers revert with `ContractError::Unauthorized`.
- Trait updates emit an informative audit event.

### Testing requirements
- Test: Owner successfully updates pet traits.
- Test: Non-owner fails with unauthorized error.
- Test: Verify traits persist accurately in storage.

### Dependencies
Dependencies: None

### Code references
- `stellar-contracts/src/lib.rs`: lines 11116–11140
- `stellar-contracts/src/test_breeding_genetics.rs`

---

## #15: Validate sire and dam existence, gender compatibility, and caller identity in `add_breeding_record`

### Category
Smart Contract

### Priority
P1

### Problem
In `stellar-contracts/src/lib.rs` (lines 10977–11030), `add_breeding_record` creates a breeding record linking a `sire_id` and `dam_id`. The function contains severe logic defects:
1. It does not authenticate the caller (`require_auth()` is missing).
2. It hardcodes `breeder: env.current_contract_address()` instead of the actual breeder.
3. It does not verify whether `sire_id` or `dam_id` actually exist in storage.
4. It does not verify that sire and dam are of the same species.
5. It does not verify that sire is male and dam is female.

### Current implementation
In `stellar-contracts/src/lib.rs` (lines 10977–11030):
```rust
pub fn add_breeding_record(
    env: Env,
    sire_id: u64,
    dam_id: u64,
    mating_date: u64,
    expected_due_date: u64,
    notes: String,
) -> u64 {
    // No auth check
    // No pet existence check
    // Breeder hardcoded to contract address:
    let record = BreedingRecord {
        ...
        breeder: env.current_contract_address(),
        ...
    };
```

### Why it matters
Anyone can construct bogus breeding records linking non-existent pets, cross-breeding incompatible species (e.g. dogs with cats), or mating two male pets. Hardcoding `breeder` to the contract address breaks ownership tracking and lineage audits.

### Proposed work
- Add `breeder: Address` argument with `breeder.require_auth()`.
- Load both `sire` and `dam` from storage, verifying existence.
- Assert `sire.species == dam.species`.
- Assert `sire.gender == Gender::Male` and `dam.gender == Gender::Female`.
- Verify that `breeder` owns either the sire or dam.

### Acceptance criteria
- Non-existent sire or dam returns `ContractError::PetNotFound`.
- Mismatched species or invalid genders revert with `ContractError::InvalidInput`.
- Breeder must be authenticated and authorized.
- Breeding record correctly stores the authentic breeder address.

### Testing requirements
- Test: Male sire + female dam of same species succeeds.
- Test: Same-sex breeding attempt reverts.
- Test: Cross-species breeding attempt reverts.
- Test: Non-existent parent IDs revert.

### Dependencies
Dependencies: None

### Code references
- `stellar-contracts/src/lib.rs`: lines 10977–11030
- `stellar-contracts/src/test_breeding.rs`

---

## #16: Fix COI (Coefficient of Inbreeding) calculation to account for parent-child inbreeding

### Category
Data Integrity

### Priority
P2

### Problem
In `stellar-contracts/src/lib.rs` (lines 11210–11260), `calculate_coi` calculates Wright's Coefficient of Inbreeding using `build_pedigree_map`. However, `build_pedigree_map` only inserts the ancestors of the root pet into the pedigree map and omits the root pet itself.
When evaluating a parent-offspring mating (e.g., father bred back to daughter), the father is an ancestor of the daughter, but because the father is the root of one branch and not included in his own ancestor map, the algorithm identifies no common ancestors and returns 0 basis points instead of the true 2500 bps (25%).

### Current implementation
In `stellar-contracts/src/lib.rs` (lines 11221–11245):
```rust
fn build_pedigree_map(env: &Env, root: u64, max_depth: u32) -> Map<u64, u32> {
    let mut map = Map::new(env);
    // Recursively adds parents of root at depth 1, but NEVER inserts (root, 0)
    ...
}
```

### Why it matters
Breeders relying on the on-chain COI calculator will receive completely false zero-inbreeding scores for direct incestuous parent-child matings, defeating the fundamental purpose of pedigree verification and genetic health management.

### Proposed work
- Modify `build_pedigree_map` (or the common ancestor intersection logic) to include `(root, 0)` in the kinship traversal.
- Ensure depth calculation handles root-to-ancestor paths accurately ($F = \sum (1/2)^{n_1 + n_2 + 1}$).
- Verify parent-offspring matings return exactly 2500 bps (25%).

### Acceptance criteria
- Parent-child mating yields COI of 2500 basis points.
- Full sibling mating yields COI of 2500 basis points.
- Unrelated animals yield COI of 0 basis points.

### Testing requirements
- Unit tests in `test_breeding_coi.rs` testing parent-child, sibling-sibling, and unrelated pairings.

### Dependencies
Depends on #15

### Code references
- `stellar-contracts/src/lib.rs`: lines 11210–11260
- `stellar-contracts/src/test_breeding_coi.rs`: lines 1–120

---

## #17: Make `ActivityKey::IdempotencyWindow` configurable and implement `purge_expired_idempotency_keys`

### Category
Smart Contract

### Priority
P2

### Problem
In `stellar-contracts/src/lib.rs` (lines 10760–10780), `add_activity_record` attempts to enforce idempotency via `idempotency_key`. It reads `ActivityKey::IdempotencyWindow` into variable `_window`, but immediately ignores it and hardcodes 86,400 seconds (24 hours).
Furthermore, `purge_expired_idempotency_keys` (line 10830) is an empty stub that returns 0 without deleting expired keys from storage.

### Current implementation
In `stellar-contracts/src/lib.rs`:
- Line 10778: `let _window: u64 = env.storage().instance().get(&ActivityKey::IdempotencyWindow).unwrap_or(86400);`
- Line 10785: Uses hardcoded constant `86400` in expiration comparison instead of `_window`.
- Lines 10830–10840: `purge_expired_idempotency_keys` contains `// TODO` and returns `0u32`.

### Why it matters
Idempotency configuration cannot be adjusted by administrators. Meanwhile, idempotency keys accumulate indefinitely in instance storage without cleanup, steadily inflating ledger footprint and storage costs.

### Proposed work
- Use the configured `_window` value rather than the hardcoded 86,400 seconds.
- Provide admin setter `set_idempotency_window(env: Env, admin: Address, seconds: u64)`.
- Implement `purge_expired_idempotency_keys` to iterate or track registered idempotency keys and purge expired records.

### Acceptance criteria
- Dynamic idempotency window is respected during duplicate submission checks.
- Purge function removes expired idempotency entries and returns the count of purged items.

### Testing requirements
- Unit test: Configure window to 300s; verify submission at 301s is accepted as a new record.
- Unit test: Purge function correctly deletes expired records.

### Dependencies
Dependencies: None

### Code references
- `stellar-contracts/src/lib.rs`: lines 10760–10785, 10830–10840
- `stellar-contracts/src/test_activity_idempotency.rs`

---

## #18: Enforce `caller.require_auth()` in `get_emergency_info_with_reason` and `notify_emergency_contacts` to prevent audit forgery

### Category
Security

### Priority
P0

### Problem
In `stellar-contracts/src/lib.rs`, emergency access functions accept a `caller: Address` parameter but never call `caller.require_auth()`:
- Line 9143: `get_emergency_info_with_reason(env: Env, pet_id: u64, caller: Address, reason: String)`
- Line 9334: `notify_emergency_contacts(env: Env, pet_id: u64, caller: Address, notes: String)`

### Current implementation
In `stellar-contracts/src/lib.rs`:
```rust
pub fn get_emergency_info_with_reason(
    env: Env,
    pet_id: u64,
    caller: Address,
    reason: String,
) -> Option<EmergencyInfo> {
    // Caller address is logged in audit trail, but caller.require_auth() is NEVER called!
    Self::log_emergency_access(&env, pet_id, &caller, &reason);
```

### Why it matters
An attacker can pass any legitimate veterinarian's or clinic's address as `caller`. The contract will execute the emergency override, disclose private emergency and medical data, and write a forged entry into the immutable audit trail blaming the innocent veterinarian.

### Proposed work
- Add `caller.require_auth()` at the start of `get_emergency_info_with_reason` and `notify_emergency_contacts`.
- Verify caller is authorized (licensed vet, owner, or designated emergency contact).
- Ensure audit logging records the authenticated caller.

### Acceptance criteria
- Invoking emergency access functions without cryptographic authorization for `caller` reverts.
- Audit trail accurately records only authenticated callers.

### Testing requirements
- Test: Calling `get_emergency_info_with_reason` with another address without signature reverts.
- Test: Authenticated emergency access logs accurate audit event.

### Dependencies
Dependencies: None

### Code references
- `stellar-contracts/src/lib.rs`: lines 9143–9220, 9334–9360
- `stellar-contracts/src/test_emergency_override.rs`

---

## #19: Replace static public-key pseudo-encryption in `get_encryption_key` with secure user-derived key management

### Category
Security

### Priority
P0

### Problem
In `stellar-contracts/src/lib.rs` (lines 8594–8630), `get_encryption_key` derives the symmetric encryption key used for sensitive pet data (`name`, `breed`, `birthday`, `allergies`, `medical_alerts`) entirely from public on-chain values:
```rust
let seed = ("kora:encryption-key:v1", env.current_contract_address(), admin);
```
Furthermore, it retrieves `admin` as `admins.get(0)`.

### Current implementation
In `stellar-contracts/src/lib.rs` (lines 8594–8630):
- The key is derived by hashing public ledger strings, the contract address, and the first admin address.
- Any party observing the public blockchain can compute the exact same key off-chain and decrypt all "encrypted" pet data.
- If admin 0 is removed or rotated, the derived key changes, permanently corrupting and bricking all previously encrypted pet data across the contract.

### Why it matters
1. "Encryption" is completely illusory: zero confidentiality is provided against network observers.
2. Routine administrative rotation causes catastrophic data loss, rendering all existing pet names, breeds, and medical alerts permanently unreadable.

### Proposed work
- Redesign sensitive data handling to store ciphertext encrypted off-chain using the pet owner's public key (or a per-pet data encryption key managed via Diffie-Hellman / envelope encryption).
- Decouple pet decryption keys from contract administrator addresses.
- If on-chain symmetric derivation is retained for obfuscation, store a static dedicated encryption salt that survives admin rotations.

### Acceptance criteria
- Admin key rotation does not alter the key used to decrypt existing pet records.
- Off-chain encryption architecture specification documented in `docs/architecture.md`.
- Regression test confirms pet profile remains decryptable after adding or removing contract admins.

### Testing requirements
- Test: Register pet -> rotate primary admin -> retrieve and decrypt pet profile -> verify name and breed match original input.

### Dependencies
Dependencies: None

### Code references
- `stellar-contracts/src/lib.rs`: lines 8594–8630
- `stellar-contracts/src/test_get_pet_decryption.rs`
- `docs/architecture.md`

---

## #20: Enforce upgrade proposal timelock delay and multi-admin quorum in `execute_upgrade`

### Category
Upgradeability

### Priority
P0

### Problem
In `stellar-contracts/src/lib.rs` (lines 11643–11713), contract upgrade governance contains critical vulnerabilities:
1. `execute_upgrade` reads `proposal.timelock_duration` and `proposal.approved_at`, but never asserts that `env.ledger().timestamp() >= proposal.approved_at + proposal.timelock_duration`. Upgrades can be executed immediately upon creation.
2. A single admin can approve and execute an upgrade proposal even when multisig thresholds require multiple admins.

### Current implementation
In `stellar-contracts/src/lib.rs`:
```rust
pub fn execute_upgrade(env: Env, admin: Address, proposal_id: u64) -> bool {
    admin.require_auth();
    // Checks proposal status is Approved, but NEVER checks timelock timestamp!
    env.deployer().update_current_contract_wasm(proposal.new_wasm_hash);
```

### Why it matters
Timelocks are designed to give users and token holders time to exit or react before unvetted code is deployed. Bypassing the timelock and multisig checks allows a single compromised admin key to instantly overwrite the entire contract bytecode with arbitrary malicious logic.

### Proposed work
- Verify `env.ledger().timestamp() >= proposal.approved_at.checked_add(proposal.timelock_duration)`.
- Verify that the proposal approval count meets the active multisig governance threshold.
- Revert with `ContractError::TimelockNotExpired` if executed prematurely.

### Acceptance criteria
- Attempting to execute an upgrade before `approved_at + timelock_duration` reverts.
- Attempting to execute an upgrade without meeting the required admin threshold reverts.
- Execution succeeds once the timelock duration has elapsed.

### Testing requirements
- Test: Create upgrade proposal -> approve -> execute immediately (fails).
- Test: Advance ledger timestamp past timelock duration -> execute (succeeds).

### Dependencies
Dependencies: None

### Code references
- `stellar-contracts/src/lib.rs`: lines 11643–11713
- `stellar-contracts/src/test_upgrade_proposal.rs`

---

## #21: Correct `PreviousWasmHash` storage in `execute_upgrade` to prevent corrupted upgrade rollbacks

### Category
Upgradeability

### Priority
P1

### Problem
In `stellar-contracts/src/lib.rs` (line 11703), `execute_upgrade` attempts to preserve the previous WASM hash to enable emergency rollback via `rollback_upgrade` (line 11867).
However, line 11703 records the *new* WASM hash as `PreviousWasmHash`:
```rust
env.storage().instance().set(&DataKey::PreviousWasmHash, &proposal.new_wasm_hash);
```

### Current implementation
In `stellar-contracts/src/lib.rs`:
- Line 11703: Stores `proposal.new_wasm_hash` into `DataKey::PreviousWasmHash`.
- Line 11867 (`rollback_upgrade`): Reads `PreviousWasmHash` and updates contract to that hash.

### Why it matters
When an emergency occurs and admins invoke `rollback_upgrade`, the contract redeploys the broken/malicious `new_wasm_hash` instead of the original working code! The rollback mechanism is completely corrupted.

### Proposed work
- Before calling `update_current_contract_wasm`, query the current active WASM executable hash.
- Store the true current hash into `DataKey::PreviousWasmHash`.
- Alternatively, record `proposal.current_wasm_hash` captured at proposal submission time.

### Acceptance criteria
- `PreviousWasmHash` stores the bytecode hash active immediately prior to the upgrade.
- Calling `rollback_upgrade` successfully restores the pre-upgrade bytecode hash.

### Testing requirements
- Test: Upgrade contract from WASM A to WASM B -> verify `PreviousWasmHash == WASM A`.
- Test: Invoke `rollback_upgrade` -> verify active contract WASM reverts to WASM A.

### Dependencies
Depends on #20

### Code references
- `stellar-contracts/src/lib.rs`: lines 11703, 11867–11880
- `stellar-contracts/src/test_upgrade_proposal.rs`

---

## #22: Prevent single-admin denial of service on multisig threshold proposals

### Category
Security

### Priority
P1

### Problem
In `stellar-contracts/src/lib.rs` (lines 4011–4016), `set_threshold` allows an admin to propose or vote on a new signature threshold. However, if a different admin submits a distinct threshold value, the contract completely wipes out all existing approvals:
```rust
if pending.threshold != threshold {
    pending.threshold = threshold;
    pending.approvals = Vec::new(&env);
}
```

### Current implementation
In `stellar-contracts/src/lib.rs` (lines 4011–4016):
- Any single admin can observe pending approvals for a legitimate threshold proposal and submit a competing threshold, immediately wiping `pending.approvals`.

### Why it matters
A single rogue or compromised admin can repeatedly alternate threshold proposals, perpetually resetting pending approvals and staging a denial-of-service attack against multisig governance.

### Proposed work
- Transition threshold updates to structured proposal IDs (e.g. `ThresholdProposalKey(u64)`).
- Prevent competing proposals from modifying other proposals' vote accumulators.
- Allow proposals to expire cleanly without resetting concurrent proposals.

### Acceptance criteria
- Admin A's threshold proposal retains its approvals independently of Admin B's proposal.
- A single admin cannot erase other admins' recorded votes.

### Testing requirements
- Test: Admin A proposes threshold 3, Admin B proposes threshold 2; verify Admin A's proposal retains approvals.

### Dependencies
Dependencies: None

### Code references
- `stellar-contracts/src/lib.rs`: lines 4011–4025
- `stellar-contracts/src/test_admin_threshold_quorum.rs`

---

## #23: Require multisig proposal threshold to modify `set_quorum_percent` and prevent zero-quorum bypass

### Category
Access Control

### Priority
P1

### Problem
In `stellar-contracts/src/lib.rs` (lines 4057–4075), `set_quorum_percent` allows any single admin to instantly update the contract's quorum percentage without proposal or multisig consensus:
```rust
pub fn set_quorum_percent(env: Env, admin: Address, quorum_percent: u32) -> bool {
    admin.require_auth();
    Self::require_admin(&env, &admin);
    // Directly writes to storage!
    env.storage().instance().set(&DataKey::QuorumPercent, &quorum_percent);
```
Furthermore, it allows setting `quorum_percent = 0`.

### Current implementation
In `stellar-contracts/src/lib.rs`:
- Any admin can unilaterally execute `set_quorum_percent`.
- Setting `quorum_percent = 0` allows subsequent multisig actions to pass with zero votes.

### Why it matters
This completely undermines multisig security. A rogue admin can bypass threshold voting by setting quorum to 0% and immediately executing arbitrary actions.

### Proposed work
- Require `set_quorum_percent` to follow the standard multisig proposal and approval flow.
- Enforce boundary validation: `require(quorum_percent >= 51 && quorum_percent <= 100)`.

### Acceptance criteria
- Quorum percent cannot be set below 51% or above 100%.
- Updating quorum percentage requires threshold multisig approval.

### Testing requirements
- Test: Single admin attempting to set quorum fails without threshold consensus.
- Test: Setting quorum to 0% or 40% reverts with `ContractError::InvalidInput`.

### Dependencies
Depends on #22

### Code references
- `stellar-contracts/src/lib.rs`: lines 4057–4075
- `stellar-contracts/src/test_admin_threshold_quorum.rs`

---

## #24: Deprecate unauthenticated `propose_init` and resolve competing contract initialization entrypoints

### Category
Security

### Priority
P1

### Problem
`stellar-contracts/src/lib.rs` has three competing initialization pathways:
1. `init_admin` (line 3694): Standard single-admin initializer.
2. `init_multisig` (line 3712): Standard multisig initializer.
3. `propose_init` (line 3744): Alternative initializer taking no authentication, vulnerable to front-running.

### Current implementation
In `stellar-contracts/src/lib.rs`:
- Lines 3744–3765: `propose_init(env: Env, admin: Address, threshold: u32)` can be called by anyone before contract deployment setup completes.
- Inconsistent flags: `init_admin` checks `DataKey::Initialized`, while `propose_init` checks `DataKey::InitProposal`.

### Why it matters
An attacker monitoring the mempool during contract deployment can front-run the deployer, call `propose_init`, and register themselves as admin or lock the initialization state.

### Proposed work
- Consolidate initialization into a single atomic entrypoint (`initialize`) that sets `DataKey::Initialized`.
- Require the deployer/admin to sign the initialization transaction (`admin.require_auth()`).
- Deprecate and remove `propose_init`.

### Acceptance criteria
- Contract cannot be initialized more than once.
- Unauthenticated initialization attempts revert.
- Exactly one canonical initialization path exists.

### Testing requirements
- Test: Deploy contract and initialize with authenticated admin.
- Test: Re-initialization reverts with `ContractError::AlreadyInitialized`.
- Test: Front-running attempt without authorization reverts.

### Dependencies
Dependencies: None

### Code references
- `stellar-contracts/src/lib.rs`: lines 3694–3780
- `stellar-contracts/src/test_admin_initialization.rs`

---

## #25: Fix `new_owner` initialization in `register_pet` and implement pet transfer cancellation in `KoraContract`

### Category
Smart Contract

### Priority
P1

### Problem
In `stellar-contracts/src/lib.rs`:
1. In `register_pet` (line 4748), `pet.new_owner` is initialized to `owner.clone()` instead of an empty/sentinel state:
   ```rust
   new_owner: owner.clone(),
   ```
2. In `accept_pet_transfer` (line 5954), the check is:
   ```rust
   if pet.new_owner != new_owner { panic!(...); }
   ```
   Because `new_owner` defaults to `owner`, calling `accept_pet_transfer` when no transfer is pending succeeds and logs a transfer from the owner to themselves.
3. `KoraContract` has no `cancel_pet_transfer` method to allow an owner to revoke an unaccepted transfer offer.

### Current implementation
In `stellar-contracts/src/lib.rs`:
- Line 4748: `new_owner: owner.clone()`
- Lines 5950–5970: `accept_pet_transfer` lacks pending transfer status check.
- Absence of `cancel_pet_transfer`.

### Why it matters
Spurious ownership transfer events can be emitted at any time. Furthermore, if an owner mistakenly initiates a transfer to the wrong address, they cannot cancel or reclaim the offer before acceptance.

### Proposed work
- Make `new_owner` an `Option<Address>` in `Pet` struct (or use a dedicated `PendingPetTransfer` storage key).
- In `register_pet`, initialize pending transfer to `None`.
- In `accept_pet_transfer`, require `pet.new_owner == Some(new_owner)` and reset it to `None` upon acceptance.
- Implement `cancel_pet_transfer(env: Env, pet_id: u64)` requiring `pet.owner.require_auth()`.

### Acceptance criteria
- `accept_pet_transfer` fails if no transfer was initiated.
- Owner can cancel an initiated transfer prior to recipient acceptance.
- `PetTransferCancelled` event is emitted upon cancellation.

### Testing requirements
- Test: Calling `accept_pet_transfer` on newly registered pet reverts.
- Test: Initiate transfer -> cancel transfer -> recipient acceptance reverts.
- Test: Initiate transfer -> accept transfer -> ownership successfully updates.

### Dependencies
Dependencies: None

### Code references
- `stellar-contracts/src/lib.rs`: lines 4748, 5928–5970
- `stellar-contracts/src/test_multisig_transfer.rs`

---

## #26: Fix caller resolution in `get_pet_age`, `get_pet_age_with_lifespan`, and `get_pet_by_tag` for non-public pets

### Category
Smart Contract

### Priority
P1

### Problem
In `stellar-contracts/src/lib.rs`, several utility functions query pet details by internally calling `KoraContract::get_pet(env.clone(), caller, pet_id)`. However, instead of passing the transaction caller or a designated user address, they hardcode `env.current_contract_address()` as the caller:
- Line 5092: `get_pet_age`
- Line 8142: `get_pet_age_with_lifespan`
- Line 10348: `get_pet_by_tag`
When `get_pet` runs, it checks whether the pet's privacy level allows access. For `Restricted` or `Private` pets, the contract address itself is neither the owner nor an authorized veterinarian. Consequently, `get_pet` returns an error or empty option, causing `get_pet_age` to return `(0, 0)` and `get_pet_by_tag` to fail.

### Current implementation
In `stellar-contracts/src/lib.rs`:
```rust
pub fn get_pet_age(env: Env, pet_id: u64) -> (u32, u32) {
    let contract_addr = env.current_contract_address();
    match KoraContract::get_pet(env.clone(), contract_addr, pet_id) {
        Ok(pet) => { ... },
        Err(_) => (0, 0), // Silently masks failure!
    }
}
```

### Why it matters
Owners of restricted or private pets cannot retrieve valid age calculations or lifespan statistics. More severely, lost pets wearing tag IDs cannot be identified via `get_pet_by_tag` if the owner marked the pet restricted or private, defeating the tag identification system.

### Proposed work
- Accept `caller: Option<Address>` in `get_pet_age`, `get_pet_age_with_lifespan`, and `get_pet_by_tag`.
- For `get_pet_age`, read the non-encrypted birthday timestamp directly from internal storage when called within authorized contexts, or verify caller access.
- For `get_pet_by_tag`, expose designated emergency contact or identifier fields intended for tag scanning without exposing full private medical histories.

### Acceptance criteria
- Owners of private pets can successfully query their pet's age and lifespan.
- Tag lookups successfully return designated recovery information regardless of pet privacy settings.
- Errors are not silently masked with dummy `(0, 0)` tuples.

### Testing requirements
- Unit test: Query `get_pet_age` for Private pet with owner authentication (returns true age).
- Unit test: Query `get_pet_by_tag` for Restricted pet and verify recovery contact resolution.

### Dependencies
Dependencies: None

### Code references
- `stellar-contracts/src/lib.rs`: lines 5092–5115, 8142–8170, 10348–10375
- `stellar-contracts/src/test_pet_age.rs`

---

## #27: Synchronize error codes between `ContractError`, `KoraError`, and runtime error registry

### Category
Data Integrity

### Priority
P1

### Problem
Across `stellar-contracts/src/lib.rs` and `docs/error-codes.md`, error code values and discriminants are severely desynchronized:
1. In `stellar-contracts/src/lib.rs`, `initialize_error_messages` (lines 4325–4365) maps code 1 to "Unauthorized access", code 2 to "Admin not initialized", and code 3 to "Pet not found".
2. However, the `ContractError` enum (lines 866–920) was reorganized: discriminant 1 is `AdminAlreadyApproved`, discriminant 2 is `AdminAlreadySet`, and discriminant 3 is `ThresholdTooHigh`.
3. In addition, an older `KoraError` enum (lines 1000–1035) defines discriminants 1..5 (`AlreadyInitialized = 1`, `NotInitialized = 2`, etc.), colliding with `ContractError`.
4. `docs/error-codes.md` documents the legacy mapping from `initialize_error_messages`.

### Current implementation
- `stellar-contracts/src/lib.rs`:
  - Lines 866–920: `ContractError` enum variants with implicit discriminants.
  - Lines 1000–1035: `KoraError` enum variants with explicit colliding discriminants.
  - Lines 4325–4365: `initialize_error_messages` registering mismatched error messages.
- `docs/error-codes.md`: Completely outdated relative to on-chain errors.

### Why it matters
When a transaction fails on-chain, indexers, client SDKs, and frontend apps query the error registry and display completely false diagnostic messages (e.g. an "AdminAlreadyApproved" error is reported to users as "Unauthorized access").

### Proposed work
- Remove redundant `KoraError` enum and standardize on a single canonical `ContractError`.
- Assign explicit discriminant values (`#[repr(u32)]`) to each `ContractError` variant.
- Update `initialize_error_messages` to map exactly to the canonical `ContractError` discriminant values.
- Synchronize `docs/error-codes.md` to reflect the updated mapping.

### Acceptance criteria
- Every variant of `ContractError` has an explicit, unique `u32` discriminant.
- `initialize_error_messages` accurately reflects all `ContractError` variants.
- Automated test verifies that `ContractError as u32` matches the stored string in `ErrorMessage(code)`.

### Testing requirements
- Test in `test_error_registry.rs` iterating all `ContractError` variants and asserting matching registered error text.

### Dependencies
Dependencies: None

### Code references
- `stellar-contracts/src/lib.rs`: lines 866–920, 1000–1035, 4325–4365
- `stellar-contracts/src/test_error_registry.rs`
- `docs/error-codes.md`

---

## #28: Replace full linear scan in `matching_subscription_ids` with indexed subscription mapping

### Category
Performance

### Priority
P2

### Problem
In `stellar-contracts/src/lib.rs` (lines 2586–2620), `matching_subscription_ids` executes an unbounded linear scan over all registered event subscriptions:
```rust
for id in 1..=total_subs {
    let sub = env.storage().instance().get(&EventKey::Subscription(id));
    if matches(&sub) { ... }
}
```
This function is invoked synchronously inside `add_medical_record`, `register_pet`, and `update_pet_profile`.

### Current implementation
In `stellar-contracts/src/lib.rs`:
- Every state mutation triggers `emit_subscription_event`, which calls `matching_subscription_ids`.
- Reads `1..=total_subs` instance storage keys sequentially.

### Why it matters
As the number of webhook/event subscriptions grows (e.g. 50+ subscribers), the gas/CPU budget required to log a simple medical record or register a pet scales linearly until transactions hit the Soroban instruction limit and fail. This introduces a critical denial-of-service vulnerability.

### Proposed work
- Replace the linear scan with an indexed mapping by event type (e.g., `EventKey::TypeSubscribers(EventType) -> Vec<u64>`).
- When a subscription is registered, append its ID only to the matching event-type index.
- In `matching_subscription_ids`, fetch only the subscriber list for the specific event type being emitted.

### Acceptance criteria
- Event emission performs $O(1)$ index lookups instead of $O(N)$ full storage scans.
- Gas consumption of `add_medical_record` remains constant regardless of total subscriptions to unrelated event types.

### Testing requirements
- Benchmark test: Register 100 subscriptions and verify instruction usage for `add_medical_record` remains within threshold.
- Correctness test: Ensure subscribers receive only events matching their filter criteria.

### Dependencies
Dependencies: None

### Code references
- `stellar-contracts/src/lib.rs`: lines 2586–2620, 11495–11520
- `stellar-contracts/src/test_event_subscriptions.rs`

---

## #29: Fix pagination offset boundary check in `get_lab_results`

### Category
Smart Contract

### Priority
P2

### Problem
In `stellar-contracts/src/lib.rs` (lines 6908–6935), `get_lab_results` implements paginated retrieval of laboratory records. However, the loop exit condition uses:
```rust
if included_count >= limit {
    break;
}
```
where `included_count` is incremented on *every* matching item, including those skipped due to `included_count < offset`.

### Current implementation
In `stellar-contracts/src/lib.rs` (lines 6908–6935):
- If `offset >= limit`, the loop terminates prematurely during the offset-skipping phase before collecting any items.
- As a result, querying page 2 (`offset = 10, limit = 10`) immediately exits and returns an empty list, even when dozens of lab results exist.

### Why it matters
Pagination is broken for all queries where `offset >= limit`. Applications and veterinarians cannot view historical lab results past the first page.

### Proposed work
- Separate the skipped items counter from the collected results counter:
  - Increment `skipped_count` until `skipped_count == offset`.
  - Collect results into `result: Vec<LabResult>` until `result.len() == limit`.
- Break the loop only when `result.len() == limit` or all records are exhausted.

### Acceptance criteria
- Pagination returns correct subsets across multiple pages.
- Querying with `offset = 10, limit = 10` returns items 11 through 20 when at least 20 records exist.

### Testing requirements
- Unit test in `test_get_lab_results.rs` populating 25 lab results and verifying sequential pagination across pages 1, 2, and 3.

### Dependencies
Dependencies: None

### Code references
- `stellar-contracts/src/lib.rs`: lines 6908–6935
- `stellar-contracts/src/test_get_lab_results.rs`

---

## #30: Re-link and fix 49 orphaned test modules in `stellar-contracts/src/` into the crate test harness

### Category
Testing

### Priority
P1

### Problem
In `stellar-contracts/src/`, there are 71 individual `test_*.rs` files. However, `stellar-contracts/src/lib.rs` declares only 18 of them as `mod ...;` (lines 152–198).
The remaining 49 test files (such as `test_access_control.rs`, `test_activity.rs`, `test_insurance.rs`, `test_proptest_medical.rs`, `test_storage_quota.rs`) are completely orphaned. They are never compiled, never checked for syntax or type errors, and never run by `cargo test`.

### Current implementation
In `stellar-contracts/src/lib.rs`:
- Lines 152–198 declare 18 test modules (with two commented out).
- Over 49 test files reside in `stellar-contracts/src/` without corresponding `mod` statements.
- Several orphaned test files reference deprecated function signatures (e.g. `add_insurance_policy`), which would fail compilation if linked today.

### Why it matters
A massive portion of the test suite is dormant. Regressions in access control, insurance, storage quotas, and property-based fuzzing are completely undetected by CI, giving a false sense of test coverage.

### Proposed work
- Review each of the 49 orphaned test files.
- Update stale function calls and signatures to match current contract interfaces.
- Add `#[cfg(test)] mod test_name;` statements in `stellar-contracts/src/lib.rs` for each file.
- Move tests to a dedicated `stellar-contracts/tests/` integration directory or clean `src/tests/` hierarchy.

### Acceptance criteria
- All test files in `stellar-contracts/src/` are actively compiled and executed during `cargo test`.
- `cargo test --workspace` passes without warnings or skipped modules.

### Testing requirements
- Execute `cargo test --package kora-stellar` and verify test count increases from ~30 to over 150 tests.

### Dependencies
Depends on #12, #13, #25, #43

### Code references
- `stellar-contracts/src/lib.rs`: lines 150–200
- `stellar-contracts/src/test_*.rs` (71 files)

---

## #31: Deduplicate redundant CI workflows between `stellar-contracts.yml` and `stellar.yml`

### Category
CI/CD

### Priority
P2

### Problem
The repository contains two redundant GitHub Actions workflow files performing overlapping Soroban checks:
1. `.github/workflows/stellar-contracts.yml`: Triggers on push and PR to `main`/`develop` for `stellar-contracts/**`, running formatting, clippy, build, and tests.
2. `.github/workflows/stellar.yml`: Triggers on push and PR to `main`/`develop` for `stellar-contracts/**`, running an almost identical set of toolchain installations, builds, and test commands.

### Current implementation
- `.github/workflows/stellar-contracts.yml`: 65 lines defining build, test, and lint jobs.
- `.github/workflows/stellar.yml`: 54 lines repeating the exact same jobs with slight naming differences.

### Why it matters
Every pull request triggers both workflows in parallel, consuming double the GitHub Actions runner minutes, congesting CI queues, and creating confusion when one workflow passes while the other fails due to divergent toolchain parameters.

### Proposed work
- Consolidate all Soroban CI steps into a single canonical workflow file (`.github/workflows/stellar-contracts.yml`).
- Delete redundant `.github/workflows/stellar.yml`.
- Standardize triggers, Rust toolchain version, and cargo cache actions.

### Acceptance criteria
- Exactly one CI workflow handles Stellar contract builds and tests.
- CI runtime is halved per pull request.
- No redundant runner invocations.

### Testing requirements
- Validate workflow syntax using GitHub Actions schema or workflow linter (`actionlint`).
- Open a draft PR touching `stellar-contracts/` and confirm exactly one unified workflow executes.

### Dependencies
Dependencies: None

### Code references
- `.github/workflows/stellar-contracts.yml`: lines 1–65
- `.github/workflows/stellar.yml`: lines 1–54

---

## #32: Cache `stellar-cli` installation in GitHub Actions workflows to eliminate redundant compilation

### Category
CI/CD

### Priority
P2

### Problem
In `.github/workflows/stellar-contracts.yml` and `.github/workflows/stellar.yml`, the CI jobs run:
```bash
cargo install --locked stellar-cli --features opt
```
without any caching mechanism.

### Current implementation
In `.github/workflows/stellar-contracts.yml`:
- Lines 28–32 execute `cargo install --locked stellar-cli` directly on an un-cached Ubuntu runner on every push and PR.

### Why it matters
Compiling `stellar-cli` from source takes between 10 to 15 minutes on standard GitHub runners. This inflates developer feedback loops from 1 minute to over 15 minutes per commit and wastes hundreds of compute minutes daily.

### Proposed work
- Integrate `actions/cache` or `Swatinem/rust-cache` to cache `$HOME/.cargo/bin/stellar` and cargo build artifacts keyed by `stellar-cli` version.
- Alternatively, download pre-compiled binary releases of `stellar-cli` directly from GitHub releases during CI setup.

### Acceptance criteria
- CI workflow does not recompile `stellar-cli` on every run.
- CI pipeline total execution duration decreases from ~18 minutes to under 4 minutes.

### Testing requirements
- Trigger workflow twice in succession; verify the second run hits the cache and completes in under 4 minutes.

### Dependencies
Depends on #31

### Code references
- `.github/workflows/stellar-contracts.yml`: lines 28–35

---

## #33: Fix unsound auto-merge rule that bypasses pull request review approval in `auto-merge.yml`

### Category
CI/CD

### Priority
P1

### Problem
In `.github/workflows/auto-merge.yml` (lines 10–22), the auto-merge condition is evaluated using a logical OR (`||`):
```yaml
if: github.event.review.state == 'approved' || github.event.check_suite.conclusion == 'success'
```

### Current implementation
In `.github/workflows/auto-merge.yml`:
- If any check suite completes with `success`, the workflow attempts to merge the PR immediately, regardless of whether a human code review was submitted, pending, or marked `changes_requested`.

### Why it matters
Any automated commit or contributor PR that passes basic linting can be merged without human peer review or security sign-off. This introduces an unacceptable supply-chain and code-tampering vulnerability.

### Proposed work
- Change the condition to require BOTH conditions (logical AND) and verify branch protection enforcement:
  ```yaml
  if: github.event.pull_request.mergeable && ...
  ```
- Alternatively, rely on GitHub's native native auto-merge feature (`gh pr merge --auto --squash`) configured strictly with branch protection rules requiring approved reviews.

### Acceptance criteria
- Auto-merge is never triggered if review approval is missing or if reviews request changes.
- Successful CI checks alone cannot trigger a merge without explicit review approval.

### Testing requirements
- Test CI pass on unapproved PR: ensure merge is not attempted.
- Test approved PR with passing CI: ensure merge succeeds.

### Dependencies
Dependencies: None

### Code references
- `.github/workflows/auto-merge.yml`: lines 10–25

---

## #34: Add test and build steps for `pet-transfer-adoption` contract to CI pipeline

### Category
CI/CD

### Priority
P2

### Problem
The CI workflow `.github/workflows/stellar-contracts.yml` specifically changes directories only into `stellar-contracts/` and runs tests there:
```bash
cargo test --manifest-path stellar-contracts/Cargo.toml
```
The peripheral contract `stellar-contracts/contracts/pet-transfer-adoption` is completely omitted from CI builds and tests.

### Current implementation
- `.github/workflows/stellar-contracts.yml`: No reference to `pet-transfer-adoption`.
- Changes made to adoption logic never undergo CI automated testing.

### Why it matters
Code regressions, breaking SDK bumps, and compiler failures in `pet-transfer-adoption` can be merged into `main` without detection.

### Proposed work
- Update CI workflow test step to test all contracts:
  ```bash
  cargo test --workspace --all-targets
  ```
- Add a build step compiling `pet_transfer_adoption.wasm` to ensure WASM size and protocol compliance.

### Acceptance criteria
- CI builds and runs tests for both `kora-stellar` and `pet_transfer_adoption`.
- CI fails if any contract fails compilation or tests.

### Testing requirements
- Introduce a deliberate test assertion failure in `pet-transfer-adoption` on a test branch; verify CI catches and reports the failure.

### Dependencies
Depends on #6, #7

### Code references
- `.github/workflows/stellar-contracts.yml`: lines 24–45
- `stellar-contracts/contracts/pet-transfer-adoption/`

---

## #35: Fix permission scopes and branch triggers in `pr-review-bot.yml`

### Category
CI/CD

### Priority
P2

### Problem
In `.github/workflows/pr-review-bot.yml`, the workflow lacks the explicit `permissions` block required by GitHub Actions with modern token security. Specifically, posting review comments requires `pull-requests: write` and `issues: write`.
Furthermore, triggering on `pull_request_target` without commit-pinning exposes repository secrets to fork PRs.

### Current implementation
In `.github/workflows/pr-review-bot.yml`:
- Lines 1–15: Missing top-level `permissions:` configuration.
- Uses default `GITHUB_TOKEN` which is read-only in contemporary GitHub default repository settings, causing bot comments to fail with HTTP 403 Forbidden.

### Why it matters
The review bot fails silently on every external contributor pull request with authorization errors, generating noisy CI failures.

### Proposed work
- Add explicit permissions block:
  ```yaml
  permissions:
    contents: read
    pull-requests: write
    issues: write
  ```
- Ensure secure checkout semantics for fork PRs.

### Acceptance criteria
- Review bot successfully posts automated review comments on pull requests.
- Workflow runs with least-privilege security permissions.

### Testing requirements
- Trigger workflow on a test PR; verify successful bot comment posting without HTTP 403 errors.

### Dependencies
Dependencies: None

### Code references
- `.github/workflows/pr-review-bot.yml`: lines 1–30

---

## #36: Synchronize `docs/error-codes.md` with on-chain `ContractError` enum variants

### Category
Documentation

### Priority
P2

### Problem
The documentation file `docs/error-codes.md` contains an obsolete, incomplete table of error codes. It documents code 1 as `Unauthorized`, code 2 as `AdminNotInitialized`, and code 3 as `PetNotFound`.
In reality, the on-chain contract `stellar-contracts/src/lib.rs` defines over 30 error codes starting with `AdminAlreadyApproved = 1`, `AdminAlreadySet = 2`, and `ThresholdTooHigh = 3`.

### Current implementation
- `docs/error-codes.md`: Displays an outdated 10-row error code table.
- `stellar-contracts/src/lib.rs` (lines 866–920): Defines `ContractError` enum with dozens of modern variants.

### Why it matters
External developers, backend engineers, and mobile integration teams reading `docs/error-codes.md` will misinterpret error responses, resulting in improper client-side error handling and incorrect UX states.

### Proposed work
- Rewrite `docs/error-codes.md` to reflect every variant of `ContractError`.
- Document the numeric code, enum variant name, cause, and recommended client recovery action for each error.
- Add an automated documentation lint/check to ensure parity between code and documentation.

### Acceptance criteria
- `docs/error-codes.md` lists all `ContractError` discriminants accurately.
- Error descriptions match actual contract revert conditions.

### Testing requirements
- Manual or automated cross-check between `ContractError` in `stellar-contracts/src/lib.rs` and `docs/error-codes.md`.

### Dependencies
Depends on #27

### Code references
- `docs/error-codes.md`: lines 1–50
- `stellar-contracts/src/lib.rs`: lines 866–920

---

## #37: Update `docs/api.md` and `docs/openapi.yaml` to document missing medical record privacy and authorization parameters

### Category
Documentation

### Priority
P2

### Problem
`docs/api.md` and `docs/openapi.yaml` specify REST/RPC endpoints for medical record retrieval (`GET /pets/{id}/medical-records`) as unauthenticated public queries requiring only `pet_id`.
This contradicts the on-chain security requirements and the privacy model, which require caller signatures and access verification for `Restricted` and `Private` pets.

### Current implementation
- `docs/api.md`: Line 74 documents `GET /pets/{petId}/medical-records` with no authorization header or cryptographic caller parameters.
- `docs/openapi.yaml`: Line 112 specifies 200 OK responses with full clinical notes for unauthenticated clients.

### Why it matters
Frontend and backend engineers building against `openapi.yaml` will implement unauthenticated endpoints that will fail against the secured smart contract or accidentally leak data if implemented naively in caching layers.

### Proposed work
- Update `docs/api.md` and `docs/openapi.yaml` to document required `Authorization: Bearer <signature>` headers and `caller_address` parameters.
- Document HTTP 401 Unauthorized and HTTP 403 Forbidden responses when querying restricted pet records.
- Document emergency access override query parameters (`reason`, `emergency_caller`).

### Acceptance criteria
- OpenAPI specification defines security schemes and parameter requirements for all medical record queries.
- `docs/api.md` accurately describes authentication rules for different pet privacy levels.

### Testing requirements
- Validate `docs/openapi.yaml` using Swagger/OpenAPI 3.0 validator CLI.

### Dependencies
Depends on #12

### Code references
- `docs/api.md`: lines 60–110
- `docs/openapi.yaml`: lines 95–160

---

## #38: Update `docs/architecture.md` to document dual-chain differences between Stellar and Celo implementations

### Category
Documentation

### Priority
P2

### Problem
In `docs/architecture.md`, the architecture is described as having identical parity across Stellar (Soroban) and Celo (EVM). Specifically, it asserts that pet ownership transfers, access controls, and emergency overrides operate under identical 2-step protocols on both networks.
In actual code:
- Stellar implements 2-step transfer (`accept_pet_transfer`), while Celo implements 1-step direct transfer (`transferPet`).
- Stellar features field-level client encryption, while Celo stores records in plaintext strings.
- Stellar features multisig governance, while Celo uses single-admin `onlyAdmin`.

### Current implementation
- `docs/architecture.md`: Describes a generic, unified architecture without acknowledging fundamental divergence between the Celo and Stellar smart contracts.

### Why it matters
Architectural divergence without documentation leads to incorrect assumptions by bridge developers, auditor confusion, and inconsistent user experiences across ecosystems.

### Proposed work
- Add a dedicated "Dual-Chain Implementation Matrix" section to `docs/architecture.md`.
- Detail the differences between Soroban and EVM storage, encryption, multisig governance, and transfer patterns.
- Outline the roadmap for bringing Celo contracts to feature parity with Soroban.

### Acceptance criteria
- `docs/architecture.md` accurately reflects the exact capabilities and limitations of both chains.
- Differences in transfer models and encryption are clearly highlighted.

### Testing requirements
- Review against `celo-contracts/contracts/KoraRegistry.sol` and `stellar-contracts/src/lib.rs`.

### Dependencies
Depends on #4

### Code references
- `docs/architecture.md`: lines 1–150
- `celo-contracts/contracts/KoraRegistry.sol`
- `stellar-contracts/src/lib.rs`

---

## #39: Correct contract initialization and multisig onboarding guides in `docs/development.md`

### Category
Documentation

### Priority
P2

### Problem
In `docs/development.md` (lines 45–70), instructions for initializing local and testnet contracts state:
```bash
stellar contract invoke --id $CONTRACT_ID -- propose_init
```
However, `propose_init` in `stellar-contracts/src/lib.rs` requires `admin: Address` and `threshold: u32` parameters. Running the command in the documentation results in argument mismatch CLI errors. Furthermore, the guide fails to document `init_admin` or `init_multisig`.

### Current implementation
- `docs/development.md`: Contains non-functioning CLI command snippets that fail when executed.

### Why it matters
New contributors and DevOps engineers setting up development or testing environments cannot initialize the contract using the official guide, wasting setup time.

### Proposed work
- Update `docs/development.md` with tested, valid CLI invocation commands for `init_admin` and `init_multisig`.
- Provide exact example parameters, including address generation and threshold formatting.

### Acceptance criteria
- Step-by-step instructions in `docs/development.md` execute without error against local testnet instances.

### Testing requirements
- Execute documentation CLI commands sequentially against a local standalone Soroban network.

### Dependencies
Depends on #24

### Code references
- `docs/development.md`: lines 45–80
- `stellar-contracts/src/lib.rs`: lines 3694–3780

---

## #40: Align OpenAPI schema response structures for pet transfer and adoption records

### Category
API/Backend

### Priority
P2

### Problem
`docs/openapi.yaml` defines data models for pet transfers and adoptions that do not match the on-chain structures in `stellar-contracts/contracts/pet-transfer-adoption/src/lib.rs`.
Specifically, the OpenAPI schema defines `TransferRequest` with fields `{ requestId, fromUser, toUser, fee }`, whereas the on-chain contract defines `TransferRequest` with `{ pet_id, current_owner, new_owner, timestamp, status, transfer_type }`.

### Current implementation
- `docs/openapi.yaml`: Lines 280–320 contain placeholder schemas for adoption transfers that lack `status`, `transfer_type`, and timestamp fields.
- `stellar-contracts/contracts/pet-transfer-adoption/src/lib.rs`: Lines 45–85 define the authoritative on-chain types.

### Why it matters
Backend services built using OpenAPI code generators will produce models incompatible with smart contract event payloads and storage queries, breaking API integrations.

### Proposed work
- Update schemas in `docs/openapi.yaml` to match the exact field names and types of `pet-transfer-adoption/src/lib.rs`.
- Include enum values for `TransferStatus` (`Pending`, `Approved`, `Rejected`, `Cancelled`).

### Acceptance criteria
- `openapi.yaml` definitions match on-chain Rust struct definitions.
- OpenAPI validation tools report 0 errors.

### Testing requirements
- Validate schema syntax with `spectral lint docs/openapi.yaml`.

### Dependencies
Depends on #6

### Code references
- `docs/openapi.yaml`: lines 280–340
- `stellar-contracts/contracts/pet-transfer-adoption/src/lib.rs`: lines 45–85

---

## #41: Prevent duplicate pet microchip registration across different owners

### Category
Data Integrity

### Priority
P1

### Problem
In `stellar-contracts/src/lib.rs` (lines 4640–4760), `register_pet` accepts `microchip_id: Option<String>`. However, the contract never checks whether the provided microchip ID is already assigned to an existing pet.
Furthermore, there is no storage mapping from `microchip_id` to `pet_id`.

### Current implementation
In `stellar-contracts/src/lib.rs`:
- Line 4640: `microchip_id` is passed as argument.
- Line 4753: Stored directly into `Pet.microchip_id`.
- No `DataKey::Microchip(String)` check or storage write exists.

### Why it matters
Microchips are ISO-standard, globally unique physical pet identifiers. Allowing duplicate microchip registrations allows malicious actors to clone another pet's microchip ID, claim ownership of lost pets, or overwrite veterinary history associations.

### Proposed work
- Introduce storage key `DataKey::PetByMicrochip(String)`.
- In `register_pet` and `update_pet_profile`, if `microchip_id` is `Some(id)`:
  - Assert that `!env.storage().persistent().has(&DataKey::PetByMicrochip(id))`.
  - Store mapping `DataKey::PetByMicrochip(id) -> pet_id`.
- Provide query `get_pet_by_microchip(env: Env, microchip_id: String) -> Option<u64>`.

### Acceptance criteria
- Attempting to register a pet with a microchip ID already registered reverts with `ContractError::InvalidInput` (or dedicated `MicrochipAlreadyRegistered`).
- Pets without microchips (`None`) register without conflict.
- Microchip lookup correctly returns the associated pet ID.

### Testing requirements
- Test: Register Pet 1 with microchip "MC-999" (succeeds).
- Test: Attempt to register Pet 2 with microchip "MC-999" (reverts).
- Test: Query `get_pet_by_microchip("MC-999")` returns Pet 1 ID.

### Dependencies
Dependencies: None

### Code references
- `stellar-contracts/src/lib.rs`: lines 4640–4760, 4837–4885
- `stellar-contracts/src/test_pet_validation.rs`

---

## #42: Add expiration timestamp check to veterinarian license verification status

### Category
Smart Contract

### Priority
P2

### Problem
In `stellar-contracts/src/lib.rs`:
1. The `Vet` struct (line 928) contains `license_number: String` and `verified: bool`, but no license expiration timestamp (`license_expiry: u64`).
2. When admins verify a vet via `verify_vet` (line 6332), `verified` is set to `true` permanently.
3. In `is_verified_vet` (line 6493), only `vet.verified` is checked:
   ```rust
   pub fn is_verified_vet(env: Env, vet_address: Address) -> bool {
       env.storage().instance().get(&DataKey::Vet(vet_address)).map(|v| v.verified).unwrap_or(false)
   }
   ```

### Current implementation
In `stellar-contracts/src/lib.rs`:
- Lines 928–935: `Vet` struct lacks expiration timestamp.
- Lines 6332–6340: `verify_vet` sets perpetual verification without renewal requirements.

### Why it matters
Veterinary licenses are legally time-limited and require periodic state board renewal. Without expiration tracking, a veterinarian whose real-world license has lapsed or been revoked by state regulators retains permanent cryptographic authority to sign clinical records on Kora.

### Proposed work
- Add `license_expiry: u64` field to `Vet` struct.
- Update `verify_vet` to accept `expiry_timestamp: u64`.
- In `is_verified_vet`, return `true` only if `vet.verified && env.ledger().timestamp() <= vet.license_expiry`.
- Add `renew_vet_license(env: Env, admin: Address, vet: Address, new_expiry: u64)`.

### Acceptance criteria
- `is_verified_vet` returns `false` if `ledger().timestamp() > license_expiry`.
- Expired veterinarians cannot add medical records or sign vaccinations.
- Admins can renew licenses with updated expiration dates.

### Testing requirements
- Test: Verify vet with expiry at $T + 1000$. Confirm verified at $T + 500$, unverified at $T + 1001$.
- Test: Expired vet attempting `add_medical_record` reverts.

### Dependencies
Dependencies: None

### Code references
- `stellar-contracts/src/lib.rs`: lines 928–935, 6332–6340, 6493–6500
- `stellar-contracts/src/test_batch_verify_vets.rs`

---

## #43: Implement insurance policy registration and claim payout status transition

### Category
Smart Contract

### Priority
P1

### Problem
In `stellar-contracts/src/lib.rs`, the insurance module contains extensive data definitions (`InsurancePolicy`, `InsuranceClaim`, `InsuranceKey::PetPolicyIndex`, `InsuranceClaimStatusUpdatedEvent`), and read queries (`get_pet_insurance`, `get_pet_health_summary`).
However, the public mutation methods to manage insurance are completely missing from `KoraContract`:
- `add_insurance_policy` does not exist.
- `update_insurance_status` does not exist.
- `submit_insurance_claim` does not exist.
- `process_claim_payout` does not exist.
(These missing methods are even invoked in the orphaned `test_insurance.rs` file).

### Current implementation
In `stellar-contracts/src/lib.rs`:
- Lines 48–58: Storage keys defined.
- Lines 1993–2040: Structs and events defined.
- Lines 2738–2755: `get_pet_insurance` queries keys that no function ever populates.
- Absence of policy creation and claim mutation methods.

### Why it matters
The insurance feature highlighted in project documentation is completely non-operational. Users cannot attach insurance policies to pets, and insurance providers cannot process claims.

### Proposed work
- Implement `add_insurance_policy(env: Env, pet_id: u64, policy_id: String, provider: String, coverage_type: String, premium: u64, coverage_limit: u64, expiry_date: u64) -> bool`.
- Implement `update_insurance_status(env: Env, owner: Address, pet_id: u64, policy_id: String, active: bool) -> bool`.
- Implement claim submission and adjudication functions emitting `InsuranceClaimSubmittedEvent` and `InsuranceClaimStatusUpdatedEvent`.

### Acceptance criteria
- Policies can be attached to registered pets by pet owners or approved providers.
- `get_pet_insurance` and `get_pet_health_summary` return accurate policy data.
- Emits required audit events upon state changes.

### Testing requirements
- Unit tests validating policy registration, active/expired status evaluation, and claim transitions.

### Dependencies
Dependencies: None

### Code references
- `stellar-contracts/src/lib.rs`: lines 48–58, 1993–2040, 2738–2755, 5360–5385
- `stellar-contracts/src/test_insurance.rs`

---

## #44: Validate dosage, frequency, and prescribing veterinarian in `Medication` records

### Category
Smart Contract

### Priority
P2

### Problem
In `stellar-contracts/src/lib.rs`, `add_medical_record` (line 11470) accepts a vector of medications `medications: Vec<Medication>`.
The function accepts this vector without any validation:
1. It does not check that `medication.pet_id == pet_id`.
2. It does not check that `medication.prescribing_vet == vet_address`.
3. It does not validate string lengths for `name`, `dosage`, or `frequency`.
4. It does not validate that `medication.start_date <= medication.end_date`.

### Current implementation
In `stellar-contracts/src/lib.rs` (lines 11470–11520):
```rust
pub fn add_medical_record(..., medications: Vec<Medication>, ...) {
    // Inserts medications directly into MedicalRecord with zero field inspection!
    let record = MedicalRecord { ..., medications, ... };
```

### Why it matters
A veterinarian can attach medication records referencing different pets or falsely attributing prescriptions to other veterinarians. Furthermore, unbounded string sizes or inverted dates can corrupt client rendering and medication reminder engines.

### Proposed work
- Implement `validate_medications(&Env, pet_id: u64, vet: &Address, medications: &Vec<Medication>) -> Result<(), ContractError>`.
- Enforce: `med.pet_id == pet_id` and `med.prescribing_vet == *vet`.
- Enforce length limits on strings (`MAX_MEDICATION_NAME_LEN`, etc.).
- Validate date boundaries when `end_date` is `Some(ts)`.

### Acceptance criteria
- Inconsistent `pet_id` or `prescribing_vet` in medication structs reverts with `ContractError::InvalidInput`.
- Malformed dates or empty strings revert with appropriate validation errors.

### Testing requirements
- Test: Adding medical record with mismatched `medication.pet_id` reverts.
- Test: Adding medication with `end_date < start_date` reverts.

### Dependencies
Dependencies: None

### Code references
- `stellar-contracts/src/lib.rs`: lines 1671–1681, 11470–11520
- `stellar-contracts/src/test_input_limits.rs`

---

## #45: Implement caller authorization and privacy checks for pet photo queries and deletion

### Category
Access Control

### Priority
P1

### Problem
In `stellar-contracts/src/lib.rs`, pet photo queries and deletions have authorization flaws:
1. `get_pet_photos` (line 5644) and `get_pet_photos_paginated` (line 5672) take no `caller: Address` parameter and perform no checks on `pet.privacy_level`. Any public network observer can retrieve all photo hashes for private pets.
2. `remove_pet_photo` (line 5700) requires `pet.owner.require_auth()`, but does not verify whether the pet is locked or involved in an ongoing custody dispute.

### Current implementation
In `stellar-contracts/src/lib.rs` (lines 5644–5698):
```rust
pub fn get_pet_photos(env: Env, pet_id: u64) -> Vec<String> {
    if let Some(pet) = env.storage().instance().get::<DataKey, Pet>(&DataKey::Pet(pet_id)) {
        // No caller check, no privacy check
        pet.photo_hashes
    } else { Vec::new(&env) }
}
```

### Why it matters
Leaking photo hashes for private pets compromises owner privacy and can expose identifying visual characteristics, tag numbers, or owner locations depicted in images.

### Proposed work
- Add `caller: Address` to `get_pet_photos` and `get_pet_photos_paginated`.
- Verify caller is owner, authorized vet, or that `pet.privacy_level == PrivacyLevel::Public`.
- Return `ContractError::Unauthorized` for unauthorized queries.

### Acceptance criteria
- Photos of private pets cannot be read by random addresses.
- Owners and authorized vets retain full photo retrieval permissions.

### Testing requirements
- Test: Unauthorized query for private pet photos returns unauthorized error.
- Test: Owner query succeeds.

### Dependencies
Dependencies: None

### Code references
- `stellar-contracts/src/lib.rs`: lines 5644–5698, 5700–5725
- `stellar-contracts/src/test_get_pet_access_control.rs`

---

## #46: Prevent deactivated or unverified veterinarians from adding medical and vaccination records

### Category
Access Control

### Priority
P1

### Problem
In `stellar-contracts/src/lib.rs`:
1. In `add_medical_record` (line 11480):
   ```rust
   if !Self::is_verified_vet(env.clone(), vet_address.clone()) {
       panic!("Veterinarian not verified");
   }
   ```
   This uses a raw string `panic!` rather than a Soroban error (`panic_with_error!(&env, ContractError::VetNotVerified)`), preventing programmatic error interpretation.
2. When a vet's license is revoked via `revoke_vet_license` (line 6464), `vet.verified` is set to `false`. However, the contract maintains a separate `DataKey::RevokedVets` list that is never inspected in `is_verified_vet`.

### Current implementation
In `stellar-contracts/src/lib.rs`:
- Line 11481: Unformatted string panic instead of `ContractError`.
- Lines 6464–6485: `revoke_vet_license` sets `vet.verified = false` and writes `RevokedVets`, but verification checks rely solely on `vet.verified`.

### Why it matters
String panics consume excessive gas and cannot be handled gracefully by client SDKs or dApps. Furthermore, any desynchronization between `Vet.verified` and `RevokedVets` risks allowing deactivated vets to continue signing clinical records.

### Proposed work
- Replace string panics with `panic_with_error!(&env, ContractError::VetNotVerified)`.
- Ensure `is_verified_vet` checks both `vet.verified` and confirms the address is not present in `DataKey::RevokedVets`.

### Acceptance criteria
- Unverified or revoked veterinarians attempting to add records trigger `ContractError::VetNotVerified`.
- String panics are eliminated from medical and vaccination logging flows.

### Testing requirements
- Test: Revoked vet calls `add_medical_record`; verify transaction reverts with `ContractError::VetNotVerified`.

### Dependencies
Dependencies: None

### Code references
- `stellar-contracts/src/lib.rs`: lines 6464–6500, 11470–11485
- `stellar-contracts/src/test_access_control.rs`

---

## #47: Validate diet plan calorie targets and macronutrient percentage totals

### Category
Data Integrity

### Priority
P2

### Problem
In `stellar-contracts/src/lib.rs`:
- `set_diet_plan` (lines 7369–7430) accepts `calories_per_serving: u32` and `daily_target_calories: u32`. It performs zero range validation: values can be 0 or 4,000,000,000.
- String parameters (`food_type`, `portion_size`, `feeding_frequency`) have no length bounds.
- In `add_nutrition_plan` (line 7717), the calorie tolerance check compares ingredient sums, but permits empty ingredient lists or zero calorie totals.

### Current implementation
In `stellar-contracts/src/lib.rs`:
- Lines 7369–7409: Direct assignment of unvalidated integer and string values to `DietPlan`.
- Lines 7717–7743: Minimal check on `name.is_empty()`, but missing ingredient count bounds.

### Why it matters
Corrupted or extreme calorie values break downstream nutritional calculators, weight management algorithms, and mobile UI rendering. Unbounded strings allow griefing via instance storage consumption.

### Proposed work
- Define rational bounds: `MIN_DAILY_CALORIES = 10`, `MAX_DAILY_CALORIES = 20,000`.
- Enforce length limits on strings (`food_type`, `portion_size`, etc.).
- Require `ingredients` vector in `add_nutrition_plan` to have at least 1 and at most 50 items.

### Acceptance criteria
- Inputs outside bounded calorie ranges revert with `ContractError::InvalidInput`.
- Excessive string lengths revert with `ContractError::InputTooLong`.

### Testing requirements
- Test: Set diet plan with 0 calories -> reverts.
- Test: Set diet plan with 50,000 calories -> reverts.
- Test: Set diet plan with valid values (1,500 kcal) -> succeeds.

### Dependencies
Dependencies: None

### Code references
- `stellar-contracts/src/lib.rs`: lines 7369–7430, 7717–7760
- `stellar-contracts/src/test_nutrition_plan.rs`

---

## #48: Prevent re-initialization of insurance policy parameters and duplicate claim submissions

### Category
Security

### Priority
P1

### Problem
In `stellar-contracts/src/lib.rs`, insurance claim submission lacks uniqueness constraints. An owner or compromised vet can submit multiple claims for the exact same veterinary treatment or invoice, resulting in duplicate claim records and repeated payouts against the same policy coverage limit.

### Current implementation
In `stellar-contracts/src/lib.rs`:
- Lines 49–52: `ClaimCount` is incremented on every submission without checking invoice hash or treatment reference uniqueness.
- No storage key links `(pet_id, medical_record_id)` or `invoice_hash` to existing claims.

### Why it matters
Insurance fraud via double-submission of veterinary invoices is a primary threat in pet insurance. Without on-chain duplicate claim detection, insurance liquidity pools can be drained through repeated submissions of a single approved claim.

### Proposed work
- Require an `invoice_hash: BytesN<32>` or unique `medical_record_id` with every claim submission.
- Store `InsuranceKey::ClaimByInvoice(BytesN<32>) -> u64` (claim ID).
- Assert that the invoice hash has not already been submitted.

### Acceptance criteria
- Submitting a claim with an already processed invoice hash reverts with `ContractError::InvalidInput` (or `ClaimAlreadySubmitted`).
- Unique claims proceed through adjudication normally.

### Testing requirements
- Test: Submit claim with Invoice Hash A (succeeds).
- Test: Submit second claim with Invoice Hash A (reverts).

### Dependencies
Depends on #43

### Code references
- `stellar-contracts/src/lib.rs`: lines 48–58, 2040–2095
- `stellar-contracts/src/test_insurance_claims.rs`

---

## #49: Add integration test suite for cross-contract calls between `KoraContract` and `pet-transfer-adoption`

### Category
Testing

### Priority
P2

### Problem
The `pet-transfer-adoption` contract (`stellar-contracts/contracts/pet-transfer-adoption/`) is intended to interact with the core `KoraContract` to verify pet existence, check custody, and transfer ownership upon adoption completion.
However, there are currently zero cross-contract integration tests in the repository. Both contracts are tested only in complete isolation with mock objects.

### Current implementation
- `stellar-contracts/tests/`: Directory does not exist; integration tests are absent.
- `pet-transfer-adoption/src/test.rs`: Uses localized mock environments without deploying real `KoraContract` clients.

### Why it matters
Inter-contract call interfaces, cross-contract authorization, and argument serialization errors are completely untested. In production, differences in SDK versions or type layouts will cause cross-contract calls to fail.

### Proposed work
- Create integration test suite in `stellar-contracts/tests/integration_adoption.rs`.
- In test environment:
  - Register and deploy `KoraContract`.
  - Register and deploy `pet-transfer-adoption` contract.
  - Execute a full end-to-end flow: Pet registration in Kora -> Adoption listing -> Applicant application -> Approval -> Ownership transfer in Kora.

### Acceptance criteria
- Full integration test executes successfully using Soroban SDK test utilities.
- Ownership of pet in `KoraContract` transitions seamlessly to adopter upon final adoption step.

### Testing requirements
- Run `cargo test --test integration_adoption`.

### Dependencies
Depends on #6, #7, #25

### Code references
- `stellar-contracts/src/lib.rs`
- `stellar-contracts/contracts/pet-transfer-adoption/src/lib.rs`
- `stellar-contracts/contracts/pet-transfer-adoption/src/test.rs`

---

## #50: Clean up unused dependencies and redundant profile configurations in `stellar-contracts`

### Category
Technical Debt

### Priority
P3

### Problem
In `stellar-contracts/Cargo.toml` and root `Cargo.toml`:
1. `stellar-contracts/Cargo.toml` contains extensive comments and note blocks regarding Cargo workspace profiles that are redundant with root configuration.
2. In `stellar-contracts/Cargo.toml`, `proptest = "1.4.0"` is included under `[dev-dependencies]`, but property-based test files (`test_proptest_medical.rs`) are orphaned.
3. Feature flag definitions and unused imports across sub-crates create compilation noise and expand dependency graphs unnecessarily.

### Current implementation
- `/workspaces/kora-app/Cargo.toml`: Root manifest.
- `stellar-contracts/Cargo.toml`: Lines 21–26 contain historical commentary regarding `overflow-checks` and profile locations.
- Unused dependencies remain in `Cargo.lock`.

### Why it matters
Redundant manifest configurations confuse developers, bloat dependency resolution times, and clutter build logs with deprecation warnings.

### Proposed work
- Clean up manifest files, removing stale commentary and redundant profile declarations.
- Prune unused transitive dependencies via `cargo autostatus` / `cargo machete`.
- Ensure uniform edition, license, and repository metadata across all workspace members.

### Acceptance criteria
- `cargo check --workspace` produces zero warnings regarding unused dependencies or manifest keys.
- Dependency tree is minimal and unified across crates.

### Testing requirements
- Run `cargo check --workspace --all-targets`.

### Dependencies
Depends on #7

### Code references
- `stellar-contracts/Cargo.toml`: lines 1–26
- `Cargo.toml`: lines 1–20

---

# Wave 9 Dependency Map

The 50 issues in Drips Wave 9 form clear engineering dependency graphs. Implementing foundational storage, access control, and workspace fixes unlocks downstream contract logic, integration testing, and documentation alignment.

```
+-------------------------------------------------------------------------------+
|                       FOUNDATIONAL & BLOCKING ISSUES                         |
|                                                                               |
|  [#6] Upgrade pet-transfer SDK v28         [#8] Implement TTL Extensions     |
|         |                                          |                          |
|         v                                          v                          |
|  [#7] Integrate Workspace Cargo.toml       [#9] Migrate to Persistent Storage |
|         |                                          |                          |
|         |                                          v                          |
|         |                                  [#11] Single-Pass Compaction       |
+---------|------------------------------------------|--------------------------+
          |                                          |
          |   +--------------------------------------+
          |   |
          v   v
+-------------------------------------------------------------------------------+
|                    SECURITY, ACCESS CONTROL & AUDIT                           |
|                                                                               |
|  [#12] Restrict Medical Record Access    [#18] Emergency Auth / Audit Forgery |
|  [#13] Add Activity Record Auth          [#19] Replace Pseudo-Encryption      |
|  [#14] Set Pet Traits Authorization      [#20] Enforce Upgrade Timelock Delay |
|  [#22] Multisig Threshold DoS Fix                  |                          |
|         |                                          v                          |
|         v                                  [#21] Fix Rollback PreviousWasmHash|
|  [#23] Quorum Percent Protection         [#24] Deprecate propose_init         |
+-------------------------------------------------------------------------------+
          |
          v
+-------------------------------------------------------------------------------+
|                   SMART CONTRACT LOGIC & DATA INTEGRITY                       |
|                                                                               |
|  [#1] Celo correctMedicalRecord Fix     [#10] Decrement Pet Storage Quota    |
|         |                               [#15] Validate Breeding Records       |
|         v                                      |                              |
|  [#2] Celo Pause Modifier                      v                              |
|         |                               [#16] COI Parent-Child Calculation    |
|         v                               [#25] Fix new_owner & Pet Transfer    |
|  [#5] Deduplicate Celo Tests            [#26] Fix Pet Age Resolution          |
|  [#4] Celo Recipient Consent Transfer   [#41] Microchip Uniqueness Index      |
|  [#3] Celo 2-Step Admin Transfer        [#43] Insurance Policy & Claims       |
|  [#27] Error Code Standardization              |                              |
|         |                                      v                              |
|         v                               [#48] Insurance Claim Anti-Fraud      |
|  [#36] Sync error-codes.md              [#44] Validate Medication Records     |
|                                         [#47] Validate Diet Plan Calories     |
+-------------------------------------------------------------------------------+
          |
          v
+-------------------------------------------------------------------------------+
|                      TESTING & CI/CD INFRASTRUCTURE                           |
|                                                                               |
|  [#30] Re-link 49 OrphanedTest Modules (Depends on #12, #13, #25, #43)        |
|  [#31] Deduplicate CI Workflows                                               |
|         |                                                                     |
|         v                                                                     |
|  [#32] Cache stellar-cli in CI                                                |
|  [#33] Fix Auto-Merge Review Rule                                             |
|  [#34] CI pet-transfer-adoption Test Step (Depends on #6, #7)                 |
|  [#35] PR Review Bot Permissions                                              |
|  [#49] Cross-Contract Adoption Integration Suite (Depends on #6, #7, #25)      |
|  [#50] Clean Unused Dependencies (Depends on #7)                              |
+-------------------------------------------------------------------------------+
          |
          v
+-------------------------------------------------------------------------------+
|                   DOCUMENTATION & SPECIFICATION ALIGNMENT                     |
|                                                                               |
|  [#37] Update api.md & openapi.yaml Medical Privacy (Depends on #12)          |
|  [#38] Update architecture.md Dual-Chain Matrix (Depends on #4)               |
|  [#39] Correct development.md Setup Guide (Depends on #24)                    |
|  [#40] Align openapi.yaml Pet Transfer Models (Depends on #6)                 |
+-------------------------------------------------------------------------------+
```

---

# Audit Notes

### 1. Storage Footprint & Imminent Protocol Exhaustion
The single most dangerous architectural risk identified in `stellar-contracts/src/lib.rs` is the near-total reliance on `env.storage().instance()` for storing unbounded entities (`Pet`, `MedicalRecord`, `DietPlan`, `ActivityKey`, `Dispute`). On Soroban, instance storage is capped at protocol limits (~128 KiB) and loads all keys into VM memory during every invocation. Migrating these unbounded collections to `persistent()` storage (Issue #9) and implementing systematic TTL extension (Issue #8) must precede production scaling.

### 2. Illusory Cryptography & Key Management Risks
The cryptographic obfuscation implemented in `get_encryption_key` (Issue #19) provides zero security against network observers because the symmetric key is derived purely from public blockchain data (the contract address and the first admin address). Worse, rotating administrators will alter the derived key, permanently rendering all previously encrypted pet names, birthdays, and medical alerts unrecoverable. Transitioning to genuine client-side envelope encryption or static salt management is critical.

### 3. Orphaned Test Suites & Testing Reality
While `stellar-contracts/src/` contains 71 test files, 49 of them are not included in `lib.rs` and never compile or run in CI (Issue #30). Re-linking these tests revealed that several test suites had decayed against recent contract refactors (such as missing `add_insurance_policy` methods). Restoring the dormant test suite is the single highest-leverage quality improvement available for Wave 9.

### 4. Cross-Chain Disparities Between Celo and Stellar
The Celo implementation (`celo-contracts/contracts/KoraRegistry.sol`) lags significantly behind the Stellar contract in both features and security architecture. While Stellar incorporates two-step pet transfers, field-level encryption, role separation, and multisig governance, Celo uses single-step transfers, plaintext storage, and a single admin. In addition, `KoraRegistry.sol` contains severe code defects, including duplicate mappings, conflicting event declarations, and inverted state logging in `correctMedicalRecord` (Issue #1).

### 5. Recommended Execution Roadmap
1. **Sprint 1 (Infrastructure & Critical Fixes)**: Resolve compilation issues (#6, #7), fix Celo compilation/shadowing (#1, #2), and implement TTL extensions (#8).
2. **Sprint 2 (Storage & Security Core)**: Migrate instance storage to persistent (#9), fix upgrade timelock & rollback (#20, #21), address unauthenticated endpoints (#12, #13, #18), and fix emergency caller verification.
3. **Sprint 3 (Contract Parity & Testing)**: Re-link orphaned test suites (#30), implement missing insurance methods (#43), fix pet transfer cancellation (#25), and resolve duplicate CI workflows (#31, #32).
4. **Sprint 4 (Refinements & Documentation)**: Align error codes (#27, #36), update OpenAPI/architecture documentation (#37, #38, #39, #40), and add cross-contract integration tests (#49).

