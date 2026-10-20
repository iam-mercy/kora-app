//! Booking-bounds and slot-ID tests for `book_grooming_slot`.
//!
//! Issue #216: the incremented `GroomingKey::GroomingRecordCount` must be
//! persisted so every booked slot receives a unique, monotonically
//! increasing `slot_id` (previously every slot was assigned id 1).
//!
//! Issue #217: `start_time` must be strictly in the future and
//! `duration_mins` must lie within 15..=480 minutes.

use crate::{
    ContractError, DataKey, Gender, KoraContract, KoraContractClient, Pet, PrivacyLevel,
    Species,
};
use soroban_sdk::{
    testutils::{Address as _, Ledger},
    Address, Env, String,
};

/// A fixed ledger timestamp (2027-01-15) so "future" start times are
/// unambiguous and after the registered pet's birthday.
const NOW: u64 = 1_800_000_000;

/// `register_pet` persists the pet via `store_pet` (persistent storage) while
/// `book_grooming_slot` reads `DataKey::Pet` from instance storage. Copy the
/// pet entry into instance storage so the booking entrypoint sees it — the
/// same seed-what-the-target-reads convention used by
/// `test_attachment_limit.rs`.
fn mirror_pet_into_instance_storage(env: &Env, contract_id: &Address, pet_id: u64) {
    env.as_contract(contract_id, || {
        if let Some(pet) = env
            .storage()
            .persistent()
            .get::<DataKey, Pet>(&DataKey::Pet(pet_id))
        {
            env.storage().instance().set(&DataKey::Pet(pet_id), &pet);
        }
    });
}

fn setup() -> (Env, KoraContractClient<'static>, Address, Address, u64) {
    let env = Env::default();
    env.mock_all_auths();
    env.budget().reset_unlimited();
    env.ledger().with_mut(|li| li.timestamp = NOW);

    let contract_id = env.register_contract(None, KoraContract);
    let client = KoraContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.init_admin(&admin);
    let groomer = Address::generate(&env);
    client.register_groomer(
        &admin,
        &groomer,
        &String::from_str(&env, "Groomer Tester"),
        &String::from_str(&env, "LIC-GRM-217"),
    );
    client.verify_groomer(&admin, &groomer);

    let owner = Address::generate(&env);
    let pet_id = client.register_pet(
        &owner,
        &String::from_str(&env, "Max"),
        &String::from_str(&env, "2019-06-15"),
        &Gender::Male,
        &Species::Dog,
        &String::from_str(&env, "Beagle"),
        &String::from_str(&env, "Tricolor"),
        &20u32,
        &None,
        &PrivacyLevel::Public,
    );
    mirror_pet_into_instance_storage(&env, &contract_id, pet_id);

    (env, client, groomer, owner, pet_id)
}

// -------------------------------------------------------
// Issue #216: persisted GroomingRecordCount yields unique IDs
// -------------------------------------------------------

/// Booking several slots in a row must produce monotonically increasing
/// unique IDs (1, 2, 3, ...) instead of assigning every slot id 1.
#[test]
fn test_successive_bookings_receive_unique_increasing_ids() {
    let (_env, client, groomer, owner, pet_id) = setup();

    let slot_a = client.book_grooming_slot(&groomer, &(NOW + 1000), &60u64, &pet_id, &owner);
    let slot_b = client.book_grooming_slot(&groomer, &(NOW + 5000), &60u64, &pet_id, &owner);
    let slot_c = client.book_grooming_slot(&groomer, &(NOW + 9000), &60u64, &pet_id, &owner);

    assert_eq!(slot_a, 1);
    assert_eq!(slot_b, 2);
    assert_eq!(slot_c, 3);

    // The persisted slots must carry the same unique IDs.
    let slots = client.get_groomer_slots(&groomer);
    assert_eq!(slots.len(), 3);
    assert_eq!(slots.get(0).unwrap().slot_id, 1);
    assert_eq!(slots.get(1).unwrap().slot_id, 2);
    assert_eq!(slots.get(2).unwrap().slot_id, 3);
}

// -------------------------------------------------------
// Issue #217: future start time and duration bounds
// -------------------------------------------------------

fn invalid_input() -> soroban_sdk::Error {
    soroban_sdk::Error::from_contract_error(ContractError::InvalidInput as u32)
}

/// A start time equal to "now" must be rejected with InvalidInput.
#[test]
fn test_start_time_now_rejected() {
    let (env, client, groomer, owner, pet_id) = setup();

    let result = client.try_book_grooming_slot(&groomer, &NOW, &60u64, &pet_id, &owner);
    assert_eq!(result, Err(Ok(invalid_input())));
}

/// A past start time must be rejected with InvalidInput.
#[test]
fn test_past_start_time_rejected() {
    let (env, client, groomer, owner, pet_id) = setup();

    let result = client.try_book_grooming_slot(&groomer, &(NOW - 1), &60u64, &pet_id, &owner);
    assert_eq!(result, Err(Ok(invalid_input())));
}

/// A zero-minute slot must be rejected with InvalidInput.
#[test]
fn test_zero_duration_rejected() {
    let (env, client, groomer, owner, pet_id) = setup();

    let result = client.try_book_grooming_slot(&groomer, &(NOW + 1000), &0u64, &pet_id, &owner);
    assert_eq!(result, Err(Ok(invalid_input())));
}

/// A duration below the 15-minute floor must be rejected with InvalidInput.
#[test]
fn test_too_short_duration_rejected() {
    let (env, client, groomer, owner, pet_id) = setup();

    let result = client.try_book_grooming_slot(&groomer, &(NOW + 1000), &14u64, &pet_id, &owner);
    assert_eq!(result, Err(Ok(invalid_input())));
}

/// A duration above the 480-minute (8 hour) cap must be rejected with
/// InvalidInput.
#[test]
fn test_too_long_duration_rejected() {
    let (env, client, groomer, owner, pet_id) = setup();

    let result = client.try_book_grooming_slot(&groomer, &(NOW + 1000), &481u64, &pet_id, &owner);
    assert_eq!(result, Err(Ok(invalid_input())));
}

/// The boundary durations (15 and 480 minutes) must be accepted.
#[test]
fn test_boundary_durations_accepted() {
    let (env, client, groomer, owner, pet_id) = setup();

    let slot_a = client.book_grooming_slot(&groomer, &(NOW + 1000), &15u64, &pet_id, &owner);
    assert_eq!(slot_a, 1);

    let slot_b = client.book_grooming_slot(&groomer, &(NOW + 5000), &480u64, &pet_id, &owner);
    assert_eq!(slot_b, 2);
}

// -------------------------------------------------------
// Issue #283: maximum advance booking window (90 days)
// -------------------------------------------------------

/// A start time 180 days in the future must be rejected with InvalidInput.
#[test]
fn test_booking_180_days_in_advance_rejected() {
    let (env, client, groomer, owner, pet_id) = setup();
    // 180 days * 86400 s/day = 15_552_000 s
    let far_future = NOW + 180 * 86_400;
    let result =
        client.try_book_grooming_slot(&groomer, &far_future, &60u64, &pet_id, &owner);
    assert_eq!(
        result,
        Err(Ok(invalid_input())),
        "Booking 180 days ahead must be rejected"
    );
}

/// A start time exactly at the 90-day boundary must be accepted.
#[test]
fn test_booking_at_90_day_boundary_accepted() {
    let (env, client, groomer, owner, pet_id) = setup();
    // Exactly 90 days from now (within the window)
    let boundary = NOW + 90 * 86_400;
    let slot_id =
        client.book_grooming_slot(&groomer, &boundary, &60u64, &pet_id, &owner);
    assert!(slot_id > 0, "Booking at the 90-day boundary should succeed");
}

/// A start time one second past the 90-day limit must be rejected.
#[test]
fn test_booking_one_second_past_90_day_limit_rejected() {
    let (env, client, groomer, owner, pet_id) = setup();
    let just_over = NOW + 90 * 86_400 + 1;
    let result =
        client.try_book_grooming_slot(&groomer, &just_over, &60u64, &pet_id, &owner);
    assert_eq!(
        result,
        Err(Ok(invalid_input())),
        "Booking one second past the 90-day limit must be rejected"
    );
}
