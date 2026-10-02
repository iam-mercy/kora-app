/// Tests for issue #110: get_ownership_history privacy access control.
///
/// Rules under test:
/// - `PrivacyLevel::Private` → only current owner or admin may call; any other
///   caller causes a panic (ContractError::Unauthorized).
/// - `PrivacyLevel::Public` → any caller may retrieve the history.
/// - `PrivacyLevel::Restricted` → any caller may retrieve the history (same
///   as Public for this endpoint; prior-owner addresses are not redacted).
use crate::{Gender, KoraContract, KoraContractClient, PrivacyLevel, Species};
use soroban_sdk::{testutils::Address as _, Address, Env, String};

// -------------------------------------------------------
// Shared helpers
// -------------------------------------------------------

fn setup(env: &Env) -> (KoraContractClient<'_>, Address) {
    env.mock_all_auths();
    let contract_id = env.register_contract(None, KoraContract);
    let client = KoraContractClient::new(env, &contract_id);
    let admin = Address::generate(env);
    client.init_admin(&admin);
    (client, admin)
}

fn register_pet(
    client: &KoraContractClient,
    env: &Env,
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
        &30,
        &None,
        &privacy,
    )
}

// -------------------------------------------------------
// Private pet — access control
// -------------------------------------------------------

/// The current owner can always retrieve the full ownership history of their
/// own private pet.
#[test]
fn private_pet_owner_can_view_ownership_history() {
    let env = Env::default();
    let (client, _admin) = setup(&env);
    let owner = Address::generate(&env);
    let new_owner = Address::generate(&env);

    let pet_id = register_pet(&client, &env, &owner, PrivacyLevel::Private);

    // Perform one transfer so the history has more than the initial record.
    client.transfer_pet_ownership(&pet_id, &new_owner, &0);
    client.accept_pet_transfer(&pet_id);

    // new_owner is now the current owner — they should see the full history.
    let history = client.get_ownership_history(&pet_id, &new_owner, &0u64, &10u32);
    assert!(
        history.len() >= 1,
        "current owner must receive at least one ownership record"
    );
}

/// A registered admin can retrieve the full ownership history of a private pet.
#[test]
fn private_pet_admin_can_view_ownership_history() {
    let env = Env::default();
    let (client, admin) = setup(&env);
    let owner = Address::generate(&env);

    let pet_id = register_pet(&client, &env, &owner, PrivacyLevel::Private);

    let history = client.get_ownership_history(&pet_id, &admin, &0u64, &10u32);
    // At least the registration record should be present.
    assert!(
        history.len() >= 1,
        "admin must receive at least one ownership record"
    );
}

/// An unauthenticated / unrecognised caller must NOT be able to retrieve the
/// ownership history of a private pet.
#[test]
#[should_panic]
fn private_pet_stranger_cannot_view_ownership_history() {
    let env = Env::default();
    let (client, _admin) = setup(&env);
    let owner = Address::generate(&env);
    let stranger = Address::generate(&env);

    let pet_id = register_pet(&client, &env, &owner, PrivacyLevel::Private);

    // This must panic with ContractError::Unauthorized.
    client.get_ownership_history(&pet_id, &stranger, &0u64, &10u32);
}

// -------------------------------------------------------
// Public pet — unrestricted access
// -------------------------------------------------------

/// Any caller can read the ownership history of a public pet.
#[test]
fn public_pet_anyone_can_view_ownership_history() {
    let env = Env::default();
    let (client, _admin) = setup(&env);
    let owner = Address::generate(&env);
    let stranger = Address::generate(&env);

    let pet_id = register_pet(&client, &env, &owner, PrivacyLevel::Public);

    // Stranger — neither owner nor admin — must still succeed.
    let history = client.get_ownership_history(&pet_id, &stranger, &0u64, &10u32);
    assert!(
        history.len() >= 1,
        "public pet history must be readable by any caller"
    );
}

// -------------------------------------------------------
// Restricted pet — unrestricted access for this endpoint
// -------------------------------------------------------

/// Any caller can read the ownership history of a restricted pet (this
/// endpoint does not further redact based on Restricted level).
#[test]
fn restricted_pet_anyone_can_view_ownership_history() {
    let env = Env::default();
    let (client, _admin) = setup(&env);
    let owner = Address::generate(&env);
    let stranger = Address::generate(&env);

    let pet_id = register_pet(&client, &env, &owner, PrivacyLevel::Restricted);

    let history = client.get_ownership_history(&pet_id, &stranger, &0u64, &10u32);
    assert!(
        history.len() >= 1,
        "restricted pet history must be readable by any caller"
    );
}

// -------------------------------------------------------
// Pagination still works for private pets (authorised caller)
// -------------------------------------------------------

/// Pagination parameters are honoured when the caller is authorised.
#[test]
fn private_pet_owner_history_pagination_works() {
    let env = Env::default();
    let (client, _admin) = setup(&env);
    let owner = Address::generate(&env);
    let new_owner = Address::generate(&env);
    let third_owner = Address::generate(&env);

    let pet_id = register_pet(&client, &env, &owner, PrivacyLevel::Private);

    // Two transfers: owner → new_owner → third_owner
    client.transfer_pet_ownership(&pet_id, &new_owner, &0);
    client.accept_pet_transfer(&pet_id);
    client.transfer_pet_ownership(&pet_id, &third_owner, &0);
    client.accept_pet_transfer(&pet_id);

    // third_owner is current owner; retrieve page 1 (1 record, skip 1).
    let page = client.get_ownership_history(&pet_id, &third_owner, &1u64, &1u32);
    assert_eq!(page.len(), 1, "pagination must return exactly one record");

    // Empty page beyond history length.
    let empty = client.get_ownership_history(&pet_id, &third_owner, &100u64, &10u32);
    assert_eq!(empty.len(), 0, "out-of-range offset must return empty vec");
}
