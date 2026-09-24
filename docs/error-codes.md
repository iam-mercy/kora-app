# Kora App Contract Error Codes

`ContractError` in `stellar-contracts/src/lib.rs` is the canonical error registry. Every code below is an explicit, unique `u32` discriminant and is registered by `initialize_error_messages` for both `en` and `es`.

| Code | ContractError | Default message |
| ---: | --- | --- |
| 1 | AdminAlreadyApproved | Admin already approved |
| 2 | AdminAlreadySet | Admin already set |
| 3 | AdminNotInitialized | Admin not initialized |
| 4 | AdminsNotSet | Admins not set |
| 5 | BatchTooLarge | Batch too large |
| 6 | CertificateAlreadyAnchored | Certificate already anchored |
| 7 | CounterOverflow | Counter overflow |
| 8 | InputStringTooLong | Input string too long |
| 9 | InvalidBreed | Invalid breed |
| 10 | InvalidCallerNonce | Invalid caller nonce |
| 11 | InvalidCertificateHash | Invalid certificate hash |
| 12 | InvalidInput | Invalid input |
| 13 | InvalidIpfsHash | Invalid IPFS hash |
| 14 | InvalidPetName | Invalid pet name |
| 15 | InvalidRating | Invalid rating |
| 16 | InvalidState | Invalid state |
| 17 | InvalidThreshold | Invalid threshold |
| 18 | InvokerNotInAdminList | Invoker not in admin list |
| 19 | LicenseAlreadyRegistered | License already registered |
| 20 | NoAdminsConfigured | No admins configured |
| 21 | NotAnAdmin | Not an admin |
| 22 | NotPetOwner | Not pet owner |
| 23 | PetAlreadyHasLinkedTag | Pet already has linked tag |
| 24 | PetNotFound | Pet not found |
| 25 | StorageQuotaExceeded | Storage quota exceeded |
| 26 | ThresholdNotMet | Threshold not met |
| 27 | TooManyItems | Too many items |
| 28 | Unauthorized | Unauthorized |
| 29 | VaccinationNotFound | Vaccination not found |
| 30 | VetAlreadyRegistered | Vet already registered |
| 31 | VetNotFound | Vet not found |
| 32 | VetNotVerified | Vet not verified |
| 33 | VeterinarianNotVerified | Veterinarian not verified |
| 34 | SlotAlreadyBooked | Slot already booked |
| 35 | DuplicateActivity | Duplicate activity |
| 36 | InbreedingThresholdExceeded | Inbreeding threshold exceeded |
| 37 | SelfBreeding | Self breeding |
| 38 | ProposalAlreadyExecuted | Proposal already executed |
| 39 | InvalidNonce | Invalid nonce |
| 40 | RollbackWindowExpired | Rollback window expired |
| 41 | NoPreviousUpgrade | No previous upgrade |
| 43 | ProposalExpired | Proposal expired |
| 44 | ProposalNotApproved | Proposal not approved |
| 45 | QuorumNotMet | Quorum not met |
| 46 | RateLimitExceeded | Rate limit exceeded |
| 80 | ProposalNotFound | Proposal not found |
| 160 | AlreadyDeleted | Already deleted |
| 161 | RecordAlreadyDeleted | Record already deleted |
| 162 | RetentionPeriodNotMet | Retention period not met |
| 163 | RecordNotFound | Record not found |
| 164 | NonceReused | Nonce reused |
| 165 | SelfLineage | Self lineage |
| 166 | CircularLineage | Circular lineage |
| 167 | KeywordTooLong | Keyword too long |
| 168 | TooManySearchTokens | Too many search tokens |

The Spanish registry uses the same canonical message keys as the English registry so clients never receive a message for a different error code. Localized text can be supplied later through `set_error_message`.

## Registry API

- `get_error_message(error_code, language)` reads a registered message.
- `get_supported_languages()` lists initialized languages.
- `set_error_message` and `batch_set_error_messages` update messages for an authenticated admin.
- `initialize_error_messages` registers the complete canonical table.
- `remove_error_message` removes one language/code entry for an authenticated admin.
