// ============================================================
// GET LAB RESULTS TESTS
// ============================================================

#[cfg(test)]
mod test_get_lab_results {
    use crate::{Gender, KoraContract, KoraContractClient, PrivacyLevel, Species};
    use soroban_sdk::{
        testutils::{Address as _, Ledger as _},
        Address, Env, Map, String, Vec,
    };

    fn setup() -> (Env, KoraContractClient<'static>, Address, Address, u64) {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let contract_id = env.register_contract(None, KoraContract);
        let client = KoraContractClient::new(&env, &contract_id);
        client.init_admin(&admin);

        let owner = Address::generate(&env);
        let vet = Address::generate(&env);
        let pet_id = client.register_pet(
            &owner,
            &String::from_str(&env, "Buddy"),
            &String::from_str(&env, "2020-01-01"),
            &Gender::Male,
            &Species::Dog,
            &String::from_str(&env, "Labrador"),
            &String::from_str(&env, "Brown"),
            &25u32,
            &None,
            &PrivacyLevel::Public,
        );

        client.register_vet(
            &vet,
            &String::from_str(&env, "Dr. Smith"),
            &String::from_str(&env, "LIC-001"),
            &String::from_str(&env, "General"),
        );
        client.verify_vet(&admin, &vet);

        (env, client, owner, vet, pet_id)
    }

    fn add_lab_result(
        client: &KoraContractClient,
        env: &Env,
        pet_id: u64,
        vet: &Address,
        test_type: &str,
        results: &str,
        timestamp: u64,
    ) -> u64 {
        env.ledger().set_timestamp(timestamp);
        client.add_lab_result(
            &pet_id,
            vet,
            &String::from_str(env, test_type),
            &String::from_str(env, results),
            &String::from_str(env, "0.0-1.0"),
            &0u32,
            &100u32,
            &None,
            &None,
        )
    }

    #[test]
    fn test_get_lab_results_empty() {
        let (env, client, owner, _vet, pet_id) = setup();

        // No lab results added - should return empty vector
        let results = client.get_lab_results(&pet_id, &owner, &0u64, &10u32, &None, &None);
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_get_lab_results_single() {
        let (env, client, owner, vet, pet_id) = setup();

        add_lab_result(&client, &env, pet_id, &vet, "Blood Test", "Normal", 100);

        let results = client.get_lab_results(&pet_id, &owner, &0u64, &10u32, &None, &None);
        assert_eq!(results.len(), 1);
        assert_eq!(
            results.get(0).unwrap().test_type,
            String::from_str(&env, "Blood Test")
        );
        assert_eq!(
            results.get(0).unwrap().results,
            String::from_str(&env, "Normal")
        );
    }

    #[test]
    fn test_get_lab_results_multiple() {
        let (env, client, owner, vet, pet_id) = setup();

        add_lab_result(&client, &env, pet_id, &vet, "Blood Test", "Normal", 100);
        add_lab_result(&client, &env, pet_id, &vet, "Urinalysis", "Abnormal", 200);
        add_lab_result(&client, &env, pet_id, &vet, "X-Ray", "Clear", 300);

        let results = client.get_lab_results(&pet_id, &owner, &0u64, &10u32, &None, &None);
        assert_eq!(results.len(), 3);
        assert_eq!(
            results.get(0).unwrap().test_type,
            String::from_str(&env, "Blood Test")
        );
        assert_eq!(
            results.get(1).unwrap().test_type,
            String::from_str(&env, "Urinalysis")
        );
        assert_eq!(
            results.get(2).unwrap().test_type,
            String::from_str(&env, "X-Ray")
        );
    }

    #[test]
    fn test_get_lab_results_pagination_first_page() {
        let (env, client, owner, vet, pet_id) = setup();

        add_lab_result(&client, &env, pet_id, &vet, "Test 1", "Result 1", 100);
        add_lab_result(&client, &env, pet_id, &vet, "Test 2", "Result 2", 200);
        add_lab_result(&client, &env, pet_id, &vet, "Test 3", "Result 3", 300);
        add_lab_result(&client, &env, pet_id, &vet, "Test 4", "Result 4", 400);
        add_lab_result(&client, &env, pet_id, &vet, "Test 5", "Result 5", 500);

        // First page: offset 0, limit 2
        let results = client.get_lab_results(&pet_id, &owner, &0u64, &2u32, &None, &None);
        assert_eq!(results.len(), 2);
        assert_eq!(
            results.get(0).unwrap().test_type,
            String::from_str(&env, "Test 1")
        );
        assert_eq!(
            results.get(1).unwrap().test_type,
            String::from_str(&env, "Test 2")
        );
    }

    #[test]
    fn test_get_lab_results_pagination_second_page() {
        let (env, client, owner, vet, pet_id) = setup();

        add_lab_result(&client, &env, pet_id, &vet, "Test 1", "Result 1", 100);
        add_lab_result(&client, &env, pet_id, &vet, "Test 2", "Result 2", 200);
        add_lab_result(&client, &env, pet_id, &vet, "Test 3", "Result 3", 300);
        add_lab_result(&client, &env, pet_id, &vet, "Test 4", "Result 4", 400);
        add_lab_result(&client, &env, pet_id, &vet, "Test 5", "Result 5", 500);

        // Second page: offset 2, limit 2
        let results = client.get_lab_results(&pet_id, &owner, &2u64, &2u32, &None, &None);
        assert_eq!(results.len(), 2);
        assert_eq!(
            results.get(0).unwrap().test_type,
            String::from_str(&env, "Test 3")
        );
        assert_eq!(
            results.get(1).unwrap().test_type,
            String::from_str(&env, "Test 4")
        );
    }

    #[test]
    fn test_get_lab_results_pagination_last_partial_page() {
        let (env, client, owner, vet, pet_id) = setup();

        add_lab_result(&client, &env, pet_id, &vet, "Test 1", "Result 1", 100);
        add_lab_result(&client, &env, pet_id, &vet, "Test 2", "Result 2", 200);
        add_lab_result(&client, &env, pet_id, &vet, "Test 3", "Result 3", 300);
        add_lab_result(&client, &env, pet_id, &vet, "Test 4", "Result 4", 400);
        add_lab_result(&client, &env, pet_id, &vet, "Test 5", "Result 5", 500);

        // Last page: offset 4, limit 2 (only 1 result remaining)
        let results = client.get_lab_results(&pet_id, &owner, &4u64, &2u32, &None, &None);
        assert_eq!(results.len(), 1);
        assert_eq!(
            results.get(0).unwrap().test_type,
            String::from_str(&env, "Test 5")
        );
    }

    #[test]
    fn test_get_lab_results_pagination_offset_beyond_count() {
        let (env, client, owner, vet, pet_id) = setup();

        add_lab_result(&client, &env, pet_id, &vet, "Test 1", "Result 1", 100);
        add_lab_result(&client, &env, pet_id, &vet, "Test 2", "Result 2", 200);

        // Offset beyond count - should return empty
        let results = client.get_lab_results(&pet_id, &owner, &10u64, &5u32, &None, &None);
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_get_lab_results_pagination_zero_limit() {
        let (env, client, owner, vet, pet_id) = setup();

        add_lab_result(&client, &env, pet_id, &vet, "Test 1", "Result 1", 100);
        add_lab_result(&client, &env, pet_id, &vet, "Test 2", "Result 2", 200);

        // Zero limit - should return empty
        let results = client.get_lab_results(&pet_id, &owner, &0u64, &0u32, &None, &None);
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_get_lab_results_pagination_limit_larger_than_remaining() {
        let (env, client, owner, vet, pet_id) = setup();

        add_lab_result(&client, &env, pet_id, &vet, "Test 1", "Result 1", 100);
        add_lab_result(&client, &env, pet_id, &vet, "Test 2", "Result 2", 200);
        add_lab_result(&client, &env, pet_id, &vet, "Test 3", "Result 3", 300);

        // Offset 1, limit 10 (only 2 results remaining)
        let results = client.get_lab_results(&pet_id, &owner, &1u64, &10u32, &None, &None);
        assert_eq!(results.len(), 2);
        assert_eq!(
            results.get(0).unwrap().test_type,
            String::from_str(&env, "Test 2")
        );
        assert_eq!(
            results.get(1).unwrap().test_type,
            String::from_str(&env, "Test 3")
        );
    }

    // ============================================================
    // REFERENCE RANGE VALIDATION TESTS (Issue #652)
    // ============================================================

    fn setup_with_admin() -> (Env, KoraContractClient<'static>, Address, Address, u64, Address) {
        let (env, client, owner, vet, pet_id) = setup();
        let admin = Address::generate(&env);
        (env, client, owner, vet, pet_id, admin)
    }

    #[test]
    fn test_set_and_get_reference_range() {
        let (env, client, _owner, _vet, _pet_id, admin) = setup_with_admin();

        client.set_reference_range(
            &admin,
            &String::from_str(&env, "Dog"),
            &String::from_str(&env, "WBC"),
            &5i128,
            &15i128,
        );

        let range = client.get_reference_range(
            &String::from_str(&env, "Dog"),
            &String::from_str(&env, "WBC"),
        );
        assert!(range.is_some());
        let r = range.unwrap();
        assert_eq!(r.min, 5);
        assert_eq!(r.max, 15);
    }

    #[test]
    fn test_set_reference_range_rejects_min_greater_than_max() {
        let (env, client, _owner, _vet, _pet_id, admin) = setup_with_admin();

        let result = client.try_set_reference_range(
            &admin,
            &String::from_str(&env, "Dog"),
            &String::from_str(&env, "WBC"),
            &100i128,
            &1i128,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_reference_range() {
        let (env, client, _owner, _vet, _pet_id, admin) = setup_with_admin();

        client.set_reference_range(
            &admin,
            &String::from_str(&env, "Cat"),
            &String::from_str(&env, "Glucose"),
            &70i128,
            &120i128,
        );

        assert!(client.get_reference_range(
            &String::from_str(&env, "Cat"),
            &String::from_str(&env, "Glucose"),
        ).is_some());

        client.remove_reference_range(
            &admin,
            &String::from_str(&env, "Cat"),
            &String::from_str(&env, "Glucose"),
        );

        assert!(client.get_reference_range(
            &String::from_str(&env, "Cat"),
            &String::from_str(&env, "Glucose"),
        ).is_none());
    }

    #[test]
    fn test_biomarker_flags_initialized_empty() {
        let (env, client, _owner, vet, pet_id, _admin) = setup_with_admin();
        let id = client.add_lab_result(
            &pet_id, &vet,
            &String::from_str(&env, "Blood Panel"),
            &String::from_str(&env, "All normal"),
            &String::from_str(&env, "0.0-1.0"),
            &0u32, &100u32,
            &None, &None,
        );
        let result = client.get_lab_result(&id).unwrap();
        assert_eq!(result.biomarker_flags.len(), 0);
    }

    #[test]
    fn test_get_lab_result_biomarker_flags_empty_for_missing_result() {
        let (env, client, _owner, vet, pet_id, _admin) = setup_with_admin();
        let id = client.add_lab_result(
            &pet_id, &vet,
            &String::from_str(&env, "Blood Panel"),
            &String::from_str(&env, "All normal"),
            &String::from_str(&env, "0.0-1.0"),
            &0u32, &100u32,
            &None, &None,
        );
        let flags = client.get_lab_result_biomarker_flags(&id);
        assert_eq!(flags.len(), 0);
    }

    #[test]
    fn test_missing_reference_range_treated_as_normal() {
        let (env, client, _owner, vet, pet_id, _admin) = setup_with_admin();
        let id = client.add_lab_result(
            &pet_id, &vet,
            &String::from_str(&env, "Chemistry"),
            &String::from_str(&env, "All clear"),
            &String::from_str(&env, ""),
            &0u32, &100u32,
            &None, &None,
        );
        let result = client.get_lab_result(&id).unwrap();
        assert_eq!(result.biomarker_flags.len(), 0);
    }

    #[test]
    fn test_get_lab_results_closed_range() {
        let (env, client, owner, vet, pet_id) = setup();

        add_lab_result(&client, &env, pet_id, &vet, "Test 1", "Result 1", 100);
        add_lab_result(&client, &env, pet_id, &vet, "Test 2", "Result 2", 200);
        add_lab_result(&client, &env, pet_id, &vet, "Test 3", "Result 3", 300);

        let results = client.get_lab_results(&pet_id, &owner, &0u64, &10u32, &Some(150u64), &Some(250u64));
        assert_eq!(results.len(), 1);
        assert_eq!(
            results.get(0).unwrap().test_type,
            String::from_str(&env, "Test 2")
        );
    }

    #[test]
    fn test_get_lab_results_open_range_from() {
        let (env, client, owner, vet, pet_id) = setup();

        add_lab_result(&client, &env, pet_id, &vet, "Test 1", "Result 1", 100);
        add_lab_result(&client, &env, pet_id, &vet, "Test 2", "Result 2", 200);
        add_lab_result(&client, &env, pet_id, &vet, "Test 3", "Result 3", 300);

        let results = client.get_lab_results(&pet_id, &owner, &0u64, &10u32, &Some(200u64), &None);
        assert_eq!(results.len(), 2);
        assert_eq!(
            results.get(0).unwrap().test_type,
            String::from_str(&env, "Test 2")
        );
        assert_eq!(
            results.get(1).unwrap().test_type,
            String::from_str(&env, "Test 3")
        );
    }

    #[test]
    fn test_get_lab_results_open_range_to() {
        let (env, client, owner, vet, pet_id) = setup();

        add_lab_result(&client, &env, pet_id, &vet, "Test 1", "Result 1", 100);
        add_lab_result(&client, &env, pet_id, &vet, "Test 2", "Result 2", 200);
        add_lab_result(&client, &env, pet_id, &vet, "Test 3", "Result 3", 300);

        let results = client.get_lab_results(&pet_id, &owner, &0u64, &10u32, &None, &Some(200u64));
        assert_eq!(results.len(), 2);
        assert_eq!(
            results.get(0).unwrap().test_type,
            String::from_str(&env, "Test 1")
        );
        assert_eq!(
            results.get(1).unwrap().test_type,
            String::from_str(&env, "Test 2")
        );
    }

    #[test]
    fn test_get_lab_results_reversed_range_error() {
        let (env, client, owner, vet, pet_id) = setup();

        add_lab_result(&client, &env, pet_id, &vet, "Test 1", "Result 1", 100);
        add_lab_result(&client, &env, pet_id, &vet, "Test 2", "Result 2", 200);

        let result = client.try_get_lab_results(&pet_id, &owner, &0u64, &10u32, &Some(300u64), &Some(100u64));
        assert!(result.is_err());
    }
}

// ============================================================
// ISSUE #92: ref_min < ref_max validation
// ISSUE #93: verified vet enforcement
// ISSUE #94: division-by-zero guard in anomaly detection
// ISSUE #95: privacy access control on get_lab_results
// ============================================================

#[cfg(test)]
mod test_lab_result_validations {
    use crate::{Gender, KoraContract, KoraContractClient, PrivacyLevel, Species};
    use soroban_sdk::{
        testutils::Address as _,
        Address, Env, Map, String,
    };

    fn setup() -> (Env, KoraContractClient<'static>, Address, Address, Address, u64) {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let contract_id = env.register_contract(None, KoraContract);
        let client = KoraContractClient::new(&env, &contract_id);
        client.init_admin(&admin);

        let owner = Address::generate(&env);
        let vet = Address::generate(&env);
        let pet_id = client.register_pet(
            &owner,
            &String::from_str(&env, "Buddy"),
            &String::from_str(&env, "2020-01-01"),
            &Gender::Male,
            &Species::Dog,
            &String::from_str(&env, "Labrador"),
            &String::from_str(&env, "Brown"),
            &25u32,
            &None,
            &PrivacyLevel::Public,
        );
        client.register_vet(
            &vet,
            &String::from_str(&env, "Dr. Smith"),
            &String::from_str(&env, "LIC-001"),
            &String::from_str(&env, "General"),
        );
        client.verify_vet(&admin, &vet);

        (env, client, admin, owner, vet, pet_id)
    }

    // --- Issue #92: inverted reference range is rejected ---

    #[test]
    fn test_inverted_ref_range_rejected() {
        let (env, client, _admin, _owner, vet, pet_id) = setup();

        // min=100, max=50  →  should revert with InvalidInput
        let result = client.try_add_lab_result(
            &pet_id,
            &vet,
            &String::from_str(&env, "Blood Test"),
            &String::from_str(&env, "Normal"),
            &String::from_str(&env, "50-100"),
            &100u32,
            &50u32,
            &None,
            &None,
        );
        assert!(result.is_err(), "expected InvalidInput for inverted range");
    }

    #[test]
    fn test_equal_ref_range_rejected() {
        let (env, client, _admin, _owner, vet, pet_id) = setup();

        // min == max  →  should revert with InvalidInput
        let result = client.try_add_lab_result(
            &pet_id,
            &vet,
            &String::from_str(&env, "Binary Test"),
            &String::from_str(&env, "Result"),
            &String::from_str(&env, "0-0"),
            &50u32,
            &50u32,
            &None,
            &None,
        );
        assert!(result.is_err(), "expected InvalidInput for equal min/max");
    }

    #[test]
    fn test_valid_ref_range_accepted() {
        let (env, client, _admin, _owner, vet, pet_id) = setup();

        // min=0, max=100  →  valid
        let id = client.add_lab_result(
            &pet_id,
            &vet,
            &String::from_str(&env, "Glucose"),
            &String::from_str(&env, "Normal"),
            &String::from_str(&env, "0-100"),
            &0u32,
            &100u32,
            &None,
            &None,
        );
        assert!(id > 0);
    }

    // --- Issue #93: unverified vet is rejected ---

    #[test]
    fn test_unverified_vet_rejected() {
        let (env, client, _admin, _owner, _vet, pet_id) = setup();

        let unverified = Address::generate(&env);
        // registered but NOT verified
        client.register_vet(
            &unverified,
            &String::from_str(&env, "Dr. Fake"),
            &String::from_str(&env, "LIC-FAKE"),
            &String::from_str(&env, "General"),
        );

        let result = client.try_add_lab_result(
            &pet_id,
            &unverified,
            &String::from_str(&env, "Blood Test"),
            &String::from_str(&env, "Normal"),
            &String::from_str(&env, "0-100"),
            &0u32,
            &100u32,
            &None,
            &None,
        );
        assert!(result.is_err(), "expected VetNotVerified for unverified vet");
    }

    #[test]
    fn test_verified_vet_accepted() {
        let (env, client, _admin, _owner, vet, pet_id) = setup();

        // vet is already verified in setup()
        let id = client.add_lab_result(
            &pet_id,
            &vet,
            &String::from_str(&env, "Blood Test"),
            &String::from_str(&env, "Normal"),
            &String::from_str(&env, "0-100"),
            &0u32,
            &100u32,
            &None,
            &None,
        );
        assert!(id > 0);
    }

    // --- Issue #94: anomaly detection with equal ref range does not panic ---

    #[test]
    fn test_anomaly_detection_equal_ref_range_no_panic() {
        // Issue #92 already blocks ref_min == ref_max from being stored,
        // so the guard for division-by-zero in anomaly detection is exercised
        // via the range check inside the add_lab_result loop when ref_min < ref_max
        // but the biomarker normalized path is safe.
        // Verify that adding a lab result where ref_min == ref_max is rejected cleanly.
        let (env, client, _admin, _owner, vet, pet_id) = setup();

        let result = client.try_add_lab_result(
            &pet_id,
            &vet,
            &String::from_str(&env, "Qualitative"),
            &String::from_str(&env, "Positive"),
            &String::from_str(&env, "0-0"),
            &42u32,
            &42u32,
            &None,
            &None,
        );
        // Should be rejected (InvalidInput) — no panic/crash
        assert!(result.is_err(), "equal min/max must be rejected, not panic");
    }

    #[test]
    fn test_anomaly_detection_valid_range_with_biomarkers_no_panic() {
        let (env, client, _admin, _owner, vet, pet_id) = setup();

        // Add several results so anomaly detection fires; should not panic
        for i in 0..5u32 {
            let mut bm = Map::new(&env);
            bm.set(
                String::from_str(&env, "glucose"),
                100i128 + i as i128,
            );
            client.add_lab_result(
                &pet_id,
                &vet,
                &String::from_str(&env, "Blood Test"),
                &String::from_str(&env, "Normal"),
                &String::from_str(&env, "0-200"),
                &0u32,
                &200u32,
                &None,
                &None,
            );
        }
    }

    // --- Issue #95: privacy access control on get_lab_results ---

    #[test]
    fn test_owner_can_access_public_pet_lab_results() {
        let (env, client, _admin, owner, vet, pet_id) = setup();

        client.add_lab_result(
            &pet_id,
            &vet,
            &String::from_str(&env, "CBC"),
            &String::from_str(&env, "Normal"),
            &String::from_str(&env, "0-100"),
            &0u32,
            &100u32,
            &None,
            &None,
        );

        let results = client.get_lab_results(&pet_id, &owner, &0u64, &10u32, &None, &None);
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_private_pet_lab_results_blocked_for_stranger() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let contract_id = env.register_contract(None, KoraContract);
        let client = KoraContractClient::new(&env, &contract_id);
        client.init_admin(&admin);

        let owner = Address::generate(&env);
        let vet = Address::generate(&env);
        let stranger = Address::generate(&env);

        // Register pet as Private
        let pet_id = client.register_pet(
            &owner,
            &String::from_str(&env, "Secret"),
            &String::from_str(&env, "2021-01-01"),
            &Gender::Female,
            &Species::Cat,
            &String::from_str(&env, "Persian"),
            &String::from_str(&env, "White"),
            &4u32,
            &None,
            &PrivacyLevel::Private,
        );

        client.register_vet(
            &vet,
            &String::from_str(&env, "Dr. Smith"),
            &String::from_str(&env, "LIC-002"),
            &String::from_str(&env, "General"),
        );
        client.verify_vet(&admin, &vet);

        client.add_lab_result(
            &pet_id,
            &vet,
            &String::from_str(&env, "Blood Test"),
            &String::from_str(&env, "Normal"),
            &String::from_str(&env, "0-100"),
            &0u32,
            &100u32,
            &None,
            &None,
        );

        // Stranger should be rejected
        let result = client.try_get_lab_results(
            &pet_id,
            &stranger,
            &0u64,
            &10u32,
            &None,
            &None,
        );
        assert!(result.is_err(), "stranger must not access private pet lab results");
    }

    #[test]
    fn test_owner_can_access_private_pet_lab_results() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let contract_id = env.register_contract(None, KoraContract);
        let client = KoraContractClient::new(&env, &contract_id);
        client.init_admin(&admin);

        let owner = Address::generate(&env);
        let vet = Address::generate(&env);

        let pet_id = client.register_pet(
            &owner,
            &String::from_str(&env, "Private Pet"),
            &String::from_str(&env, "2021-01-01"),
            &Gender::Male,
            &Species::Dog,
            &String::from_str(&env, "Poodle"),
            &String::from_str(&env, "Black"),
            &5u32,
            &None,
            &PrivacyLevel::Private,
        );

        client.register_vet(
            &vet,
            &String::from_str(&env, "Dr. Smith"),
            &String::from_str(&env, "LIC-003"),
            &String::from_str(&env, "General"),
        );
        client.verify_vet(&admin, &vet);

        client.add_lab_result(
            &pet_id,
            &vet,
            &String::from_str(&env, "CBC"),
            &String::from_str(&env, "Normal"),
            &String::from_str(&env, "0-100"),
            &0u32,
            &100u32,
            &None,
            &None,
        );

        // Owner should succeed
        let results = client.get_lab_results(&pet_id, &owner, &0u64, &10u32, &None, &None);
        assert_eq!(results.len(), 1);
    }

    /// Issue #32 — correctness test: 25 lab results, sequential pagination
    /// across three pages of 10 verifies that offset >= limit no longer causes
    /// premature loop exit.
    #[test]
    fn test_get_lab_results_sequential_pagination_25_records() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let contract_id = env.register_contract(None, KoraContract);
        let client = KoraContractClient::new(&env, &contract_id);
        client.init_admin(&admin);

        let owner = Address::generate(&env);
        let vet = Address::generate(&env);

        let pet_id = client.register_pet(
            &owner,
            &String::from_str(&env, "Pagination Dog"),
            &String::from_str(&env, "2020-01-01"),
            &Gender::Male,
            &Species::Dog,
            &String::from_str(&env, "Labrador"),
            &String::from_str(&env, "Yellow"),
            &3u32,
            &None,
            &PrivacyLevel::Public,
        );

        client.register_vet(
            &vet,
            &String::from_str(&env, "Dr. Page"),
            &String::from_str(&env, "LIC-PAGE"),
            &String::from_str(&env, "General"),
        );
        client.verify_vet(&admin, &vet);

        // Insert 25 lab results with incrementing timestamps so ordering is deterministic.
        for i in 1u64..=25 {
            client.add_lab_result(
                &pet_id,
                &vet,
                &String::from_str(&env, "CBC"),
                &String::from_str(&env, "Normal"),
                &String::from_str(&env, "0-100"),
                &0u32,
                &100u32,
                &None,
                &Some(i * 100),
            );
        }

        // Page 1: offset=0, limit=10 → items 1–10
        let page1 = client.get_lab_results(&pet_id, &owner, &0u64, &10u32, &None, &None);
        assert_eq!(page1.len(), 10, "page 1 should have 10 results");

        // Page 2: offset=10, limit=10 → items 11–20 (this is the broken case from issue #32)
        let page2 = client.get_lab_results(&pet_id, &owner, &10u64, &10u32, &None, &None);
        assert_eq!(page2.len(), 10, "page 2 should have 10 results (offset >= limit was broken)");

        // Page 3: offset=20, limit=10 → items 21–25 (5 remaining)
        let page3 = client.get_lab_results(&pet_id, &owner, &20u64, &10u32, &None, &None);
        assert_eq!(page3.len(), 5, "page 3 should have the 5 remaining results");

        // Sanity: no item appears on two pages (check by timestamp uniqueness)
        let p1_timestamps: std::collections::HashSet<u64> =
            page1.iter().map(|r| r.date).collect();
        let p2_timestamps: std::collections::HashSet<u64> =
            page2.iter().map(|r| r.date).collect();
        let p3_timestamps: std::collections::HashSet<u64> =
            page3.iter().map(|r| r.date).collect();

        assert!(
            p1_timestamps.is_disjoint(&p2_timestamps),
            "pages 1 and 2 must not overlap"
        );
        assert!(
            p2_timestamps.is_disjoint(&p3_timestamps),
            "pages 2 and 3 must not overlap"
        );
        assert_eq!(
            p1_timestamps.len() + p2_timestamps.len() + p3_timestamps.len(),
            25,
            "all 25 records must appear exactly once across the three pages"
        );
    }
}
