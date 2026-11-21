# Smart Contract API

## View Functions (pure reads — no storage writes or event emissions)

The following functions are guaranteed to have no side effects. They do not write to storage, emit events, or update access timestamps.

| Function | Description |
|---|---|
| `get_pet` | Returns decrypted pet profile; enforces privacy level |
| `get_pet_data` | Returns minimal pet data (name, species, breed) |
| `get_pet_age` | Computes age from birthday timestamp |
| `get_pet_full_profile` | Returns full profile with vaccination/medication summary |
| `get_pet_full_profile_batch` | **[Batch]** Returns pet profile, owner, active consents, and latest medical record in one call |
| `get_pet_health_summary` | **[Batch]** Returns latest vaccination, lab result, and active insurance in one call |
| `is_pet_active` | Returns whether a pet is active |
| `get_pet_owner` | Returns the owner address for a pet |
| `get_pet_photos` | Returns all photo hashes for a pet |
| `get_pet_photo_count` | Returns photo count |
| `get_pet_photos_paginated` | Returns paginated photo hashes |
| `get_total_pets` | Returns global pet count |
| `get_species_count` | Returns pet count for a species |
| `get_active_pets_count` | Returns count of active pets |
| `get_vet_stats` | Returns vet treatment/vaccination statistics |
| `get_vet_treatment_history` | Returns paginated treatment history for a vet |
| `get_vet_vaccination_history` | Returns paginated vaccination history for a vet |
| `get_pets_overdue_vaccinations` | Returns pet IDs with overdue vaccinations |
| `get_admins` | Returns list of admin addresses |
| `get_admin_threshold` | Returns multisig approval threshold |
| `get_verified_vets` | Returns paginated list of verified vets |
| `is_vet_registered` | Returns whether a vet address is registered |
| `is_verified_vet` | Returns whether a vet is verified |
| `get_vet` | Returns vet record by address |
| `get_vet_by_license` | Returns vet record by license number |
| `get_vaccinations` | Returns a vaccination record by ID |
| `get_vaccination_history` | Returns paginated vaccination history for a pet |
| `get_upcoming_vaccinations` | Returns upcoming vaccinations for a pet |
| `is_vaccination_current` | Returns whether a vaccine type is current |
| `get_medical_record` | Returns a medical record by ID |
| `get_pet_medical_records` | Returns paginated medical records for a pet |
| `search_medical_records` | Returns filtered medical records |
| `get_attachments` | Returns attachments for a record (Malicious hidden from non-admins) |
| `get_attachment_count` | Returns attachment count for a record |
| `is_owner_registered` | Returns whether an owner address is registered |
| `get_access_logs` | Returns access logs for a pet (owner/admin only) |
| `take_statistics_snapshot` | Captures and returns a statistics snapshot (counts, totals) |
| `export_access_log` | Exports the access log for a pet (owner/admin only) |

> **Audit note:** All `log_access` (storage write) calls were removed from the above functions. Write functions (`add_medical_record`, `update_pet_profile`, `grant_access`, `revoke_access`, `add_attachment`, etc.) retain their access log writes.

## Grooming, Nutrition, and Rating Functions

| Function | Description |
|---|---|
| `book_grooming_slot` | Books a grooming appointment slot for a pet |
| `rate_groomer` | Records a rating for a groomer after a completed appointment |
| `set_nutrition_version` | Sets the nutrition plan version for a pet |

## Medical record retrieval authorization

The backend endpoint `GET /pets/{petId}/medical-records` is an authenticated
adapter around Soroban's `get_pet_medical_records(pet_id, offset, limit)`.
Clients must send `Authorization: Bearer <signature>` and `caller_address`.
The backend verifies the signature and performs the privacy/access check before
calling the read; `caller_address` must not be trusted as an unsigned identity.

- **Public pets:** records may be returned after signature verification.
- **Restricted pets:** the caller needs an active access grant with read access.
- **Private pets:** only the pet owner may retrieve records.
- **401 Unauthorized:** missing, malformed, or invalid signature.
- **403 Forbidden:** authenticated caller lacks access to the pet.

Emergency access is a separate, audited flow. Supply `emergency_caller` and a
non-empty `reason` when requesting the emergency view. The backend must verify
that caller's authorization, pass the authenticated caller to the contract, and
record the access; these parameters do not bypass contract checks.

---

## Smart Contracts

### Main contract

The primary contract lives in `stellar-contracts/src/lib.rs` and exposes functionality around:

- pet registration and profile management
- ownership and access grants
- vet registration and verification
- medical records, lab results, and attachments
- insurance, grooming, nutrition, activity, and behavior records
- emergency contacts and emergency access logs
- multisig administration and upgrade proposals

### Transfer and adoption contract

The transfer-focused contract lives in `stellar-contracts/contracts/pet-transfer-adoption/src/lib.rs` and handles:

- pet creation
- transfer initiation and acceptance
- transfer cancellation and reclaim flows
- ownership history tracking

---

## Batch Read Operations

Batch read operations reduce the number of round trips required to fetch related data. These functions aggregate multiple data points into a single call while respecting access control.

### `get_pet_full_profile_batch`

Returns comprehensive pet information including profile, owner, active consents, and latest medical record.

**Signature:**
```rust
pub fn get_pet_full_profile_batch(
    env: Env,
    pet_id: u64,
    caller: Address,
) -> Option<PetFullProfileBatch>
```

**Returns:**
```rust
pub struct PetFullProfileBatch {
    pub profile: PetProfile,
    pub owner: Address,
    pub active_consents: Vec<Consent>,
    pub latest_medical_record: Option<MedicalRecord>,
}
```

**Access Control:**
- **Public pets**: Accessible to anyone
- **Restricted pets**: Requires at least Basic access grant
- **Private pets**: Only accessible to owner

**Use Cases:**
- Dashboard views showing complete pet information
- Profile pages requiring owner and consent data
- Applications needing pet data with medical history

**Example:**
```rust
let batch = client.get_pet_full_profile_batch(&pet_id, &caller);
if let Some(data) = batch {
    // Access all data in one call
    let profile = data.profile;
    let owner = data.owner;
    let consents = data.active_consents;
    let latest_record = data.latest_medical_record;
}
```

### `get_pet_health_summary`

Returns health-related information including latest vaccination, lab result, and active insurance policy.

**Signature:**
```rust
pub fn get_pet_health_summary(
    env: Env,
    pet_id: u64,
    caller: Address,
) -> Option<PetHealthSummary>
```

**Returns:**
```rust
pub struct PetHealthSummary {
    pub pet_id: u64,
    pub latest_vaccination: Option<Vaccination>,
    pub latest_lab_result: Option<LabResult>,
    pub active_insurance_policy: Option<InsurancePolicy>,
}
```

**Access Control:**
- **Public pets**: Accessible to anyone
- **Restricted pets**: Requires at least Basic access grant
- **Private pets**: Only accessible to owner

**Use Cases:**
- Health dashboard views
- Veterinary appointment preparation
- Insurance claim verification
- Quick health status checks

**Example:**
```rust
let summary = client.get_pet_health_summary(&pet_id, &caller);
if let Some(health) = summary {
    // Check vaccination status
    if let Some(vax) = health.latest_vaccination {
        // Display vaccination info
    }

    // Check lab results
    if let Some(lab) = health.latest_lab_result {
        // Display lab results
    }

    // Check insurance coverage
    if let Some(policy) = health.active_insurance_policy {
        // Display insurance info
    }
}
```

### Performance Benefits

**Without Batch Operation

/* … truncated 1749 chars — edit only what you need near the top … */
