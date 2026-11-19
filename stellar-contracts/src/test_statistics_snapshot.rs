use crate::*;
use soroban_sdk::{testutils::{Address as _, Events}, Address, Env, String, Symbol, IntoVal};

fn setup_env() -> (Env, KoraContractClient<'static>, Address) {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, KoraContract);
    let client = KoraContractClient::new(&env, &contract_id);

    // Initialize admin
    let admin = Address::generate(&env);
    client.init_admin(&admin);

    (env, client, admin)
}

fn register_pet_with_species(
    client: &KoraContractClient,
    env: &Env,
    owner: &Address,
    species: Species,
) -> u64 {
    client.register_pet(
        owner,
        &String::from_str(env, "Pet"),
        &String::from_str(env, "2020-01-01"),
        &Gender::Male,
        &species,
        &String::from_str(env, "Breed"),
        &String::from_str(env, "Color"),
        &10u32,
        &None,
        &PrivacyLevel::Public,
    )
}

fn setup_vet(client: &KoraContractClient, env: &Env, admin: &Address) -> Address {
    let vet = Address::generate(env);
    client.register_vet(
        &vet,
        &String::from_str(env, "Dr. Smith"),
        &String::from_str(env, "VET123"),
        &String::from_str(env, "Animal Clinic"),
    );
    client.verify_vet(admin, &vet);
    vet
}

// ── Test: Take and Retrieve Snapshot ──────────────────────────────────────────

#[test]
fn test_take_and_retrieve_snapshot() {
    let (env, client, admin) = setup_env();
    let owner = Address::generate(&env);

    // Register some pets
    let pet_id1 = register_pet_with_species(&client, &env, &owner, Species::Dog);
    let pet_id2 = register_pet_with_species(&client, &env, &owner, Species::Cat);

    // Activate one pet
    client.activate_pet(&pet_id1);

    // Setup vet and add a vaccination
    let vet = setup_vet(&client, &env, &admin);
    client.add_vaccination(
        &pet_id1,
        &vet,
        &VaccineType::Rabies,
        &String::from_str(&env, "Rabies"),
        &env.ledger().timestamp(),
        &(env.ledger().timestamp() + 365 * 24 * 60 * 60),
        &0u64,
        &String::from_str(&env, "BATCH123"),
    );

    // Add a medical record
    client.add_medical_record(
        &pet_id2,
        &vet,
        &String::from_str(&env, "Checkup"),
        &String::from_str(&env, "Healthy"),
        &Vec::new(&env),
        &String::from_str(&env, "Regular checkup"),
    );

    // Take a snapshot
    let snapshot_id = client.take_statistics_snapshot(&admin);
    assert_eq!(snapshot_id, 1);

    // Retrieve the snapshot
    let snapshot = client.get_snapshot(&snapshot_id).unwrap();

    // Verify snapshot contents
    assert_eq!(snapshot.snapshot_id, 1);
    assert_eq!(snapshot.total_pets, 2);
    assert_eq!(snapshot.active_pets, 1);
    assert_eq!(snapshot.total_vets, 1);
    assert_eq!(snapshot.total_vaccinations, 1);
    assert_eq!(snapshot.total_medical_records, 1);
    assert_eq!(snapshot.total_insurance_claims, 0);

    // Verify species distribution
    assert_eq!(
        snapshot.species_distribution.get(String::from_str(&env, "Dog")).unwrap(),
        1
    );
    assert_eq!(
        snapshot.species_distribution.get(String::from_str(&env, "Cat")).unwrap(),
        1
    );
    assert_eq!(
        snapshot.species_distribution.get(String::from_str(&env, "Bird")).unwrap(),
        0
    );
}

// ── Test: Snapshot Emits SnapshotTaken Event ──────────────────────────────────

#[test]
fn test_take_snapshot_emits_event() {
    let (env, client, admin) = setup_env();
    let owner = Address::generate(&env);

    // Register a pet and a vet so totals are non-zero
    register_pet_with_species(&client, &env, &owner, Species::Dog);
    setup_vet(&client, &env, &admin);

    let snapshot_id = client.take_statistics_snapshot(&admin);
    assert_eq!(snapshot_id, 1);

    let snapshot = client.get_snapshot(&snapshot_id).unwrap();

    // Verify the SnapshotTaken event was published with the expected payload
    let expected_topics = (Symbol::new(&env, "SnapshotTaken"),).into_val(&env);
    let expected_data = (
        snapshot_id,
        snapshot.total_pets,
        snapshot.total_vets,
        snapshot.timestamp,
    )
        .into_val(&env);

    let events = env.events().all();
    let mut found = false;
    for event in events.iter() {
        if event.0 == expected_topics && event.2 == expected_data {
            found = true;
            break;
        }
    }
    assert!(found, "SnapshotTaken event not emitted with expected payload");
}

// ── Test: Multiple Snapshots ───────────────────────────────────────────────────

#[test]
fn test_multiple_snapshots() {
    let (env, client, admin) = setup_env();
    let owner = Address::generate(&env);

    // Take first snapshot (empty state)
    let snapshot_id1 = client.take_statistics_snapshot(&admin);
    assert_eq!(snapshot_id1, 1);

    // Register a pet
    register_pet_with_species(&client, &env, &owner, Species::Dog);

    // Take second snapshot
    let snapshot_id2 = client.take_statistics_snapshot(&admin);
    assert_eq!(snapshot_id2, 2);

    // Verify first snapshot still has 0 pets
    let snapshot1 = client.get_snapshot(&snapshot_id1).unwrap();
    assert_eq!(snapshot1.total_pets, 0);

    // Verify second snapshot has 1 pet
    let snapshot2 = client.get_snapshot(&snapshot_id2).unwrap();
    assert_eq!(snapshot2.total_pets, 1);
}

// ── Test: Snapshot Count ───────────────────────────────────────────────────────

#[test]
fn test_snapshot_count() {
    let (env, client, admin) = setup_env();

    // Initially 0
    assert_eq!(client.get_snapshot_count(), 0);

    // Take a snapshot
    client.take_statistics_snapshot(&admin);
    assert_eq!(client.get_snapshot_count(), 1);

    // Take another snapshot
    client.take_statistics_snapshot(&admin);
    assert_eq!(client.get_snapshot_count(), 2);
}

// ── Test: Available Snapshot IDs ───────────────────────────────────────────────

#[test]
fn test_available_snapshot_ids() {
    let (env, client, admin) = setup_env();

    // Initially empty
    let ids = client.get_available_snapshot_ids();
    assert_eq!(ids.len(), 0);

    // Take 3 snapshots
    client.take_statistics_snapshot(&admin);
    client.take_statistics_snapshot(&admin);
    client.take_statistics_snapshot(&admin);

    // Should have 3 available IDs
    let ids = client.get_available_snapshot_ids();
    assert_eq!(ids.len(), 3);
    assert_eq!(ids.get(0).unwrap(), 1);
    assert_eq!(ids.get(1).unwrap(), 2);
    assert_eq!(ids.get(2).unwrap(), 3);
}

// ── Test: 101st Snapshot Triggers Purge ─────────────────────────────────────────

#[test]
fn test_snapshot_purge_on_101st() {
    let (env, client, admin) = setup_env();

    // Take 100 snapshots
    for i in 1..=100 {
        let snapshot_id = client.take_statistics_snapshot(&admin);
        assert_eq!(snapshot_id, i);
    }

    // Verify we have 100 snapshots
    let ids = client.get_available_snapshot_ids();
    assert_eq!(ids.len(), 100);

    // Verify first snapshot exists
    assert!(client.get_snapshot(&1).is_some());

    // Take the 101st snapshot
    let snapshot_id_101 = client.take_statistics_snapshot(&admin);
    assert_eq!(snapshot_id_101, 101);

    // Verify first snapshot (ID 1) has been purged
    assert!(client.get_snapshot(&1).is_none());

    // Verify 101st snapshot exists
    assert!(client.get_snapshot(&101).is_some());

    // Verify we still have 100 snapshots (IDs 2-101)
    let ids = client.get_available_snapshot_ids();
    assert_eq!(ids.len(), 100);
    assert_eq!(ids.get(0).unwrap(), 2);
    assert_eq!(ids.get(99).unwrap(), 101);

    // Verify snapshot count continues to increment (not reset)
    assert_eq!(client.get_snapshot_count(), 101);
}

// ── Test: Purge Continues After 101 ──────────────────────────────────────────────

#[test]
fn test_snapshot_purge_continues() {
    let (env, client, admin) = setup_env();

    // Take 105 snapshots
    for i in 1..=105 {
        client.take_statistics_snapshot(&admin);
    }

    // Verify oldest 5 are purged (IDs 1-5)
    assert!(client.get_snapshot(&1).is_none());
    assert!(client.get_snapshot(&2).is_none());
    assert!(client.get_snapshot(&3).is_none());
    assert!(client.get_snapshot(&4).is_none());
    assert!(client.get_snapshot(&5).is_none());

    // Verify IDs 6-105 still exist
    assert!(client.get_snapshot(&6).is_some());
    assert!(client.get_snapshot(&105).is_some());

    // Verify we have exactly 100 snapshots
    let ids = client.get_available_snapshot_ids();
    assert_eq!(ids.len(), 100);
}

// ── Test: Snapshot Requires Admin Auth ───────────────────────────────────────────

#[test]
#[should_panic(expected = "NotAnAdmin")]
fn test_snapshot_requires_admin_auth() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, KoraContract);
    let client = KoraContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.init_admin(&admin);

    // Clear auth mocking so real auth checks apply
    env.set_auths(&[]);

    // Non-admin tries to take snapshot
    let non_admin = Address::generate(&env);
    client.take_statistics_snapshot(&non_admin);
}
