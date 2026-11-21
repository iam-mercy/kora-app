use crate::{Gender, KoraContract, KoraContractClient, PrivacyLevel, Species};
use soroban_sdk::{testutils::Address as _, Address, Env, String};

fn register_pet(env: &Env, client: &KoraContractClient, owner: &Address, name: &str) -> u64 {
    client.register_pet(
        owner,
        &String::from_str(env, name),
        &String::from_str(env, "2021-01-01"),
        &Gender::Female,
        &Species::Dog,
        &String::from_str(env, "Brown"),
        &String::from_str(env, "Mixed"),
        &20,
        &None,
        &PrivacyLevel::Public,
    )
}

fn setup() -> (
    Env,
    KoraContractClient<'static>,
    Address,
    u64,
    u64,
    u64,
    u64,
) {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(KoraContract, ());
    let client = KoraContractClient::new(&env, &contract_id);
    let owner = Address::generate(&env);
    let sire = register_pet(&env, &client, &owner, "Sire");
    let dam = register_pet(&env, &client, &owner, "Dam");
    let offspring = register_pet(&env, &client, &owner, "Offspring");
    let grandchild = register_pet(&env, &client, &owner, "Grandchild");
    (env, client, owner, sire, dam, offspring, grandchild)
}

#[test]
#[should_panic(expected = "Error(Contract, #12)")]
fn test_self_reference_rejected() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(KoraContract, ());
    let client = KoraContractClient::new(&env, &contract_id);
    let owner = Address::generate(&env);
    let sire = register_pet(&env, &client, &owner, "Sire");
    let dam = register_pet(&env, &client, &owner, "Dam");
    let record_id = client.add_breeding_record(
        &sire,
        &dam,
        &env.ledger().timestamp(),
        &String::from_str(&env, "bad"),
    );

    client.add_offspring(&record_id, &sire, &owner);
}

#[test]
#[should_panic(expected = "Error(Contract, #16)")]
fn test_cycle_insertion_rejected() {
    let (env, client, owner, sire, dam, offspring, _grandchild) = setup();
    let record_id = client.add_breeding_record(
        &sire,
        &dam,
        &env.ledger().timestamp(),
        &String::from_str(&env, "first"),
    );
    assert!(client.add_offspring(&record_id, &offspring, &owner));

    let cycle_record = client.add_breeding_record(
        &offspring,
        &dam,
        &env.ledger().timestamp(),
        &String::from_str(&env, "cycle"),
    );
    client.add_offspring(&cycle_record, &sire, &owner);
}

#[test]
fn test_get_lineage_returns_parent_tree() {
    let (env, client, owner, sire, dam, offspring, grandchild) = setup();
    let first = client.add_breeding_record(
        &sire,
        &dam,
        &env.ledger().timestamp(),
        &String::from_str(&env, "first"),
    );
    assert!(client.add_offspring(&first, &offspring, &owner));

    let second = client.add_breeding_record(
        &offspring,
        &dam,
        &env.ledger().timestamp(),
        &String::from_str(&env, "second"),
    );
    assert!(client.add_offspring(&second, &grandchild, &owner));

    let lineage = client.get_lineage(&grandchild, &2);
    assert_eq!(lineage.len(), 4);
    assert_eq!(lineage.get(0).unwrap(), offspring);
    assert_eq!(lineage.get(1).unwrap(), dam);
    assert_eq!(lineage.get(2).unwrap(), sire);
    assert_eq!(lineage.get(3).unwrap(), dam);
}

#[test]
#[should_panic(expected = "Error(Contract, #28)")]
fn test_add_offspring_unauthorized_caller_rejected() {
    let (env, client, _owner, sire, dam, offspring, _grandchild) = setup();
    let record_id = client.add_breeding_record(
        &sire,
        &dam,
        &env.ledger().timestamp(),
        &String::from_str(&env, "unauth"),
    );
    let stranger = Address::generate(&env);
    client.add_offspring(&record_id, &offspring, &stranger);
}

#[test]
#[should_panic(expected = "Error(Contract, #24)")]
fn test_add_offspring_nonexistent_pet_rejected() {
    let (env, client, owner, sire, dam, _offspring, _grandchild) = setup();
    let record_id = client.add_breeding_record(
        &sire,
        &dam,
        &env.ledger().timestamp(),
        &String::from_str(&env, "nonexistent"),
    );
    let nonexistent_pet_id = 999_999u64;
    client.add_offspring(&record_id, &nonexistent_pet_id, &owner);
}

#[test]
#[should_panic(expected = "Error(Contract, #12)")]
fn test_get_lineage_max_depth_exceeded_rejected() {
    let (env, client, owner, sire, dam, offspring, grandchild) = setup();
    let first = client.add_breeding_record(
        &sire,
        &dam,
        &env.ledger().timestamp(),
        &String::from_str(&env, "first"),
    );
    assert!(client.add_offspring(&first, &offspring, &owner));
    let second = client.add_breeding_record(
        &offspring,
        &dam,
        &env.ledger().timestamp(),
        &String::from_str(&env, "second"),
    );
    assert!(client.add_offspring(&second, &grandchild, &owner));

    // max_depth 15 exceeds MAX_ALLOWED_LINEAGE_DEPTH (8), must reject with InvalidInput (#12)
    client.get_lineage(&grandchild, &15);
}

#[test]
fn test_get_lineage_up_to_max_depth_succeeds() {
    let (env, client, owner, sire, dam, offspring, grandchild) = setup();
    let first = client.add_breeding_record(
        &sire,
        &dam,
        &env.ledger().timestamp(),
        &String::from_str(&env, "first"),
    );
    assert!(client.add_offspring(&first, &offspring, &owner));
    let second = client.add_breeding_record(
        &offspring,
        &dam,
        &env.ledger().timestamp(),
        &String::from_str(&env, "second"),
    );
    assert!(client.add_offspring(&second, &grandchild, &owner));

    // max_depth 8 must succeed safely
    let lineage = client.get_lineage(&grandchild, &8);
    assert_eq!(lineage.len(), 4);
}

#[test]
fn test_add_breed_metadata_creates_new_entry() {
    let (env, client, _owner, sire, _dam, _offspring, _grandchild) = setup();
    let breed_id = 1u64;
    client.add_breed_metadata(
        &breed_id,
        &String::from_str(&env, "Labrador"),
        &String::from_str(&env, "Friendly"),
    );

    let metadata = client.get_breed_metadata(&breed_id);
    assert_eq!(metadata.name, String::from_str(&env, "Labrador"));
    assert_eq!(metadata.description, String::from_str(&env, "Friendly"));
    let _ = sire;
}

#[test]
#[should_panic(expected = "Error(Contract, #12)")]
fn test_add_breed_metadata_rejects_existing_id() {
    let (env, client, _owner, _sire, _dam, _offspring, _grandchild) = setup();
    let breed_id = 1u64;
    client.add_breed_metadata(
        &breed_id,
        &String::from_str(&env, "Labrador"),
        &String::from_str(&env, "Friendly"),
    );

    // Creating again with the same ID must be rejected with a typed error.
    client.add_breed_metadata(
        &breed_id,
        &String::from_str(&env, "Poodle"),
        &String::from_str(&env, "Curly"),
    );
}

#[test]
fn test_add_breed_metadata_failure_preserves_existing() {
    let (env, client, _owner, _sire, _dam, _offspring, _grandchild) = setup();
    let breed_id = 1u64;
    client.add_breed_metadata(
        &breed_id,
        &String::from_str(&env, "Labrador"),
        &String::from_str(&env, "Friendly"),
    );

    let result = client.try_add_breed_metadata(
        &breed_id,
        &String::from_str(&env, "Poodle"),
        &String::from_str(&env, "Curly"),
    );
    assert!(result.is_err());

    let metadata = client.get_breed_metadata(&breed_id);
    assert_eq!(metadata.name, String::from_str(&env, "Labrador"));
    assert_eq!(metadata.description, String::from_str(&env, "Friendly"));
}

#[test]
fn test_update_breed_metadata_updates_existing_entry() {
    let (env, client, _owner, _sire, _dam, _offspring, _grandchild) = setup();
    let breed_id = 1u64;
    client.add_breed_metadata(
        &breed_id,
        &String::from_str(&env, "Labrador"),
        &String::from_str(&env, "Friendly"),
    );

    client.update_breed_metadata(
        &breed_id,
        &String::from_str(&env, "Labrador Retriever"),
        &String::from_str(&env, "Very friendly"),
    );

    let metadata = client.get_breed_metadata(&breed_id);
    assert_eq!(metadata.name, String::from_str(&env, "Labrador Retriever"));
    assert_eq!(metadata.description, String::from_str(&env, "Very friendly"));
}

#[test]
fn test_add_breed_metadata_rejects_empty_name() {
    let (env, client, _owner, _sire, _dam, _offspring, _grandchild) = setup();
    let breed_id = 42u64;
    let result = client.try_add_breed_metadata(
        &breed_id,
        &String::from_str(&env, ""),
        &String::from_str(&env, "Friendly"),
    );
    assert!(result.is_err());
    // Invalid input must not create a key.
    assert!(client.try_get_breed_metadata(&breed_id).is_err());
}

#[test]
fn test_add_breed_metadata_rejects_empty_description() {
    let (env, client, _owner, _sire, _dam, _offspring, _grandchild) = setup();
    let breed_id = 43u64;
    let result = client.try_add_breed_metadata(
        &breed_id,
        &String::from_str(&env, "Labrador"),
        &String::from_str(&env, ""),
    );
    assert!(result.is_err());
    assert!(client.try_get_breed_metadata(&breed_id).is_err());
}

#[test]
fn test_add_breed_metadata_accepts_max_length_strings() {
    let (env, client, _owner, _sire, _dam, _offspring, _grandchild) = setup();
    let breed_id = 44u64;
    // 64-byte ASCII strings are at the upper bound and must be accepted.
    let name = "a".repeat(64);
    let description = "b".repeat(64);
    client.add_breed_metadata(
        &breed_id,
        &String::from_str(&env, &name),
        &String::from_str(&env, &description),
    );
    let metadata = client.get_breed_metadata(&breed_id);
    assert_eq!(metadata.name, String::from_str(&env, &name));
    assert_eq!(metadata.description, String::from_str(&env, &description));
}

#[test]
fn test_add_breed_metadata_rejects_over_length_name() {
    let (env, client, _owner, _sire, _dam, _offspring, _grandchild) = setup();
    let breed_id = 45u64;
    let name = "a".repeat(65);
    let result = client.try_add_breed_metadata(
        &breed_id,
        &String::from_str(&env, &name),
        &String::from_str(&env, "Friendly"),
    );
    assert!(result.is_err());
    assert!(client.try_get_breed_metadata(&breed_id).is_err());
}

#[test]
fn test_add_breed_metadata_rejects_over_length_description() {
    let (env, client, _owner, _sire, _dam, _offspring, _grandchild) = setup();
    let breed_id = 46u64;
    let description = "b".repeat(65);
    let result = client.try_add_breed_metadata(
        &breed_id,
        &String::from_str(&env, "Labrador"),
        &String::from_str(&env, &description),
    );
    assert!(result.is_err());
    assert!(client.try_get_breed_metadata(&breed_id).is_err());
}

#[test]
fn test_update_breed_metadata_rejects_empty_name() {
    let (env, client, _owner, _sire, _dam, _offspring, _grandchild) = setup();
    let breed_id = 47u64;
    client.add_breed_metadata(
        &breed_id,
        &String::from_str(&env, "Labrador"),
        &String::from_str(&env, "Friendly"),
    );

    let result = client.try_update_breed_metadata(
        &breed_id,
        &String::from_str(&env, ""),
        &String::from_str(&env, "Very friendly"),
    );
    assert!(result.is_err());
    // Existing entry must be preserved on invalid update.
    let metadata = client.get_breed_metadata(&breed_id);
    assert_eq!(metadata.name, String::from_str(&env, "Labrador"));
    assert_eq!(metadata.description, String::from_str(&env, "Friendly"));
}

#[test]
fn test_update_breed_metadata_rejects_over_length_description() {
    let (env, client, _owner, _sire, _dam, _offspring, _grandchild) = setup();
    let breed_id = 48u64;
    client.add_breed_metadata(
        &breed_id,
        &String::from_str(&env, "Labrador"),
        &String::from_str(&env, "Friendly"),
    );

    let description = "b".repeat(65);
    let result = client.try_update_breed_metadata(
        &breed_id,
        &String::from_str(&env, "Labrador Retriever"),
        &String::from_str(&env, &description),
    );
    assert!(result.is_err());
    let metadata = client.get_breed_metadata(&breed_id);
    assert_eq!(metadata.name, String::from_str(&env, "Labrador"));
    assert_eq!(metadata.description, String::from_str(&env, "Friendly"));
}
