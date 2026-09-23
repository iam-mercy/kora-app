# Kora App Contract Error Codes

## Overview

This document provides a comprehensive reference for all error codes in the Kora App Soroban smart contract, including multi-language support for error messages.

Every error code below comes from an `#[contracterror]` enum in the repository. The
source of truth for each enum is:

| Enum | Source file | Used by |
|------|-------------|---------|
| `ContractError` | `stellar-contracts/src/lib.rs` | `KoraContract` (main contract) |
| `KoraError` | `stellar-contracts/src/lib.rs` | `KoraContract` search/lineage validation |
| `ContractError` | `stellar-contracts/contracts/pet-transfer-adoption/src/lib.rs` | `pet-transfer-adoption` contract |
| `EscrowError` | `stellar-contracts/contracts/pet-transfer-adoption/src/escrow.rs` | escrow module of `pet-transfer-adoption` |
| `ContractError` | `stellar-contracts/contracts/pet-transfer-adoption/src/vet_registry.rs` | vet registry module of `pet-transfer-adoption` |

> The discriminants in these enums are **not** a stable API. They were re-assigned
> when the enum bodies were re-ordered, so the numbers are no longer grouped by
> domain and client code must read the value from the tables below rather than
> assuming a sequential layout.

## Multi-Language Error Registry (Issue #684)

The contract supports human-readable error messages in multiple languages through an error registry system. Error messages can be queried programmatically and managed by multisig admins.

### Supported Languages

- **English** (`en`)
- **Spanish** (`es`)

Additional languages can be added by admins using the `set_error_message` function.

### API Functions

#### Query Functions

**`get_error_message(error_code: u32, language: String) -> Option<String>`**
- Returns the error message for a specific error code and language
- Returns `None` if no message is found for that code/language combination

**`get_supported_languages() -> Vec<String>`**
- Returns a list of all supported languages in the error registry

#### Admin Functions (Multisig Admin Only)

**`set_error_message(admin: Address, error_code: u32, language: String, message: String)`**
- Sets an error message for a specific error code and language
- Automatically adds the language to supported languages list
- Validates: language length (1-10 chars, otherwise `InvalidInput = 12`),
  message length (1-500 chars, otherwise `InputStringTooLong = 8`)

**`batch_set_error_messages(admin: Address, messages: Vec<ErrorMessage>)`**
- Sets multiple error messages at once
- More efficient for bulk operations
- Each message must pass validation

**`initialize_error_messages(admin: Address)`**
- Initializes default error messages in English and Spanish
- Covers a small subset of codes (see [Seeded messages](#seeded-messages))
- Should be called once after contract deployment
- Authorization is enforced by `batch_set_error_messages`, which it delegates to

**`remove_error_message(admin: Address, error_code: u32, language: String)`**
- Removes an error message for a specific error code and language
- Used for cleanup or corrections

---

## Error Code Reference

### `ContractError` — main contract

The enum declares 50 variants. 47 of them are raised from
`stellar-contracts/src/lib.rs`; the three that are declared but never raised
(`AdminNotInitialized`, `InvalidCallerNonce`, `RecordAlreadyDeleted`) are marked
below. Discriminants `42` and `47`-`79` are unused.

#### Admin, multisig and authorization

<!-- check-error-codes: enum=ContractError file=stellar-contracts/src/lib.rs -->

| Code | Variant | Raised by | Recommended client action |
|-----:|---------|-----------|---------------------------|
| 1 | `AdminAlreadyApproved` | `approve_proposal`, `confirm_init`, `set_threshold` | This admin already approved the proposal. Do not re-submit; wait for the remaining quorum. |
| 2 | `AdminAlreadySet` | `init_admin`, `init_multisig`, `propose_init` | The contract is already initialized. Read the current admin/threshold state instead of initializing again. |
| 3 | `AdminNotInitialized` | *(declared, never raised)* | Reserved. Treat as an uninitialized-admin state if it ever surfaces. |
| 4 | `AdminsNotSet` | `require_admin`, `require_admin_auth`, `remove_admin`, `set_threshold` | No admin list is configured. Initialize the admin set before calling admin-gated entrypoints. |
| 17 | `InvalidThreshold` | `init_multisig`, `propose_init`, `remove_admin`, `set_threshold` | The threshold must be `>= 1` and `<=` the number of admins. Correct the value and retry. |
| 18 | `InvokerNotInAdminList` | `init_multisig` | The supplied initial admin is not part of the proposed admin list. Align the two lists. |
| 20 | `NoAdminsConfigured` | `execute_proposal`, `require_admin` | Admin state is empty. Re-initialize the admin set. |
| 21 | `NotAnAdmin` | `add_breed_metadata`, `approve_proposal`, `approve_upgrade_proposal`, `confirm_init`, `propose_upgrade`, `rollback_upgrade`, `set_quorum_percent`, `update_breed_metadata`, and other admin-gated entrypoints | The caller is not in the admin list. Surface a "permission denied" state and do not retry. |
| 26 | `ThresholdNotMet` | `activate_init`, `execute_proposal` | Not enough approvals to execute. Collect further approvals. |
| 28 | `Unauthorized` | `assign_arbitrator`, `delete_medical_record`, `export_access_log`, `get_emergency_audit`, `notify_emergency_contacts`, `purge_deleted_records`, `require_vet_specialization`, `rotate_record_encryption`, and other authenticated entrypoints | `require_auth()` failed, or the caller does not own the resource. Ask the user to sign with the correct account. |
| 45 | `QuorumNotMet` | `execute_proposal` | The admin quorum percentage has not been reached. Collect more approvals. |

#### Upgrade and proposal governance

| Code | Variant | Raised by | Recommended client action |
|-----:|---------|-----------|---------------------------|
| 38 | `ProposalAlreadyExecuted` | `approve_upgrade_proposal`, `execute_upgrade` | The proposal is consumed. Create a new proposal for further changes. |
| 40 | `RollbackWindowExpired` | `rollback_upgrade` | The rollback window has elapsed. Roll back through a new upgrade proposal instead. |
| 41 | `NoPreviousUpgrade` | `rollback_upgrade` | There is no previous version to roll back to. |
| 43 | `ProposalExpired` | `approve_upgrade_proposal`, `execute_upgrade` | The proposal passed its expiry. Propose it again. |
| 44 | `ProposalNotApproved` | `execute_upgrade` | The proposal does not have enough approvals yet. |
| 80 | `ProposalNotFound` | `approve_upgrade_proposal`, `execute_upgrade` | Unknown proposal id. Re-read the proposal list. |

#### Pets, tags and storage quota

| Code | Variant | Raised by | Recommended client action |
|-----:|---------|-----------|---------------------------|
| 22 | `NotPetOwner` | `batch_transfer`, `book_grooming_slot`, `get_emergency_access_logs` | The caller is not the pet owner. Authenticate as the owner or drop the request. |
| 23 | `PetAlreadyHasLinkedTag` | `link_tag_to_pet` | Unlink the existing tag before linking a new one. |
| 24 | `PetNotFound` | most pet-scoped entrypoints, e.g. `add_medical_record`, `add_vaccination`, `log_feeding`, `link_tag_to_pet`, `set_diet_plan`, `verify_custody_chain` | The pet id does not exist. Refresh the pet list before retrying. |
| 25 | `StorageQuotaExceeded` | `add_attachment`, `get_record_encrypted_payload`, `increment_pet_storage` | The pet's storage quota is full. Ask an admin to raise it with `set_pet_storage_quota` / `set_global_storage_quota`, or purge records first. |

#### Veterinarians, certificates and vaccinations

| Code | Variant | Raised by | Recommended client action |
|-----:|---------|-----------|---------------------------|
| 6 | `CertificateAlreadyAnchored` | `anchor_certificate` | The certificate hash is already anchored. Reuse the existing anchoring. |
| 11 | `InvalidCertificateHash` | `anchor_certificate` | The hash is malformed or empty. Recompute it and retry. |
| 19 | `LicenseAlreadyRegistered` | `register_vet` | That veterinary license id is already taken. Use the registered vet profile. |
| 29 | `VaccinationNotFound` | `anchor_certificate`, `revoke_vaccination_certificate` | Unknown vaccination id. Refresh the vaccination list. |
| 30 | `VetAlreadyRegistered` | `register_vet` | The address already has a vet profile. Update the existing profile instead. |
| 31 | `VetNotFound` | `anchor_certificate`, `batch_verify_vets`, `register_vet_specializations`, `update_clinic_info` | Unknown vet. Register the vet before calling vet-scoped entrypoints. |
| 32 | `VetNotVerified` | `anchor_certificate` | The vet is registered but not verified. Ask an admin to verify the vet. |
| 33 | `VeterinarianNotVerified` | `register_vet_specializations` | Same condition as `VetNotVerified`, raised by the specializations entrypoint. |

#### Input validation and limits

| Code | Variant | Raised by | Recommended client action |
|-----:|---------|-----------|---------------------------|
| 5 | `BatchTooLarge` | `batch_transfer`, `batch_verify_vets` | Split the batch into smaller chunks and retry. |
| 8 | `InputStringTooLong` | `register_pet_owner`, `set_error_message`, `batch_set_error_messages` | Shorten the string. Error messages are capped at 500 characters. |
| 9 | `InvalidBreed` | `validate_breed` | The breed string is empty or too long. Fix the payload. |
| 12 | `InvalidInput` | `add_attachment`, `add_behavior_record`, `add_nutrition_plan`, `advance_schedule`, `batch_transfer`, `delete_medical_record`, `log_feeding`, `rate_groomer`, `register_pet`, `register_subscription`, `set_error_message`, `set_quorum_percent`, `validate_len`, and other validators | A generic range or format check failed — for example an empty language code in `set_error_message`, or a non-positive interval. Read the entrypoint's validation rules and correct the payload. |
| 13 | `InvalidIpfsHash` | `validate_ipfs_hash` | The IPFS hash is not a valid CID. Re-upload the content and retry. |
| 14 | `InvalidPetName` | `validate_pet_name` | The pet name is empty or too long. |
| 15 | `InvalidRating` | `rate_groomer` | The rating is outside 1-5. Clamp before submitting. |
| 16 | `InvalidState` | `activate_init`, `add_offspring`, `approve_proposal`, `confirm_init`, `execute_proposal`, `propose_init`, `rate_groomer`, `set_threshold` | The entrypoint was called in the wrong lifecycle state. Re-read the resource state first. |
| 27 | `TooManyItems` | `amend_medical_record`, `register_subscription` | The request exceeds a per-record item limit. Trim the payload. |

#### Nonces and replay protection

| Code | Variant | Raised by | Recommended client action |
|-----:|---------|-----------|---------------------------|
| 10 | `InvalidCallerNonce` | *(declared, never raised)* | Reserved. Replay failures surface as `InvalidNonce = 39`. |
| 39 | `InvalidNonce` | `consume_caller_nonce` | The nonce is stale or reused. Fetch the current nonce and sign a new request. |

#### Activity, breeding and grooming

| Code | Variant | Raised by | Recommended client action |
|-----:|---------|-----------|---------------------------|
| 7 | `CounterOverflow` | `add_vaccination`, `increment_pet_storage`, `log_feeding`, `register_pet`, `update_vet_stats` | An internal counter is saturated. Report the condition; retrying will not help. |
| 34 | `SlotAlreadyBooked` | `book_grooming_slot` | The groomer slot is taken. Pick another slot. |
| 35 | `DuplicateActivity` | `add_activity_record` | The same activity was already recorded inside the idempotency window. Treat as success and do not retry. |
| 36 | `InbreedingThresholdExceeded` | `register_breeding_pair` | The pair's coefficient of inbreeding is above the allowed threshold. Choose different parents. |
| 37 | `SelfBreeding` | `register_breeding_pair` | Sire and dam are the same pet. |

#### Medical-record soft delete and retention

| Code | Variant | Raised by | Recommended client action |
|-----:|---------|-----------|---------------------------|
| 160 | `AlreadyDeleted` | `delete_medical_record` | The record was already soft-deleted. Treat as idempotent success. |
| 161 | `RecordAlreadyDeleted` | *(declared, never raised)* | Superseded by `AlreadyDeleted = 160`. |
| 162 | `RetentionPeriodNotMet` | `purge_deleted_records` | The regulatory retention window has not elapsed. Wait until the configured period passes. |
| 163 | `RecordNotFound` | `amend_medical_record` | Unknown medical record id, or the record is not readable in its current state. |

#### Rate limiting

| Code | Variant | Raised by | Recommended client action |
|-----:|---------|-----------|---------------------------|
| 46 | `RateLimitExceeded` | `notify_emergency_contacts` | The per-caller notification window is exhausted. Back off until the window rolls over. |

### Quick index

| Code | Variant | Code | Variant |
|-----:|---------|-----:|---------|
| 1 | `AdminAlreadyApproved` | 26 | `ThresholdNotMet` |
| 2 | `AdminAlreadySet` | 27 | `TooManyItems` |
| 3 | `AdminNotInitialized` | 28 | `Unauthorized` |
| 4 | `AdminsNotSet` | 29 | `VaccinationNotFound` |
| 5 | `BatchTooLarge` | 30 | `VetAlreadyRegistered` |
| 6 | `CertificateAlreadyAnchored` | 31 | `VetNotFound` |
| 7 | `CounterOverflow` | 32 | `VetNotVerified` |
| 8 | `InputStringTooLong` | 33 | `VeterinarianNotVerified` |
| 9 | `InvalidBreed` | 34 | `SlotAlreadyBooked` |
| 10 | `InvalidCallerNonce` | 35 | `DuplicateActivity` |
| 11 | `InvalidCertificateHash` | 36 | `InbreedingThresholdExceeded` |
| 12 | `InvalidInput` | 37 | `SelfBreeding` |
| 13 | `InvalidIpfsHash` | 38 | `ProposalAlreadyExecuted` |
| 14 | `InvalidPetName` | 39 | `InvalidNonce` |
| 15 | `InvalidRating` | 40 | `RollbackWindowExpired` |
| 16 | `InvalidState` | 41 | `NoPreviousUpgrade` |
| 17 | `InvalidThreshold` | 43 | `ProposalExpired` |
| 18 | `InvokerNotInAdminList` | 44 | `ProposalNotApproved` |
| 19 | `LicenseAlreadyRegistered` | 45 | `QuorumNotMet` |
| 20 | `NoAdminsConfigured` | 46 | `RateLimitExceeded` |
| 21 | `NotAnAdmin` | 80 | `ProposalNotFound` |
| 22 | `NotPetOwner` | 160 | `AlreadyDeleted` |
| 23 | `PetAlreadyHasLinkedTag` | 161 | `RecordAlreadyDeleted` |
| 24 | `PetNotFound` | 162 | `RetentionPeriodNotMet` |
| 25 | `StorageQuotaExceeded` | 163 | `RecordNotFound` |

### `KoraError` — keyword search and lineage validation

Only `KeywordTooLong` is raised by the current implementation; the other four
variants are part of the same enum but have no call site.

<!-- check-error-codes: enum=KoraError file=stellar-contracts/src/lib.rs -->

| Code | Variant | Raised by | Recommended client action |
|-----:|---------|-----------|---------------------------|
| 1 | `NonceReused` | *(declared, never raised)* | Reserved. Nonce replay surfaces as `ContractError::InvalidNonce = 39`. |
| 2 | `SelfLineage` | *(declared, never raised)* | Reserved for a pet registered as its own ancestor. |
| 3 | `CircularLineage` | *(declared, never raised)* | Reserved for a lineage edge that would create a cycle. |
| 4 | `KeywordTooLong` | `search_by_keyword` | The keyword exceeds `MAX_SEARCH_KEYWORD_LEN` (32 characters). Shorten the query. |
| 5 | `TooManySearchTokens` | *(declared, never raised)* | Reserved for over-long token lists. |

### `pet-transfer-adoption` — contract and module errors

`stellar-contracts/contracts/pet-transfer-adoption` is a separate contract with its
own enums. Their discriminants are independent of the main contract, so a `3` from
the escrow module means something different from a `3` from `KoraContract`.

<!-- check-error-codes: enum=ContractError file=stellar-contracts/contracts/pet-transfer-adoption/src/lib.rs -->

| Code | Variant | Meaning |
|-----:|---------|---------|
| 1 | `PetNotFound` | The pet id does not exist in the adoption contract. |
| 2 | `Unauthorized` | The caller may not perform this transfer action. |
| 3 | `TransferAlreadyPending` | The pet already has an open transfer. Cancel it or wait for it to expire. |
| 4 | `NoPendingTransfer` | There is no pending transfer to act on. |
| 5 | `InvalidRecipient` | The recipient is missing or equal to the current owner. |
| 6 | `EmptyOwnershipHistory` | The ownership history is empty. |
| 7 | `MissingOwnershipRecord` | An ownership entry referenced by the history is absent. |
| 8 | `TransferNotExpired` | The transfer is still inside its confirmation window. |
| 9 | `StaleCancellation` | The cancellation targets a superseded transfer. |
| 10 | `EmptyBatch` | The batch contained no items. |
| 11 | `BatchOwnerMismatch` | The batch mixes pets with different owners. |
| 12 | `NoEscrowedTransfer` | The transfer has no escrow entry. |
| 13 | `DisputeWindowNotElapsed` | The dispute window is still open. |
| 14 | `TransferAlreadyDisputed` | The transfer is already under dispute. |
| 15 | `AlreadyInitialized` | The contract is already initialized. |
| 16 | `InvalidThreshold` | The multisig threshold is out of range. |
| 17 | `NotMultisigAdmin` | The caller is not a multisig admin. |
| 18 | `ThresholdNotMet` | Not enough admin approvals. |
| 19 | `UntrustedContract` | The calling contract is not on the trusted list. |
| 20 | `NoPendingAdoption` | There is no pending adoption for this pet. |
| 21 | `WaitingPeriodNotElapsed` | The adoption waiting period has not finished. |
| 22 | `AdoptionAlreadyCompleted` | The adoption is already final. |
| 23 | `AdoptionNotConfigurable` | The contract does not allow adoption configuration. |
| 24 | `InvalidWaitingPeriod` | The waiting period is out of bounds. |
| 25 | `AdoptionConfigNotFound` | The adoption configuration is missing. |
| 26 | `BatchTooLarge` | The batch exceeds the allowed size. |
| 27 | `InvalidBatch` | The batch payload is malformed. |
| 28 | `OrganizationApprovalRequired` | A shelter or organization approval is still missing. |
| 29 | `AdoptionRejected` | The adoption was rejected. |
| 30 | `InvalidApprover` | The approver is not permitted. |
| 31 | `InvalidTimeoutDays` | The timeout is out of bounds. |
| 32 | `AdopterApprovalRequired` | The adopter has not approved yet. |
| 33 | `InputStringTooLong` | A string input exceeds its limit. |
| 34 | `AdoptionNotExpired` | The adoption request has not expired yet. |

<!-- check-error-codes: enum=EscrowError file=stellar-contracts/contracts/pet-transfer-adoption/src/escrow.rs -->

| Code | Variant | Meaning |
|-----:|---------|---------|
| 1 | `FeeBpsTooHigh` | The platform fee in basis points exceeds the allowed maximum. |
| 2 | `InvalidAmount` | The escrowed amount must be positive. |
| 3 | `EscrowAlreadyExists` | An escrow entry already exists for this transfer. |
| 4 | `EscrowNotFound` | No escrow entry exists for this transfer. |
| 5 | `InvalidEscrowState` | The escrow is in a state that forbids the requested transition. |
| 6 | `Unauthorized` | The caller is not a party to this escrow. |

<!-- check-error-codes: enum=ContractError file=stellar-contracts/contracts/pet-transfer-adoption/src/vet_registry.rs -->

| Code | Variant | Meaning |
|-----:|---------|---------|
| 0 | `AlreadyInitialized` | The vet registry is already initialized. |
| 1 | `Unauthorized` | The caller is not the registry admin. |
| 2 | `VetAlreadyRegistered` | The address already has a vet profile. |
| 3 | `VetNotFound` | Unknown vet. |
| 4 | `LicenseAlreadyUsed` | The license id is already registered. |
| 5 | `VetNotVerified` | The vet has not been verified. |
| 6 | `InputTooLong` | A string input exceeds its limit. |
| 7 | `VetAlreadyVerified` | The vet is already verified. |

Four variants of this contract are declared but never raised: `InvalidRecipient`,
`InvalidWaitingPeriod` and `ThresholdNotMet` in the contract itself, and
`VetNotVerified` in its vet registry. They remain part of the published ABI.

### Seeded messages

`initialize_error_messages` writes the pairs below into the on-chain registry.
The table mirrors the literals in `stellar-contracts/src/lib.rs`.

<!-- check-error-codes: seed-messages -->

| Code | Language | Seeded message |
|-----:|----------|----------------|
| 1 | `en` | Unauthorized access |
| 2 | `en` | Admin not initialized |
| 3 | `en` | Pet not found |
| 4 | `en` | Veterinarian not found |
| 5 | `en` | Veterinarian not verified |
| 6 | `en` | Veterinarian already registered |
| 7 | `en` | License already registered |
| 8 | `en` | Input string too long |
| 160 | `en` | Storage quota exceeded |
| 1 | `es` | Acceso no autorizado |
| 2 | `es` | Administrador no inicializado |
| 3 | `es` | Mascota no encontrada |
| 4 | `es` | Veterinario no encontrado |
| 5 | `es` | Veterinario no verificado |
| 6 | `es` | Veterinario ya registrado |
| 7 | `es` | Licencia ya registrada |
| 8 | `es` | Cadena de entrada demasiado larga |
| 160 | `es` | Cuota de almacenamiento excedida |

> **Known inconsistency.** These literals predate the discriminant re-assignment,
> so the text no longer matches the variant that owns the code. Code `1` is
> `AdminAlreadyApproved` but is seeded with *"Unauthorized access"* (which is
> `Unauthorized = 28`), code `3` is `AdminNotInitialized` but is seeded with
> *"Pet not found"* (which is `PetNotFound = 24`), and code `160` is
> `AlreadyDeleted` but is seeded with *"Storage quota exceeded"* (which is
> `StorageQuotaExceeded = 25`). Until the seed table is corrected, clients should
> treat the registry text as advisory and switch on the numeric code.

---

## Keeping this document in sync

`scripts/check_error_codes.py` extracts every `#[contracterror]` enum referenced
above and compares it with the tables in this document. Run it after changing an
error enum:

```bash
python3 scripts/check_error_codes.py
```

It exits non-zero when a code is missing here, when the two disagree on a variant
name, when the enum owns a code this document does not list, or when the
seeded-message table no longer matches `initialize_error_messages`. Variants that
are declared but never raised are reported as warnings and do not fail the run.

---

## Usage Examples

### Example 1: Query Error Message

```rust
// Get error message in English
let message = client.get_error_message(&24, &String::from_str(&env, "en"));
// Returns: Some("Pet not found")

// Get error message in Spanish
let message = client.get_error_message(&24, &String::from_str(&env, "es"));
// Returns: Some("Mascota no encontrada")

// Get error message for a code that was never seeded
let message = client.get_error_message(&46, &String::from_str(&env, "en"));
// Returns: None — fall back to the numeric code

// Get error message for an unsupported language
let message = client.get_error_message(&24, &String::from_str(&env, "fr"));
// Returns: None
```

### Example 2: Set Custom Error Message

```rust
// Admin sets a custom error message for RateLimitExceeded (46)
client.set_error_message(
    &admin,
    &46,
    &String::from_str(&env, "en"),
    &String::from_str(&env, "Too many emergency notifications, try again later")
);
```

### Example 3: Add New Language

```rust
// Admin adds French translations
client.set_error_message(
    &admin,
    &24,
    &String::from_str(&env, "fr"),
    &String::from_str(&env, "Animal de compagnie non trouvé")
);

client.set_error_message(
    &admin,
    &25,
    &String::from_str(&env, "fr"),
    &String::from_str(&env, "Quota de stockage dépassé")
);
```

### Example 4: Batch Initialize Messages

```rust
// Initialize default messages in English and Spanish
client.initialize_error_messages(&admin);

// Check supported languages
let languages = client.get_supported_languages();
// Returns: ["en", "es"]
```

### Example 5: Handle Errors with Messages

```rust
// Catch error and display localized message
match client.try_add_medical_record(...) {
    Ok(record_id) => println!("Record added: {}", record_id),
    Err(error) => {
        let error_code = error as u32;
        let user_language = get_user_language(); // e.g., "es"
        
        if let Some(message) = client.get_error_message(&error_code, &user_language) {
            println!("Error: {}", message);
        } else {
            println!("Error code: {}", error_code);
        }
    }
}
```

---

## Best Practices

### For Developers

1. **Always Initialize Error Messages**
   - Call `initialize_error_messages()` after contract deployment
   - Ensures basic error messages are available

2. **Provide Fallback Logic**
   - Check if message exists before displaying
   - Fall back to error code if message not found
   - Consider default language (English) as fallback
   - Switch on the numeric code for control flow — registry text is display-only

3. **Use Batch Operations**
   - Use `batch_set_error_messages()` for multiple messages
   - More efficient than individual calls

4. **Keep Messages Concise**
   - Maximum 500 characters per message
   - Focus on clarity and actionability

### For Administrators

1. **Maintain Consistency**
   - Keep messages consistent across languages
   - Use similar tone and terminology

2. **Regular Updates**
   - Add messages for new error codes
   - Update messages when functionality changes

3. **Language Coverage**
   - Prioritize languages based on user base
   - Ensure critical errors are translated first

4. **Test Translations**
   - Verify translations are accurate
   - Consider cultural context

---

## Error Message Guidelines

### Writing Good Error Messages

1. **Be Specific**
   - ❌ "Error occurred"
   - ✅ "Pet not found"

2. **Be Actionable**
   - ❌ "Invalid input"
   - ✅ "Input string too long (max 500 characters)"

3. **Be User-Friendly**
   - ❌ "Unauthorized"
   - ✅ "You don't have permission to perform this action"

4. **Be Consistent**
   - Use consistent terminology across messages
   - Follow the same structure

### Translation Guidelines

1. **Maintain Meaning**
   - Preserve the original intent
   - Don't add or remove information

2. **Cultural Adaptation**
   - Consider cultural context
   - Use appropriate formality level

3. **Technical Accuracy**
   - Keep technical terms consistent
   - Use standard translations for technical concepts

4. **Length Considerations**
   - Some languages are more verbose
   - Ensure translations fit within 500 character limit

---

## API Reference Summary

### Query Functions (Public)

| Function | Parameters | Returns | Description |
|----------|------------|---------|-------------|
| `get_error_message` | `error_code: u32, language: String` | `Option<String>` | Get error message for code and language |
| `get_supported_languages` | None | `Vec<String>` | Get list of supported languages |

### Admin Functions (Multisig Admin Only)

| Function | Parameters | Returns | Description |
|----------|------------|---------|-------------|
| `set_error_message` | `admin: Address, error_code: u32, language: String, message: String` | None | Set single error message |
| `batch_set_error_messages` | `admin: Address, messages: Vec<ErrorMessage>` | None | Set multiple error messages |
| `initialize_error_messages` | `admin: Address` | None | Initialize default English and Spanish messages |
| `remove_error_message` | `admin: Address, error_code: u32, language: String` | None | Remove error message |

---

## Events

### ErrorMessageSet
**Emitted when:** A single error message is set  
**Topics:** `("ErrorMessageSet", error_code)`  
**Data:** `(language, message)`

### ErrorMessagesBatchSet
**Emitted when:** Multiple error messages are set  
**Topics:** `("ErrorMessagesBatchSet")`  
**Data:** `count` (number of messages set)

### ErrorMessageRemoved
**Emitted when:** An error message is removed  
**Topics:** `("ErrorMessageRemoved", error_code)`  
**Data:** `language`

---

## Storage Structure

### ErrorRegistryKey Enum

```rust
pub enum ErrorRegistryKey {
    ErrorMessage((u32, String)),  // (error_code, language) -> message
    SupportedLanguages,            // Vec<String> of supported languages
}
```

### ErrorMessage Struct

```rust
pub struct ErrorMessage {
    pub code: u32,
    pub language: String,
    pub message: String,
}
```

---

## Migration Guide

### Adding Error Messages to Existing Contract

1. **Deploy Updated Contract**
   - Contract includes error registry system

2. **Initialize Default Messages**
   ```rust
   client.initialize_error_messages(&admin);
   ```

3. **Add Custom Messages (Optional)**
   ```rust
   client.set_error_message(&admin, &custom_code, &lang, &message);
   ```

4. **Verify Messages**
   ```rust
   let languages = client.get_supported_languages();
   let message = client.get_error_message(&code, &lang);
   ```

---

## Security Considerations

1. **Admin-Only Modification**
   - Only multisig admins can set/remove messages
   - Prevents unauthorized message manipulation

2. **Input Validation**
   - Language code: 1-10 characters
   - Message: 1-500 characters
   - Prevents storage abuse

3. **No Sensitive Information**
   - Error messages should not contain sensitive data
   - Keep messages generic and safe for public display

4. **Immutable Error Codes**
   - Error codes themselves cannot be changed
   - Only messages can be updated

---

## Future Enhancements

### Potential Improvements (Not in Current Scope)

1. **Additional Languages**
   - French, German, Portuguese, Chinese, etc.
   - Community-contributed translations

2. **Message Templates**
   - Support for parameter substitution
   - Dynamic error messages with context

3. **Versioning**
   - Track message versions
   - Support for message history

4. **Bulk Export/Import**
   - Export all messages to JSON
   - Import translations from external sources

5. **Message Categories**
   - Group messages by category
   - Easier management and organization

---

## Support

For questions or issues related to error codes:
- Review this documentation
- Check the contract implementation in `stellar-contracts/src/lib.rs`
- Run `python3 scripts/check_error_codes.py` to confirm the tables still match
- Search for "Issue #684" in code comments for the error registry design

---

## Changelog

### Version 1.1.0
- Rebuilt the `ContractError` reference from the enum in
  `stellar-contracts/src/lib.rs`: 50 variants across codes 1-46, 80 and 160-163.
  The previous table documented a pre-renumbering layout in which code `1` was
  `Unauthorized`, code `2` was `AdminNotInitialized` and code `3` was
  `PetNotFound`; those codes now belong to `AdminAlreadyApproved`,
  `AdminAlreadySet` and `AdminNotInitialized` respectively.
- Added a numeric quick index, references for `KoraError`, and references for the
  three error enums of the `pet-transfer-adoption` contract.
- Documented the deltas against the enum: three `ContractError` variants and four
  `KoraError` variants are declared but never raised.
- Added `scripts/check_error_codes.py` and documented the
  `initialize_error_messages` seed mismatch.

### Version 1.0.0 - Initial Implementation (Issue #684)
- Added multi-language error registry system
- Implemented English and Spanish translations
- Added admin functions for message management
- Added query functions for message retrieval
- Documented all error codes

---

**Last Updated:** 2026-09-23  
**Status:** ✅ Complete
