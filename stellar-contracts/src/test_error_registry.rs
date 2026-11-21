use crate::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String, Vec};

fn setup_env() -> (Env, KoraContractClient<'static>, Address) {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(KoraContract, ());
    let client = KoraContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.init_admin(&admin);

    (env, client, admin)
}

// --- BASIC FUNCTIONALITY ---

#[test]
fn test_set_and_get_error_message() {
    let (env, client, admin) = setup_env();

    // Set an error message in English
    client.set_error_message(
        &admin,
        &3,
        &String::from_str(&env, "en"),
        &String::from_str(&env, "Pet not found"),
    );

    // Retrieve the message
    let message = client.get_error_message(&3, &String::from_str(&env, "en"));
    assert_eq!(message, Some(String::from_str(&env, "Pet not found")));
}

#[test]
fn test_get_nonexistent_error_message_returns_none() {
    let (env, client, _admin) = setup_env();

    // Try to get a message that doesn't exist
    let message = client.get_error_message(&999, &String::from_str(&env, "en"));
    assert_eq!(message, None);
}

#[test]
fn test_set_error_message_multiple_languages() {
    let (env, client, admin) = setup_env();

    // Set English message
    client.set_error_message(
        &admin,
        &3,
        &String::from_str(&env, "en"),
        &String::from_str(&env, "Pet not found"),
    );

    // Set Spanish message
    client.set_error_message(
        &admin,
        &3,
        &String::from_str(&env, "es"),
        &String::from_str(&env, "Mascota no encontrada"),
    );

    // Retrieve both messages
    let en_message = client.get_error_message(&3, &String::from_str(&env, "en"));
    let es_message = client.get_error_message(&3, &String::from_str(&env, "es"));

    assert_eq!(en_message, Some(String::from_str(&env, "Pet not found")));
    assert_eq!(
        es_message,
        Some(String::from_str(&env, "Mascota no encontrada"))
    );
}

#[test]
fn test_supported_languages_updated() {
    let (env, client, admin) = setup_env();

    // Initially no languages
    let languages = client.get_supported_languages();
    assert_eq!(languages.len(), 0);

    // Add English message
    client.set_error_message(
        &admin,
        &3,
        &String::from_str(&env, "en"),
        &String::from_str(&env, "Pet not found"),
    );

    let languages = client.get_supported_languages();
    assert_eq!(languages.len(), 1);
    assert!(languages.contains(&String::from_str(&env, "en")));

    // Add Spanish message
    client.set_error_message(
        &admin,
        &3,
        &String::from_str(&env, "es"),
        &String::from_str(&env, "Mascota no encontrada"),
    );

    let languages = client.get_supported_languages();
    assert_eq!(languages.len(), 2);
    assert!(languages.contains(&String::from_str(&env, "en")));
    assert!(languages.contains(&String::from_str(&env, "es")));
}

#[test]
fn test_overwrite_existing_message() {
    let (env, client, admin) = setup_env();

    // Set initial message
    client.set_error_message(
        &admin,
        &3,
        &String::from_str(&env, "en"),
        &String::from_str(&env, "Pet not found"),
    );

    // Overwrite with new message
    client.set_error_message(
        &admin,
        &3,
        &String::from_str(&env, "en"),
        &String::from_str(&env, "Pet record not found in database"),
    );

    // Retrieve the updated message
    let message = client.get_error_message(&3, &String::from_str(&env, "en"));
    assert_eq!(
        message,
        Some(String::from_str(&env, "Pet record not found in database"))
    );
}

// --- BATCH OPERATIONS ---

#[test]
fn test_batch_set_error_messages() {
    let (env, client, admin) = setup_env();

    let mut messages = Vec::new(&env);
    messages.push_back(ErrorMessage {
        code: 1,
        language: String::from_str(&env, "en"),
        message: String::from_str(&env, "Unauthorized"),
    });
    messages.push_back(ErrorMessage {
        code: 2,
        language: String::from_str(&env, "en"),
        message: String::from_str(&env, "Admin not initialized"),
    });
    messages.push_back(ErrorMessage {
        code: 1,
        language: String::from_str(&env, "es"),
        message: String::from_str(&env, "No autorizado"),
    });

    client.batch_set_error_messages(&admin, &messages);

    // Verify all messages were set
    let msg1_en = client.get_error_message(&1, &String::from_str(&env, "en"));
    let msg2_en = client.get_error_message(&2, &String::from_str(&env, "en"));
    let msg1_es = client.get_error_message(&1, &String::from_str(&env, "es"));

    assert_eq!(msg1_en, Some(String::from_str(&env, "Unauthorized")));
    assert_eq!(
        msg2_en,
        Some(String::from_str(&env, "Admin not initialized"))
    );
    assert_eq!(msg1_es, Some(String::from_str(&env, "No autorizado")));
}

#[test]
fn test_initialize_error_messages() {
    let (env, client, admin) = setup_env();

    // Initialize default messages
    client.initialize_error_messages(&admin);

    // Check that English and Spanish are supported
    let languages = client.get_supported_languages();
    assert!(languages.contains(&String::from_str(&env, "en")));
    assert!(languages.contains(&String::from_str(&env, "es")));

    // Check some specific messages
    let msg_en = client.get_error_message(&3, &String::from_str(&env, "en"));
    let msg_es = client.get_error_message(&3, &String::from_str(&env, "es"));

    assert_eq!(msg_en, Some(String::from_str(&env, "Pet not found")));
    assert_eq!(
        msg_es,
        Some(String::from_str(&env, "Mascota no encontrada"))
    );
}

// --- REMOVE MESSAGES ---

#[test]
fn test_remove_error_message() {
    let (env, client, admin) = setup_env();

    // Set a message
    client.set_error_message(
        &admin,
        &3,
        &String::from_str(&env, "en"),
        &String::from_str(&env, "Pet not found"),
    );

    // Verify it exists
    let message = client.get_error_message(&3, &String::from_str(&env, "en"));
    assert!(message.is_some());

    // Remove it
    client.remove_error_message(&admin, &3, &String::from_str(&env, "en"));

    // Verify it's gone
    let message = client.get_error_message(&3, &String::from_str(&env, "en"));
    assert_eq!(message, None);
}

// --- AUTHORIZATION ---

#[test]
#[should_panic]
fn test_set_error_message_requires_admin() {
    let (env, client, _admin) = setup_env();
    let non_admin = Address::generate(&env);

    // Non-admin should not be able to set messages
    client.set_error_message(
        &non_admin,
        &3,
        &String::from_str(&env, "en"),
        &String::from_str(&env, "Pet not found"),
    );
}

#[test]
#[should_panic]
fn test_batch_set_requires_admin() {
    let (env, client, _admin) = setup_env();
    let non_admin = Address::generate(&env);

    let mut messages = Vec::new(&env);
    messages.push_back(ErrorMessage {
        code: 1,
        language: String::from_str(&env, "en"),
        message: String::from_str(&env, "Unauthorized"),
    });

    client.batch_set_error_messages(&non_admin, &messages);
}

#[test]
#[should_panic]
fn test_initialize_requires_admin() {
    let (env, client, _admin) = setup_env();
    let non_admin = Address::generate(&env);

    client.initialize_error_messages(&non_admin);
}

#[test]
#[should_panic]
fn test_remove_requires_admin() {
    let (env, client, admin) = setup_env();
    let non_admin = Address::generate(&env);

    // Set a message as admin
    client.set_error_message(
        &admin,
        &3,
        &String::from_str(&env, "en"),
        &String::from_str(&env, "Pet not found"),
    );

    // Try to remove as non-admin
    client.remove_error_message(&non_admin, &3, &String::from_str(&env, "en"));
}

// --- VALIDATION ---

#[test]
#[should_panic]
fn test_empty_language_rejected() {
    let (env, client, admin) = setup_env();

    client.set_error_message(
        &admin,
        &3,
        &String::from_str(&env, ""),
        &String::from_str(&env, "Pet not found"),
    );
}

#[test]
#[should_panic]
fn test_invalid_language_too_long_rejected() {
    let (env, client, admin) = setup_env();

    client.set_error_message(
        &admin,
        &3,
        &String::from_str(&env, "english"),
        &String::from_str(&env, "Pet not found"),
    );
}

#[test]
#[should_panic]
fn test_empty_message_rejected() {
    let (env, client, admin) = setup_env();

    client.set_error_message(
        &admin,
        &3,
        &String::from_str(&env, "en"),
        &String::from_str(&env, ""),
    );
}

#[test]
#[should_panic]
fn test_message_too_long_rejected() {
    let (env, client, admin) = setup_env();

    let long_message = String::from_str(
        &env,
        "This error message is intentionally made much longer than two hundred characters so that the contract rejects it during validation of the message length constraint in set_error_message. Padding padding padding padding padding padding padding padding padding padding padding padding.",
    );

    client.set_error_message(&admin, &3, &String::from_str(&env, "en"), &long_message);
}
