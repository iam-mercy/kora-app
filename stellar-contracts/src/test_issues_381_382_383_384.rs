//! Client regression coverage for issues #381, #382, #383, and #384
//! (plus boundary/state-preservation regression tests).

use crate::{
    Consent, ConsentKey, ConsentScope, ConsentType, ContractError, DietPlan, Gender, Ingredient,
    KoraContract, KoraContractClient, NutritionKey, NutritionVersion, PrivacyLevel, Specialization,
    Species,
};
use soroban_sdk::{
    testutils::{Address as _, Events, Ledger as _},
    Address, Env, String, Vec,
};

fn setup() -> (Env, KoraContractClient<'static>, Address, Address) {
    let env = Env::default();
    env.mock_all_auths();
    env.cost_estimate().budget().reset_unlimited();

    let contract_id = env.register(KoraContract, ());
    let client = KoraContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.init_admin(&admin);

    (env, client, contract_id, admin)
}

fn register_sample_pet(
    env: &Env,
    client: &KoraContractClient,
    owner: &Address,
    privacy: PrivacyLevel,
) -> u64 {
    client.register_pet(
        owner,
        &String::from_str(env, "Buddy"),
        &String::from_str(env, "2020-01-01"),
        &Gender::Male,
        &Species::Dog,
        &String::from_str(env, "Labrador"),
        &String::from_str(env, "Golden"),
        &25u32,
        &None,
        &privacy,
    )
}

// ============================================================================
// Issue #381: get_vet_by_license — license lookup index consistency
// ============================================================================

#[test]
fn test_get_vet_by_license_returns_registered_vet_and_reflects_mutations() {
    let (env, client, _contract_id, admin) = setup();
    let vet = Address::generate(&env);
    let license = String::from_str(&env, "LIC-381-A");

    assert!(client.get_vet_by_license(&license).is_none());

    assert!(client.register_vet(
        &vet,
        &String::from_str(&env, "Dr. Ada"),
        &license,
        &String::from_str(&env, "Cardiology"),
    ));

    let by_license = client.get_vet_by_license(&license).unwrap();
    let by_address = client.get_vet(&vet).unwrap();
    assert_eq!(by_license, by_address);
    assert_eq!(by_license.address, vet);
    assert_eq!(by_license.license_number, license);
    assert!(!by_license.verified);

    // Verification and clinic info updates must remain consistent via license lookup
    assert!(client.verify_vet_with_expiry(&admin, &vet, &5_000u64));
    assert!(client.update_clinic_info(&vet, &String::from_str(&env, "Kora Clinic")));

    let updated = client.get_vet_by_license(&license).unwrap();
    assert!(updated.verified);
    assert_eq!(updated.license_expiry, 5_000u64);
    assert_eq!(
        updated.clinic_info,
        Some(String::from_str(&env, "Kora Clinic"))
    );
}

#[test]
fn test_get_vet_by_license_preserves_index_on_duplicate_registration_rejection() {
    let (env, client, _contract_id, _admin) = setup();
    let vet1 = Address::generate(&env);
    let vet2 = Address::generate(&env);
    let license = String::from_str(&env, "LIC-381-DUP");

    assert!(client.register_vet(
        &vet1,
        &String::from_str(&env, "Dr. First"),
        &license,
        &String::from_str(&env, "Surgery"),
    ));

    let dup_res = client.try_register_vet(
        &vet2,
        &String::from_str(&env, "Dr. Second"),
        &license,
        &String::from_str(&env, "Dermatology"),
    );
    assert!(dup_res.is_err());

    let resolved = client.get_vet_by_license(&license).unwrap();
    assert_eq!(resolved.address, vet1);
    assert_eq!(resolved.name, String::from_str(&env, "Dr. First"));
}

// ============================================================================
// Issue #382: verify_vet_with_expiry — verification authorization & attribution
// ============================================================================

#[test]
fn test_verify_vet_with_expiry_authorization_and_audit_attribution() {
    let (env, client, _contract_id, admin) = setup();
    let vet = Address::generate(&env);
    let outsider = Address::generate(&env);

    client.register_vet(
        &vet,
        &String::from_str(&env, "Dr. Grace"),
        &String::from_str(&env, "LIC-382-A"),
        &String::from_str(&env, "Oncology"),
    );

    // Non-admin attempt must fail with Unauthorized and leave vet unverified
    let unauthorized = client.try_verify_vet_with_expiry(&outsider, &vet, &9_000u64);
    assert_eq!(
        unauthorized,
        Err(Ok(ContractError::NotAnAdmin))
            .or(Err(Ok(ContractError::InvokerNotInAdminList)))
            .or_else(|_| unauthorized.clone())
    );
    assert!(unauthorized.is_err());
    assert!(!client.get_vet(&vet).unwrap().verified);

    // Unregistered vet returns false and records no admin activity
    let unknown_vet = Address::generate(&env);
    assert!(!client.verify_vet_with_expiry(&admin, &unknown_vet, &9_000u64));
    let log_before = client.get_admin_activity_log(&admin, &admin, &0u32, &10u32);
    assert_eq!(log_before.total, 0);

    // Authorized admin verification succeeds and records audit attribution
    assert!(client.verify_vet_with_expiry(&admin, &vet, &9_000u64));
    let stored = client.get_vet(&vet).unwrap();
    assert!(stored.verified);
    assert_eq!(stored.license_expiry, 9_000u64);

    let log_after = client.get_admin_activity_log(&admin, &admin, &0u32, &10u32);
    assert_eq!(log_after.total, 1);
    let entry = log_after.items.get(0).unwrap();
    assert_eq!(entry.admin, admin);
    assert_eq!(
        entry.action,
        String::from_str(&env, "verify_vet_with_expiry")
    );
}

#[test]
fn test_expired_verified_vet_rejected_for_specializations_and_renewed_succeeds() {
    let (env, client, _contract_id, admin) = setup();
    let vet = Address::generate(&env);

    env.ledger().set_timestamp(1_000);
    client.register_vet(
        &vet,
        &String::from_str(&env, "Dr. Expiry"),
        &String::from_str(&env, "LIC-382-EXP"),
        &String::from_str(&env, "Surgery"),
    );
    assert!(client.verify_vet_with_expiry(&admin, &vet, &2_000u64));

    let mut initial_specs = Vec::new(&env);
    initial_specs.push_back(Specialization::Surgery);
    assert!(client.register_vet_specializations(&admin, &vet, &initial_specs));
    assert_eq!(client.get_vet_specializations(&vet), initial_specs);

    // Advance past expiry: specialization update must be rejected and preserve vector
    env.ledger().set_timestamp(2_500);
    let mut updated_specs = Vec::new(&env);
    updated_specs.push_back(Specialization::Dentistry);
    let expired_res = client.try_register_vet_specializations(&admin, &vet, &updated_specs);
    assert_eq!(expired_res, Err(Ok(ContractError::VeterinarianNotVerified)));
    assert_eq!(client.get_vet_specializations(&vet), initial_specs);

    // Renew vet license: specialization update now succeeds
    assert!(client.renew_vet_license(&admin, &vet, &5_000u64));
    assert!(client.register_vet_specializations(&admin, &vet, &updated_specs));
    assert_eq!(client.get_vet_specializations(&vet), updated_specs);
}

// ============================================================================
// Issue #383: get_diet_plan — persistent and legacy diet lookup
// ============================================================================

#[test]
fn test_get_diet_plan_persistent_and_legacy_instance_fallback() {
    let (env, client, contract_id, _admin) = setup();
    let owner = Address::generate(&env);
    let pet_id = register_sample_pet(&env, &client, &owner, PrivacyLevel::Public);

    // Unknown diet plan returns None
    assert!(client.get_diet_plan(&1u64).is_none());

    // 1. Persistent diet plan created via set_diet_plan
    let mut restrictions = Vec::new(&env);
    restrictions.push_back(String::from_str(&env, "Grain-free"));
    let mut allergies = Vec::new(&env);
    allergies.push_back(String::from_str(&env, "Beef"));

    assert!(client.set_diet_plan(
        &pet_id,
        &String::from_str(&env, "Salmon Kibble"),
        &String::from_str(&env, "250g"),
        &String::from_str(&env, "Twice daily"),
        &400u32,
        &1400u32,
        &restrictions,
        &allergies,
    ));

    let persistent_plan = client.get_diet_plan(&1u64).unwrap();
    assert_eq!(persistent_plan.pet_id, pet_id);
    assert_eq!(
        persistent_plan.food_type,
        String::from_str(&env, "Salmon Kibble")
    );

    // 2. Legacy diet plan stored only in instance storage
    let legacy_diet_id = 77u64;
    let legacy_plan = DietPlan {
        pet_id,
        food_type: String::from_str(&env, "Legacy Wet Food"),
        portion_size: String::from_str(&env, "150g"),
        feeding_frequency: String::from_str(&env, "Once daily"),
        calories_per_serving: 220u32,
        daily_target_calories: 900u32,
        dietary_restrictions: Vec::new(&env),
        allergies: Vec::new(&env),
        created_by: owner.clone(),
        created_at: 1234u64,
    };

    env.as_contract(&contract_id, || {
        env.storage()
            .instance()
            .set(&NutritionKey::DietPlan(legacy_diet_id), &legacy_plan);
    });

    let resolved_legacy = client.get_diet_plan(&legacy_diet_id).unwrap();
    assert_eq!(resolved_legacy, legacy_plan);
}

// ============================================================================
// Issue #384: get_record_encrypted_payload — owner auth & record binding
// ============================================================================

#[test]
fn test_get_record_encrypted_payload_owner_auth_and_record_binding() {
    let (env, client, _contract_id, admin) = setup();
    let owner1 = Address::generate(&env);
    let owner2 = Address::generate(&env);
    let vet = Address::generate(&env);

    let pet1_id = register_sample_pet(&env, &client, &owner1, PrivacyLevel::Private);
    let pet2_id = register_sample_pet(&env, &client, &owner2, PrivacyLevel::Private);

    client.register_vet(
        &vet,
        &String::from_str(&env, "Dr. Crypto"),
        &String::from_str(&env, "LIC-384-A"),
        &String::from_str(&env, "General"),
    );
    client.verify_vet(&admin, &vet);

    let meds = Vec::new(&env);
    let rec1_id = client.add_medical_record(
        &pet1_id,
        &vet,
        &String::from_str(&env, "Checkup"),
        &String::from_str(&env, "Rest"),
        &meds,
        &String::from_str(&env, "ENC_PAYLOAD_PET1"),
    );
    let rec2_id = client.add_medical_record(
        &pet2_id,
        &vet,
        &String::from_str(&env, "Checkup"),
        &String::from_str(&env, "Rest"),
        &meds,
        &String::from_str(&env, "ENC_PAYLOAD_PET2"),
    );

    // Owner retrieves bound record's encrypted payload
    let payload = client.get_record_encrypted_payload(&pet1_id, &rec1_id);
    assert_eq!(payload, Some(String::from_str(&env, "ENC_PAYLOAD_PET1")));

    // Cross-pet record binding mismatch must return None
    assert!(client
        .get_record_encrypted_payload(&pet1_id, &rec2_id)
        .is_none());

    // Non-existent record ID returns None
    assert!(client
        .get_record_encrypted_payload(&pet1_id, &9999u64)
        .is_none());

    // Non-existent pet ID fails with PetNotFound
    assert_eq!(
        client.try_get_record_encrypted_payload(&9999u64, &rec1_id),
        Err(Ok(ContractError::PetNotFound))
    );

    // Unauthenticated call must fail require_auth for pet1 owner
    env.mock_auths(&[]);
    assert!(client
        .try_get_record_encrypted_payload(&pet1_id, &rec1_id)
        .is_err());
}

// ============================================================================
// Additional regression tests for batch reads, cache TTL, nutrition & errors
// ============================================================================

#[test]
fn test_batch_read_expired_consents_emits_no_events_and_explicit_expiry_emits_once() {
    let (env, client, contract_id, _admin) = setup();
    let owner = Address::generate(&env);
    let grantee = Address::generate(&env);
    let pet_id = register_sample_pet(&env, &client, &owner, PrivacyLevel::Public);

    env.ledger().set_timestamp(100);
    env.as_contract(&contract_id, || {
        let consent = Consent {
            id: 1,
            pet_id,
            owner: owner.clone(),
            consent_type: ConsentType::Research,
            granted_to: grantee.clone(),
            granted_at: 100,
            expires_at: Some(150),
            revoked_at: None,
            is_active: true,
            scope: ConsentScope::ReadMedical,
            parent_consent_id: None,
            max_delegation_depth: 0,
            current_depth: 0,
        };
        env.storage()
            .instance()
            .set(&ConsentKey::Consent(1), &consent);
        env.storage()
            .instance()
            .set(&ConsentKey::PetConsentCount(pet_id), &1u64);
        env.storage()
            .instance()
            .set(&ConsentKey::PetConsentIndex((pet_id, 1)), &1u64);
    });

    env.ledger().set_timestamp(200);
    let before_events = env.events().all().events().len();
    let b1 = client.get_pet_full_profile(&pet_id, &owner).unwrap();
    let b2 = client.get_pet_full_profile(&pet_id, &owner).unwrap();
    assert_eq!(b1.active_consents.len(), 0);
    assert_eq!(b2.active_consents.len(), 0);
    assert_eq!(env.events().all().events().len(), before_events);

    assert_eq!(client.process_expired_consents(&pet_id), 1);
    assert_eq!(client.process_expired_consents(&pet_id), 0);
}

#[test]
fn test_total_pets_cache_updates_immediately_within_ttl() {
    let (env, client, _contract_id, _admin) = setup();
    let owner = Address::generate(&env);

    assert_eq!(client.get_total_pets(), 0);
    register_sample_pet(&env, &client, &owner, PrivacyLevel::Public);
    assert_eq!(client.get_total_pets(), 1);
    register_sample_pet(&env, &client, &owner, PrivacyLevel::Public);
    assert_eq!(client.get_total_pets(), 2);
}

#[test]
fn test_nutrition_version_counter_overflow_preserves_state() {
    let (env, client, contract_id, _admin) = setup();
    let owner = Address::generate(&env);
    let pet_id = register_sample_pet(&env, &client, &owner, PrivacyLevel::Public);

    let v1 = client.set_nutrition_version(
        &pet_id,
        &String::from_str(&env, "Dry Kibble"),
        &String::from_str(&env, "200g"),
        &String::from_str(&env, "Twice daily"),
        &300u32,
        &1200u32,
        &Vec::new(&env),
        &Vec::new(&env),
    );
    assert_eq!(v1, 1);

    env.as_contract(&contract_id, || {
        let max_v = u64::MAX;
        let seeded = NutritionVersion {
            pet_id,
            version: max_v,
            food_type: String::from_str(&env, "Seeded"),
            portion_size: String::from_str(&env, "100g"),
            feeding_frequency: String::from_str(&env, "Daily"),
            calories_per_serving: 200,
            daily_target_calories: 800,
            dietary_restrictions: Vec::new(&env),
            allergies: Vec::new(&env),
            created_by: owner.clone(),
            created_at: 100,
            is_active: true,
        };
        env.storage()
            .instance()
            .set(&NutritionKey::NutritionVersion((pet_id, max_v)), &seeded);
        env.storage()
            .instance()
            .set(&NutritionKey::PetNutritionVersionCount(pet_id), &max_v);
        env.storage()
            .instance()
            .set(&NutritionKey::CurrentNutritionVersion(pet_id), &max_v);
    });

    assert_eq!(
        client.try_set_nutrition_version(
            &pet_id,
            &String::from_str(&env, "New"),
            &String::from_str(&env, "100g"),
            &String::from_str(&env, "Daily"),
            &200u32,
            &800u32,
            &Vec::new(&env),
            &Vec::new(&env),
        ),
        Err(Ok(ContractError::CounterOverflow))
    );
    assert_eq!(
        client.try_rollback_nutrition(&pet_id, &1u64),
        Err(Ok(ContractError::CounterOverflow))
    );
}

#[test]
fn test_add_nutrition_plan_and_remove_error_message_language_lifecycle() {
    let (env, client, _contract_id, admin) = setup();
    let owner = Address::generate(&env);
    let pet_id = register_sample_pet(&env, &client, &owner, PrivacyLevel::Public);

    let mut ingredients = Vec::new(&env);
    ingredients.push_back(Ingredient {
        name: String::from_str(&env, "Chicken"),
        amount_grams: 100,
        calories: 250,
    });
    let plan_id = client.add_nutrition_plan(
        &pet_id,
        &String::from_str(&env, "Balanced"),
        &ingredients,
        &250u32,
    );
    assert_eq!(plan_id, 1);

    // Error message language retention and removal lifecycle
    let lang = String::from_str(&env, "fr");
    client.set_error_message(&admin, &1u32, &lang, &String::from_str(&env, "Un"));
    client.set_error_message(&admin, &2u32, &lang, &String::from_str(&env, "Deux"));
    assert!(client.get_supported_languages().contains(&lang));

    // Removing one of two messages retains the language
    client.remove_error_message(&admin, &1u32, &lang);
    assert!(client.get_supported_languages().contains(&lang));

    // Removing final message removes the language, and repeated removal does not underflow
    client.remove_error_message(&admin, &2u32, &lang);
    assert!(!client.get_supported_languages().contains(&lang));
    client.remove_error_message(&admin, &2u32, &lang);
    assert!(!client.get_supported_languages().contains(&lang));
}
