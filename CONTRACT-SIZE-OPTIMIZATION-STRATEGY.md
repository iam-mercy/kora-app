# Contract Size Optimization Strategy — Issue #2

## Problem Statement

KoraContract WASM is **219,161 bytes** (after wasm-opt optimization), exceeding Soroban's 128 KiB on-chain limit by **67%** (88,089 bytes over).

**Impact:** Contract cannot be deployed to any live Stellar network (testnet, mainnet, or local).

## Section Breakdown

| Section | Size | Details |
|---------|------|---------|
| Code (section 10) | 140,822 B | Main bytecode — largest contributor |
| contractspecv0 | 57,023 B | Mandatory spec, cannot strip |
| Data (section 11) | 14,744 B | Static data |
| Other (exports, types, meta) | ~6,500 B | Minimal |
| **Total** | **219,161 B** | vs 131,072 B cap |

## Solution: Contract Splitting Strategy

Split KoraContract into 3 focused contracts, each within the 128 KiB limit:

### Contract 1: **KoraPets** (~50 KiB)
**Responsibility:** Pet registry and lifecycle

**Functions:**
- `register_pet()` → create pet profile
- `update_pet()` → modify pet metadata
- `deactivate_pet()` → soft-delete pet
- `get_pet()` → retrieve pet data
- `list_pets_by_owner()` → query user's pets
- `add_photo()` → store photo hash
- `remove_photo()` → remove photo + decrement storage

**Storage:**
- Pet(pet_id) → Pet struct
- OwnerPets(owner) → Vec<u64> (pet IDs)
- PetStorageUsage(pet_id) → u64
- PhotoIndex → counter

**Rationale:** Pure data management, no medical logic.

---

### Contract 2: **KoraMedical** (~60 KiB)
**Responsibility:** Medical records and treatments

**Functions:**
- `create_medical_record()` → new record
- `correct_medical_record()` → amend diagnosis/treatment
- `add_vaccination()` → vaccination tracking
- `add_behavior_record()` → behavior notes
- `get_medical_records()` → list records
- `soft_delete_record()` → mark for deletion
- `purge_deleted_records()` → permanent cleanup
- `get_record_count()` → statistics

**Storage:**
- MedicalRecord(record_id) → MedicalRecord struct
- PetMedicalRecordIndex → linked list
- TombstoneRecords → deleted entries awaiting purge

**Rationale:** Medical logic concentrated here, clear scope.

---

### Contract 3: **KoraAuth** (~40 KiB)
**Responsibility:** Permissions, vet verification, and encryption

**Functions:**
- `register_vet()` → vet signup
- `verify_vet()` → admin verification
- `revoke_vet()` → disable vet access
- `get_encryption_key_for_user()` → user-derived encryption key
- `is_authorized_vet()` → access check
- `set_admin()` → admin management

**Storage:**
- Vet(vet_address) → Vet struct
- VetLicense(license_number) → vet_address
- Admins → Vec<Address>
- VetVerificationStatus → cache

**Rationale:** Security-sensitive functions isolated, easier to audit.

---

## Implementation Plan

### Phase 1: Preparation
1. Extract enums & types into shared library (`kora-common`)
   - ContractError
   - DataKey enums
   - All struct definitions (Pet, MedicalRecord, Vet)

2. Create library module structure:
   ```
   stellar-contracts/
   ├── src/
   │   ├── common.rs      (shared types/errors)
   │   ├── pets.rs        (KoraPets logic)
   │   ├── medical.rs     (KoraMedical logic)
   │   └── auth.rs        (KoraAuth logic)
   ├── contracts/
   │   ├── kora-pets/
   │   ├── kora-medical/
   │   └── kora-auth/
   ```

### Phase 2: Split Contracts
3. Create 3 separate Cargo packages
4. Move functions to respective contracts
5. Implement inter-contract calls (if needed)

### Phase 3: Integration
6. Update storage patterns:
   - Use versioned keys to avoid collisions
   - Separate storage paths: `pets/*`, `medical/*`, `auth/*`

7. Event harmonization:
   - All contracts emit events with same versioning scheme
   - Off-chain indexers handle multi-contract events

### Phase 4: Testing & Deployment
8. Unit tests for each contract (~30-40 tests each)
9. Integration tests for cross-contract calls
10. Staging → Testnet → Mainnet

---

## Size Estimates (Post-Split)

```
KoraPets (~50 KiB):
  - Pet struct + storage: ~8 KiB
  - Query logic: ~12 KiB
  - Photo management: ~8 KiB
  - Events/exports: ~22 KiB

KoraMedical (~60 KiB):
  - MedicalRecord struct: ~10 KiB
  - CRUD operations: ~20 KiB
  - Purge/cleanup logic: ~15 KiB
  - Vaccination/behavior: ~15 KiB

KoraAuth (~40 KiB):
  - Vet struct + storage: ~8 KiB
  - Authorization checks: ~12 KiB
  - Encryption key derivation: ~10 KiB
  - Admin management: ~10 KiB

Total: ~150 KiB (target: < 3 × 128 KiB = 384 KiB)
```

---

## Cross-Contract Communication

### KoraMedical → KoraPets
```rust
// Verify pet ownership before creating medical record
let pet = invoke_pet_contract(pet_id);
assert!(pet.owner == caller);
```

### KoraMedical → KoraAuth
```rust
// Verify vet authorization before accepting medical record
let is_authorized = invoke_auth_contract(vet_address);
assert!(is_authorized);
```

### KoraPets → KoraAuth
```rust
// Check admin permissions before pet modifications
let is_admin = invoke_auth_contract_admin_check(caller);
```

---

## Encryption Key Migration

**Current (Broken):** Admin-derived key → breaks on admin rotation
**New (Fixed):** User-derived key → stable per user

```rust
// Old (vulnerable)
fn get_encryption_key() -> Bytes {
    sha256(["kora:key", admin_address])  // ← vulnerable!
}

// New (secure)
fn get_encryption_key_for_user(user: Address) -> Bytes {
    sha256(["kora:user-key:v2", user_address, contract_address])
}
```

---

## Benefits

✅ **Deployability** — All contracts fit within 128 KiB
✅ **Modularity** — Each contract has clear responsibility
✅ **Auditability** — Smaller contracts easier to review
✅ **Upgradability** — Update one contract without touching others
✅ **Performance** — Reduced memory footprint per contract
✅ **Security** — User-derived keys eliminate admin-rotation vuln

---

## Risks & Mitigations

| Risk | Mitigation |
|------|-----------|
| Cross-contract call latency | Cache frequently-accessed data locally |
| Data inconsistency | Use versioned storage keys, audit logging |
| Admin coordination | Unified admin interface that calls all 3 |
| Breaking existing dApps | Provide adapters for v1 → v2 migration |

---

## Timeline

- **Week 1:** Extract types → library, create 3 contract stubs
- **Week 2:** Migrate functions, implement cross-contract calls
- **Week 3:** Testing, integration tests, optimization pass
- **Week 4:** Staging → Testnet validation → Mainnet deploy

---

## Rollback Plan

1. Keep v1 contracts on-chain during transition
2. Adapter contract bridges v1 and v2 calls
3. Gradual migration of dApp clients to v2
4. Deprecate v1 after 90-day stabilization period
