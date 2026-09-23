# Drips Wave 9 Continued — Kora App Engineering Backlog (#51 – #125)

### Repository
`iam-mercy/kora-app`

### Backlog purpose
This document records the continuation of the Drips Wave 9 engineering backlog (issues #51 through #125). All 75 issues have been reviewed and published to GitHub under the `wave-9` label. The live discussion threads, acceptance criteria, and full specifications are tracked below.

### Summary
- **Total new issues**: 75
- **Priority breakdown**:
  - **P0 (Critical / Blocking)**: 2
  - **P1 (High)**: 30
  - **P2 (Medium)**: 39
  - **P3 (Lower priority)**: 4
- **Category breakdown**:
  - **Data Integrity**: 23
  - **Smart Contract**: 21
  - **Access Control**: 15
  - **Security**: 7
  - **Performance**: 5
  - **Celo/Solidity**: 4

---

### Published GitHub Issues (Batch 2: #51 – #125)
All 75 continuation engineering issues have been published to GitHub in `iam-mercy/kora-app` under the `wave-9` label:

| Wave 9 # | GitHub Issue | Title | URL | Status |
|:---:|:---:|---|---|:---:|
| #51 | [#56](https://github.com/iam-mercy/kora-app/issues/56) | Prevent unauthorized callers from raising frivolous disputes against arbitrary pets | https://github.com/iam-mercy/kora-app/issues/56 | Published |
| #52 | [#57](https://github.com/iam-mercy/kora-app/issues/57) | Enforce registered arbitrator verification in `resolve_dispute` | https://github.com/iam-mercy/kora-app/issues/57 | Published |
| #53 | [#58](https://github.com/iam-mercy/kora-app/issues/58) | Prevent duplicate voting and post-resolution voting in dispute ballots | https://github.com/iam-mercy/kora-app/issues/58 | Published |
| #54 | [#59](https://github.com/iam-mercy/kora-app/issues/59) | Validate evidence CID format and input length in `submit_evidence` | https://github.com/iam-mercy/kora-app/issues/59 | Published |
| #55 | [#60](https://github.com/iam-mercy/kora-app/issues/60) | Enforce lower and upper bounds on dispute appeal window duration | https://github.com/iam-mercy/kora-app/issues/60 | Published |
| #56 | [#61](https://github.com/iam-mercy/kora-app/issues/61) | Implement evidence cryptographic signature verification in `verify_evidence` | https://github.com/iam-mercy/kora-app/issues/61 | Published |
| #57 | [#62](https://github.com/iam-mercy/kora-app/issues/62) | Pause pet transfer expiration timers during active custody disputes in `pet-transfer-adoption` | https://github.com/iam-mercy/kora-app/issues/62 | Published |
| #58 | [#63](https://github.com/iam-mercy/kora-app/issues/63) | Replace linear scan in `get_pet_disputes` with indexed dispute lookup | https://github.com/iam-mercy/kora-app/issues/63 | Published |
| #59 | [#64](https://github.com/iam-mercy/kora-app/issues/64) | Validate positive interval days in `create_grooming_schedule` | https://github.com/iam-mercy/kora-app/issues/64 | Published |
| #60 | [#65](https://github.com/iam-mercy/kora-app/issues/65) | Enforce admin verification on professional groomer registration | https://github.com/iam-mercy/kora-app/issues/65 | Published |
| #61 | [#66](https://github.com/iam-mercy/kora-app/issues/66) | Restrict groomer rating to verified appointment clients and enforce 1-5 star scale | https://github.com/iam-mercy/kora-app/issues/66 | Published |
| #62 | [#67](https://github.com/iam-mercy/kora-app/issues/67) | Prevent duplicate slot booking at identical timestamps for groomers | https://github.com/iam-mercy/kora-app/issues/67 | Published |
| #63 | [#68](https://github.com/iam-mercy/kora-app/issues/68) | Enforce pet owner authorization on `cancel_grooming_schedule` | https://github.com/iam-mercy/kora-app/issues/68 | Published |
| #64 | [#69](https://github.com/iam-mercy/kora-app/issues/69) | Enforce groomer caller authentication in `advance_schedule` | https://github.com/iam-mercy/kora-app/issues/69 | Published |
| #65 | [#70](https://github.com/iam-mercy/kora-app/issues/70) | Validate severity bounds and caller role in `add_behavior_record` | https://github.com/iam-mercy/kora-app/issues/70 | Published |
| #66 | [#71](https://github.com/iam-mercy/kora-app/issues/71) | Add pagination support to `get_behavior_history` | https://github.com/iam-mercy/kora-app/issues/71 | Published |
| #67 | [#72](https://github.com/iam-mercy/kora-app/issues/72) | Fix UTC midnight boundary calculation in `get_activity_streak` | https://github.com/iam-mercy/kora-app/issues/72 | Published |
| #68 | [#73](https://github.com/iam-mercy/kora-app/issues/73) | Fix milestone threshold comparison in `has_reached_milestone` | https://github.com/iam-mercy/kora-app/issues/73 | Published |
| #69 | [#74](https://github.com/iam-mercy/kora-app/issues/74) | Enforce storage quota tracking on `add_behavior_record` | https://github.com/iam-mercy/kora-app/issues/74 | Published |
| #70 | [#75](https://github.com/iam-mercy/kora-app/issues/75) | Reject future timestamps in `log_feeding` | https://github.com/iam-mercy/kora-app/issues/75 | Published |
| #71 | [#76](https://github.com/iam-mercy/kora-app/issues/76) | Enforce positive weight and realistic upper bounds in `add_weight_entry` | https://github.com/iam-mercy/kora-app/issues/76 | Published |
| #72 | [#77](https://github.com/iam-mercy/kora-app/issues/77) | Implement pagination in `get_weight_history` | https://github.com/iam-mercy/kora-app/issues/77 | Published |
| #73 | [#78](https://github.com/iam-mercy/kora-app/issues/78) | Enforce pet owner authorization on `set_nutrition_version` | https://github.com/iam-mercy/kora-app/issues/78 | Published |
| #74 | [#79](https://github.com/iam-mercy/kora-app/issues/79) | Prevent rollback to invalid or non-existent versions in `rollback_nutrition` | https://github.com/iam-mercy/kora-app/issues/79 | Published |
| #75 | [#80](https://github.com/iam-mercy/kora-app/issues/80) | Prevent integer overflow in `get_daily_summary` calorie accumulation | https://github.com/iam-mercy/kora-app/issues/80 | Published |
| #76 | [#81](https://github.com/iam-mercy/kora-app/issues/81) | Enforce pet owner authorization on `link_tag_to_pet` | https://github.com/iam-mercy/kora-app/issues/81 | Published |
| #77 | [#82](https://github.com/iam-mercy/kora-app/issues/82) | Enforce string length validation on `update_tag_message` | https://github.com/iam-mercy/kora-app/issues/82 | Published |
| #78 | [#83](https://github.com/iam-mercy/kora-app/issues/83) | Enforce caller authentication on `deactivate_tag` and `reactivate_tag` | https://github.com/iam-mercy/kora-app/issues/83 | Published |
| #79 | [#84](https://github.com/iam-mercy/kora-app/issues/84) | Unlink active tag from previous pet before re-linking in `link_tag_to_pet` | https://github.com/iam-mercy/kora-app/issues/84 | Published |
| #80 | [#85](https://github.com/iam-mercy/kora-app/issues/85) | Distinguish non-existent tags from deactivated tags in `is_tag_active` | https://github.com/iam-mercy/kora-app/issues/85 | Published |
| #81 | [#86](https://github.com/iam-mercy/kora-app/issues/86) | Enforce expiration date strictly greater than administration date in `add_vaccination` | https://github.com/iam-mercy/kora-app/issues/86 | Published |
| #82 | [#87](https://github.com/iam-mercy/kora-app/issues/87) | Enforce administering vet or admin authorization on `revoke_vaccination_certificate` | https://github.com/iam-mercy/kora-app/issues/87 | Published |
| #83 | [#88](https://github.com/iam-mercy/kora-app/issues/88) | Prevent duplicate certificate hash anchoring in `anchor_certificate` | https://github.com/iam-mercy/kora-app/issues/88 | Published |
| #84 | [#89](https://github.com/iam-mercy/kora-app/issues/89) | Fix timestamp comparison inequality in `verify_certificate` | https://github.com/iam-mercy/kora-app/issues/89 | Published |
| #85 | [#90](https://github.com/iam-mercy/kora-app/issues/90) | Support configurable window duration in `get_upcoming_vaccinations` | https://github.com/iam-mercy/kora-app/issues/90 | Published |
| #86 | [#91](https://github.com/iam-mercy/kora-app/issues/91) | Optimize vet-filtered vaccination expiry query in `get_expiring_vaccinations` | https://github.com/iam-mercy/kora-app/issues/91 | Published |
| #87 | [#92](https://github.com/iam-mercy/kora-app/issues/92) | Validate reference range boundaries in `add_lab_result` | https://github.com/iam-mercy/kora-app/issues/92 | Published |
| #88 | [#93](https://github.com/iam-mercy/kora-app/issues/93) | Enforce veterinarian verification on `add_lab_result` | https://github.com/iam-mercy/kora-app/issues/93 | Published |
| #89 | [#94](https://github.com/iam-mercy/kora-app/issues/94) | Guard against division by zero in biomarker anomaly detection calculations | https://github.com/iam-mercy/kora-app/issues/94 | Published |
| #90 | [#95](https://github.com/iam-mercy/kora-app/issues/95) | Enforce pet privacy access controls on `get_lab_results` | https://github.com/iam-mercy/kora-app/issues/95 | Published |
| #91 | [#96](https://github.com/iam-mercy/kora-app/issues/96) | Scope `get_lab_result_count` per pet to prevent global transaction metrics leakage | https://github.com/iam-mercy/kora-app/issues/96 | Published |
| #92 | [#97](https://github.com/iam-mercy/kora-app/issues/97) | Capture author address and block timestamp in `amend_medical_record` | https://github.com/iam-mercy/kora-app/issues/97 | Published |
| #93 | [#98](https://github.com/iam-mercy/kora-app/issues/98) | Prevent out-of-bounds index panic in `diff_record_versions` | https://github.com/iam-mercy/kora-app/issues/98 | Published |
| #94 | [#99](https://github.com/iam-mercy/kora-app/issues/99) | Enforce caller authorization on `delete_medical_record` soft deletion | https://github.com/iam-mercy/kora-app/issues/99 | Published |
| #95 | [#100](https://github.com/iam-mercy/kora-app/issues/100) | Preserve audit history and emit event on `update_medical_record_notes` | https://github.com/iam-mercy/kora-app/issues/100 | Published |
| #96 | [#101](https://github.com/iam-mercy/kora-app/issues/101) | Update active record count when purging expired records in `purge_expired_records` | https://github.com/iam-mercy/kora-app/issues/101 | Published |
| #97 | [#102](https://github.com/iam-mercy/kora-app/issues/102) | Enforce minimum regulatory retention floor in `set_retention_period` | https://github.com/iam-mercy/kora-app/issues/102 | Published |
| #98 | [#103](https://github.com/iam-mercy/kora-app/issues/103) | Validate medical record existence before attaching files in `add_attachment` | https://github.com/iam-mercy/kora-app/issues/103 | Published |
| #99 | [#104](https://github.com/iam-mercy/kora-app/issues/104) | Enforce `MAX_ATTACHMENTS_PER_RECORD` bound in `add_attachment` | https://github.com/iam-mercy/kora-app/issues/104 | Published |
| #100 | [#105](https://github.com/iam-mercy/kora-app/issues/105) | Enforce allowed MIME types on attachment metadata | https://github.com/iam-mercy/kora-app/issues/105 | Published |
| #101 | [#106](https://github.com/iam-mercy/kora-app/issues/106) | Enforce privacy level access controls on `get_attachments` | https://github.com/iam-mercy/kora-app/issues/106 | Published |
| #102 | [#107](https://github.com/iam-mercy/kora-app/issues/107) | Enforce pet storage quota consumption on attachment additions | https://github.com/iam-mercy/kora-app/issues/107 | Published |
| #103 | [#108](https://github.com/iam-mercy/kora-app/issues/108) | Record transfer reason and animal condition in custody chain entries | https://github.com/iam-mercy/kora-app/issues/108 | Published |
| #104 | [#109](https://github.com/iam-mercy/kora-app/issues/109) | Fix same-ledger timestamp validation bug in `verify_custody_chain` | https://github.com/iam-mercy/kora-app/issues/109 | Published |
| #105 | [#110](https://github.com/iam-mercy/kora-app/issues/110) | Redact previous owner wallet addresses for private pets in `get_ownership_history` | https://github.com/iam-mercy/kora-app/issues/110 | Published |
| #106 | [#111](https://github.com/iam-mercy/kora-app/issues/111) | Log custody entry on `finalize_transfer` in `pet-transfer-adoption` | https://github.com/iam-mercy/kora-app/issues/111 | Published |
| #107 | [#112](https://github.com/iam-mercy/kora-app/issues/112) | Make `batch_initiate_transfer` all-or-nothing atomic across pet batches | https://github.com/iam-mercy/kora-app/issues/112 | Published |
| #108 | [#113](https://github.com/iam-mercy/kora-app/issues/113) | Prevent duplicate emergency responder address registrations | https://github.com/iam-mercy/kora-app/issues/113 | Published |
| #109 | [#114](https://github.com/iam-mercy/kora-app/issues/114) | Verify responder existence before deletion in `remove_emergency_responder` | https://github.com/iam-mercy/kora-app/issues/114 | Published |
| #110 | [#115](https://github.com/iam-mercy/kora-app/issues/115) | Validate index bounds in `reorder_contact` to prevent VM panics | https://github.com/iam-mercy/kora-app/issues/115 | Published |
| #111 | [#116](https://github.com/iam-mercy/kora-app/issues/116) | Add pagination support to `get_emergency_access_logs` | https://github.com/iam-mercy/kora-app/issues/116 | Published |
| #112 | [#117](https://github.com/iam-mercy/kora-app/issues/117) | Standardize emergency notification rate limits to timestamp-based windows | https://github.com/iam-mercy/kora-app/issues/117 | Published |
| #113 | [#118](https://github.com/iam-mercy/kora-app/issues/118) | Prevent registering the same offspring pet ID to multiple litters | https://github.com/iam-mercy/kora-app/issues/118 | Published |
| #114 | [#119](https://github.com/iam-mercy/kora-app/issues/119) | Replace manipulable timestamp pseudo-randomness in `compute_offspring_traits` | https://github.com/iam-mercy/kora-app/issues/119 | Published |
| #115 | [#120](https://github.com/iam-mercy/kora-app/issues/120) | Cap cumulative basis points in `get_trait_probability` to 10,000 bps | https://github.com/iam-mercy/kora-app/issues/120 | Published |
| #116 | [#121](https://github.com/iam-mercy/kora-app/issues/121) | Verify active and non-archived status of breeding pairs in `register_breeding_pair` | https://github.com/iam-mercy/kora-app/issues/121 | Published |
| #117 | [#122](https://github.com/iam-mercy/kora-app/issues/122) | Enforce administrator authorization on `add_breed_metadata` | https://github.com/iam-mercy/kora-app/issues/122 | Published |
| #118 | [#123](https://github.com/iam-mercy/kora-app/issues/123) | Validate pet existence in `deactivatePet` and `reactivatePet` in `KoraRegistry.sol` | https://github.com/iam-mercy/kora-app/issues/123 | Published |
| #119 | [#124](https://github.com/iam-mercy/kora-app/issues/124) | Prevent orphan medical records by verifying pet existence in `addMedicalRecord` in `KoraRegistry.sol` | https://github.com/iam-mercy/kora-app/issues/124 | Published |
| #120 | [#125](https://github.com/iam-mercy/kora-app/issues/125) | Bound loop iterations in `getPetRecordsByDateRange` to prevent out-of-gas reverts | https://github.com/iam-mercy/kora-app/issues/125 | Published |
| #121 | [#126](https://github.com/iam-mercy/kora-app/issues/126) | Enforce maximum string length validation on license and specialization in `registerVet` in `KoraRegistry.sol` | https://github.com/iam-mercy/kora-app/issues/126 | Published |
| #122 | [#127](https://github.com/iam-mercy/kora-app/issues/127) | Prevent concurrent signing of duplicate adoption applications in `pet-transfer-adoption` | https://github.com/iam-mercy/kora-app/issues/127 | Published |
| #123 | [#128](https://github.com/iam-mercy/kora-app/issues/128) | Enforce administrator authentication in `waive_waiting_period` in `pet-transfer-adoption` | https://github.com/iam-mercy/kora-app/issues/128 | Published |
| #124 | [#129](https://github.com/iam-mercy/kora-app/issues/129) | Enforce non-zero timeout duration in `initiate_transfer_with_timeout` in `pet-transfer-adoption` | https://github.com/iam-mercy/kora-app/issues/129 | Published |
| #125 | [#130](https://github.com/iam-mercy/kora-app/issues/130) | Prevent reclaim of already accepted or finalized transfers in `reclaim_transfer` in `pet-transfer-adoption` | https://github.com/iam-mercy/kora-app/issues/130 | Published |

---

## #51: Prevent unauthorized callers from raising frivolous disputes against arbitrary pets

- **GitHub Issue**: [#56](https://github.com/iam-mercy/kora-app/issues/56)
- **Status**: `Published`

### Category
Access Control

### Priority
P1

### Problem
In `stellar-contracts/src/lib.rs` (lines 9528–9578), `raise_dispute` accepts `caller: Address` and invokes `caller.require_auth()`. However, it never validates that `caller` is the registered pet owner, a prior custodian in the custody chain, or an authorized party with a pending transfer. Any random address on the network can freeze pet custody by raising an unsolicited dispute.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 9528–9578):
```rust
pub fn raise_dispute(
    env: Env,
    pet_id: u64,
    caller: Address,
    reason: String,
) -> u64 {
    caller.require_auth();
    // Loads pet and creates dispute, but checks zero relationship between caller and pet!
```

### Why It Matters
A malicious actor can raise disputes across all registered pets, locking pet records and freezing legitimate ownership transfers without possessing any custodial relationship to the animals.

### Proposed Work
Validate that `caller` is either the current pet owner, a recipient with an active pending transfer, a verified veterinarian with care records, or an authorized emergency contact. Revert with `ContractError::Unauthorized` for unrelated callers.

### Acceptance Criteria
- [ ] Current pet owners and pending transfer recipients can raise disputes.
- [ ] Unrelated third parties attempting to raise disputes are rejected with `ContractError::Unauthorized`.
- [ ] Emits `DisputeRaisedEvent` only upon valid authorization.

### Testing Requirements
Unit test verifying that an unauthorized third party fails to raise a dispute, while the pet owner and pending transfer recipient succeed.

### Dependencies
Depends on #25

### Code References
- `stellar-contracts/src/lib.rs`: lines 9528–9578
- `stellar-contracts/src/test_disputes.rs`

---

## #52: Enforce registered arbitrator verification in `resolve_dispute`

- **GitHub Issue**: [#57](https://github.com/iam-mercy/kora-app/issues/57)
- **Status**: `Published`

### Category
Security

### Priority
P0

### Problem
In `stellar-contracts/src/lib.rs` (lines 9588–9620), `resolve_dispute` accepts `arbitrator: Address` and calls `arbitrator.require_auth()`. However, the function never validates that `arbitrator` matches the assigned arbitrator stored in `DataKey::Arbitrator(pet_id)` or contract admin. Any arbitrary caller can resolve disputes and reassign pet custody.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 9588–9620):
```rust
pub fn resolve_dispute(env: Env, dispute_id: u64, arbitrator: Address, resolution: DisputeResolution) -> bool {
    arbitrator.require_auth();
    // Reads dispute, mutates status to Resolved, but never compares arbitrator with assigned arbitrator!
```

### Why It Matters
This represents a critical custody theft vulnerability. An attacker can raise a dispute, immediately call `resolve_dispute` signing as themselves, and reassign ownership of any pet on the platform.

### Proposed Work
Query `DataKey::Arbitrator(pet_id)` (or administrative multisig) and assert `arbitrator == assigned_arbitrator`. Revert with `ContractError::Unauthorized` if mismatched.

### Acceptance Criteria
- [ ] Only the cryptographically designated arbitrator or contract admin can resolve disputes.
- [ ] Unauthorized addresses attempting resolution are rejected with `ContractError::Unauthorized`.
- [ ] Custody transition occurs strictly upon valid resolution.

### Testing Requirements
Test verifying that an unassigned address attempting to call `resolve_dispute` reverts with `ContractError::Unauthorized`.

### Dependencies
Depends on #51

### Code References
- `stellar-contracts/src/lib.rs`: lines 9588–9620
- `stellar-contracts/src/test_dispute_voting.rs`

---

## #53: Prevent duplicate voting and post-resolution voting in dispute ballots

- **GitHub Issue**: [#58](https://github.com/iam-mercy/kora-app/issues/58)
- **Status**: `Published`

### Category
Smart Contract

### Priority
P1

### Problem
In `stellar-contracts/src/lib.rs` (lines 9621–9720), `vote_on_dispute` allows voting on disputes. However, it fails to check if `dispute.status == DisputeStatus::Active`. If invoked after a dispute is resolved or cancelled, votes continue to be tallied. Furthermore, it does not check if the voter has already voted on this dispute.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 9621–9720):
```rust
pub fn vote_on_dispute(env: Env, dispute_id: u64, voter: Address, support: bool) -> bool {
    voter.require_auth();
    // Lacks check for dispute.status == Active
    // Lacks check for DataKey::DisputeVote((dispute_id, voter))
```

### Why It Matters
A single voter can repeatedly vote in a loop, artificially inflating vote tallies and overriding community or panel consensus. Voters can also vote on dead disputes.

### Proposed Work
Check `dispute.status == DisputeStatus::Active`. Record votes under `DataKey::DisputeVote((dispute_id, voter.clone()))` and assert that the voter has not already voted.

### Acceptance Criteria
- [ ] Re-voting by the same address reverts with `ContractError::AlreadyVoted`.
- [ ] Voting on resolved or cancelled disputes reverts with `ContractError::InvalidState`.
- [ ] Vote tallies accurately increment by 1 per unique voter.

### Testing Requirements
Unit tests verifying duplicate voting rejection and rejection of votes on closed disputes.

### Dependencies
Depends on #51

### Code References
- `stellar-contracts/src/lib.rs`: lines 9621–9720
- `stellar-contracts/src/test_dispute_voting.rs`

---

## #54: Validate evidence CID format and input length in `submit_evidence`

- **GitHub Issue**: [#59](https://github.com/iam-mercy/kora-app/issues/59)
- **Status**: `Published`

### Category
Data Integrity

### Priority
P2

### Problem
In `stellar-contracts/src/lib.rs` (lines 9768–9827), `submit_evidence` accepts `evidence_uri: String` and stores it into `Evidence` without checking maximum string length or validating IPFS/CID hash formatting. Arbitrarily large strings can be pushed into contract instance storage.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 9768–9827):
```rust
pub fn submit_evidence(env: Env, dispute_id: u64, submitter: Address, evidence_uri: String) -> u64 {
    submitter.require_auth();
    // Inserts evidence_uri directly without length or format validation
```

### Why It Matters
Unvalidated evidence URIs can be used to mount storage exhaustion attacks against contract instance storage, inflating state costs and degrading contract execution budget.

### Proposed Work
Enforce `Self::validate_len("evidence_uri", &evidence_uri, MAX_EVIDENCE_URI_LEN)`. Validate CID format if IPFS URI is provided.

### Acceptance Criteria
- [ ] Evidence URIs exceeding length bounds revert with `ContractError::InputTooLong`.
- [ ] Empty evidence strings revert with `ContractError::InvalidInput`.
- [ ] Valid CIDs are accepted and stored.

### Testing Requirements
Test submitting oversized evidence string and verify rejection.

### Dependencies
Dependencies: None

### Code References
- `stellar-contracts/src/lib.rs`: lines 9768–9827
- `stellar-contracts/src/test_disputes.rs`

---

## #55: Enforce lower and upper bounds on dispute appeal window duration

- **GitHub Issue**: [#60](https://github.com/iam-mercy/kora-app/issues/60)
- **Status**: `Published`

### Category
Smart Contract

### Priority
P2

### Problem
In `stellar-contracts/src/lib.rs` (lines 9505–9512), `set_appeal_window` allows an admin to set the dispute appeal duration without sanity bounds. Setting it to 0 renders appeals impossible; setting it to extreme values locks disputes permanently.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 9505–9512):
```rust
pub fn set_appeal_window(env: Env, admin: Address, seconds: u64) -> bool {
    admin.require_auth();
    Self::require_admin(&env, &admin);
    env.storage().instance().set(&DataKey::AppealWindow, &seconds);
    true
}
```

### Why It Matters
Accidental or malicious setting of `seconds = 0` deprives owners of procedural due process during custody disputes, while setting `seconds = u64::MAX` indefinitely paralyzes transfer finalization.

### Proposed Work
Enforce bounds: `require(seconds >= MIN_APPEAL_WINDOW_SECS && seconds <= MAX_APPEAL_WINDOW_SECS)`. Recommended: 2 days (172,800s) to 30 days (2,592,000s).

### Acceptance Criteria
- [ ] Appeal window values below 2 days or above 30 days revert with `ContractError::InvalidInput`.
- [ ] Legitimate appeal windows within bounds update successfully.

### Testing Requirements
Test setting window to 0 (fails), 100 years (fails), and 7 days (succeeds).

### Dependencies
Dependencies: None

### Code References
- `stellar-contracts/src/lib.rs`: lines 9505–9512
- `stellar-contracts/src/test_disputes.rs`

---

## #56: Implement evidence cryptographic signature verification in `verify_evidence`

- **GitHub Issue**: [#61](https://github.com/iam-mercy/kora-app/issues/61)
- **Status**: `Published`

### Category
Security

### Priority
P1

### Problem
In `stellar-contracts/src/lib.rs` (lines 9828–9836), `verify_evidence` is an empty stub returning unconditional `true`: `pub fn verify_evidence(...) -> bool { true }`. It performs no cryptographic validation or timestamp verification.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 9828–9836):
```rust
pub fn verify_evidence(env: Env, evidence_id: u64, verifier: Address) -> bool {
    // Empty stub
    true
}
```

### Why It Matters
Downstream dispute adjudication logic relies on `verify_evidence` to validate forensic documents and veterinary proofs. A stub returning `true` allows forged evidence to pass verification automatically.

### Proposed Work
Require `verifier.require_auth()`. Validate that `verifier` is an authorized arbitrator or notary. Record verification status and timestamp on the `Evidence` record.

### Acceptance Criteria
- [ ] `verify_evidence` requires valid authorization from an authorized verifier.
- [ ] Non-existent evidence IDs return `ContractError::NotFound`.
- [ ] Verification state is persisted and observable on-chain.

### Testing Requirements
Test: Non-arbitrator attempting verification fails; valid arbitrator succeeds and updates evidence record.

### Dependencies
Depends on #52

### Code References
- `stellar-contracts/src/lib.rs`: lines 9828–9836
- `stellar-contracts/src/test_disputes.rs`

---

## #57: Pause pet transfer expiration timers during active custody disputes in `pet-transfer-adoption`

- **GitHub Issue**: [#62](https://github.com/iam-mercy/kora-app/issues/62)
- **Status**: `Published`

### Category
Smart Contract

### Priority
P1

### Problem
In `stellar-contracts/contracts/pet-transfer-adoption/src/lib.rs` (lines 1200–1235), `raise_dispute` transitions a transfer status to `Disputed`. However, `cancel_expired_transfer` (line 798) and `reclaim_transfer` (line 1279) inspect only `ledger().timestamp() > transfer.timeout` without checking if a dispute is active.

### Current Implementation
In `stellar-contracts/contracts/pet-transfer-adoption/src/lib.rs`:
- Line 798: `cancel_expired_transfer` cancels any transfer whose timeout passed.
- Line 1279: `reclaim_transfer` allows previous owner to claw back pet if timeout expired, ignoring active dispute state.

### Why It Matters
An owner facing an adverse dispute ruling can stall arbitration until `timeout` expires, then call `reclaim_transfer` to bypass the dispute process entirely.

### Proposed Work
In `cancel_expired_transfer` and `reclaim_transfer`, assert `transfer.status != TransferStatus::Disputed`. Extend or freeze timeouts while dispute status is `Disputed`.

### Acceptance Criteria
- [ ] Disputed transfers cannot be expired or reclaimed while arbitration is active.
- [ ] Attempting to reclaim a disputed transfer reverts with `ContractError::TransferDisputed`.

### Testing Requirements
Unit test in `pet-transfer-adoption` verifying that `reclaim_transfer` fails when transfer status is `Disputed`.

### Dependencies
Depends on #6, #7

### Code References
- `stellar-contracts/contracts/pet-transfer-adoption/src/lib.rs`: lines 798–820, 1200–1235, 1279–1307

---

## #58: Replace linear scan in `get_pet_disputes` with indexed dispute lookup

- **GitHub Issue**: [#63](https://github.com/iam-mercy/kora-app/issues/63)
- **Status**: `Published`

### Category
Performance

### Priority
P2

### Problem
In `stellar-contracts/src/lib.rs` (lines 9746–9765), `get_pet_disputes` executes a full linear scan from `1..=dispute_count`, reading every dispute from instance storage to filter by `pet_id`.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 9746–9765):
```rust
for id in 1..=total {
    if let Some(d) = env.storage().instance().get(&DataKey::Dispute(id)) {
        if d.pet_id == pet_id { result.push_back(d); }
    }
}
```

### Why It Matters
As the global dispute count grows, querying disputes for a pet requires reading hundreds of keys into VM memory, exceeding Soroban instruction limits and bricking the query.

### Proposed Work
Introduce secondary index `DataKey::PetDisputes(pet_id) -> Vec<u64>`. Append dispute IDs to this vector upon creation, enabling $O(1)$ lookups.

### Acceptance Criteria
- [ ] `get_pet_disputes` fetches only disputes associated with the pet via secondary index.
- [ ] Query instruction cost remains constant regardless of total global dispute count.

### Testing Requirements
Benchmark test querying pet disputes with 100 global disputes, verifying minimal instruction consumption.

### Dependencies
Depends on #9

### Code References
- `stellar-contracts/src/lib.rs`: lines 9746–9765
- `stellar-contracts/src/test_disputes.rs`

---

## #59: Validate positive interval days in `create_grooming_schedule`

- **GitHub Issue**: [#64](https://github.com/iam-mercy/kora-app/issues/64)
- **Status**: `Published`

### Category
Smart Contract

### Priority
P2

### Problem
In `stellar-contracts/src/lib.rs` (lines 9863–9977), `create_grooming_schedule` accepts `interval_days: u32`. If passed `0`, the schedule creates recurring reminders that never advance in time, corrupting subsequent schedule queries.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 9863–9977):
```rust
pub fn create_grooming_schedule(..., interval_days: u32, ...) -> u64 {
    // Lacks check for interval_days > 0
    let next_due = now + (interval_days as u64 * 86400);
```

### Why It Matters
Zero interval days cause `next_due` to equal `created_at`, resulting in permanent overdue status and breaking client notification systems.

### Proposed Work
Enforce `if interval_days == 0 || interval_days > 365 { panic_with_error!(&env, ContractError::InvalidInput); }`.

### Acceptance Criteria
- [ ] Passing `interval_days == 0` reverts with `ContractError::InvalidInput`.
- [ ] Intervals between 1 and 365 days are accepted.

### Testing Requirements
Test creating schedule with 0 interval days (reverts) and 14 interval days (succeeds).

### Dependencies
Dependencies: None

### Code References
- `stellar-contracts/src/lib.rs`: lines 9863–9977
- `stellar-contracts/src/test_grooming.rs`

---

## #60: Enforce admin verification on professional groomer registration

- **GitHub Issue**: [#65](https://github.com/iam-mercy/kora-app/issues/65)
- **Status**: `Published`

### Category
Access Control

### Priority
P1

### Problem
In `stellar-contracts/src/lib.rs` (lines 10073–10103), `register_groomer` allows any caller to self-register as an approved groomer with arbitrary license strings and business names without admin approval or verification.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 10073–10103):
```rust
pub fn register_groomer(env: Env, groomer: Address, business_name: String, license: String) -> bool {
    groomer.require_auth();
    // Directly sets verified = true or registers without admin sign-off
```

### Why It Matters
Unverified entities can pose as licensed professional pet groomers, accepting bookings and pet custody on the platform without vetting.

### Proposed Work
Register groomers with `verified: false`. Implement `verify_groomer(env: Env, admin: Address, groomer: Address)` requiring contract admin authorization.

### Acceptance Criteria
- [ ] Newly registered groomers default to unverified.
- [ ] Only contract admins can verify groomer licenses.
- [ ] Unverified groomers cannot accept booked grooming slots.

### Testing Requirements
Test: Groomer registers -> verify unverified -> admin verifies -> verify verified.

### Dependencies
Dependencies: None

### Code References
- `stellar-contracts/src/lib.rs`: lines 10073–10103
- `stellar-contracts/src/test_grooming.rs`

---

## #61: Restrict groomer rating to verified appointment clients and enforce 1-5 star scale

- **GitHub Issue**: [#66](https://github.com/iam-mercy/kora-app/issues/66)
- **Status**: `Published`

### Category
Data Integrity

### Priority
P2

### Problem
In `stellar-contracts/src/lib.rs` (lines 10104–10146), `rate_groomer` accepts `rating: u32` without checking `rating >= 1 && rating <= 5` (permits 0 or 1,000,000), and does not verify whether `reviewer` ever booked a grooming slot with the groomer.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 10104–10146):
```rust
pub fn rate_groomer(env: Env, reviewer: Address, groomer: Address, rating: u32, review: String) -> bool {
    reviewer.require_auth();
    // Lacks rating bounds check (rating <= 5)
    // Lacks verification of previous completed booking
```

### Why It Matters
Groomers can be griefed with zero-star spam or boosted with inflated ratings by arbitrary addresses that never utilized the service.

### Proposed Work
Enforce `if rating < 1 || rating > 5 { panic_with_error!(&env, ContractError::InvalidInput); }`. Assert that `reviewer` has at least one completed booking with `groomer`.

### Acceptance Criteria
- [ ] Ratings must be between 1 and 5 inclusive.
- [ ] Reviewers must have a verified booking record with the groomer.
- [ ] Cumulative groomer score correctly reflects authentic client reviews.

### Testing Requirements
Unit test verifying rejection of out-of-bounds ratings and rejection of reviewers without booking history.

### Dependencies
Dependencies: None

### Code References
- `stellar-contracts/src/lib.rs`: lines 10104–10146
- `stellar-contracts/src/test_grooming.rs`

---

## #62: Prevent duplicate slot booking at identical timestamps for groomers

- **GitHub Issue**: [#67](https://github.com/iam-mercy/kora-app/issues/67)
- **Status**: `Published`

### Category
Smart Contract

### Priority
P1

### Problem
In `stellar-contracts/src/lib.rs` (lines 10159–10237), `book_grooming_slot` creates a booking record without checking if a slot for `(groomer, slot_time)` has already been confirmed for another pet.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 10159–10237):
```rust
pub fn book_grooming_slot(env: Env, owner: Address, groomer: Address, pet_id: u64, slot_time: u64, service: GroomingService) -> u64 {
    owner.require_auth();
    // Increments booking count and sets booking, but NEVER checks DataKey::GroomerSlot((groomer, slot_time))!
```

### Why It Matters
Groomers can be double-booked for the exact same hour across multiple pets, leading to scheduling conflicts and broken service delivery.

### Proposed Work
Check storage key `DataKey::GroomerSlot((groomer.clone(), slot_time))`. If already occupied, revert with `ContractError::SlotAlreadyBooked`. Mark slot as occupied upon successful booking.

### Acceptance Criteria
- [ ] Booking an occupied slot time reverts with `ContractError::SlotAlreadyBooked`.
- [ ] Distinct slot times for the same groomer book successfully.

### Testing Requirements
Test attempting to book the same groomer slot twice with two different pet IDs.

### Dependencies
Dependencies: None

### Code References
- `stellar-contracts/src/lib.rs`: lines 10159–10237
- `stellar-contracts/src/test_book_slot.rs`

---

## #63: Enforce pet owner authorization on `cancel_grooming_schedule`

- **GitHub Issue**: [#68](https://github.com/iam-mercy/kora-app/issues/68)
- **Status**: `Published`

### Category
Access Control

### Priority
P1

### Problem
In `stellar-contracts/src/lib.rs` (lines 10044–10072), `cancel_grooming_schedule` accepts `schedule_id: u64` and caller address, but does not verify that `caller == pet.owner`.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 10044–10072):
```rust
pub fn cancel_grooming_schedule(env: Env, schedule_id: u64, caller: Address) -> bool {
    caller.require_auth();
    // Loads schedule, sets active = false, but fails to check caller == pet.owner!
```

### Why It Matters
Any authenticated caller on the network can cancel active grooming schedules for any pet, disrupting ongoing pet care plans.

### Proposed Work
Load `schedule.pet_id`, query the registered pet owner, and assert `caller == pet.owner`. Revert with `ContractError::Unauthorized` for non-owners.

### Acceptance Criteria
- [ ] Only the verified pet owner can cancel a grooming schedule.
- [ ] Unauthorized callers are rejected with `ContractError::Unauthorized`.

### Testing Requirements
Test: Non-owner calls `cancel_grooming_schedule` (fails); pet owner calls it (succeeds).

### Dependencies
Dependencies: None

### Code References
- `stellar-contracts/src/lib.rs`: lines 10044–10072
- `stellar-contracts/src/test_grooming.rs`

---

## #64: Enforce groomer caller authentication in `advance_schedule`

- **GitHub Issue**: [#69](https://github.com/iam-mercy/kora-app/issues/69)
- **Status**: `Published`

### Category
Access Control

### Priority
P1

### Problem
In `stellar-contracts/src/lib.rs` (lines 9978–10043), `advance_schedule` marks grooming appointments as completed without authenticating the registered groomer or pet owner.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 9978–10043):
```rust
pub fn advance_schedule(env: Env, schedule_id: u64) -> bool {
    // Takes NO caller address and calls NO require_auth()!
```

### Why It Matters
Unauthenticated third parties can repeatedly advance grooming schedules into future dates without any grooming service having taken place.

### Proposed Work
Add `caller: Address` argument with `caller.require_auth()`. Assert that `caller == schedule.groomer || caller == pet.owner`.

### Acceptance Criteria
- [ ] `advance_schedule` requires cryptographic authorization from the groomer or pet owner.
- [ ] Random callers cannot advance pet grooming appointments.

### Testing Requirements
Test verifying that unauthenticated or non-groomer invocations revert.

### Dependencies
Dependencies: None

### Code References
- `stellar-contracts/src/lib.rs`: lines 9978–10043
- `stellar-contracts/src/test_grooming.rs`

---

## #65: Validate severity bounds and caller role in `add_behavior_record`

- **GitHub Issue**: [#70](https://github.com/iam-mercy/kora-app/issues/70)
- **Status**: `Published`

### Category
Data Integrity

### Priority
P2

### Problem
In `stellar-contracts/src/lib.rs` (lines 4469–4524), `add_behavior_record` accepts `severity: u32` without bounds checking (allows arbitrary integers instead of 1–5 scale) and does not verify caller authorization.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 4469–4524):
```rust
pub fn add_behavior_record(..., severity: u32, ...) -> u64 {
    // Direct storage write without checking severity <= 5
    // No caller authorization check
```

### Why It Matters
Arbitrary severity values distort clinical behavioral analytics. Lack of caller check allows spoofing aggression or behavioral problems on other users' pets.

### Proposed Work
Require caller authorization (`owner` or `certified_trainer`). Assert `severity >= 1 && severity <= 5`. Revert with `ContractError::InvalidInput` if out of bounds.

### Acceptance Criteria
- [ ] Severity values outside 1..=5 are rejected.
- [ ] Only authorized owners and certified trainers can log behavior records.

### Testing Requirements
Unit test asserting rejection of severity = 0 or severity = 10.

### Dependencies
Dependencies: None

### Code References
- `stellar-contracts/src/lib.rs`: lines 4469–4524
- `stellar-contracts/src/test_behavior.rs`

---

## #66: Add pagination support to `get_behavior_history`

- **GitHub Issue**: [#71](https://github.com/iam-mercy/kora-app/issues/71)
- **Status**: `Published`

### Category
Performance

### Priority
P2

### Problem
In `stellar-contracts/src/lib.rs` (lines 4541–4568), `get_behavior_history` loads every behavior record for a pet into a single `Vec<BehaviorRecord>`, exceeding Soroban transaction return size limits for pets with active training logs.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 4541–4568):
```rust
pub fn get_behavior_history(env: Env, pet_id: u64) -> Vec<BehaviorRecord> {
    // Loads all 1..=count records into memory
```

### Why It Matters
Pets with dozens of training milestones and behavioral observations will fail to return records once the serialized payload exceeds Soroban VM limits.

### Proposed Work
Implement `get_behavior_history_paginated(env: Env, pet_id: u64, offset: u64, limit: u32) -> Vec<BehaviorRecord>`. Cap `limit <= 50`.

### Acceptance Criteria
- [ ] Behavior history queries accept `offset` and `limit`.
- [ ] Queries return exact slices without memory exhaustion.

### Testing Requirements
Test querying paginated behavior records across multiple pages.

### Dependencies
Dependencies: None

### Code References
- `stellar-contracts/src/lib.rs`: lines 4541–4568
- `stellar-contracts/src/test_behavior_records.rs`

---

## #67: Fix UTC midnight boundary calculation in `get_activity_streak`

- **GitHub Issue**: [#72](https://github.com/iam-mercy/kora-app/issues/72)
- **Status**: `Published`

### Category
Smart Contract

### Priority
P2

### Problem
In `stellar-contracts/src/lib.rs` (lines 10920–10937), `get_activity_streak` divides timestamps by 86,400 to determine day indices, but fails to account for consecutive day rollover when activities occur 25 hours apart across calendar days.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 10920–10937):
```rust
let day_index = entry.timestamp / 86400;
if day_index == prev_day - 1 { streak += 1; }
```

### Why It Matters
Users logging activities at 11:00 PM on Day 1 and 7:00 AM on Day 3 have an 32-hour gap but forfeit their streak, while activities logged 47 hours apart can erroneously maintain a streak.

### Proposed Work
Implement standardized day difference calculation: `let days_diff = current_day.saturating_sub(prev_day)`. Maintain streak only if `days_diff == 1`.

### Acceptance Criteria
- [ ] Streaks increment only on consecutive UTC days.
- [ ] Breaks in activity (>1 day gap) correctly reset streak to 1.

### Testing Requirements
Test activities logged across midnight boundaries and multiple calendar days.

### Dependencies
Dependencies: None

### Code References
- `stellar-contracts/src/lib.rs`: lines 10920–10937
- `stellar-contracts/src/test_activity.rs`

---

## #68: Fix milestone threshold comparison in `has_reached_milestone`

- **GitHub Issue**: [#73](https://github.com/iam-mercy/kora-app/issues/73)
- **Status**: `Published`

### Category
Smart Contract

### Priority
P2

### Problem
In `stellar-contracts/src/lib.rs` (lines 10938–10954), `has_reached_milestone` checks `current_val == milestone.target_value`. If an activity entry jumps past the target (e.g. from 95 km to 105 km when target is 100 km), milestone achievement is missed permanently.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 10938–10954):
```rust
pub fn has_reached_milestone(..., current_val: u32, target_val: u32) -> bool {
    current_val == target_val // Strict equality bug!
}
```

### Why It Matters
Pets with activities that overshoot milestone boundaries never receive on-chain milestone achievements or insurance rewards.

### Proposed Work
Change comparison to `current_val >= target_val`.

### Acceptance Criteria
- [ ] Cumulative values greater than or equal to target return `true`.
- [ ] Values strictly less than target return `false`.

### Testing Requirements
Test passing `current_val = 105, target_val = 100` and assert `true`.

### Dependencies
Dependencies: None

### Code References
- `stellar-contracts/src/lib.rs`: lines 10938–10954
- `stellar-contracts/src/test_activity.rs`

---

## #69: Enforce storage quota tracking on `add_behavior_record`

- **GitHub Issue**: [#74](https://github.com/iam-mercy/kora-app/issues/74)
- **Status**: `Published`

### Category
Data Integrity

### Priority
P2

### Problem
In `stellar-contracts/src/lib.rs` (lines 4469–4524), `add_behavior_record` allocates storage for notes and logs but fails to call `Self::increment_pet_storage(&env, pet_id)`.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 4469–4524):
```rust
// Stores BehaviorRecord without quota tracking
env.storage().instance().set(&BehaviorKey::BehaviorRecord(record_id), &record);
```

### Why It Matters
Malicious owners can circumvent per-pet storage quota caps by storing massive notes within behavior records instead of medical records.

### Proposed Work
Call `Self::increment_pet_storage(&env, pet_id)` within `add_behavior_record` before persisting to storage.

### Acceptance Criteria
- [ ] Adding a behavior record consumes 1 storage quota unit.
- [ ] Reaching storage quota prevents adding further behavior records.

### Testing Requirements
Test storage quota exhaustion blocking `add_behavior_record`.

### Dependencies
Dependencies: Depends on #10

### Code References
- `stellar-contracts/src/lib.rs`: lines 4469–4524
- `stellar-contracts/src/test_storage_quota.rs`

---

## #70: Reject future timestamps in `log_feeding`

- **GitHub Issue**: [#75](https://github.com/iam-mercy/kora-app/issues/75)
- **Status**: `Published`

### Category
Data Integrity

### Priority
P2

### Problem
In `stellar-contracts/src/lib.rs` (lines 7500–7578), `log_feeding` accepts arbitrary `timestamp: u64` without checking against `env.ledger().timestamp()`, allowing logging of feeding events years in the future.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 7500–7578):
```rust
pub fn log_feeding(..., timestamp: u64, ...) -> u64 {
    // Lacks check: timestamp <= env.ledger().timestamp()
```

### Why It Matters
Future feeding logs corrupt daily nutrition summaries, weight gain projections, and automated feeder integration schedules.

### Proposed Work
Assert `timestamp <= env.ledger().timestamp() + CLOCK_SKEW_TOLERANCE`. Revert with `ContractError::InvalidInput` for future timestamps.

### Acceptance Criteria
- [ ] Feeding timestamps in the future revert with `ContractError::InvalidInput`.
- [ ] Historical and current timestamps are accepted.

### Testing Requirements
Test logging feeding with timestamp $T + 86400$ and verify rejection.

### Dependencies
Dependencies: None

### Code References
- `stellar-contracts/src/lib.rs`: lines 7500–7578
- `stellar-contracts/src/test_nutrition.rs`

---

## #71: Enforce positive weight and realistic upper bounds in `add_weight_entry`

- **GitHub Issue**: [#76](https://github.com/iam-mercy/kora-app/issues/76)
- **Status**: `Published`

### Category
Data Integrity

### Priority
P2

### Problem
In `stellar-contracts/src/lib.rs` (lines 7615–7671), `add_weight_entry` accepts `weight: u32` without checking `weight > 0` (permits 0g) or upper bounds (permits 4,000,000,000g).

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 7615–7671):
```rust
pub fn add_weight_entry(env: Env, pet_id: u64, weight: u32, ...) -> u64 {
    // Directly sets WeightEntry with unvalidated weight
```

### Why It Matters
Zero or astronomically high weights corrupt pet dosage calculators in veterinary clinics and break health tracking graphs.

### Proposed Work
Enforce `if weight == 0 || weight > MAX_PET_WEIGHT_GRAMS { panic_with_error!(&env, ContractError::InvalidInput); }` (e.g. max 500,000g / 500kg).

### Acceptance Criteria
- [ ] Weight entries with 0g revert with `ContractError::InvalidInput`.
- [ ] Weight entries exceeding 500kg revert with `ContractError::InvalidInput`.
- [ ] Valid weights store accurately.

### Testing Requirements
Test logging weight of 0 and 1,000,000,000g; assert both revert.

### Dependencies
Dependencies: None

### Code References
- `stellar-contracts/src/lib.rs`: lines 7615–7671
- `stellar-contracts/src/test_nutrition.rs`

---

## #72: Implement pagination in `get_weight_history`

- **GitHub Issue**: [#77](https://github.com/iam-mercy/kora-app/issues/77)
- **Status**: `Published`

### Category
Performance

### Priority
P2

### Problem
In `stellar-contracts/src/lib.rs` (lines 7672–7706), `get_weight_history` returns all weight entries in a single vector without `offset` or `limit` parameters.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 7672–7706):
```rust
pub fn get_weight_history(env: Env, pet_id: u64) -> Vec<WeightEntry> {
    // Unbounded vector allocation
```

### Why It Matters
Long-lived pets with weekly weigh-ins over a decade accumulate hundreds of entries, eventually exceeding Soroban return data limits.

### Proposed Work
Add `get_weight_history_paginated(env: Env, pet_id: u64, offset: u64, limit: u32) -> Vec<WeightEntry>`.

### Acceptance Criteria
- [ ] Weight history supports pagination with bounded limits.
- [ ] Memory footprint is bounded per invocation.

### Testing Requirements
Test paginated retrieval of 50 weight entries.

### Dependencies
Dependencies: None

### Code References
- `stellar-contracts/src/lib.rs`: lines 7672–7706
- `stellar-contracts/src/test_nutrition.rs`

---

## #73: Enforce pet owner authorization on `set_nutrition_version`

- **GitHub Issue**: [#78](https://github.com/iam-mercy/kora-app/issues/78)
- **Status**: `Published`

### Category
Access Control

### Priority
P1

### Problem
In `stellar-contracts/src/lib.rs` (lines 7799–7883), `set_nutrition_version` increments and writes nutrition versions without verifying `caller == pet.owner`.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 7799–7883):
```rust
pub fn set_nutrition_version(..., pet_id: u64, ...) -> u64 {
    // Does not call pet.owner.require_auth()
```

### Why It Matters
Unauthorized callers can tamper with a pet's prescribed diet plan, altering nutritional targets and allergy warnings.

### Proposed Work
Assert `pet.owner.require_auth()`. Revert unauthorized calls with `ContractError::Unauthorized`.

### Acceptance Criteria
- [ ] Only the registered pet owner can update nutrition versions.
- [ ] Unauthorized callers are rejected.

### Testing Requirements
Test non-owner attempting `set_nutrition_version` reverts.

### Dependencies
Dependencies: None

### Code References
- `stellar-contracts/src/lib.rs`: lines 7799–7883
- `stellar-contracts/src/test_nutrition_plan.rs`

---

## #74: Prevent rollback to invalid or non-existent versions in `rollback_nutrition`

- **GitHub Issue**: [#79](https://github.com/iam-mercy/kora-app/issues/79)
- **Status**: `Published`

### Category
Smart Contract

### Priority
P2

### Problem
In `stellar-contracts/src/lib.rs` (lines 7938–8023), `rollback_nutrition` accepts `target_version: u64` and fails to check that `target_version > 0 && target_version < current_version`.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 7938–8023):
```rust
pub fn rollback_nutrition(env: Env, pet_id: u64, target_version: u64) -> u64 {
    // Lacks check that target_version exists in history and is older than current
```

### Why It Matters
Rolling back to non-existent or forward versions creates orphaned nutritional state and panics when loading ingredients.

### Proposed Work
Verify `target_version > 0 && target_version < current_version && has_version(target_version)`. Revert with `ContractError::InvalidInput` otherwise.

### Acceptance Criteria
- [ ] Rolling back to version 0 or future version reverts.
- [ ] Valid rollbacks restore historical plan properties accurately.

### Testing Requirements
Test rollback to invalid version numbers.

### Dependencies
Dependencies: None

### Code References
- `stellar-contracts/src/lib.rs`: lines 7938–8023
- `stellar-contracts/src/test_nutrition_plan.rs`

---

## #75: Prevent integer overflow in `get_daily_summary` calorie accumulation

- **GitHub Issue**: [#80](https://github.com/iam-mercy/kora-app/issues/80)
- **Status**: `Published`

### Category
Data Integrity

### Priority
P2

### Problem
In `stellar-contracts/src/lib.rs` (lines 7579–7607), `get_daily_summary` sums calories using standard integer addition (`sum + entry.calories`) rather than `saturating_add` or `checked_add`.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 7579–7607):
```rust
for entry in entries.iter() {
    total_calories += entry.calories; // Unchecked arithmetic!
}
```

### Why It Matters
If malicious or corrupted entries contain large calorie values, `get_daily_summary` panics in debug or wraps in release, crashing the summary query.

### Proposed Work
Use `total_calories = total_calories.saturating_add(entry.calories)`.

### Acceptance Criteria
- [ ] Calorie summation uses saturating arithmetic.
- [ ] Daily summary query never panics on large inputs.

### Testing Requirements
Test summary calculation with extreme calorie inputs.

### Dependencies
Dependencies: None

### Code References
- `stellar-contracts/src/lib.rs`: lines 7579–7607
- `stellar-contracts/src/test_nutrition.rs`

---

## #76: Enforce pet owner authorization on `link_tag_to_pet`

- **GitHub Issue**: [#81](https://github.com/iam-mercy/kora-app/issues/81)
- **Status**: `Published`

### Category
Access Control

### Priority
P1

### Problem
In `stellar-contracts/src/lib.rs` (lines 8072–8132), `link_tag_to_pet` links a hardware tag ID to a pet without calling `pet.owner.require_auth()`. Any user can link tags to pets they do not own.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 8072–8132):
```rust
pub fn link_tag_to_pet(env: Env, pet_id: u64, tag_id: String, ...) -> bool {
    // Loads pet, but does NOT verify caller is pet.owner!
```

### Why It Matters
An attacker can maliciously link arbitrary NFC or QR tags to someone else's pet, intercepting rescue communications or falsifying pet recovery messages.

### Proposed Work
Require `caller: Address` argument with `caller.require_auth()`. Assert `caller == pet.owner`.

### Acceptance Criteria
- [ ] Only the verified pet owner can link a physical tag to their pet.
- [ ] Unauthorized callers are rejected with `ContractError::Unauthorized`.

### Testing Requirements
Test: Owner links tag (succeeds); non-owner attempts linking (fails).

### Dependencies
Dependencies: None

### Code References
- `stellar-contracts/src/lib.rs`: lines 8072–8132

---

## #77: Enforce string length validation on `update_tag_message`

- **GitHub Issue**: [#82](https://github.com/iam-mercy/kora-app/issues/82)
- **Status**: `Published`

### Category
Data Integrity

### Priority
P2

### Problem
In `stellar-contracts/src/lib.rs` (lines 8156–8178), `update_tag_message` updates `tag.recovery_message` without validating length against `MAX_TAG_MESSAGE_LEN`.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 8156–8178):
```rust
pub fn update_tag_message(env: Env, tag_id: String, new_message: String) -> bool {
    // Lacks length check on new_message
```

### Why It Matters
Unbounded tag messages can bloat contract storage and cause out-of-gas errors when scanning tags on mobile dApps with low bandwidth.

### Proposed Work
Enforce `Self::validate_len("new_message", &new_message, MAX_TAG_MESSAGE_LEN)`. Recommended: 256 characters max.

### Acceptance Criteria
- [ ] Messages exceeding length limit revert with `ContractError::InputTooLong`.
- [ ] Valid messages update successfully.

### Testing Requirements
Test updating tag with 500-character message and verify rejection.

### Dependencies
Dependencies: None

### Code References
- `stellar-contracts/src/lib.rs`: lines 8156–8178

---

## #78: Enforce caller authentication on `deactivate_tag` and `reactivate_tag`

- **GitHub Issue**: [#83](https://github.com/iam-mercy/kora-app/issues/83)
- **Status**: `Published`

### Category
Access Control

### Priority
P1

### Problem
In `stellar-contracts/src/lib.rs` (lines 8179–8246), `deactivate_tag` and `reactivate_tag` update tag status in storage without checking caller authorization.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 8179–8246):
```rust
pub fn deactivate_tag(env: Env, tag_id: String) -> bool {
    // No caller parameter, no require_auth()
```

### Why It Matters
Anyone can deactivate physical pet recovery tags, preventing finders of lost pets from scanning the tag and contacting the owner.

### Proposed Work
Require caller address and verify `caller == pet.owner`. Revert with `ContractError::Unauthorized` otherwise.

### Acceptance Criteria
- [ ] Deactivating or reactivating a tag requires pet owner authorization.
- [ ] Unauthorized attempts revert.

### Testing Requirements
Test deactivating tag without owner auth fails.

### Dependencies
Dependencies: None

### Code References
- `stellar-contracts/src/lib.rs`: lines 8179–8246

---

## #79: Unlink active tag from previous pet before re-linking in `link_tag_to_pet`

- **GitHub Issue**: [#84](https://github.com/iam-mercy/kora-app/issues/84)
- **Status**: `Published`

### Category
Data Integrity

### Priority
P1

### Problem
In `stellar-contracts/src/lib.rs` (lines 8072–8132), `link_tag_to_pet` allows assigning an existing tag ID to a new pet without deleting the reverse mapping `DataKey::TagByPet(old_pet_id)`.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 8072–8132):
```rust
// Sets new pet tag, but leaves old pet tag mapping intact!
env.storage().instance().set(&DataKey::Tag(tag_id), &tag);
env.storage().instance().set(&DataKey::TagByPet(pet_id), &tag_id);
```

### Why It Matters
Both pets will point to the same tag, but reverse lookups will return conflicting or corrupted pet associations.

### Proposed Work
If `tag.pet_id != 0`, remove `DataKey::TagByPet(old_pet_id)` before writing new link.

### Acceptance Criteria
- [ ] Re-linking a tag cleanly disassociates it from the previous pet.
- [ ] Reverse lookup mappings remain 1-to-1 unique.

### Testing Requirements
Test re-linking tag from Pet A to Pet B; verify Pet A no longer has linked tag.

### Dependencies
Dependencies: None

### Code References
- `stellar-contracts/src/lib.rs`: lines 8072–8132

---

## #80: Distinguish non-existent tags from deactivated tags in `is_tag_active`

- **GitHub Issue**: [#85](https://github.com/iam-mercy/kora-app/issues/85)
- **Status**: `Published`

### Category
Smart Contract

### Priority
P3

### Problem
In `stellar-contracts/src/lib.rs` (lines 8247–8279), `is_tag_active` returns `false` whether the tag exists and is inactive, or does not exist at all.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 8247–8279):
```rust
pub fn is_tag_active(env: Env, tag_id: String) -> bool {
    env.storage().instance().get(&DataKey::Tag(tag_id)).map(|t: Tag| t.active).unwrap_or(false)
}
```

### Why It Matters
External applications scanning a physical tag cannot distinguish between an unactivated/new tag and a deactivated tag.

### Proposed Work
Change signature to return `Result<bool, ContractError>` or `Option<bool>` to differentiate between unknown tags and inactive tags.

### Acceptance Criteria
- [ ] Non-existent tag lookups return `None` or error.
- [ ] Known tags return `Some(active_bool)`.

### Testing Requirements
Test tag query on non-existent tag returns error.

### Dependencies
Dependencies: None

### Code References
- `stellar-contracts/src/lib.rs`: lines 8247–8279

---

## #81: Enforce expiration date strictly greater than administration date in `add_vaccination`

- **GitHub Issue**: [#86](https://github.com/iam-mercy/kora-app/issues/86)
- **Status**: `Published`

### Category
Data Integrity

### Priority
P1

### Problem
In `stellar-contracts/src/lib.rs` (lines 6536–6673), `add_vaccination` does not assert `expiration_date > administration_date`, permitting records where expiration date precedes administration date.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 6536–6673):
```rust
pub fn add_vaccination(..., admin_date: u64, expiration_date: u64, ...) -> u64 {
    // Lacks check: expiration_date > admin_date
```

### Why It Matters
Retroactive or inverted expiration dates corrupt vaccination certificates, bypass health score algorithms, and trigger false overdue alerts.

### Proposed Work
Assert `if expiration_date <= admin_date { panic_with_error!(&env, ContractError::InvalidInput); }`.

### Acceptance Criteria
- [ ] Vaccination records with `expiration_date <= admin_date` revert with `ContractError::InvalidInput`.
- [ ] Valid chronological vaccination entries succeed.

### Testing Requirements
Test submitting vaccination where expiration is before administration date.

### Dependencies
Dependencies: None

### Code References
- `stellar-contracts/src/lib.rs`: lines 6536–6673
- `stellar-contracts/src/test_vaccination_expiry.rs`

---

## #82: Enforce administering vet or admin authorization on `revoke_vaccination_certificate`

- **GitHub Issue**: [#87](https://github.com/iam-mercy/kora-app/issues/87)
- **Status**: `Published`

### Category
Access Control

### Priority
P1

### Problem
In `stellar-contracts/src/lib.rs` (lines 6674–6761), `revoke_vaccination_certificate` takes a caller address but does not verify that `caller` is the administering veterinarian or contract admin.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 6674–6761):
```rust
pub fn revoke_vaccination_certificate(env: Env, cert_id: u64, caller: Address, reason: String) -> bool {
    caller.require_auth();
    // Fails to check caller == cert.administering_vet || is_admin(caller)
```

### Why It Matters
Any random user can revoke valid rabies or distemper vaccination certificates for another pet, causing border transit or boarding denial for the pet.

### Proposed Work
Assert `caller == cert.administering_vet || Self::is_admin(&env, &caller)`. Revert with `ContractError::Unauthorized` otherwise.

### Acceptance Criteria
- [ ] Only the administering veterinarian or contract administrator can revoke a certificate.
- [ ] Unauthorized revocation attempts revert.

### Testing Requirements
Test revocation by unauthorized caller fails.

### Dependencies
Dependencies: None

### Code References
- `stellar-contracts/src/lib.rs`: lines 6674–6761
- `stellar-contracts/src/test_vaccination_certificate.rs`

---

## #83: Prevent duplicate certificate hash anchoring in `anchor_certificate`

- **GitHub Issue**: [#88](https://github.com/iam-mercy/kora-app/issues/88)
- **Status**: `Published`

### Category
Data Integrity

### Priority
P2

### Problem
In `stellar-contracts/src/lib.rs` (lines 7245–7330), `anchor_certificate` overwrites existing certificate anchor timestamps without checking if `DataKey::CertificateAnchor(hash)` already exists.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 7245–7330):
```rust
pub fn anchor_certificate(env: Env, cert_hash: BytesN<32>) -> bool {
    // Directly sets anchor without checking if already anchored!
    env.storage().instance().set(&DataKey::CertificateAnchor(cert_hash), &now);
```

### Why It Matters
Re-anchoring updates the original timestamp, corrupting the immutable timestamp proof required for legal and travel certificates.

### Proposed Work
Check if `CertificateAnchor(cert_hash)` exists. If present, revert with `ContractError::CertificateAlreadyAnchored`.

### Acceptance Criteria
- [ ] Anchoring an existing certificate hash reverts.
- [ ] Initial anchoring sets immutable timestamp.

### Testing Requirements
Test anchoring certificate twice; verify second attempt reverts.

### Dependencies
Dependencies: None

### Code References
- `stellar-contracts/src/lib.rs`: lines 7245–7330
- `stellar-contracts/src/test_vaccination_certificate.rs`

---

## #84: Fix timestamp comparison inequality in `verify_certificate`

- **GitHub Issue**: [#89](https://github.com/iam-mercy/kora-app/issues/89)
- **Status**: `Published`

### Category
Smart Contract

### Priority
P2

### Problem
In `stellar-contracts/src/lib.rs` (lines 7331–7358), `verify_certificate` checks `anchor.timestamp < ledger_timestamp`. If verified in the same block as anchored, verification returns `false`.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 7331–7358):
```rust
if anchor_time < ledger_now { true } else { false } // In same block, anchor_time == ledger_now!
```

### Why It Matters
Transactions chaining certificate anchoring and verification within the same ledger block fail verification erroneously.

### Proposed Work
Change comparison to `anchor_time <= ledger_now`.

### Acceptance Criteria
- [ ] Certificates verified in the anchor block return `true`.
- [ ] Future timestamps return `false`.

### Testing Requirements
Test anchor and verify in same block returns `true`.

### Dependencies
Dependencies: None

### Code References
- `stellar-contracts/src/lib.rs`: lines 7331–7358

---

## #85: Support configurable window duration in `get_upcoming_vaccinations`

- **GitHub Issue**: [#90](https://github.com/iam-mercy/kora-app/issues/90)
- **Status**: `Published`

### Category
Smart Contract

### Priority
P3

### Problem
In `stellar-contracts/src/lib.rs` (lines 7023–7040), `get_upcoming_vaccinations` hardcodes 30 days (`30 * 86400`) instead of accepting a configurable lookup window parameter.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 7023–7040):
```rust
let window_end = now + (30 * 86400);
```

### Why It Matters
Veterinary clinics and pet owners planning 60-day or 90-day vaccination schedules cannot query upcoming expiries beyond 30 days.

### Proposed Work
Accept `window_days: Option<u32>` defaulting to 30 with max limit of 365.

### Acceptance Criteria
- [ ] Allows custom lookup windows up to 365 days.
- [ ] Default retains 30-day window.

### Testing Requirements
Test querying upcoming vaccinations with 60-day window.

### Dependencies
Dependencies: None

### Code References
- `stellar-contracts/src/lib.rs`: lines 7023–7040

---

## #86: Optimize vet-filtered vaccination expiry query in `get_expiring_vaccinations`

- **GitHub Issue**: [#91](https://github.com/iam-mercy/kora-app/issues/91)
- **Status**: `Published`

### Category
Performance

### Priority
P2

### Problem
In `stellar-contracts/src/lib.rs` (lines 7149–7213), `get_expiring_vaccinations` performs nested iterations over all pets and all pet vaccinations, hitting Soroban instruction budget limits.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 7149–7213):
```rust
for pet_id in 1..=pet_count {
    for vax_id in 1..=vax_count {
        // Unbounded nested storage scan
```

### Why It Matters
Querying expiring vaccinations for a clinic with 500 patients will exceed Soroban execution budgets and abort.

### Proposed Work
Index vaccinations by expiration ledger or maintain a clinic patient index.

### Acceptance Criteria
- [ ] Query uses indexed lookup instead of full matrix scan.
- [ ] Instruction consumption remains within limits for large registries.

### Testing Requirements
Benchmark test with 100 pets and 500 vaccinations.

### Dependencies
Dependencies: Depends on #9

### Code References
- `stellar-contracts/src/lib.rs`: lines 7149–7213

---

## #87: Validate reference range boundaries in `add_lab_result`

- **GitHub Issue**: [#92](https://github.com/iam-mercy/kora-app/issues/92)
- **Status**: `Published`

### Category
Data Integrity

### Priority
P2

### Problem
In `stellar-contracts/src/lib.rs` (lines 6762–6883), `add_lab_result` does not validate that `reference_min < reference_max`, allowing inverted diagnostic ranges.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 6762–6883):
```rust
pub fn add_lab_result(..., ref_min: u32, ref_max: u32, ...) -> u64 {
    // Lacks check: ref_min < ref_max
```

### Why It Matters
Inverted reference ranges cause health scoring and biomarker anomaly algorithms to falsely classify normal lab values as severe abnormalities.

### Proposed Work
Enforce `if ref_min >= ref_max { panic_with_error!(&env, ContractError::InvalidInput); }`.

### Acceptance Criteria
- [ ] Reference ranges where min >= max revert with `ContractError::InvalidInput`.
- [ ] Valid diagnostic ranges store accurately.

### Testing Requirements
Test submitting lab result with min = 100, max = 50 and verify rejection.

### Dependencies
Dependencies: None

### Code References
- `stellar-contracts/src/lib.rs`: lines 6762–6883
- `stellar-contracts/src/test_get_lab_results.rs`

---

## #88: Enforce veterinarian verification on `add_lab_result`

- **GitHub Issue**: [#93](https://github.com/iam-mercy/kora-app/issues/93)
- **Status**: `Published`

### Category
Access Control

### Priority
P1

### Problem
In `stellar-contracts/src/lib.rs` (lines 6762–6883), `add_lab_result` takes `vet_address: Address` but does not verify `Self::is_verified_vet(&env, &vet_address)`.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 6762–6883):
```rust
pub fn add_lab_result(env: Env, pet_id: u64, vet_address: Address, ...) -> u64 {
    vet_address.require_auth();
    // Missing Self::is_verified_vet() check!
```

### Why It Matters
Any unverified wallet can forge official veterinary blood work, urinalysis, and biopsy reports on-chain.

### Proposed Work
Assert `if !Self::is_verified_vet(env.clone(), vet_address.clone()) { panic_with_error!(&env, ContractError::VetNotVerified); }`.

### Acceptance Criteria
- [ ] Only active, verified veterinarians can add lab results.
- [ ] Unverified callers revert with `ContractError::VetNotVerified`.

### Testing Requirements
Test adding lab result from unverified address reverts.

### Dependencies
Dependencies: Depends on #46

### Code References
- `stellar-contracts/src/lib.rs`: lines 6762–6883

---

## #89: Guard against division by zero in biomarker anomaly detection calculations

- **GitHub Issue**: [#94](https://github.com/iam-mercy/kora-app/issues/94)
- **Status**: `Published`

### Category
Smart Contract

### Priority
P1

### Problem
In `stellar-contracts/src/lib.rs` (lines 12066–12130), `detect_anomalies` divides by `reference_max - reference_min` without checking for zero denominator when min equals max.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 12066–12130):
```rust
let normalized = (val - min) * 100 / (max - min); // Panic when max == min!
```

### Why It Matters
If a qualitative or binary lab test has equal min and max values, calling anomaly detection panics and crashes the transaction.

### Proposed Work
Check `if max <= min { return false; }` (or handle zero range safely) before computing normalized ratios.

### Acceptance Criteria
- [ ] Anomaly detection gracefully handles `min == max` without division by zero.
- [ ] Normalized ratios compute safely.

### Testing Requirements
Test anomaly detection with identical min and max values.

### Dependencies
Dependencies: None

### Code References
- `stellar-contracts/src/lib.rs`: lines 12066–12130

---

## #90: Enforce pet privacy access controls on `get_lab_results`

- **GitHub Issue**: [#95](https://github.com/iam-mercy/kora-app/issues/95)
- **Status**: `Published`

### Category
Access Control

### Priority
P1

### Problem
In `stellar-contracts/src/lib.rs` (lines 6884–6907), `get_lab_results` exposes diagnostic lab results without caller authorization or privacy checks.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 6884–6907):
```rust
pub fn get_lab_results(env: Env, pet_id: u64, ...) -> Vec<LabResult> {
    // No caller parameter, no privacy check
```

### Why It Matters
Sensitive diagnostic findings for private pets are exposed to any on-chain observer, violating privacy settings.

### Proposed Work
Add `caller: Address` argument with `caller.require_auth()`. Verify caller is owner or authorized vet when pet privacy is not Public.

### Acceptance Criteria
- [ ] Lab results of private pets are inaccessible to unauthorized callers.
- [ ] Owners and authorized vets retain read access.

### Testing Requirements
Test unauthorized caller querying lab results for private pet reverts.

### Dependencies
Dependencies: Depends on #12

### Code References
- `stellar-contracts/src/lib.rs`: lines 6884–6907
- `stellar-contracts/src/test_get_lab_results.rs`

---

## #91: Scope `get_lab_result_count` per pet to prevent global transaction metrics leakage

- **GitHub Issue**: [#96](https://github.com/iam-mercy/kora-app/issues/96)
- **Status**: `Published`

### Category
Smart Contract

### Priority
P3

### Problem
In `stellar-contracts/src/lib.rs` (lines 11891–11894), `get_lab_result_count` returns the global contract-wide lab result count instead of per-pet count.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 11891–11894):
```rust
pub fn get_lab_result_count(env: Env) -> u64 {
    env.storage().instance().get(&MedicalKey::LabResultCount).unwrap_or(0)
}
```

### Why It Matters
External observers can monitor global medical throughput and activity rates, and clients cannot determine how many lab results an individual pet has.

### Proposed Work
Update signature to `get_lab_result_count(env: Env, pet_id: u64) -> u64` using `MedicalKey::PetLabResultCount(pet_id)`.

### Acceptance Criteria
- [ ] Count returns per-pet lab result total.
- [ ] Global metrics are not exposed unnecessarily.

### Testing Requirements
Test query returns pet's count rather than contract global count.

### Dependencies
Dependencies: None

### Code References
- `stellar-contracts/src/lib.rs`: lines 11891–11894

---

## #92: Capture author address and block timestamp in `amend_medical_record`

- **GitHub Issue**: [#97](https://github.com/iam-mercy/kora-app/issues/97)
- **Status**: `Published`

### Category
Data Integrity

### Priority
P1

### Problem
In `stellar-contracts/src/lib.rs` (lines 11936–11947), `amend_medical_record` updates diagnosis and treatment without recording the modifying veterinarian's address or amendment timestamp.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 11936–11947):
```rust
pub fn amend_medical_record(..., new_diagnosis: String, new_treatment: String) -> bool {
    // Mutates record in place without recording author or timestamp
```

### Why It Matters
Veterinary malpractice investigations and insurance audits require knowing who amended clinical notes and when. Anonymous in-place overwrites destroy evidentiary integrity.

### Proposed Work
Store `amended_by: Address` and `amended_at: u64` in the `MedicalRecord` struct or historical version log. Emit `MedicalRecordAmendedEvent`.

### Acceptance Criteria
- [ ] Medical record amendments record modifying vet address and block timestamp.
- [ ] Audit event is emitted with old and new values.

### Testing Requirements
Test amending record; verify author address and timestamp are persisted.

### Dependencies
Dependencies: None

### Code References
- `stellar-contracts/src/lib.rs`: lines 11936–11947

---

## #93: Prevent out-of-bounds index panic in `diff_record_versions`

- **GitHub Issue**: [#98](https://github.com/iam-mercy/kora-app/issues/98)
- **Status**: `Published`

### Category
Smart Contract

### Priority
P2

### Problem
In `stellar-contracts/src/lib.rs` (lines 11948–11960), `diff_record_versions` accesses version vectors by index without verifying that `version_a` and `version_b` exist in storage.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 11948–11960):
```rust
pub fn diff_record_versions(env: Env, record_id: u64, v_a: u32, v_b: u32) -> RecordDiff {
    let versions = env.storage().instance().get(...).unwrap();
    let a = versions.get(v_a).unwrap(); // Panics if out of bounds!
```

### Why It Matters
Passing an invalid version number crashes the transaction via unhandled host panic instead of returning a clean error.

### Proposed Work
Check `if v_a >= versions.len() || v_b >= versions.len() { panic_with_error!(&env, ContractError::InvalidInput); }`.

### Acceptance Criteria
- [ ] Invalid version queries revert gracefully with `ContractError::InvalidInput`.
- [ ] Valid version diffs compute cleanly.

### Testing Requirements
Test diffing non-existent version indices.

### Dependencies
Dependencies: None

### Code References
- `stellar-contracts/src/lib.rs`: lines 11948–11960

---

## #94: Enforce caller authorization on `delete_medical_record` soft deletion

- **GitHub Issue**: [#99](https://github.com/iam-mercy/kora-app/issues/99)
- **Status**: `Published`

### Category
Access Control

### Priority
P1

### Problem
In `stellar-contracts/src/lib.rs` (lines 11336–11384), `delete_medical_record` soft-deletes a medical record without requiring pet owner or attending veterinarian authorization.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 11336–11384):
```rust
pub fn delete_medical_record(env: Env, record_id: u64, reason: String) -> bool {
    // Takes NO caller parameter and calls NO require_auth()!
```

### Why It Matters
Any network user can soft-delete any pet's medical records, removing vital allergy and medication history from standard view.

### Proposed Work
Add `caller: Address` with `caller.require_auth()`. Verify `caller == record.vet_address || caller == pet.owner || is_admin(caller)`.

### Acceptance Criteria
- [ ] Soft deletion requires authorization from attending vet, pet owner, or admin.
- [ ] Unauthorized calls revert with `ContractError::Unauthorized`.

### Testing Requirements
Test unauthorized attempt to soft delete medical record fails.

### Dependencies
Dependencies: None

### Code References
- `stellar-contracts/src/lib.rs`: lines 11336–11384
- `stellar-contracts/src/test_medical_record_soft_delete.rs`

---

## #95: Preserve audit history and emit event on `update_medical_record_notes`

- **GitHub Issue**: [#100](https://github.com/iam-mercy/kora-app/issues/100)
- **Status**: `Published`

### Category
Data Integrity

### Priority
P2

### Problem
In `stellar-contracts/src/lib.rs` (lines 11924–11935), `update_medical_record_notes` overwrites notes in-place without preserving previous clinical notes or emitting an update event.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 11924–11935):
```rust
pub fn update_medical_record_notes(env: Env, record_id: u64, notes: String) -> bool {
    rec.notes = notes;
    env.storage().instance().set(&MedicalKey::MedicalRecord(record_id), &rec);
    // No event emitted, no historical notes saved!
```

### Why It Matters
Silent clinical note modifications make it impossible to track changes or audit veterinarian record updates.

### Proposed Work
Emit `MedicalRecordNotesUpdatedEvent` containing `record_id`, `old_notes_hash`, and `new_notes_hash`.

### Acceptance Criteria
- [ ] Note updates emit an auditable event.
- [ ] Event contains hash references of previous and updated content.

### Testing Requirements
Test updating notes; verify event emission and correct hash values.

### Dependencies
Dependencies: None

### Code References
- `stellar-contracts/src/lib.rs`: lines 11924–11935

---

## #96: Update active record count when purging expired records in `purge_expired_records`

- **GitHub Issue**: [#101](https://github.com/iam-mercy/kora-app/issues/101)
- **Status**: `Published`

### Category
Data Integrity

### Priority
P2

### Problem
In `stellar-contracts/src/lib.rs` (lines 11465–11469), `purge_expired_records` deletes entries but does not update `MedicalKey::MedicalRecordCount`, causing pagination gaps.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 11465–11469):
```rust
pub fn purge_expired_records(env: Env, pet_id: u64) -> u32 {
    // Deletes records but leaves MedicalRecordCount unchanged
```

### Why It Matters
Dangling indices cause paginated queries to encounter missing records, returning empty or fragmented pages.

### Proposed Work
Synchronize `MedicalRecordCount` or maintain a compacted active index when purging records.

### Acceptance Criteria
- [ ] Purging expired records updates record count accurately.
- [ ] Paginated queries remain dense and contiguous.

### Testing Requirements
Test purging records followed by paginated retrieval.

### Dependencies
Dependencies: Depends on #11

### Code References
- `stellar-contracts/src/lib.rs`: lines 11465–11469

---

## #97: Enforce minimum regulatory retention floor in `set_retention_period`

- **GitHub Issue**: [#102](https://github.com/iam-mercy/kora-app/issues/102)
- **Status**: `Published`

### Category
Security

### Priority
P1

### Problem
In `stellar-contracts/src/lib.rs` (lines 11319–11335), `set_retention_period` permits setting retention to 0 days, allowing immediate deletion of medical records subject to statutory retention mandates.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 11319–11335):
```rust
pub fn set_retention_period(env: Env, admin: Address, days: u32) -> bool {
    // Lacks minimum floor: allows days = 0
```

### Why It Matters
Veterinary licensing boards mandate medical record retention for a minimum of 3 to 7 years. Allowing `days = 0` exposes clinics and the platform to legal non-compliance.

### Proposed Work
Enforce `if days < MIN_RETENTION_DAYS { panic_with_error!(&env, ContractError::InvalidInput); }` (e.g. minimum 365 or 1095 days).

### Acceptance Criteria
- [ ] Setting retention period below statutory minimum reverts.
- [ ] Retention periods at or above threshold update successfully.

### Testing Requirements
Test setting retention to 0 days fails with `ContractError::InvalidInput`.

### Dependencies
Dependencies: None

### Code References
- `stellar-contracts/src/lib.rs`: lines 11319–11335

---

## #98: Validate medical record existence before attaching files in `add_attachment`

- **GitHub Issue**: [#103](https://github.com/iam-mercy/kora-app/issues/103)
- **Status**: `Published`

### Category
Data Integrity

### Priority
P1

### Problem
In `stellar-contracts/src/lib.rs` (lines 8707–8774), `add_attachment` accepts `record_id: u64` and pushes attachment metadata without verifying whether `MedicalRecord(record_id)` exists.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 8707–8774):
```rust
pub fn add_attachment(env: Env, record_id: u64, metadata: AttachmentMetadata) -> u64 {
    // Checks metadata, but NEVER verifies MedicalKey::MedicalRecord(record_id) exists!
```

### Why It Matters
Orphaned file attachments can be stored against non-existent record IDs, polluting storage and stranding files without associated clinical contexts.

### Proposed Work
Check `env.storage().instance().has(&MedicalKey::MedicalRecord(record_id))`. Revert with `ContractError::RecordNotFound` if absent.

### Acceptance Criteria
- [ ] Attaching files to non-existent medical records reverts with `ContractError::RecordNotFound`.
- [ ] Valid records accept attachments.

### Testing Requirements
Test adding attachment to non-existent record ID.

### Dependencies
Dependencies: None

### Code References
- `stellar-contracts/src/lib.rs`: lines 8707–8774
- `stellar-contracts/src/test_attachments.rs`

---

## #99: Enforce `MAX_ATTACHMENTS_PER_RECORD` bound in `add_attachment`

- **GitHub Issue**: [#104](https://github.com/iam-mercy/kora-app/issues/104)
- **Status**: `Published`

### Category
Security

### Priority
P2

### Problem
In `stellar-contracts/src/lib.rs` (lines 8707–8774), `add_attachment` pushes hashes to `record.attachment_hashes` without checking if the vector has reached `MAX_ATTACHMENTS_PER_RECORD`.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 8707–8774):
```rust
let mut hashes = KoraContract::get_attachments(&env, record_id);
hashes.push_back(metadata.cid.clone()); // Unbounded growth!
```

### Why It Matters
An attacker can upload hundreds of attachment hashes to a single medical record, bloating its serialization size until loading the record exceeds VM memory.

### Proposed Work
Enforce `if hashes.len() >= MAX_ATTACHMENTS_PER_RECORD { panic_with_error!(&env, ContractError::AttachmentLimitReached); }` (e.g. limit 10).

### Acceptance Criteria
- [ ] Attachment additions beyond limit revert with `ContractError::AttachmentLimitReached`.
- [ ] Attachment counts within limit succeed.

### Testing Requirements
Test adding 11 attachments when limit is 10; verify 11th reverts.

### Dependencies
Dependencies: None

### Code References
- `stellar-contracts/src/lib.rs`: lines 8707–8774
- `stellar-contracts/src/test_attachment_limit.rs`

---

## #100: Enforce allowed MIME types on attachment metadata

- **GitHub Issue**: [#105](https://github.com/iam-mercy/kora-app/issues/105)
- **Status**: `Published`

### Category
Security

### Priority
P2

### Problem
In `stellar-contracts/src/lib.rs` (lines 8707–8774), `add_attachment` accepts arbitrary `file_type: String` without validating against an allowed list of safe MIME types (`image/jpeg`, `application/pdf`, etc.).

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 8707–8774):
```rust
pub fn add_attachment(..., metadata: AttachmentMetadata) -> u64 {
    // Stores metadata.file_type with zero MIME type validation
```

### Why It Matters
Users can upload executable files or deceptive scripts (.exe, .sh, .html) disguised as medical images, exposing client dApps and clinics to client-side code execution vulnerabilities.

### Proposed Work
Validate `metadata.file_type` against an allowed set (`image/png`, `image/jpeg`, `application/pdf`, `image/dicom`). Revert unapproved types.

### Acceptance Criteria
- [ ] Only whitelisted veterinary media MIME types are accepted.
- [ ] Unrecognized or dangerous file types revert with `ContractError::InvalidInput`.

### Testing Requirements
Test adding attachment with `file_type = application/x-executable` fails.

### Dependencies
Dependencies: None

### Code References
- `stellar-contracts/src/lib.rs`: lines 8707–8774
- `stellar-contracts/src/test_attachments.rs`

---

## #101: Enforce privacy level access controls on `get_attachments`

- **GitHub Issue**: [#106](https://github.com/iam-mercy/kora-app/issues/106)
- **Status**: `Published`

### Category
Access Control

### Priority
P1

### Problem
In `stellar-contracts/src/lib.rs` (lines 8775–8783), `get_attachments` returns file hashes and metadata without caller authorization or pet privacy checks.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 8775–8783):
```rust
pub fn get_attachments(env: Env, record_id: u64) -> Vec<AttachmentMetadata> {
    // No caller check, returns full metadata
```

### Why It Matters
Confidential diagnostic scans, X-rays, and adoption agreements for private pets are exposed to unauthenticated network queries.

### Proposed Work
Add `caller: Address` parameter and verify caller authorization when pet privacy is not Public.

### Acceptance Criteria
- [ ] Attachments of private pets are hidden from unauthorized callers.
- [ ] Authorized owners and vets can retrieve attachments.

### Testing Requirements
Test unauthorized caller querying attachments for private pet reverts.

### Dependencies
Dependencies: Depends on #12

### Code References
- `stellar-contracts/src/lib.rs`: lines 8775–8783
- `stellar-contracts/src/test_attachments.rs`

---

## #102: Enforce pet storage quota consumption on attachment additions

- **GitHub Issue**: [#107](https://github.com/iam-mercy/kora-app/issues/107)
- **Status**: `Published`

### Category
Data Integrity

### Priority
P2

### Problem
In `stellar-contracts/src/lib.rs` (lines 8707–8774), `add_attachment` writes attachment metadata to storage without calling `Self::increment_pet_storage(&env, pet_id)`.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 8707–8774):
```rust
// Stores attachment without calling increment_pet_storage
env.storage().instance().set(&MedicalKey::Attachment(attachment_id), &metadata);
```

### Why It Matters
Users can circumvent pet storage quotas by storing large amounts of metadata in attachments rather than within medical records.

### Proposed Work
Call `Self::increment_pet_storage(&env, pet_id)` within `add_attachment`.

### Acceptance Criteria
- [ ] Adding an attachment consumes 1 storage quota unit.
- [ ] Exceeding quota prevents adding new attachments.

### Testing Requirements
Test adding attachment when storage quota is depleted.

### Dependencies
Dependencies: Depends on #10

### Code References
- `stellar-contracts/src/lib.rs`: lines 8707–8774
- `stellar-contracts/src/test_storage_quota.rs`

---

## #103: Record transfer reason and animal condition in custody chain entries

- **GitHub Issue**: [#108](https://github.com/iam-mercy/kora-app/issues/108)
- **Status**: `Published`

### Category
Data Integrity

### Priority
P2

### Problem
In `stellar-contracts/src/lib.rs` (lines 8860–8872), `CustodyEntry` records only previous owner, new owner, and timestamp, omitting transfer reasons and physical condition logs.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 8860–8872):
```rust
pub struct CustodyEntry {
    pub from: Address,
    pub to: Address,
    pub timestamp: u64,
    // Missing reason and physical condition
}
```

### Why It Matters
Shelters and rescue organizations require animal intake condition and reason (adoption, surrender, foster) for legal custody documentation.

### Proposed Work
Add `transfer_type: CustodyTransferType` and `condition_notes: Option<String>` to `CustodyEntry`.

### Acceptance Criteria
- [ ] Custody entries capture transfer type and condition notes.
- [ ] Existing queries preserve backwards compatibility.

### Testing Requirements
Test custody entry logging with transfer reason.

### Dependencies
Dependencies: None

### Code References
- `stellar-contracts/src/lib.rs`: lines 8860–8872
- `stellar-contracts/src/test_custody_chain.rs`

---

## #104: Fix same-ledger timestamp validation bug in `verify_custody_chain`

- **GitHub Issue**: [#109](https://github.com/iam-mercy/kora-app/issues/109)
- **Status**: `Published`

### Category
Smart Contract

### Priority
P2

### Problem
In `stellar-contracts/src/lib.rs` (lines 8873–8931), `verify_custody_chain` asserts `entry.timestamp > prev.timestamp`. If two transfers occur within the same block, verification reverts with `ContractError::InvalidCustodyChain`.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 8873–8931):
```rust
if entry.timestamp <= prev_timestamp { return false; } // Strict inequality bug in same block!
```

### Why It Matters
Consecutive custody changes executed within a single transaction or block (e.g. shelter intake and immediate foster placement) fail chain-of-custody verification.

### Proposed Work
Change validation to `entry.timestamp >= prev_timestamp` and enforce sequential entry indices.

### Acceptance Criteria
- [ ] Transfers within the same ledger block pass custody verification.
- [ ] Truly retroactive entries (< prev) are rejected.

### Testing Requirements
Test custody verification with entries sharing identical block timestamps.

### Dependencies
Dependencies: None

### Code References
- `stellar-contracts/src/lib.rs`: lines 8873–8931
- `stellar-contracts/src/test_custody_chain.rs`

---

## #105: Redact previous owner wallet addresses for private pets in `get_ownership_history`

- **GitHub Issue**: [#110](https://github.com/iam-mercy/kora-app/issues/110)
- **Status**: `Published`

### Category
Access Control

### Priority
P2

### Problem
In `stellar-contracts/src/lib.rs` (lines 8932–8977), `get_ownership_history` exposes the entire historical address chain of prior owners to unauthenticated callers.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 8932–8977):
```rust
pub fn get_ownership_history(env: Env, pet_id: u64) -> Vec<Address> {
    // Returns all historical owners with zero auth checks
```

### Why It Matters
Adopters of rescued pets can trace prior owners' on-chain wallet addresses, threatening privacy in sensitive animal rescue or domestic violence situations.

### Proposed Work
Require caller authorization. If pet privacy is Private, only current owner and registered shelter admins can view prior owner addresses.

### Acceptance Criteria
- [ ] Prior owner addresses are redacted for unauthenticated queries.
- [ ] Current owner retains full ownership history access.

### Testing Requirements
Test unauthenticated query for ownership history on private pet reverts.

### Dependencies
Dependencies: Depends on #12

### Code References
- `stellar-contracts/src/lib.rs`: lines 8932–8977

---

## #106: Log custody entry on `finalize_transfer` in `pet-transfer-adoption`

- **GitHub Issue**: [#111](https://github.com/iam-mercy/kora-app/issues/111)
- **Status**: `Published`

### Category
Data Integrity

### Priority
P1

### Problem
In `stellar-contracts/contracts/pet-transfer-adoption/src/lib.rs` (lines 1129–1199), `finalize_transfer` updates owner storage but fails to append a `CustodyEntry` to `DataKey::CustodyChain(pet_id)`.

### Current Implementation
In `stellar-contracts/contracts/pet-transfer-adoption/src/lib.rs` (lines 1129–1199):
```rust
// Sets new owner, but omits append to custody chain!
env.storage().persistent().set(&DataKey::PetOwner(pet_id), &transfer.to);
```

### Why It Matters
Adoptions and transfers completed through the peripheral adoption contract leave gaps in the custody chain audit trail.

### Proposed Work
Append a new `CustodyEntry` within `finalize_transfer` to maintain complete provenance.

### Acceptance Criteria
- [ ] Finalizing a transfer records a custody chain entry.
- [ ] Provenance history reflects the adoption transfer.

### Testing Requirements
Test finalizing adoption; verify custody chain entry exists.

### Dependencies
Dependencies: Depends on #6, #7

### Code References
- `stellar-contracts/contracts/pet-transfer-adoption/src/lib.rs`: lines 1129–1199, 1508–1514

---

## #107: Make `batch_initiate_transfer` all-or-nothing atomic across pet batches

- **GitHub Issue**: [#112](https://github.com/iam-mercy/kora-app/issues/112)
- **Status**: `Published`

### Category
Smart Contract

### Priority
P1

### Problem
In `stellar-contracts/contracts/pet-transfer-adoption/src/lib.rs` (lines 1356–1415), `batch_initiate_transfer` iterates through pet IDs and mutates state iteratively; if pet 4 of 5 fails, the first 3 transfers remain pending without rollback.

### Current Implementation
In `stellar-contracts/contracts/pet-transfer-adoption/src/lib.rs` (lines 1356–1415):
```rust
for pet_id in pet_ids.iter() {
    // Mutates state iteratively per pet without atomic pre-validation!
    Self::initiate_transfer(env.clone(), pet_id, to.clone());
}
```

### Why It Matters
Partial batch transfer failure leaves half a litter or shelter group in limbo, requiring messy manual cancellations.

### Proposed Work
Pre-validate all pet ownerships and states before committing any state updates, or panic on first error to abort the entire transaction.

### Acceptance Criteria
- [ ] If any pet in the batch fails validation, the entire batch transaction reverts.
- [ ] State mutations commit only if all pets succeed.

### Testing Requirements
Test batch transfer with 1 invalid pet ID in a batch of 5; verify 0 transfers initiated.

### Dependencies
Dependencies: Depends on #6, #7

### Code References
- `stellar-contracts/contracts/pet-transfer-adoption/src/lib.rs`: lines 1356–1415

---

## #108: Prevent duplicate emergency responder address registrations

- **GitHub Issue**: [#113](https://github.com/iam-mercy/kora-app/issues/113)
- **Status**: `Published`

### Category
Data Integrity

### Priority
P2

### Problem
In `stellar-contracts/src/lib.rs` (lines 8978–9002), `add_emergency_responder` appends responder addresses without checking if the address is already registered in `DataKey::EmergencyResponders`.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 8978–9002):
```rust
pub fn add_emergency_responder(env: Env, admin: Address, responder: Address) -> bool {
    let mut responders = Self::get_emergency_responders(env.clone());
    responders.push_back(responder); // Lacks duplicate check!
```

### Why It Matters
Duplicate responder entries waste instance storage and trigger duplicate notification dispatches during emergencies.

### Proposed Work
Check if `responders.iter().any(|r| r == responder)`. If present, return `ContractError::AlreadyExists` or return early.

### Acceptance Criteria
- [ ] Duplicate responder addresses are rejected.
- [ ] Responders list maintains unique entries.

### Testing Requirements
Test adding the same responder address twice.

### Dependencies
Dependencies: None

### Code References
- `stellar-contracts/src/lib.rs`: lines 8978–9002
- `stellar-contracts/src/test_emergency_contacts.rs`

---

## #109: Verify responder existence before deletion in `remove_emergency_responder`

- **GitHub Issue**: [#114](https://github.com/iam-mercy/kora-app/issues/114)
- **Status**: `Published`

### Category
Smart Contract

### Priority
P3

### Problem
In `stellar-contracts/src/lib.rs` (lines 9003–9047), `remove_emergency_responder` does not check if the responder exists before deletion and returns `true` even when the address was never registered.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 9003–9047):
```rust
pub fn remove_emergency_responder(env: Env, admin: Address, responder: Address) -> bool {
    // Loops, doesn'''t find item, writes unchanged list, returns true!
```

### Why It Matters
Returning `true` on no-op deletions produces misleading client state and spurious admin activity audit logs.

### Proposed Work
Return `false` or revert with `ContractError::NotFound` if the address was not present in the responder list.

### Acceptance Criteria
- [ ] Removing non-existent responder returns `false` or error.
- [ ] Removing existing responder removes address and returns `true`.

### Testing Requirements
Test removing non-existent responder returns false.

### Dependencies
Dependencies: None

### Code References
- `stellar-contracts/src/lib.rs`: lines 9003–9047
- `stellar-contracts/src/test_emergency_contacts.rs`

---

## #110: Validate index bounds in `reorder_contact` to prevent VM panics

- **GitHub Issue**: [#115](https://github.com/iam-mercy/kora-app/issues/115)
- **Status**: `Published`

### Category
Smart Contract

### Priority
P2

### Problem
In `stellar-contracts/src/lib.rs` (lines 9427–9475), `reorder_contact` uses `contacts.get(from_index)` and `contacts.swap(from_index, to_index)` without validating `from_index < contacts.len()` and `to_index < contacts.len()`.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 9427–9475):
```rust
pub fn reorder_contact(..., from_index: u32, to_index: u32) -> bool {
    let contact = contacts.get(from_index).unwrap(); // Host panic on out of bounds!
```

### Why It Matters
Passing invalid index parameters triggers unhandled host panics instead of returning user-friendly contract errors.

### Proposed Work
Assert `if from_index >= contacts.len() || to_index >= contacts.len() { panic_with_error!(&env, ContractError::InvalidInput); }`.

### Acceptance Criteria
- [ ] Index parameters out of bounds revert gracefully with `ContractError::InvalidInput`.
- [ ] Valid index swaps reorder contacts accurately.

### Testing Requirements
Test reordering with `from_index = 99` on a pet with 2 contacts.

### Dependencies
Dependencies: None

### Code References
- `stellar-contracts/src/lib.rs`: lines 9427–9475
- `stellar-contracts/src/test_emergency_contacts.rs`

---

## #111: Add pagination support to `get_emergency_access_logs`

- **GitHub Issue**: [#116](https://github.com/iam-mercy/kora-app/issues/116)
- **Status**: `Published`

### Category
Performance

### Priority
P2

### Problem
In `stellar-contracts/src/lib.rs` (lines 9476–9504), `get_emergency_access_logs` returns all emergency access events in a single unpaginated vector.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 9476–9504):
```rust
pub fn get_emergency_access_logs(env: Env, pet_id: u64) -> Vec<EmergencyAccessLog> {
    // Returns entire vector from storage
```

### Why It Matters
Emergency overrides logged by ER clinics accumulate rapidly, risking transaction output limit breaches for frequently treated pets.

### Proposed Work
Implement `get_emergency_access_logs_paginated(env: Env, pet_id: u64, offset: u64, limit: u32) -> Vec<EmergencyAccessLog>`.

### Acceptance Criteria
- [ ] Emergency access logs support pagination with bounded limits.
- [ ] Memory consumption is bounded per call.

### Testing Requirements
Test paginated access log queries across multiple pages.

### Dependencies
Dependencies: None

### Code References
- `stellar-contracts/src/lib.rs`: lines 9476–9504

---

## #112: Standardize emergency notification rate limits to timestamp-based windows

- **GitHub Issue**: [#117](https://github.com/iam-mercy/kora-app/issues/117)
- **Status**: `Published`

### Category
Smart Contract

### Priority
P2

### Problem
In `stellar-contracts/src/lib.rs` (lines 9334–9383), `notify_emergency_contacts` rate limiting compares ledger sequence numbers instead of timestamps, creating inconsistent rate limits across environments with different block times.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 9334–9383):
```rust
let current_seq = env.ledger().sequence();
if current_seq - last_seq < RATE_LIMIT_LEDGERS { ... }
```

### Why It Matters
Testnet and mainnet have different ledger close times (3s to 6s), causing rate limits to vary widely between test and production deployments.

### Proposed Work
Switch rate limit comparison to `env.ledger().timestamp()` with a 1-hour window (`3600` seconds).

### Acceptance Criteria
- [ ] Rate limits operate on wall-clock seconds rather than ledger sequence counts.
- [ ] Behavior is uniform across localnet, testnet, and mainnet.

### Testing Requirements
Test emergency notification rate limiting using timestamp progression.

### Dependencies
Dependencies: None

### Code References
- `stellar-contracts/src/lib.rs`: lines 9334–9383
- `stellar-contracts/src/test_emergency_notify_rate_limit.rs`

---

## #113: Prevent registering the same offspring pet ID to multiple litters

- **GitHub Issue**: [#118](https://github.com/iam-mercy/kora-app/issues/118)
- **Status**: `Published`

### Category
Data Integrity

### Priority
P1

### Problem
In `stellar-contracts/src/lib.rs` (lines 11025–11096), `add_offspring` appends `offspring_pet_id` without verifying whether the pet is already registered as offspring of another breeding record.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 11025–11096):
```rust
pub fn add_offspring(env: Env, record_id: u64, offspring_pet_id: u64) -> bool {
    // Lacks check: has offspring_pet_id already been assigned to another breeding record?
```

### Why It Matters
An animal cannot have two biological mothers or distinct birth litters. Duplicate registrations corrupt lineage tracking and pedigree graphs.

### Proposed Work
Check storage key `DataKey::OffspringLineage(offspring_pet_id)`. Revert with `ContractError::AlreadyExists` if the offspring is already linked to a litter.

### Acceptance Criteria
- [ ] An offspring pet can only be added to a single breeding record.
- [ ] Duplicate offspring assignments revert.

### Testing Requirements
Test adding the same offspring ID to two different breeding records.

### Dependencies
Dependencies: None

### Code References
- `stellar-contracts/src/lib.rs`: lines 11025–11096
- `stellar-contracts/src/test_breeding.rs`

---

## #114: Replace manipulable timestamp pseudo-randomness in `compute_offspring_traits`

- **GitHub Issue**: [#119](https://github.com/iam-mercy/kora-app/issues/119)
- **Status**: `Published`

### Category
Security

### Priority
P2

### Problem
In `stellar-contracts/src/lib.rs` (lines 11129–11174), `compute_offspring_traits` uses `env.ledger().timestamp() % 2` to select inherited maternal or paternal alleles, allowing transaction miners or submitters to select desired genetic traits.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 11129–11174):
```rust
let choice = (env.ledger().timestamp() + index) % 2;
// Predictable trait selection!
```

### Why It Matters
Breeders can manipulate transaction submission timing to guarantee rare coat colors or desirable hereditary markers, undermining genetic integrity.

### Proposed Work
Derive trait inheritance randomness by hashing sire ID, dam ID, offspring ID, and contract nonce via `env.crypto().sha256()`, or utilize an off-chain oracle / VRF commit-reveal scheme.

### Acceptance Criteria
- [ ] Allele inheritance does not rely on simple modulo of ledger timestamp.
- [ ] Trait derivation is tamper-resistant against transaction timing manipulation.

### Testing Requirements
Test offspring trait computation across varied parent genetics.

### Dependencies
Dependencies: None

### Code References
- `stellar-contracts/src/lib.rs`: lines 11129–11174
- `stellar-contracts/src/test_breeding_genetics.rs`

---

## #115: Cap cumulative basis points in `get_trait_probability` to 10,000 bps

- **GitHub Issue**: [#120](https://github.com/iam-mercy/kora-app/issues/120)
- **Status**: `Published`

### Category
Data Integrity

### Priority
P2

### Problem
In `stellar-contracts/src/lib.rs` (lines 11175–11198), `get_trait_probability` can return probability calculations exceeding 10,000 basis points (100%) due to unnormalized multi-allele combinations.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 11175–11198):
```rust
let prob = prob1 + prob2; // Can exceed 10000 bps!
```

### Why It Matters
Probabilities greater than 100% break frontend prediction charts and breeding compatibility calculators.

### Proposed Work
Normalize probabilities so cumulative distribution sums to at most 10,000 bps (100.00%).

### Acceptance Criteria
- [ ] Trait probabilities are bounded between 0 and 10,000 basis points.
- [ ] Sum of mutually exclusive trait outcomes does not exceed 10,000 bps.

### Testing Requirements
Test probability calculation for compound heterozygous traits.

### Dependencies
Dependencies: None

### Code References
- `stellar-contracts/src/lib.rs`: lines 11175–11198
- `stellar-contracts/src/test_breeding_genetics.rs`

---

## #116: Verify active and non-archived status of breeding pairs in `register_breeding_pair`

- **GitHub Issue**: [#121](https://github.com/iam-mercy/kora-app/issues/121)
- **Status**: `Published`

### Category
Data Integrity

### Priority
P1

### Problem
In `stellar-contracts/src/lib.rs` (lines 11260–11281), `register_breeding_pair` pairs sire and dam without checking if either pet is archived (`pet.archived == true`) or inactive.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 11260–11281):
```rust
pub fn register_breeding_pair(env: Env, sire_id: u64, dam_id: u64) -> bool {
    // Lacks check: sire.active && !sire.archived && dam.active && !dam.archived
```

### Why It Matters
Deceased or archived animals can be registered for active breeding programs, compromising registry accuracy.

### Proposed Work
Assert `sire.active && !sire.archived && dam.active && !dam.archived`. Revert with `ContractError::PetNotActive` otherwise.

### Acceptance Criteria
- [ ] Inactive or archived pets cannot be registered in active breeding pairs.
- [ ] Only active living pets can form breeding pairs.

### Testing Requirements
Test registering breeding pair with archived pet ID reverts.

### Dependencies
Dependencies: None

### Code References
- `stellar-contracts/src/lib.rs`: lines 11260–11281
- `stellar-contracts/src/test_breeding.rs`

---

## #117: Enforce administrator authorization on `add_breed_metadata`

- **GitHub Issue**: [#122](https://github.com/iam-mercy/kora-app/issues/122)
- **Status**: `Published`

### Category
Access Control

### Priority
P2

### Problem
In `stellar-contracts/src/lib.rs` (lines 10291–10334), `add_breed_metadata` accepts breed standard definitions without verifying admin or breed registry authority.

### Current Implementation
In `stellar-contracts/src/lib.rs` (lines 10291–10334):
```rust
pub fn add_breed_metadata(env: Env, species: Species, breed_name: String, ...) -> bool {
    // Takes NO admin address, performs NO authorization check!
```

### Why It Matters
Any user can alter official breed standards, average weight ranges, and expected lifespan data for entire species.

### Proposed Work
Require `admin: Address` parameter and verify `Self::require_admin(&env, &admin)`.

### Acceptance Criteria
- [ ] Only contract administrators or authorized breed registries can define breed standards.
- [ ] Unauthorized modifications revert with `ContractError::Unauthorized`.

### Testing Requirements
Test adding breed metadata without admin authorization fails.

### Dependencies
Dependencies: None

### Code References
- `stellar-contracts/src/lib.rs`: lines 10291–10334

---

## #118: Validate pet existence in `deactivatePet` and `reactivatePet` in `KoraRegistry.sol`

- **GitHub Issue**: [#123](https://github.com/iam-mercy/kora-app/issues/123)
- **Status**: `Published`

### Category
Celo/Solidity

### Priority
P1

### Problem
In `celo-contracts/contracts/KoraRegistry.sol` (lines 386–405), `deactivatePet` and `reactivatePet` check `_pets[petId].owner == msg.sender`. For non-existent `petId`, `_pets[petId].owner` is `address(0)`. If called by zero address (or misconfigured proxy), it mutates state. More practically, it fails to emit a clear `PetNotFound` revert.

### Current Implementation
In `celo-contracts/contracts/KoraRegistry.sol` (lines 386–405):
```solidity
function deactivatePet(uint256 petId) external onlyPetOwner(petId) whenNotPaused {
    // onlyPetOwner checks: require(_pets[petId].owner == msg.sender, "...");
    // Missing: require(_pets[petId].owner != address(0), "Pet does not exist");
```

### Why It Matters
Calling deactivate or reactivate on unminted pet IDs emits misleading error messages or allows silent operations if caller matches default state.

### Proposed Work
Add explicit `require(_pets[petId].owner != address(0), "KoraRegistry: pet does not exist");` check.

### Acceptance Criteria
- [ ] Calling `deactivatePet` or `reactivatePet` on non-existent pet ID reverts with `KoraRegistry: pet does not exist`.
- [ ] Existing pets deactivate and reactivate normally.

### Testing Requirements
Hardhat test calling `deactivatePet(999999)` asserting specific non-existent revert message.

### Dependencies
Dependencies: None

### Code References
- `celo-contracts/contracts/KoraRegistry.sol`: lines 386–405
- `celo-contracts/test/KoraRegistry.test.js`

---

## #119: Prevent orphan medical records by verifying pet existence in `addMedicalRecord` in `KoraRegistry.sol`

- **GitHub Issue**: [#124](https://github.com/iam-mercy/kora-app/issues/124)
- **Status**: `Published`

### Category
Celo/Solidity

### Priority
P1

### Problem
In `celo-contracts/contracts/KoraRegistry.sol` (lines 411–449), `addMedicalRecord` creates a record and attaches it to `petId` without verifying `_pets[petId].owner != address(0)`.

### Current Implementation
In `celo-contracts/contracts/KoraRegistry.sol` (lines 411–449):
```solidity
function addMedicalRecord(uint256 petId, ...) external whenNotPaused onlyVerifiedVet returns (uint256) {
    // NEVER checks whether petId exists!
    _recordPet[recordId] = petId;
```

### Why It Matters
Veterinarians can accidentally or maliciously log clinical diagnoses, prescriptions, and surgeries against non-existent pet IDs, stranding medical records on-chain.

### Proposed Work
Add `require(_pets[petId].owner != address(0), "KoraRegistry: pet does not exist");` before record creation.

### Acceptance Criteria
- [ ] Logging a medical record against a non-existent pet ID reverts.
- [ ] Records for existing pets succeed and link correctly.

### Testing Requirements
Hardhat test adding medical record to uncreated pet ID reverts.

### Dependencies
Dependencies: None

### Code References
- `celo-contracts/contracts/KoraRegistry.sol`: lines 411–449
- `celo-contracts/test/KoraRegistry.test.js`

---

## #120: Bound loop iterations in `getPetRecordsByDateRange` to prevent out-of-gas reverts

- **GitHub Issue**: [#125](https://github.com/iam-mercy/kora-app/issues/125)
- **Status**: `Published`

### Category
Celo/Solidity

### Priority
P2

### Problem
In `celo-contracts/contracts/KoraRegistry.sol` (lines 574–606), `getPetRecordsByDateRange` reads and counts matching items via an unbounded storage array loop, risking gas exhaustion.

### Current Implementation
In `celo-contracts/contracts/KoraRegistry.sol` (lines 574–606):
```solidity
for (uint256 i = 0; i < total; i++) {
    // Unbounded scan over storage array
}
```

### Why It Matters
For pets with extensive histories (e.g. chronic illnesses with 200+ visits), querying records by date range hits EVM block gas limits and reverts.

### Proposed Work
Introduce maximum query limits or page boundaries in date-range lookups.

### Acceptance Criteria
- [ ] Date range query accepts a `limit` parameter.
- [ ] Gas consumption remains bounded regardless of total pet record count.

### Testing Requirements
Hardhat test verifying gas consumption is capped on large record lists.

### Dependencies
Dependencies: None

### Code References
- `celo-contracts/contracts/KoraRegistry.sol`: lines 574–606

---

## #121: Enforce maximum string length validation on license and specialization in `registerVet` in `KoraRegistry.sol`

- **GitHub Issue**: [#126](https://github.com/iam-mercy/kora-app/issues/126)
- **Status**: `Published`

### Category
Celo/Solidity

### Priority
P2

### Problem
In `celo-contracts/contracts/KoraRegistry.sol` (lines 250–280), `registerVet` accepts `string calldata licenseNumber` and `string calldata specialization` without length bounds, permitting gas griefing.

### Current Implementation
In `celo-contracts/contracts/KoraRegistry.sol` (lines 250–280):
```solidity
function registerVet(string calldata licenseNumber, string calldata specialization) external whenNotPaused {
    // Lacks length constraints on licenseNumber and specialization
```

### Why It Matters
Submitting oversized strings bloats contract state, increases archival node storage, and can break frontend rendering of veterinarian credentials.

### Proposed Work
Enforce `require(bytes(licenseNumber).length > 0 && bytes(licenseNumber).length <= 64, "Invalid license length");` and `require(bytes(specialization).length <= 128, "Invalid specialization length");`.

### Acceptance Criteria
- [ ] License numbers and specializations exceeding bounds are rejected.
- [ ] Valid credential strings are accepted and normalized.

### Testing Requirements
Hardhat test registering vet with 500-byte license number reverts.

### Dependencies
Dependencies: None

### Code References
- `celo-contracts/contracts/KoraRegistry.sol`: lines 250–280
- `celo-contracts/test/KoraRegistry.test.js`

---

## #122: Prevent concurrent signing of duplicate adoption applications in `pet-transfer-adoption`

- **GitHub Issue**: [#127](https://github.com/iam-mercy/kora-app/issues/127)
- **Status**: `Published`

### Category
Smart Contract

### Priority
P1

### Problem
In `stellar-contracts/contracts/pet-transfer-adoption/src/lib.rs` (lines 633–693), `sign_adoption` overwrites `DataKey::PendingAdoption(pet_id)` even if an approved adoption is already pending fulfillment.

### Current Implementation
In `stellar-contracts/contracts/pet-transfer-adoption/src/lib.rs` (lines 633–693):
```rust
pub fn sign_adoption(env: Env, pet_id: u64, to: Address, organization: Option<Address>) {
    // Overwrites pending adoption without checking if pet is already in an approved adoption state!
```

### Why It Matters
A shelter or concurrent applicant can overwrite a signed and waiting-period-cleared adoption application, canceling the previous adopter's pending rights without notification.

### Proposed Work
Assert that no active pending adoption exists (`!has_pending_adoption(pet_id)`), or require explicit cancellation before a new application is registered.

### Acceptance Criteria
- [ ] Signing an adoption application fails if an application is already active for that pet.
- [ ] Adoptions can only be initiated after cancellation or completion of prior applications.

### Testing Requirements
Unit test in `pet-transfer-adoption` verifying duplicate adoption signing reverts.

### Dependencies
Dependencies: Depends on #6, #7

### Code References
- `stellar-contracts/contracts/pet-transfer-adoption/src/lib.rs`: lines 633–693

---

## #123: Enforce administrator authentication in `waive_waiting_period` in `pet-transfer-adoption`

- **GitHub Issue**: [#128](https://github.com/iam-mercy/kora-app/issues/128)
- **Status**: `Published`

### Category
Access Control

### Priority
P0

### Problem
In `stellar-contracts/contracts/pet-transfer-adoption/src/lib.rs` (lines 961–1041), `waive_waiting_period` takes `admin: Address` but does not verify `admin.require_auth()` or check that `admin` is in the authorized admin list.

### Current Implementation
In `stellar-contracts/contracts/pet-transfer-adoption/src/lib.rs` (lines 961–1041):
```rust
pub fn waive_waiting_period(env: Env, pet_id: u64, admin: Address, reason: String) {
    // admin.require_auth() is NEVER CALLED!
    // admin role is NEVER CHECKED!
```

### Why It Matters
Anyone can pass any admin address to `waive_waiting_period`, bypass statutory animal protection waiting periods, and immediately take possession of a pet without waiting or authorization.

### Proposed Work
Add `admin.require_auth()` and verify `admin` against `DataKey::AdoptionAdmin`. Revert unauthorized callers with `ContractError::Unauthorized`.

### Acceptance Criteria
- [ ] Waiving waiting periods strictly requires authenticated administrator authorization.
- [ ] Unauthorized calls revert with `ContractError::Unauthorized`.

### Testing Requirements
Test: Calling `waive_waiting_period` without admin cryptographic signature reverts.

### Dependencies
Dependencies: Depends on #6, #7

### Code References
- `stellar-contracts/contracts/pet-transfer-adoption/src/lib.rs`: lines 961–1041

---

## #124: Enforce non-zero timeout duration in `initiate_transfer_with_timeout` in `pet-transfer-adoption`

- **GitHub Issue**: [#129](https://github.com/iam-mercy/kora-app/issues/129)
- **Status**: `Published`

### Category
Smart Contract

### Priority
P2

### Problem
In `stellar-contracts/contracts/pet-transfer-adoption/src/lib.rs` (lines 748–797), `initiate_transfer_with_timeout` allows `timeout_secs = 0`, causing the transfer to expire instantaneously upon creation.

### Current Implementation
In `stellar-contracts/contracts/pet-transfer-adoption/src/lib.rs` (lines 748–797):
```rust
pub fn initiate_transfer_with_timeout(..., timeout_secs: u64) {
    // Lacks check for timeout_secs >= MIN_TRANSFER_TIMEOUT
    let timeout = now + timeout_secs;
```

### Why It Matters
An owner can initiate a transfer with `timeout_secs = 0`, locking the pet in a transfer state that cannot be accepted because it is immediately expired.

### Proposed Work
Enforce `if timeout_secs < MIN_TRANSFER_TIMEOUT_SECS || timeout_secs > MAX_TRANSFER_TIMEOUT_SECS { panic_with_error!(&env, ContractError::InvalidInput); }`.

### Acceptance Criteria
- [ ] Transfer timeouts less than 1 hour or greater than 30 days revert.
- [ ] Valid timeouts configure successfully.

### Testing Requirements
Test initiating transfer with timeout_secs = 0 reverts.

### Dependencies
Dependencies: Depends on #6, #7

### Code References
- `stellar-contracts/contracts/pet-transfer-adoption/src/lib.rs`: lines 748–797

---

## #125: Prevent reclaim of already accepted or finalized transfers in `reclaim_transfer` in `pet-transfer-adoption`

- **GitHub Issue**: [#130](https://github.com/iam-mercy/kora-app/issues/130)
- **Status**: `Published`

### Category
Security

### Priority
P1

### Problem
In `stellar-contracts/contracts/pet-transfer-adoption/src/lib.rs` (lines 1279–1307), `reclaim_transfer` does not check whether `transfer.status == TransferStatus::Accepted` before resetting the owner to previous owner.

### Current Implementation
In `stellar-contracts/contracts/pet-transfer-adoption/src/lib.rs` (lines 1279–1307):
```rust
pub fn reclaim_transfer(env: Env, pet_id: u64) {
    // Does not check if transfer has already been accepted/finalized by the recipient!
    env.storage().persistent().set(&DataKey::PetOwner(pet_id), &transfer.from);
```

### Why It Matters
A former owner can claw back pet ownership after an adoption or transfer has already been legally accepted by the new owner, creating serious custody violations.

### Proposed Work
Assert `transfer.status == TransferStatus::Pending`. If the transfer is already accepted or finalized, revert with `ContractError::TransferAlreadyCompleted`.

### Acceptance Criteria
- [ ] Reclaim is strictly forbidden once a transfer is accepted.
- [ ] Former owners cannot unilaterally claw back pets post-acceptance.

### Testing Requirements
Test attempting `reclaim_transfer` after `accept_transfer` has completed; verify revert.

### Dependencies
Dependencies: Depends on #6, #7

### Code References
- `stellar-contracts/contracts/pet-transfer-adoption/src/lib.rs`: lines 1279–1307

---

# Wave 9 Continuation Dependency Map

```
+--------------------------------------------------------------------------------+
|                   DISPUTES, EVIDENCE & ARBITRATION CHAIN                      |
|                                                                                |
|  [#51] Restrict raise_dispute Callers ----> [#52] Enforce Arbitrator Verif    |
|         |                                            |                         |
|         v                                            v                         |
|  [#53] Prevent Re-Voting on Disputes       [#56] Evidence Sig Verification     |
|  [#54] Validate Evidence CID Length        [#57] Pause Transfer Expiry (Adoption)|
|  [#55] Bound Dispute Appeal Window         [#58] Index get_pet_disputes Lookup |
+--------------------------------------------------------------------------------+
                                        |
                                        v
+--------------------------------------------------------------------------------+
|                   GROOMING, BEHAVIOR & TRAINING LOGS                           |
|                                                                                |
|  [#59] Grooming Interval Validation        [#65] Behavior Severity Validation  |
|  [#60] Groomer Admin Verification          [#66] Behavior History Pagination   |
|  [#61] Groomer Client-Only Ratings         [#67] Activity Streak Rollover Bug  |
|  [#62] Prevent Slot Double-Booking         [#68] Milestone Comparison Fix      |
|  [#63] Owner Grooming Cancel Auth          [#69] Behavior Quota Tracking       |
|  [#64] Groomer Advance Schedule Auth                                           |
+--------------------------------------------------------------------------------+
                                        |
                                        v
+--------------------------------------------------------------------------------+
|                   NUTRITION, WEIGHT & RECOVERY TAGS                            |
|                                                                                |
|  [#70] Reject Future Feeding Timestamps    [#76] Tag Linking Owner Auth        |
|  [#71] Weight Positive Bounds              [#77] Tag Message Length Bounds     |
|  [#72] Weight History Pagination           [#78] Deactivate/Reactivate Tag Auth|
|  [#73] Nutrition Version Owner Auth        [#79] Tag Unlinking Before Re-Link  |
|  [#74] Rollback Version Bounds Validation  [#80] is_tag_active Sentinel Return |
|  [#75] Calorie Sum Overflow Guard                                              |
+--------------------------------------------------------------------------------+
                                        |
                                        v
+--------------------------------------------------------------------------------+
|                   CLINICAL, VAX & DIAGNOSTIC LABS                              |
|                                                                                |
|  [#81] Vax Expiry > Admin Date             [#87] Lab Ref Range Validation      |
|  [#82] Vax Revocation Auth                 [#88] Lab Vet License Check         |
|  [#83] Prevent Duplicate Cert Anchor       [#89] Biomarker Zero Division Guard |
|  [#84] Cert Verification Equality Bug      [#90] Lab Privacy Enforcement       |
|  [#85] Configurable Upcoming Vax Window    [#91] Scope Lab Count Per Pet       |
|  [#86] Optimize Expiring Vax Query                                             |
+--------------------------------------------------------------------------------+
                                        |
                                        v
+--------------------------------------------------------------------------------+
|                   MEDICAL AMENDMENTS, ATTACHMENTS & CUSTODY                   |
|                                                                                |
|  [#92] Medical Record Amendment Author     [#98] Attachment Record Link Check  |
|  [#93] Diff Record Versions Bounds Panic   [#99] Max Attachments Bound Check   |
|  [#94] Soft Delete Caller Authorization    [#100] Allowed MIME Types Allowlist |
|  [#95] Note Update Event & Audit History   [#101] Attachment Privacy Access    |
|  [#96] Active Record Count On Purge        [#102] Attachment Storage Quota     |
|  [#97] Retention Period Floor Minimum      [#103] Custody Transfer Reason Log  |
|  [#104] Custody Block Timestamp Equality   [#105] Prior Owner Address Redaction|
|  [#106] Custody Log on Finalize Transfer   [#107] Atomic Batch Adoption Transfer|
+--------------------------------------------------------------------------------+
                                        |
                                        v
+--------------------------------------------------------------------------------+
|                   EMERGENCY, BREEDING & CROSS-CHAIN CELO                      |
|                                                                                |
|  [#108] Duplicate Emergency Responder      [#113] Duplicate Offspring Check    |
|  [#109] Remove Non-Existent Responder      [#114] Offspring Trait VRF / Hash   |
|  [#110] Reorder Contact Index Panic        [#115] Trait Probability Bounded    |
|  [#111] Emergency Access Log Pagination    [#116] Breeding Pair Active Check   |
|  [#112] Standardize Rate Limit Window     [#117] Breed Metadata Admin Auth    |
|  [#118] Celo Pet Existence Deactivation    [#122] Adoption Concurrent Signing  |
|  [#119] Celo Medical Record Pet Existence  [#123] Waive Waiting Period Auth    |
|  [#120] Celo Date Range Loop Bound         [#124] Transfer Min Timeout Bound   |
|  [#121] Celo Vet License Length Bounds     [#125] Prevent Reclaim Post-Accept  |
+--------------------------------------------------------------------------------+
```
