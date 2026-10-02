/// Tests for issues #100, #101, #102, #103.
///
/// #100 – update_medical_record_notes emits MedicalRecordNotesUpdatedEvent
///         with old_notes_hash and new_notes_hash.
/// #101 – purge_deleted_records compacts PetMedicalRecordCount/index so
///         paginated queries remain dense after purging expired records.
/// #102 – set_retention_period rejects values below MIN_RETENTION_DAYS (365).
/// #103 – add_attachment reverts with RecordNotFound for a non-existent record.
use soroban_sdk::{
    testutils::{Address as _, Events, Ledger},
    Address, BytesN, Env, String, Vec,
};

// ---------------------------------------------------------------------------
// Shared setup helpers
// ---------------------------------------------------------------------------

fn setup_env() -> (
    Env,
    crate::KoraContractClient<'static>,
    Address,
    Address,
    Address,
) {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, crate::KoraContract);
    let client = crate::KoraContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let owner = Address::generate(&env);
    let vet = Address::generate(&env);

    client.init_admin(&admin);
    client.register_vet(
        &vet,
        &String::from_str(&env, "Dr. Test"),
        &String::from_str(&env, "LIC-TEST-001"),
        &String::from_str(&env, "General Practice"),
    );
    client.verify_vet(&admin, &vet);

    (env, client, admin, owner, vet)
}

fn register_pet(
    client: &crate::KoraContractClient<'_>,
    env: &Env,
    owner: &Address,
) -> u64 {
    client.register_pet(
        owner,
        &String::from_str(env, "Buddy"),
        &String::from_str(env, "2020-01-01"),
        &crate::Gender::Male,
        &crate::Species::Dog,
        &String::from_str(env, "Labrador"),
        &String::from_str(env, "Brown"),
        &25,
        &None,
        &crate::PrivacyLevel::Public,
    )
}

fn add_record(
    client: &crate::KoraContractClient<'_>,
    env: &Env,
    pet_id: u64,
    vet: &Address,
    notes: &str,
) -> u64 {
    client.add_medical_record(
        &pet_id,
        vet,
        &String::from_str(env, "Diagnosis"),
        &String::from_str(env, "Treatment"),
        &Vec::new(env),
        &String::from_str(env, notes),
    )
}

// ---------------------------------------------------------------------------
// Issue #100 – MedicalRecordNotesUpdatedEvent
// ---------------------------------------------------------------------------

/// Updating notes on a valid record must emit exactly one new event.
#[test]
fn test_update_notes_emits_event() {
    let (env, client, _admin, owner, vet) = setup_env();
    let pet_id = register_pet(&client, &env, &owner);
    let record_id = add_record(&client, &env, pet_id, &vet, "Original notes");

    let events_before = env.events().all().len();

    let result = client.update_medical_record_notes(
        &record_id,
        &String::from_str(&env, "Updated clinical notes"),
    );
    assert!(result, "update_medical_record_notes should return true");

    assert_eq!(
        env.events().all().len(),
        events_before + 1,
        "exactly one event should be emitted on notes update"
    );
}

/// When old and new content differ, old_notes_hash != new_notes_hash.
#[test]
fn test_update_notes_hashes_differ_when_content_changes() {
    use soroban_sdk::{xdr::ContractEventBody, TryIntoVal, Val};

    let (env, client, _admin, owner, vet) = setup_env();
    let pet_id = register_pet(&client, &env, &owner);
    let record_id = add_record(&client, &env, pet_id, &vet, "Original clinical notes");

    client.update_medical_record_notes(
        &record_id,
        &String::from_str(&env, "Different notes content"),
    );

    let target_topic = String::from_str(&env, "MedicalRecordNotesUpdated");
    let all = env.events().all();
    let mut found = false;

    for event in all.events() {
        let ContractEventBody::V0(body) = &event.body;
        // Decode topics into soroban Vals
        let mut topics: soroban_sdk::Vec<Val> = soroban_sdk::Vec::new(&env);
        for sv in body.topics.iter() {
            topics.push_back(sv.clone().try_into_val(&env).unwrap());
        }
        if topics.len() == 0 {
            continue;
        }
        let t0: Val = topics.get(0).unwrap();
        if let Ok(s) = String::try_from_val(&env, &t0) {
            if s == target_topic {
                let ev: crate::MedicalRecordNotesUpdatedEvent =
                    body.data.clone().try_into_val(&env).unwrap();
                assert_eq!(ev.record_id, record_id);
                assert_ne!(
                    ev.old_notes_hash, ev.new_notes_hash,
                    "old_notes_hash and new_notes_hash must differ when content changes"
                );
                found = true;
                break;
            }
        }
    }
    assert!(found, "MedicalRecordNotesUpdated event was not emitted");
}

/// When old and new content are identical, old_notes_hash == new_notes_hash.
#[test]
fn test_update_notes_hashes_equal_when_content_same() {
    use soroban_sdk::{xdr::ContractEventBody, TryIntoVal, Val};

    let (env, client, _admin, owner, vet) = setup_env();
    let pet_id = register_pet(&client, &env, &owner);
    let same_notes = "Same content";
    let record_id = add_record(&client, &env, pet_id, &vet, same_notes);

    client.update_medical_record_notes(&record_id, &String::from_str(&env, same_notes));

    let target_topic = String::from_str(&env, "MedicalRecordNotesUpdated");
    let all = env.events().all();
    let mut found = false;

    for event in all.events() {
        let ContractEventBody::V0(body) = &event.body;
        let mut topics: soroban_sdk::Vec<Val> = soroban_sdk::Vec::new(&env);
        for sv in body.topics.iter() {
            topics.push_back(sv.clone().try_into_val(&env).unwrap());
        }
        if topics.len() == 0 {
            continue;
        }
        let t0: Val = topics.get(0).unwrap();
        if let Ok(s) = String::try_from_val(&env, &t0) {
            if s == target_topic {
                let ev: crate::MedicalRecordNotesUpdatedEvent =
                    body.data.clone().try_into_val(&env).unwrap();
                assert_eq!(
                    ev.old_notes_hash, ev.new_notes_hash,
                    "hashes must be equal when content is unchanged"
                );
                found = true;
                break;
            }
        }
    }
    assert!(found, "MedicalRecordNotesUpdated event was not emitted");
}

/// Updating a non-existent record returns false and emits no event.
#[test]
fn test_update_notes_nonexistent_record_returns_false() {
    let (env, client, _admin, _owner, _vet) = setup_env();

    let events_before = env.events().all().len();
    let result =
        client.update_medical_record_notes(&9999, &String::from_str(&env, "Should not persist"));
    assert!(!result, "should return false for non-existent record");
    assert_eq!(
        env.events().all().len(),
        events_before,
        "no event should be emitted for a non-existent record"
    );
}

// ---------------------------------------------------------------------------
// Issue #101 – purge_deleted_records compacts PetMedicalRecordCount/index
// ---------------------------------------------------------------------------

/// After purging 2 of 3 records, paginated retrieval returns only the survivor.
#[test]
fn test_purge_compacts_index_and_count() {
    let env = Env::default();
    env.ledger().with_mut(|l| l.timestamp = 1_700_000_000);
    env.mock_all_auths();
    let contract_id = env.register_contract(None, crate::KoraContract);
    let client = crate::KoraContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let owner = Address::generate(&env);
    let vet = Address::generate(&env);
    client.init_admin(&admin);
    client.register_vet(
        &vet,
        &String::from_str(&env, "Dr. Purge"),
        &String::from_str(&env, "LIC-PURGE"),
        &String::from_str(&env, "General"),
    );
    client.verify_vet(&admin, &vet);

    let pet_id = register_pet(&client, &env, &owner);

    let r1 = add_record(&client, &env, pet_id, &vet, "Record 1");
    let r2 = add_record(&client, &env, pet_id, &vet, "Record 2");
    let r3 = add_record(&client, &env, pet_id, &vet, "Record 3");

    // Soft-delete r1 and r2, leave r3 alive.
    client.delete_medical_record(&pet_id, &r1, &vet);
    client.delete_medical_record(&pet_id, &r2, &vet);

    // Advance past the retention window (365 days minimum).
    let retention: u64 = 365 * 86_400;
    client.set_retention_period(&admin, &retention);
    env.ledger()
        .with_mut(|l| l.timestamp = 1_700_000_000 + retention + 1);

    let result = client.purge_deleted_records(&pet_id, &admin, &false);
    assert_eq!(result.deleted.len(), 2, "two records should be purged");

    // After compaction, only r3 should be retrievable.
    let records = client.get_pet_medical_records(&pet_id, &0, &10);
    assert_eq!(
        records.len(),
        1,
        "only 1 record should survive after purging r1 and r2"
    );
    assert_eq!(records.get(0).unwrap().id, r3, "the surviving record must be r3");
}

/// Purging all records leaves the pet with an empty index.
#[test]
fn test_purge_all_leaves_empty_index() {
    let env = Env::default();
    env.ledger().with_mut(|l| l.timestamp = 1_700_000_000);
    env.mock_all_auths();
    let contract_id = env.register_contract(None, crate::KoraContract);
    let client = crate::KoraContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let owner = Address::generate(&env);
    let vet = Address::generate(&env);
    client.init_admin(&admin);
    client.register_vet(
        &vet,
        &String::from_str(&env, "Dr. PurgeAll"),
        &String::from_str(&env, "LIC-PURGEALL"),
        &String::from_str(&env, "General"),
    );
    client.verify_vet(&admin, &vet);

    let pet_id = register_pet(&client, &env, &owner);
    let r1 = add_record(&client, &env, pet_id, &vet, "Old 1");
    let r2 = add_record(&client, &env, pet_id, &vet, &"Old 2");

    client.delete_medical_record(&pet_id, &r1, &vet);
    client.delete_medical_record(&pet_id, &r2, &vet);

    let retention: u64 = 365 * 86_400;
    client.set_retention_period(&admin, &retention);
    env.ledger()
        .with_mut(|l| l.timestamp = 1_700_000_000 + retention + 1);

    let result = client.purge_deleted_records(&pet_id, &admin, &false);
    assert_eq!(result.deleted.len(), 2);

    let records = client.get_pet_medical_records(&pet_id, &0, &10);
    assert_eq!(
        records.len(),
        0,
        "paginated results should be empty after purging all records"
    );
}

// ---------------------------------------------------------------------------
// Issue #102 – set_retention_period enforces MIN_RETENTION_DAYS floor
// ---------------------------------------------------------------------------

/// Retention of 0 seconds must panic.
#[test]
#[should_panic]
fn test_set_retention_period_zero_panics() {
    let (env, client, admin, _owner, _vet) = setup_env();
    client.set_retention_period(&admin, &0);
}

/// Retention below 365 days (364 * 86_400) must panic.
#[test]
#[should_panic]
fn test_set_retention_period_below_minimum_panics() {
    let (env, client, admin, _owner, _vet) = setup_env();
    let below_min: u64 = 364 * 86_400;
    client.set_retention_period(&admin, &below_min);
}

/// Retention at exactly 365 days must succeed.
#[test]
fn test_set_retention_period_at_minimum_succeeds() {
    let (env, client, admin, _owner, _vet) = setup_env();
    let min_seconds: u64 = 365 * 86_400;
    client.set_retention_period(&admin, &min_seconds);
    assert_eq!(client.get_retention_period(), min_seconds);
}

/// Retention above the floor (7 years) must succeed.
#[test]
fn test_set_retention_period_above_minimum_succeeds() {
    let (env, client, admin, _owner, _vet) = setup_env();
    let seven_years: u64 = 7 * 365 * 86_400;
    client.set_retention_period(&admin, &seven_years);
    assert_eq!(client.get_retention_period(), seven_years);
}

// ---------------------------------------------------------------------------
// Issue #103 – add_attachment reverts with RecordNotFound for missing record
// ---------------------------------------------------------------------------

fn make_metadata(env: &Env) -> crate::AttachmentMetadata {
    crate::AttachmentMetadata {
        filename: String::from_str(env, "report.pdf"),
        file_type: String::from_str(env, "application/pdf"),
        size: 1024,
        uploaded_date: env.ledger().timestamp(),
    }
}

/// Attaching a file to a non-existent record must panic (RecordNotFound).
#[test]
#[should_panic]
fn test_add_attachment_nonexistent_record_panics() {
    let (env, client, _admin, _owner, _vet) = setup_env();
    let content_hash = BytesN::from_array(&env, &[0u8; 32]);
    client.add_attachment(
        &9999,
        &String::from_str(&env, "QmYwAPJzv5CZsnA625s3Xf2nemtYgPpHdWEz79ojWnPbdG"),
        &make_metadata(&env),
        &content_hash,
    );
}

/// Attaching a file to a valid record must succeed.
#[test]
fn test_add_attachment_valid_record_succeeds() {
    let (env, client, _admin, owner, vet) = setup_env();
    let pet_id = register_pet(&client, &env, &owner);
    let record_id = add_record(&client, &env, pet_id, &vet, "Checkup notes");

    let content_hash = BytesN::from_array(&env, &[1u8; 32]);
    let result = client.add_attachment(
        &record_id,
        &String::from_str(&env, "QmYwAPJzv5CZsnA625s3Xf2nemtYgPpHdWEz79ojWnPbdG"),
        &make_metadata(&env),
        &content_hash,
    );
    assert!(result, "add_attachment should succeed for a valid record");

    let attachments = client.get_attachments(&record_id);
    assert_eq!(attachments.len(), 1, "one attachment should be stored");
}
