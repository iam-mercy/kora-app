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
This document tracks the initial batch of 50 engineering backlog issues identified during the Wave 9 codebase audit. All 50 issues have been reviewed and published to GitHub. To maintain this local backlog as a representation of pending/unpublished work, the published issue bodies have been archived to GitHub, with live issue links retained below.

### Published GitHub Issues (Batch 1: #1 – #50)
All 50 initial engineering issues were published to GitHub under the `wave-9` label. Their full specifications, acceptance criteria, and discussion threads are tracked live on GitHub:

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

### Outstanding / Deferred Backlog Items

## Issue #2 (GitHub #2): KoraContract Wasm exceeds Soroban's 128 KiB on-chain contract-size cap — not deployable until split

- **GitHub Issue**: [#2](https://github.com/iam-mercy/kora-app/issues/2)
- **Status**: `Deferred`
- **Note**: `Deferred until the surrounding Wave 9 fixes are completed and the contract decomposition can be handled as a dedicated architectural effort.`
- **Category**: Architecture / Stellar/Soroban
- **Priority**: P0 (Architectural Epic)
- **Problem**: The compiled and optimized `KoraContract` Wasm is 219,161 bytes (~214 KiB), exceeding Soroban's on-chain limit of 131,072 bytes (128 KiB) by 88,089 bytes (67%). The upload transaction envelope (~292 KB) also exceeds `tx_max_size_bytes` (132,096 B), resulting in rejection upon deployment to any live network.
- **Why It Matters**: Prevents live testnet or mainnet deployment and end-to-end integration testing against real contract instances.
- **Dependencies**: Depends on #8, #9, #20, #21

---

### Continuation Backlog
The Wave 9 backlog continues with **75 new unpublished engineering issues** (#51 through #125) in:
[**`drips wave 9 continued.md`**](file:///workspaces/kora-app/drips%20wave%209%20continued.md)

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


