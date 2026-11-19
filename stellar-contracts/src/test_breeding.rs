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
#[should_panic(expected = "Error(Contract, #24)")]
fn test_update_breed_metadata_rejects_unknown_id() {
    let (env, client, _owner, _sire, _dam, _offspring, _grandchild) = setup();
    let unknown_id = 999_999u64;

    // Updating an unknown ID must be rejected with a typed error.
    client.update_breed_metadata(
        &unknown_id,
        &String::from_str(&env, "Ghost"),
        &String::from_str(&env, "Missing"),
    );
}

#[test]
fn test_update_breed_metadata_failure_leaves_metadata_intact() {
    let (env, client, _owner, _sire, _dam, _offspring, _grandchild) = setup();
    let breed_id = 1u64;
    client.add_breed_metadata(
        &breed_id,
        &String::from_str(&env, "Labrador"),
        &String::from_str(&env, "Friendly"),
    );

    let unknown_id = 999_999u64;
    let result = client.try_update_breed_metadata(
        &unknown_id,
        &String::from_str(&env, "Ghost"),
        &String::from_str(&env, "Missing"),
    );
    assert!(result.is_err());

    // The failed update must not create metadata for the unknown ID.
    let result = client.try_get_breed_metadata(&unknown_id);
    assert!(result.is_err());

    // Existing metadata must remain unchanged.
    let metadata = client.get_breed_metadata(&breed_id);
    assert_eq!(metadata.name, String::from_str(&env, "Labrador"));
    assert_eq!(metadata.description, String::from_str(&env, "Friendly"));
}
