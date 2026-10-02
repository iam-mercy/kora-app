mod test_vaccination_expiry {
    use crate::*;
    use soroban_sdk::{testutils::Address as _, Address, Env, String};

    const DAY: u64 = 86_400;

    fn setup() -> (Env, KoraContractClient<'static>, Address, Address, u64) {
        let env = Env::default();
        env.mock_all_auths();
        env.budget().reset_unlimited();

        let contract_id = env.register_contract(None, KoraContract);
        let client = KoraContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let vet = Address::generate(&env);
        let owner = Address::generate(&env);

        client.init_admin(&admin);
        client.register_vet(
            &vet,
            &String::from_str(&env, "Dr. Vet"),
            &String::from_str(&env, "LIC-001"),
            &String::from_str(&env, "General"),
        );
        client.verify_vet(&admin, &vet);

        let pet_id = client.register_pet(
            &owner,
            &String::from_str(&env, "Buddy"),
            &String::from_str(&env, "2020-01-01"),
            &Gender::Male,
            &Species::Dog,
            &String::from_str(&env, "Retriever"),
            &PrivacyLevel::Public,
        );

        (env, client, vet, owner, pet_id)
    }

    #[test]
    fn test_get_expiring_vaccinations_within_window() {
        let (env, client, vet, _owner, pet_id) = setup();
        let now = env.ledger().timestamp();

        // Expires in 10 days — within a 30-day window
        client.add_vaccination(
            &pet_id,
            &vet,
            &VaccineType::Rabies,
            &String::from_str(&env, "RabiesVax"),
            &now,
            &(now + 10 * DAY),
            &(now + 10 * DAY),
            &String::from_str(&env, "BATCH-001"),
        );

        let expiring = client.get_expiring_vaccinations(&pet_id, &30u64);
        assert_eq!(expiring.len(), 1);
        assert!(!expiring.get(0).unwrap().already_expired);
        assert_eq!(expiring.get(0).unwrap().days_remaining, 10);
    }

    #[test]
    fn test_get_expiring_vaccinations_already_expired() {
        let (env, client, vet, _owner, pet_id) = setup();
        let now = env.ledger().timestamp();

        // Expired 5 days ago
        let past = now.saturating_sub(5 * DAY);
        client.add_vaccination(
            &pet_id,
            &vet,
            &VaccineType::Distemper,
            &String::from_str(&env, "DistemperVax"),
            &past,
            &past,
            &past,
            &String::from_str(&env, "BATCH-002"),
        );

        let expiring = client.get_expiring_vaccinations(&pet_id, &30u64);
        assert_eq!(expiring.len(), 1);
        assert!(expiring.get(0).unwrap().already_expired);
        assert_eq!(expiring.get(0).unwrap().days_remaining, 0);
    }

    #[test]
    fn test_get_expiring_vaccinations_outside_window_returns_empty() {
        let (env, client, vet, _owner, pet_id) = setup();
        let now = env.ledger().timestamp();

        // Expires in 60 days — outside a 30-day window
        client.add_vaccination(
            &pet_id,
            &vet,
            &VaccineType::Parvovirus,
            &String::from_str(&env, "ParvoVax"),
            &now,
            &(now + 60 * DAY),
            &(now + 60 * DAY),
            &String::from_str(&env, "BATCH-003"),
        );

        let expiring = client.get_expiring_vaccinations(&pet_id, &30u64);
        assert_eq!(expiring.len(), 0);
    }

    #[test]
    fn test_get_expiring_vaccinations_no_vaccinations_returns_empty() {
        let (_env, client, _vet, _owner, pet_id) = setup();
        let expiring = client.get_expiring_vaccinations(&pet_id, &30u64);
        assert_eq!(expiring.len(), 0);
    }

    #[test]
    fn test_vaccination_summary_overdue() {
        let (env, client, vet, _owner, pet_id) = setup();
        let now = env.ledger().timestamp();
        let past = now.saturating_sub(5 * DAY);

        client.add_vaccination(
            &pet_id,
            &vet,
            &VaccineType::Rabies,
            &String::from_str(&env, "RabiesVax"),
            &past,
            &past,
            &past,
            &String::from_str(&env, "BATCH-004"),
        );

        let summary = client.get_vaccination_summary(&pet_id);
        assert!(!summary.is_fully_current);
        assert_eq!(summary.overdue_types.len(), 1);
    }

    #[test]
    fn test_vaccination_summary_current() {
        let (env, client, vet, _owner, pet_id) = setup();
        let now = env.ledger().timestamp();

        client.add_vaccination(
            &pet_id,
            &vet,
            &VaccineType::Rabies,
            &String::from_str(&env, "RabiesVax"),
            &now,
            &(now + 365 * DAY),
            &(now + 365 * DAY),
            &String::from_str(&env, "BATCH-005"),
        );

        let summary = client.get_vaccination_summary(&pet_id);
        assert!(summary.is_fully_current);
        assert_eq!(summary.overdue_types.len(), 0);
    }

    // ── Issue #90 ─────────────────────────────────────────────────────────────
    // get_upcoming_vaccinations now accepts Option<u64> window_days.
    // Default is 30, max is 365.

    #[test]
    fn test_get_upcoming_vaccinations_default_window_is_30_days() {
        let (env, client, vet, _owner, pet_id) = setup();
        let now = env.ledger().timestamp();

        // Due in 20 days — inside default 30-day window
        client.add_vaccination(
            &pet_id,
            &vet,
            &VaccineType::Rabies,
            &String::from_str(&env, "RabiesVax"),
            &now,
            &(now + 20 * DAY),
            &(now + 20 * DAY),
            &String::from_str(&env, "BATCH-D01"),
        );

        // Due in 40 days — outside default 30-day window
        client.add_vaccination(
            &pet_id,
            &vet,
            &VaccineType::Distemper,
            &String::from_str(&env, "DistemperVax"),
            &now,
            &(now + 40 * DAY),
            &(now + 40 * DAY),
            &String::from_str(&env, "BATCH-D02"),
        );

        // None => default 30-day window
        let upcoming = client.get_upcoming_vaccinations(&pet_id, &None);
        assert_eq!(
            upcoming.len(),
            1,
            "default window should only include vaccinations due within 30 days"
        );
    }

    #[test]
    fn test_get_upcoming_vaccinations_custom_60_day_window() {
        let (env, client, vet, _owner, pet_id) = setup();
        let now = env.ledger().timestamp();

        // Due in 45 days — inside 60-day window but outside 30-day window
        client.add_vaccination(
            &pet_id,
            &vet,
            &VaccineType::Parvovirus,
            &String::from_str(&env, "ParvoVax"),
            &now,
            &(now + 45 * DAY),
            &(now + 45 * DAY),
            &String::from_str(&env, "BATCH-C01"),
        );

        // 60-day window should capture it
        let upcoming_60 = client.get_upcoming_vaccinations(&pet_id, &Some(60u64));
        assert_eq!(
            upcoming_60.len(),
            1,
            "60-day window should include vaccinations due within 45 days"
        );

        // 30-day window should not
        let upcoming_30 = client.get_upcoming_vaccinations(&pet_id, &Some(30u64));
        assert_eq!(
            upcoming_30.len(),
            0,
            "30-day window should not include vaccinations due in 45 days"
        );
    }

    #[test]
    fn test_get_upcoming_vaccinations_window_capped_at_365_days() {
        let (env, client, vet, _owner, pet_id) = setup();
        let now = env.ledger().timestamp();

        // Due in 400 days — beyond 365-day cap
        client.add_vaccination(
            &pet_id,
            &vet,
            &VaccineType::Leukemia,
            &String::from_str(&env, "LeukemiaVax"),
            &now,
            &(now + 400 * DAY),
            &(now + 400 * DAY),
            &String::from_str(&env, "BATCH-CAP1"),
        );

        // Requesting 500 days should be capped to 365
        let upcoming = client.get_upcoming_vaccinations(&pet_id, &Some(500u64));
        assert_eq!(
            upcoming.len(),
            0,
            "window of 500 days must be capped at 365 and not include vaccinations due in 400 days"
        );
    }

    // ── Issue #91 ─────────────────────────────────────────────────────────────
    // get_expiring_vaccinations_by_vet uses VetVaccinationIndex instead of a
    // nested all-pets scan, keeping instruction consumption within budget.

    #[test]
    fn test_get_expiring_vaccinations_by_vet_returns_vet_vaccinations() {
        let (env, client, vet, _owner, pet_id) = setup();
        let now = env.ledger().timestamp();

        // Vaccination expiring in 10 days (within 30-day window)
        client.add_vaccination(
            &pet_id,
            &vet,
            &VaccineType::Rabies,
            &String::from_str(&env, "RabiesVax"),
            &now,
            &(now + 10 * DAY),
            &(now + 10 * DAY),
            &String::from_str(&env, "BATCH-V01"),
        );

        let expiring = client.get_expiring_vaccinations_by_vet(&vet, &30u64);
        assert_eq!(expiring.len(), 1, "vet index should find the expiring vaccination");
        assert!(!expiring.get(0).unwrap().already_expired);
        assert_eq!(expiring.get(0).unwrap().days_remaining, 10);
    }

    #[test]
    fn test_get_expiring_vaccinations_by_vet_excludes_other_vet_vaccinations() {
        // Use a local setup so we hold onto the admin address for verify_vet.
        let env = Env::default();
        env.mock_all_auths();
        env.budget().reset_unlimited();

        let contract_id = env.register_contract(None, KoraContract);
        let client = KoraContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let vet = Address::generate(&env);
        let vet2 = Address::generate(&env);
        let owner = Address::generate(&env);

        client.init_admin(&admin);

        client.register_vet(
            &vet,
            &String::from_str(&env, "Dr. Vet"),
            &String::from_str(&env, "LIC-001"),
            &String::from_str(&env, "General"),
        );
        client.verify_vet(&admin, &vet);

        client.register_vet(
            &vet2,
            &String::from_str(&env, "Dr. Second"),
            &String::from_str(&env, "LIC-002"),
            &String::from_str(&env, "General"),
        );
        client.verify_vet(&admin, &vet2);

        let pet_id = client.register_pet(
            &owner,
            &String::from_str(&env, "Buddy"),
            &String::from_str(&env, "2020-01-01"),
            &Gender::Male,
            &Species::Dog,
            &String::from_str(&env, "Retriever"),
            &PrivacyLevel::Public,
        );

        let now = env.ledger().timestamp();

        // Register a second pet for vet2
        let pet_id2 = client.register_pet(
            &owner,
            &String::from_str(&env, "Max"),
            &String::from_str(&env, "2021-01-01"),
            &Gender::Male,
            &Species::Cat,
            &String::from_str(&env, "Tabby"),
            &PrivacyLevel::Public,
        );

        // vet vaccinates pet_id — expiring in 5 days
        client.add_vaccination(
            &pet_id,
            &vet,
            &VaccineType::Rabies,
            &String::from_str(&env, "RabiesVax"),
            &now,
            &(now + 5 * DAY),
            &(now + 5 * DAY),
            &String::from_str(&env, "BATCH-V02"),
        );

        // vet2 vaccinates pet_id2 — expiring in 5 days
        client.add_vaccination(
            &pet_id2,
            &vet2,
            &VaccineType::Distemper,
            &String::from_str(&env, "DistemperVax"),
            &now,
            &(now + 5 * DAY),
            &(now + 5 * DAY),
            &String::from_str(&env, "BATCH-V03"),
        );

        // vet query should only return vet's vaccination
        let expiring_vet = client.get_expiring_vaccinations_by_vet(&vet, &30u64);
        assert_eq!(expiring_vet.len(), 1, "vet index must not include vet2 vaccinations");

        // vet2 query should only return vet2's vaccination
        let expiring_vet2 = client.get_expiring_vaccinations_by_vet(&vet2, &30u64);
        assert_eq!(expiring_vet2.len(), 1, "vet2 index must not include vet1 vaccinations");
    }

    #[test]
    fn test_get_expiring_vaccinations_by_vet_empty_for_unknown_vet() {
        let (env, client, _vet, _owner, _pet_id) = setup();
        let unknown_vet = Address::generate(&env);

        let expiring = client.get_expiring_vaccinations_by_vet(&unknown_vet, &30u64);
        assert_eq!(
            expiring.len(),
            0,
            "unknown vet with no index should return empty"
        );
    }

    #[test]
    fn test_get_expiring_vaccinations_by_vet_large_registry_stays_indexed() {
        // Benchmark-style: 10 pets × 5 vaccinations each administered by 1 vet.
        // The vet-indexed query should still find expiring vaccinations without
        // needing to scan all pets (issue #91).
        let env = Env::default();
        env.mock_all_auths();
        env.budget().reset_unlimited();

        let contract_id = env.register_contract(None, KoraContract);
        let client = KoraContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let vet = Address::generate(&env);
        let owner = Address::generate(&env);

        client.init_admin(&admin);
        client.register_vet(
            &vet,
            &String::from_str(&env, "Dr. Bulk"),
            &String::from_str(&env, "LIC-BULK"),
            &String::from_str(&env, "General"),
        );
        client.verify_vet(&admin, &vet);

        let now = env.ledger().timestamp();
        let mut expiring_count: u32 = 0;

        for p in 0..10u32 {
            let pet_id = client.register_pet(
                &owner,
                &String::from_str(&env, &format!("Pet{}", p)),
                &String::from_str(&env, "2020-01-01"),
                &Gender::Female,
                &Species::Dog,
                &String::from_str(&env, "Mixed"),
                &PrivacyLevel::Public,
            );

            for v in 0..5u32 {
                // Odd-indexed vaccinations expire within 30 days; even ones are far out.
                let expires = if v % 2 == 1 {
                    expiring_count += 1;
                    now + 15 * DAY
                } else {
                    now + 90 * DAY
                };
                client.add_vaccination(
                    &pet_id,
                    &vet,
                    &VaccineType::Rabies,
                    &String::from_str(&env, "MultiVax"),
                    &now,
                    &expires,
                    &expires,
                    &String::from_str(&env, &format!("BATCH-{}-{}", p, v)),
                );
            }
        }

        let expiring = client.get_expiring_vaccinations_by_vet(&vet, &30u64);
        assert_eq!(
            expiring.len() as u32,
            expiring_count,
            "vet-indexed query must return exactly {} expiring vaccinations",
            expiring_count
        );
    }
}
