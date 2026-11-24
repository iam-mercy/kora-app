use crate::{
    test_fixtures::{create_test_pet, TestEnv, BASE_TIMESTAMP},
    BreedingKey, ContractError, DataKey, Gender, GroomerProfile, GroomingKey, GroomingRecord,
    KoraContract, KoraContractClient, PrivacyLevel, ProposalAction, Species,
};
use soroban_sdk::{testutils::Address as _, Address, Env, Error, String, Vec};

#[test]
fn signer_rotation_stores_one_day_payload_and_executes_after_quorum() {
    let te = TestEnv::new();
    let new_signer = Address::generate(&te.env);

    let proposal_id = te.client.propose_signer_rotation(
        &te.admin1,
        &te.admin2,
        &new_signer,
    );
    let proposal = te.client.get_proposal(&proposal_id).unwrap();
    assert_eq!(
        proposal.action,
        ProposalAction::RotateSigner((te.admin2.clone(), new_signer.clone()))
    );
    assert_eq!(proposal.expires_at, BASE_TIMESTAMP + 86_400);

    let admins_before = te.client.get_admins();
    assert!(admins_before.contains(te.admin2.clone()));
    assert!(!admins_before.contains(new_signer.clone()));
    te.client.approve_proposal(&te.admin1, &proposal_id);
    te.client.approve_proposal(&te.admin2, &proposal_id);
    te.advance_time(86_400);
    te.client.execute_proposal(&proposal_id);

    let admins = te.client.get_admins();
    assert!(admins.contains(te.admin1.clone()));
    assert!(!admins.contains(te.admin2.clone()));
    assert!(admins.contains(new_signer));
    assert_eq!(te.client.get_admin_threshold(), 2);
    assert!(te.client.get_proposal(&proposal_id).unwrap().executed);
}

#[test]
fn rotated_signer_approval_cannot_authorize_an_unrelated_proposal() {
    let te = TestEnv::new();
    let replacement = Address::generate(&te.env);

    let set_initial_quorum = te.client.set_quorum_percent(&te.admin1, &100);
    te.client.approve_proposal(&te.admin1, &set_initial_quorum);
    te.client.approve_proposal(&te.admin2, &set_initial_quorum);
    te.advance_time(86_400);
    te.client.execute_proposal(&set_initial_quorum);

    let pending = te.client.set_quorum_percent(&te.admin1, &75);
    te.client.approve_proposal(&te.admin1, &pending);
    te.client.approve_proposal(&te.admin2, &pending);

    let rotation = te
        .client
        .propose_signer_rotation(&te.admin1, &te.admin2, &replacement);
    te.client.approve_proposal(&te.admin1, &rotation);
    te.client.approve_proposal(&te.admin2, &rotation);
    te.advance_time(86_400);
    te.client.execute_proposal(&rotation);

    assert_eq!(
        te.client.try_execute_proposal(&pending),
        Err(Ok(Error::from_contract_error(
            ContractError::ThresholdNotMet as u32
        )))
    );
    assert_eq!(te.client.get_quorum_percent(), 100);
    assert!(!te.client.get_proposal(&pending).unwrap().executed);

    assert_eq!(
        te.client.try_approve_proposal(&te.admin2, &pending),
        Err(Ok(Error::from_contract_error(
            ContractError::NotAnAdmin as u32
        )))
    );
    te.client.approve_proposal(&replacement, &pending);
    te.client.execute_proposal(&pending);
    assert_eq!(te.client.get_quorum_percent(), 75);
    assert!(te.client.get_proposal(&pending).unwrap().executed);
}

#[test]
fn compact_delegations_counts_each_expired_token_once() {
    let te = TestEnv::new();
    let pet_id = create_test_pet(&te.env, &te.client, &te.owner);
    let expired = Address::generate(&te.env);
    let live = Address::generate(&te.env);
    let expired_key = DataKey::DecryptionToken((pet_id, expired.clone()));
    let live_key = DataKey::DecryptionToken((pet_id, live.clone()));
    let count_key = DataKey::PetDelegationCount(pet_id);

    // Seed the token layout consumed by the compactor.
    te.env.as_contract(&te.client.address, || {
        let storage = te.env.storage().instance();
        storage.set(&expired_key, &BASE_TIMESTAMP);
        storage.set(&live_key, &(BASE_TIMESTAMP + 1));
        storage.set(&count_key, &2u64);
    });
    let delegates = Vec::from_array(&te.env, [expired.clone(), live, expired]);

    assert_eq!(
        te.client
            .compact_delegations(&pet_id, &te.owner, &delegates),
        1
    );
    te.env.as_contract(&te.client.address, || {
        let storage = te.env.storage().instance();
        assert!(!storage.has(&expired_key));
        assert_eq!(storage.get(&live_key), Some(BASE_TIMESTAMP + 1));
        assert_eq!(storage.get::<_, u64>(&count_key), Some(1));
    });

    te.advance_time(1);
    assert_eq!(
        te.client
            .compact_delegations(&pet_id, &te.owner, &delegates),
        1
    );
    assert_eq!(
        te.client
            .compact_delegations(&pet_id, &te.owner, &delegates),
        0
    );
    te.env.as_contract(&te.client.address, || {
        let storage = te.env.storage().instance();
        assert!(!storage.has(&live_key));
        assert_eq!(storage.get::<_, u64>(&count_key), Some(0));
    });
}

#[test]
fn groomer_profile_projects_registration_verification_and_rating() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(KoraContract, ());
    let client = KoraContractClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    let groomer = Address::generate(&env);
    let owner = Address::generate(&env);
    client.init_admin(&admin);

    assert!(client.get_groomer_profile(&groomer).is_none());
    client.register_groomer(
        &admin,
        &groomer,
        &String::from_str(&env, "Verified Groomer"),
        &String::from_str(&env, "LIC-386"),
    );
    let mut expected_profile = GroomerProfile {
        address: groomer.clone(),
        name: String::from_str(&env, "Verified Groomer"),
        license_id: String::from_str(&env, "LIC-386"),
        verified: false,
        aggregate_rating: 0,
        review_count: 0,
    };
    assert_eq!(
        client.get_groomer_profile(&groomer),
        Some(expected_profile.clone())
    );

    client.verify_groomer(&admin, &groomer);
    expected_profile.verified = true;
    assert_eq!(
        client.get_groomer_profile(&groomer),
        Some(expected_profile.clone())
    );
    let pet_id = create_test_pet(&env, &client, &owner);
    let record_id = 1u64;

    // Current grooming record creation does not associate an address; seed the
    // linked record needed to exercise the public rating entrypoint.
    env.as_contract(&contract_id, || {
        env.storage().instance().set(
            &GroomingKey::GroomingRecord(record_id),
            &GroomingRecord {
                id: record_id,
                pet_id,
                service_type: String::from_str(&env, "Bath"),
                groomer: String::from_str(&env, "Verified Groomer"),
                groomer_address: Some(groomer.clone()),
                date: env.ledger().timestamp(),
                next_due: 0,
                cost: 0,
                notes: String::from_str(&env, ""),
            },
        );
    });

    assert!(client.rate_groomer(&pet_id, &record_id, &4u32));
    expected_profile.aggregate_rating = 400;
    expected_profile.review_count = 1;
    assert_eq!(
        client.get_groomer_profile(&groomer),
        Some(expected_profile)
    );
}

fn register_breeding_pet(
    env: &Env,
    client: &KoraContractClient,
    owner: &Address,
    name: &str,
    gender: Gender,
) -> u64 {
    client.register_pet(
        owner,
        &String::from_str(env, name),
        &String::from_str(env, "2021-01-01"),
        &gender,
        &Species::Dog,
        &String::from_str(env, "Mixed"),
        &String::from_str(env, "Brown"),
        &20u32,
        &None,
        &PrivacyLevel::Public,
    )
}

fn assert_page(actual: Vec<u64>, expected: &[u64]) {
    assert_eq!(actual.len(), expected.len() as u32);
    for (index, expected_id) in expected.iter().enumerate() {
        assert_eq!(actual.get(index as u32), Some(*expected_id));
    }
}

#[test]
fn offspring_ids_are_persisted_in_stable_paginated_order() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(KoraContract, ());
    let client = KoraContractClient::new(&env, &contract_id);
    let owner = Address::generate(&env);
    let sire = register_breeding_pet(&env, &client, &owner, "Sire", Gender::Male);
    let dam = register_breeding_pet(&env, &client, &owner, "Dam", Gender::Female);
    let offspring_ids: std::vec::Vec<u64> = (0..5)
        .map(|index| {
            register_breeding_pet(
                &env,
                &client,
                &owner,
                &format!("Offspring {index}"),
                Gender::Male,
            )
        })
        .collect();
    let record_id = client.add_breeding_record(
        &sire,
        &dam,
        &env.ledger().timestamp(),
        &String::from_str(&env, "pagination regression"),
        &owner,
    );

    for offspring_id in &offspring_ids {
        assert!(client.add_offspring(&record_id, offspring_id, &owner));
    }

    assert_page(client.get_offspring_ids(&record_id, &0, &2), &offspring_ids[0..2]);
    assert_page(client.get_offspring_ids(&record_id, &2, &2), &offspring_ids[2..4]);
    assert_page(client.get_offspring_ids(&record_id, &4, &2), &offspring_ids[4..5]);
    assert_page(client.get_offspring_ids(&record_id, &0, &0), &[]);
    assert_page(client.get_offspring_ids(&record_id, &5, &2), &[]);

    let record = client.get_breeding_record(&record_id).unwrap();
    assert_eq!(record.offspring_count, offspring_ids.len() as u32);
    let indexed_count: u64 = env.as_contract(&contract_id, || {
        env.storage()
            .persistent()
            .get(&BreedingKey::BreedingOffspringCount(record_id))
            .unwrap()
    });
    assert_eq!(indexed_count, offspring_ids.len() as u64);
}

#[test]
fn v1_to_v2_migration_requires_admin_and_is_repeatable_without_data_loss() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(KoraContract, ());
    let client = KoraContractClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    let second_admin = Address::generate(&env);
    let mut admins = Vec::new(&env);
    admins.push_back(admin.clone());
    admins.push_back(second_admin);
    client.init_multisig(&admin, &admins, &2);

    let owner = Address::generate(&env);
    let pet_id = create_test_pet(&env, &client, &owner);
    let pet_before = client.get_pet(&pet_id, &owner);
    assert_eq!(client.get_version().major, 1);

    let outsider = Address::generate(&env);
    let unauthorized = client.try_migrate_v1_to_v2(&outsider);
    assert_eq!(
        unauthorized,
        Err(Ok(Error::from_contract_error(
            ContractError::NotAnAdmin as u32
        )))
    );

    client.migrate_v1_to_v2(&admin);
    assert_eq!(
        (
            client.get_version().major,
            client.get_version().minor,
            client.get_version().patch
        ),
        (2, 0, 0)
    );
    assert_eq!(client.get_pet(&pet_id, &owner), pet_before);

    client.migrate_v1_to_v2(&admin);
    assert_eq!(
        (
            client.get_version().major,
            client.get_version().minor,
            client.get_version().patch
        ),
        (2, 0, 0)
    );
    assert_eq!(client.get_pet(&pet_id, &owner), pet_before);
}
