use crate::{Gender, KoraContract, KoraContractClient, PrivacyLevel, Species};
use soroban_sdk::{
    testutils::{Address as _, Ledger},
    Address, Env, String, Vec,
};

/// Minimum retention in seconds that satisfies the Issue #102 floor (365 days).
const MIN_RETENTION: u64 = 365 * 86_400;

fn setup(env: &Env) -> (KoraContractClient<'_>, Address, Address, u64) {
    env.mock_all_auths();
    let contract_id = env.register_contract(None, KoraContract);
    let client = KoraContractClient::new(env, &contract_id);

    let admin = Address::generate(env);
    let vet = Address::generate(env);
    let owner = Address::generate(env);

    client.init_admin(&admin);
    client.register_vet(
        &vet,
        &String::from_str(env, "Dr. Purge"),
        &String::from_str(env, "LIC-PURGE"),
        &String::from_str(env, "General"),
    );
    client.verify_vet(&admin, &vet);

    let pet_id = client.register_pet(
        &owner,
        &String::from_str(env, "TestPet"),
        &String::from_str(env, "2020-01-01"),
        &Gender::Male,
        &Species::Dog,
        &String::from_str(env, "Labrador"),
        &String::from_str(env, "Brown"),
        &25,
        &None,
        &PrivacyLevel::Public,
    );

    (client, admin, vet, pet_id)
}

fn add_record(
    client: &KoraContractClient<'_>,
    env: &Env,
    vet: &Address,
    pet_id: u64,
    diagnosis: &str,
) -> u64 {
    client.add_medical_record(
        &pet_id,
        vet,
        &String::from_str(env, diagnosis),
        &String::from_str(env, "Treatment"),
        &Vec::new(env),
        &String::from_str(env, "test notes"),
    )
}

#[test]
fn test_purge_nothing_when_no_deleted_records() {
    let env = Env::default();
    let (client, admin, vet, pet_id) = setup(&env);

    add_record(&client, &env, &vet, pet_id, "Healthy");

    // Use the minimum legal retention period (Issue #102).
    client.set_retention_period(&admin, &MIN_RETENTION);
    let purged = client.purge_deleted_records(&pet_id, &admin, &false);
    assert_eq!(purged.deleted.len(), 0);
}

#[test]
fn test_purge_partial_old_and_new() {
    let env = Env::default();
    env.ledger().with_mut(|l| l.timestamp = 1_700_000_000);
    let (client, admin, vet, pet_id) = setup(&env);

    let r1 = add_record(&client, &env, &vet, pet_id, "Old1");
    let r2 = add_record(&client, &env, &vet, pet_id, "Old2");
    let r3 = add_record(&client, &env, &vet, pet_id, "Recent");

    // Delete r1 and r2 at t=0.
    client.delete_medical_record(&pet_id, &r1, &vet);
    client.delete_medical_record(&pet_id, &r2, &vet);

    // Delete r3 after MIN_RETENTION / 2 — it won't be old enough to purge.
    env.ledger()
        .with_mut(|l| l.timestamp = 1_700_000_000 + MIN_RETENTION / 2);
    client.delete_medical_record(&pet_id, &r3, &vet);

    // Advance just past MIN_RETENTION from the original deletion of r1/r2.
    env.ledger()
        .with_mut(|l| l.timestamp = 1_700_000_000 + MIN_RETENTION + 1);

    client.set_retention_period(&admin, &MIN_RETENTION);
    let purged = client.purge_deleted_records(&pet_id, &admin, &false);
    assert_eq!(purged.deleted.len(), 2);
}

#[test]
fn test_purge_all_old_records() {
    let env = Env::default();
    env.ledger().with_mut(|l| l.timestamp = 1_700_000_000);
    let (client, admin, vet, pet_id) = setup(&env);

    let r1 = add_record(&client, &env, &vet, pet_id, "Rec1");
    let r2 = add_record(&client, &env, &vet, pet_id, "Rec2");
    let r3 = add_record(&client, &env, &vet, pet_id, "Rec3");

    client.delete_medical_record(&pet_id, &r1, &vet);
    client.delete_medical_record(&pet_id, &r2, &vet);
    client.delete_medical_record(&pet_id, &r3, &vet);

    // Advance well past the retention window.
    env.ledger()
        .with_mut(|l| l.timestamp = 1_700_000_000 + MIN_RETENTION + 86_400);

    client.set_retention_period(&admin, &MIN_RETENTION);
    let purged = client.purge_deleted_records(&pet_id, &admin, &false);
    assert_eq!(purged.deleted.len(), 3);
}

#[test]
#[should_panic]
fn test_purge_requires_admin() {
    let env = Env::default();
    let (client, admin, vet, pet_id) = setup(&env);

    let r1 = add_record(&client, &env, &vet, pet_id, "Rec1");
    client.delete_medical_record(&pet_id, &r1, &vet);

    client.set_retention_period(&admin, &MIN_RETENTION);

    let stranger = Address::generate(&env);
    client.purge_deleted_records(&pet_id, &stranger, &false);
}
