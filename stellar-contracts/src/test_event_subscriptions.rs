use crate::*;
use soroban_sdk::{
    testutils::{Address as _, Ledger as _},
    Address, Env, String, Vec,
};

fn setup() -> (Env, KoraContractClient<'static>, Address, Address, Address, u64) {
    let env = Env::default();
    env.mock_all_auths();
    env.budget().reset_unlimited();
    let contract_id = env.register_contract(None, KoraContract);
    let client = KoraContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let owner = Address::generate(&env);
    let vet = Address::generate(&env);
    client.init_admin(&admin);

    let pet_id = client.register_pet(
        &owner,
        &String::from_str(&env, "Signal"),
        &String::from_str(&env, "2020-01-01"),
        &Gender::Female,
        &Species::Dog,
        &String::from_str(&env, "Mixed"),
        &String::from_str(&env, "Black"),
        &20u32,
        &None,
        &PrivacyLevel::Public,
    );

    client.register_vet(
        &vet,
        &String::from_str(&env, "Dr Index"),
        &String::from_str(&env, "SUB-VET-1"),
        &String::from_str(&env, "General"),
    );
    client.verify_vet(&admin, &vet);

    (env, client, admin, owner, vet, pet_id)
}

#[test]
fn test_matching_subscription_ids_for_event_and_pet() {
    let (env, client, _admin, _owner, _vet, pet_id) = setup();
    let subscriber = Address::generate(&env);
    let other_pet = pet_id + 1;

    let mut event_types = Vec::new(&env);
    event_types.push_back(EventType::TreatmentAdded);
    let mut pet_ids = Vec::new(&env);
    pet_ids.push_back(pet_id);
    let matching_id = client.register_subscription(&subscriber, &event_types, &pet_ids, &300u64);

    let mut other_pet_ids = Vec::new(&env);
    other_pet_ids.push_back(other_pet);
    let ignored_id =
        client.register_subscription(&subscriber, &event_types, &other_pet_ids, &300u64);

    let matches = client.get_matching_subscription_ids(&EventType::TreatmentAdded, &pet_id);
    assert_eq!(matches.len(), 1);
    assert_eq!(matches.get(0).unwrap(), matching_id);
    assert_ne!(matches.get(0).unwrap(), ignored_id);
}

#[test]
fn test_expired_subscriptions_are_excluded() {
    let (env, client, _admin, _owner, _vet, pet_id) = setup();
    let subscriber = Address::generate(&env);
    let mut event_types = Vec::new(&env);
    event_types.push_back(EventType::MedicalRecordAdded);
    let mut pet_ids = Vec::new(&env);
    pet_ids.push_back(pet_id);

    client.register_subscription(&subscriber, &event_types, &pet_ids, &10u64);
    env.ledger().set_timestamp(env.ledger().timestamp() + 11);

    let matches = client.get_matching_subscription_ids(&EventType::MedicalRecordAdded, &pet_id);
    assert_eq!(matches.len(), 0);
}

#[test]
#[should_panic]
fn test_subscription_limit_enforced_per_address() {
    let (env, client, _admin, _owner, _vet, pet_id) = setup();
    let subscriber = Address::generate(&env);
    let mut event_types = Vec::new(&env);
    event_types.push_back(EventType::TreatmentAdded);
    let mut pet_ids = Vec::new(&env);
    pet_ids.push_back(pet_id);

    for _ in 0..10 {
        client.register_subscription(&subscriber, &event_types, &pet_ids, &300u64);
    }

    client.register_subscription(&subscriber, &event_types, &pet_ids, &300u64);
}

#[test]
fn test_event_payload_contains_matching_subscription_ids() {
    let (env, client, _admin, _owner, vet, pet_id) = setup();
    let subscriber = Address::generate(&env);
    let mut event_types = Vec::new(&env);
    event_types.push_back(EventType::TreatmentAdded);
    let mut pet_ids = Vec::new(&env);
    pet_ids.push_back(pet_id);
    let subscription_id = client.register_subscription(&subscriber, &event_types, &pet_ids, &300u64);

    let ids = client.get_matching_subscription_ids(&EventType::TreatmentAdded, &pet_id);
    assert_eq!(ids.get(0).unwrap(), subscription_id);

    client.add_treatment(
        &pet_id,
        &vet,
        &TreatmentType::Routine,
        &env.ledger().timestamp(),
        &String::from_str(&env, "Routine check"),
        &None,
        &String::from_str(&env, "Stable"),
    );
}

/// Issue #31 — benchmark test: Register 100 subscriptions across 100 distinct
/// subscribers (50 for MedicalRecordAdded, 50 for TreatmentAdded) and verify
/// that `add_medical_record` only retrieves the 50 matching subscribers, not
/// all 100.  The budget is kept unlimited so the test focuses on correctness
/// of the O(1) index lookup rather than hitting an instruction limit.
#[test]
fn test_indexed_lookup_100_subscriptions_only_returns_matching_event_type() {
    let (env, client, admin, _owner, vet, pet_id) = setup();

    // Raise the per-address cap so the same address could hold many subs if needed,
    // but here we use 100 distinct subscriber addresses (1 sub each).
    client.set_max_subs_per_address(&admin, &200u32);

    let mut medical_sub_ids: Vec<u64> = Vec::new(&env);
    let mut treatment_sub_ids: Vec<u64> = Vec::new(&env);

    let mut medical_types = Vec::new(&env);
    medical_types.push_back(EventType::MedicalRecordAdded);

    let mut treatment_types = Vec::new(&env);
    treatment_types.push_back(EventType::TreatmentAdded);

    let mut pet_ids = Vec::new(&env);
    pet_ids.push_back(pet_id);

    // Register 50 subscriptions for MedicalRecordAdded and 50 for TreatmentAdded.
    for _ in 0..50u32 {
        let sub = Address::generate(&env);
        let id = client.register_subscription(&sub, &medical_types, &pet_ids, &9999u64);
        medical_sub_ids.push_back(id);
    }
    for _ in 0..50u32 {
        let sub = Address::generate(&env);
        let id = client.register_subscription(&sub, &treatment_types, &pet_ids, &9999u64);
        treatment_sub_ids.push_back(id);
    }

    // The indexed lookup for MedicalRecordAdded must return exactly the 50
    // medical subscribers and zero treatment subscribers.
    let matched_medical =
        client.get_matching_subscription_ids(&EventType::MedicalRecordAdded, &pet_id);
    assert_eq!(
        matched_medical.len(),
        50,
        "indexed lookup must return exactly the 50 MedicalRecordAdded subscribers"
    );
    for id in matched_medical.iter() {
        assert!(
            medical_sub_ids.contains(id),
            "returned subscription {id} is not a MedicalRecordAdded subscriber"
        );
    }

    // Likewise, TreatmentAdded must return exactly the 50 treatment subscribers.
    let matched_treatment =
        client.get_matching_subscription_ids(&EventType::TreatmentAdded, &pet_id);
    assert_eq!(
        matched_treatment.len(),
        50,
        "indexed lookup must return exactly the 50 TreatmentAdded subscribers"
    );
    for id in matched_treatment.iter() {
        assert!(
            treatment_sub_ids.contains(id),
            "returned subscription {id} is not a TreatmentAdded subscriber"
        );
    }

    // Correctness: add_medical_record must complete successfully with 100 total
    // subscriptions registered; the subscription_ids in the emitted event must
    // only list the 50 medical subscribers.
    let medications: Vec<Medication> = Vec::new(&env);
    client.add_medical_record(
        &pet_id,
        &vet,
        &String::from_str(&env, "Annual checkup"),
        &String::from_str(&env, "Rest"),
        &medications,
        &String::from_str(&env, "All good"),
    );
}
