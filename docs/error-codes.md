# Kora App Contract Error Codes

The numeric values below are the Soroban `ContractError` discriminants in
`stellar-contracts/src/lib.rs`. Clients should match the numeric code, not the
display text. The error-message registry is a separate, administrator-managed
mapping and may provide English or Spanish text for a code.

## ContractError Reference

| Code | Variant | Cause | Recommended client action |
|---:|---|---|---|
| 1 | `AdminAlreadyApproved` | Admin already approved this proposal. | Refresh proposal state. |
| 2 | `AdminAlreadySet` | Contract administration is already initialized. | Read the current admin configuration. |
| 3 | `AdminNotInitialized` | Admin operation was attempted before initialization. | Initialize administration first. |
| 4 | `AdminsNotSet` | Multisig admin list is missing. | Configure the admin list. |
| 5 | `BatchTooLarge` | Batch input exceeds the contract limit. | Split the request into smaller batches. |
| 6 | `CertificateAlreadyAnchored` | Certificate hash is already anchored. | Use the existing certificate record. |
| 7 | `CounterOverflow` | A contract counter cannot be incremented safely. | Retry only after operator review. |
| 8 | `InputStringTooLong` | A string exceeds its contract limit. | Shorten the input. |
| 9 | `InvalidBreed` | Breed value is invalid. | Submit an accepted breed value. |
| 10 | `InvalidCallerNonce` | Caller nonce is invalid or stale. | Refresh nonce and sign again. |
| 11 | `InvalidCertificateHash` | Certificate hash format is invalid. | Submit a valid hash. |
| 12 | `InvalidInput` | Input fails contract validation. | Correct the request fields. |
| 13 | `InvalidIpfsHash` | IPFS hash format is invalid. | Submit a valid IPFS CID. |
| 14 | `InvalidPetName` | Pet name is invalid. | Correct the pet name. |
| 15 | `InvalidRating` | Rating is outside the allowed range. | Submit a valid rating. |
| 16 | `InvalidState` | Operation is incompatible with current state. | Refresh state and follow the workflow. |
| 17 | `InvalidThreshold` | Threshold is zero or exceeds the admin count. | Submit a valid threshold. |
| 18 | `InvokerNotInAdminList` | Initializer is not a proposed admin. | Sign with an address in the admin list. |
| 19 | `LicenseAlreadyRegistered` | Veterinarian license is already registered. | Use the existing registration or another license. |
| 20 | `NoAdminsConfigured` | No administrators are configured. | Complete admin initialization. |
| 21 | `NotAnAdmin` | Caller is not an administrator. | Use an authorized admin account. |
| 22 | `NotPetOwner` | Caller is not the pet owner. | Use the current owner account. |
| 23 | `PetAlreadyHasLinkedTag` | Pet already has a linked tag. | Unlink the existing tag first. |
| 24 | `PetNotFound` | Pet identifier does not exist. | Check the pet identifier. |
| 25 | `StorageQuotaExceeded` | Pet storage quota has been reached. | Remove eligible data or request a quota increase. |
| 26 | `ThresholdNotMet` | Required multisig approvals are missing. | Collect the remaining approvals. |
| 27 | `TooManyItems` | Collection exceeds its allowed size. | Submit fewer items. |
| 28 | `Unauthorized` | Caller lacks permission for the operation. | Authenticate as an authorized caller. |
| 29 | `VaccinationNotFound` | Vaccination identifier does not exist. | Check the vaccination identifier. |
| 30 | `VetAlreadyRegistered` | Veterinarian is already registered. | Use the existing veterinarian record. |
| 31 | `VetNotFound` | Veterinarian identifier does not exist. | Check the veterinarian address or ID. |
| 32 | `VetNotVerified` | Veterinarian is not verified. | Wait for verification or use a verified vet. |
| 33 | `VeterinarianNotVerified` | Operation requires a verified veterinarian. | Use a verified veterinarian account. |
| 34 | `SlotAlreadyBooked` | Appointment slot is already booked. | Choose another slot. |
| 35 | `DuplicateActivity` | Activity is already recorded. | Do not submit the duplicate. |
| 36 | `InbreedingThresholdExceeded` | Breeding coefficient exceeds the limit. | Choose a compatible breeding pair. |
| 37 | `SelfBreeding` | A pet was selected as both parents. | Select two distinct pets. |
| 38 | `ProposalAlreadyExecuted` | Governance proposal has already executed. | Refresh governance state. |
| 39 | `InvalidNonce` | Nonce does not match the expected value. | Refresh nonce and sign again. |
| 40 | `RollbackWindowExpired` | Upgrade rollback window has elapsed. | Start a new approved upgrade flow. |
| 41 | `NoPreviousUpgrade` | No prior upgrade is available to roll back. | Do not request a rollback. |
| 43 | `ProposalExpired` | Governance proposal passed its expiry time. | Create a new proposal. |
| 44 | `ProposalNotApproved` | Proposal lacks the required approval. | Obtain the required approval. |
| 45 | `QuorumNotMet` | Governance quorum was not reached. | Collect additional eligible votes. |
| 46 | `RateLimitExceeded` | Caller exceeded an operation rate limit. | Wait for the rate-limit window to reset. |
| 80 | `ProposalNotFound` | Governance proposal identifier does not exist. | Check the proposal identifier. |
| 160 | `AlreadyDeleted` | Medical record is already soft-deleted. | Refresh record state. |
| 161 | `RecordAlreadyDeleted` | Medical record is already marked deleted. | Do not delete it again. |
| 162 | `RetentionPeriodNotMet` | Retention period has not elapsed for purge. | Retry after the retention period. |
| 163 | `RecordNotFound` | Medical record identifier does not exist. | Check the record identifier. |

The enum currently has 50 named variants and 50 distinct numeric values. Codes
`42`, `47-79`, `81-159`, and `164+` are unassigned. Update this table whenever
the enum changes.

## Error Message Registry

`get_error_message(error_code, language)` returns an optional localized message.
Deployments may initialize `en` and `es`, but clients must fall back to the
numeric code when a translation is absent. Administrators can manage messages
with `set_error_message`, `batch_set_error_messages`, `initialize_error_messages`,
and `remove_error_message`. Language codes are 1-10 characters and messages are
1-500 characters.
