use crate::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String};

fn setup() -> (Env, KoraContractClient<'static>, Address, Address, u64) {
    let env = Env::default();
    env.mock_all_auths();
    env.cost_estimate().budget().reset_unlimited();

    let admin = Address::generate(&env);
    let owner = Address::generate(&env);

    let contract_id = env.register(KoraContract, ());
    let client = KoraContractClient::new(&env, &contract_id);
    client.init_admin(&admin);

    let pet_id = client.register_pet(
        &owner,
        &String::from_str(&env, "Buddy"),
        &String::from_str(&env, "2020-01-01"),
        &Gender::Male,
        &Species::Dog,
        &String::from_str(&env, "Golden Retriever"),
        &String::from_str(&env, "Golden"),
        &30u32,
        &None,
        &PrivacyLevel::Public,
    );

    (env, client, owner, admin, pet_id)
}

#[test]
fn test_get_behavior_records_pagination() {
    let (env, client, owner, _admin, pet_id) = setup();

    for i in 0..5u32 {
        client.add_behavior_record(
            &pet_id,
            &owner,
            &BehaviorType::Training,
            &((i % 5) + 1),
            &String::from_str(&env, "Training session"),
        );
    }

    // Page 0, size 2 → first 2 records
    let page0 = client.get_behavior_records(&pet_id, &owner, &0u32, &2u32, &None);
    assert_eq!(page0.items.len(), 2);
    assert_eq!(page0.total, 5);
    assert_eq!(page0.page, 0);
    assert_eq!(page0.page_size, 2);

    // Page 2, size 2 → records 5 (only one left)
    let page2 = client.get_behavior_records(&pet_id, &owner, &2u32, &2u32, &None);
    assert_eq!(page2.items.len(), 1);

    // Page 3, size 2 → empty
    let page3 = client.get_behavior_records(&pet_id, &owner, &3u32, &2u32, &None);
    assert_eq!(page3.items.len(), 0);
}

#[test]
fn test_get_behavior_records_type_filter() {
    let (env, client, owner, _admin, pet_id) = setup();

    client.add_behavior_record(
        &pet_id,
        &owner,
        &BehaviorType::Training,
        &5,
        &String::from_str(&env, "Sit"),
    );
    client.add_behavior_record(
        &pet_id,
        &owner,
        &BehaviorType::Aggression,
        &3,
        &String::from_str(&env, "Barking"),
    );
    client.add_behavior_record(
        &pet_id,
        &owner,
        &BehaviorType::Training,
        &4,
        &String::from_str(&env, "Stay"),
    );

    let training = client.get_behavior_records(
        &pet_id,
        &owner,
        &0u32,
        &10u32,
        &Some(BehaviorType::Training),
    );
    assert_eq!(training.items.len(), 2);
    assert_eq!(training.total, 2);
    for i in 0..training.items.len() {
        assert_eq!(
            training.items.get(i).unwrap().behavior_type,
            BehaviorType::Training
        );
    }

    let aggression = client.get_behavior_records(
        &pet_id,
        &owner,
        &0u32,
        &10u32,
        &Some(BehaviorType::Aggression),
    );
    assert_eq!(aggression.items.len(), 1);
    assert_eq!(aggression.total, 1);
}

#[test]
fn test_get_behavior_records_pagination_with_type_filter() {
    let (env, client, owner, _admin, pet_id) = setup();

    // 4 training records, 2 anxiety records
    for i in 0..4u32 {
        client.add_behavior_record(
            &pet_id,
            &owner,
            &BehaviorType::Training,
            &(i + 1),
            &String::from_str(&env, "Training"),
        );
    }
    for i in 0..2u32 {
        client.add_behavior_record(
            &pet_id,
            &owner,
            &BehaviorType::Anxiety,
            &(i + 1),
            &String::from_str(&env, "Anxiety"),
        );
    }

    // Page size 3 on Training → 3 of 4
    let page =
        client.get_behavior_records(&pet_id, &owner, &0u32, &3u32, &Some(BehaviorType::Training));
    assert_eq!(page.items.len(), 3);
    assert_eq!(page.total, 4);

    // Second page of Training → 1 remaining
    let page2 =
        client.get_behavior_records(&pet_id, &owner, &1u32, &3u32, &Some(BehaviorType::Training));
    assert_eq!(page2.items.len(), 1);
    assert_eq!(page2.total, 4);
}

#[test]
fn test_get_behavior_records_page_size_capped() {
    let (env, client, owner, _admin, pet_id) = setup();

    for _ in 0..60u32 {
        client.add_behavior_record(
            &pet_id,
            &owner,
            &BehaviorType::Other,
            &1,
            &String::from_str(&env, "Record"),
        );
    }

    let page = client.get_behavior_records(&pet_id, &owner, &0u32, &100u32, &None);
    assert_eq!(page.items.len(), 50);
    assert_eq!(page.page_size, 50);
    assert_eq!(page.total, 60);
}

/// Issue #71: get_behavior_history_paginated must return the correct slice
/// across multiple pages, and cap `limit` at 50 regardless of what's asked.
#[test]
fn test_get_behavior_history_paginated_across_pages() {
    let (env, client, owner, _admin, pet_id) = setup();

    for i in 0..5u32 {
        client.add_behavior_record(
            &pet_id,
            &owner,
            &BehaviorType::Training,
            &((i % 5) + 1),
            &String::from_str(&env, "Training session"),
        );
    }

    // First page: offset 0, limit 2 → first 2 records.
    let page0 = client.get_behavior_history_paginated(&pet_id, &0u64, &2u32);
    assert_eq!(page0.len(), 2);

    // Second page: offset 2, limit 2 → next 2 records (distinct from page0).
    let page1 = client.get_behavior_history_paginated(&pet_id, &2u64, &2u32);
    assert_eq!(page1.len(), 2);
    assert_ne!(page0.get(0).unwrap().id, page1.get(0).unwrap().id);

    // Last page: offset 4, limit 2 → only 1 record left.
    let page2 = client.get_behavior_history_paginated(&pet_id, &4u64, &2u32);
    assert_eq!(page2.len(), 1);

    // Past the end: offset 10 → empty, not an error.
    let page3 = client.get_behavior_history_paginated(&pet_id, &10u64, &2u32);
    assert_eq!(page3.len(), 0);

    // Requesting more than 50 is silently capped, not rejected.
    for _ in 0..46u32 {
        client.add_behavior_record(
            &pet_id,
            &owner,
            &BehaviorType::Training,
            &3u32,
            &String::from_str(&env, "Bulk record"),
        );
    }
    let capped = client.get_behavior_history_paginated(&pet_id, &0u64, &100u32);
    assert_eq!(capped.len(), 50);
}

/// Issue #70: severity is a 1-5 scale; 0 is below it and must be rejected.
#[test]
#[should_panic(expected = "InvalidInput")]
fn test_add_behavior_record_rejects_severity_zero() {
    let (env, client, owner, _admin, pet_id) = setup();
    client.add_behavior_record(
        &pet_id,
        &owner,
        &BehaviorType::Training,
        &0u32,
        &String::from_str(&env, "Invalid severity"),
    );
}

/// Issue #70: severity is a 1-5 scale; 10 is above it and must be rejected
/// (previously only values > 10 were rejected).
#[test]
#[should_panic(expected = "InvalidInput")]
fn test_add_behavior_record_rejects_severity_ten() {
    let (env, client, owner, _admin, pet_id) = setup();
    client.add_behavior_record(
        &pet_id,
        &owner,
        &BehaviorType::Training,
        &10u32,
        &String::from_str(&env, "Invalid severity"),
    );
}

/// Issue #70: only the pet's owner may log a behavior record for it — an
/// unrelated authenticated caller must be rejected.
#[test]
#[should_panic(expected = "Unauthorized")]
fn test_add_behavior_record_rejects_non_owner_caller() {
    let (env, client, _owner, _admin, pet_id) = setup();
    let stranger = Address::generate(&env);
    client.add_behavior_record(
        &pet_id,
        &stranger,
        &BehaviorType::Training,
        &3u32,
        &String::from_str(&env, "Not my pet"),
    );
}
