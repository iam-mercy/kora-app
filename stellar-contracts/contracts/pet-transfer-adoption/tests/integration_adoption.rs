use pet_transfer_adoption::{PetOwnershipContract, PetOwnershipContractClient, TransferType};
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn adoption_flow_transfers_pet_ownership() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, PetOwnershipContract);
    let client = PetOwnershipContractClient::new(&env, &contract_id);
    let owner = Address::generate(&env);
    let adopter = Address::generate(&env);
    let pet_id = 7;

    client.create_pet(&pet_id, &owner);
    client.set_adoption_config(&0, &owner);
    client.sign_adoption(&pet_id, &adopter, &None);
    client.approve_adoption(&pet_id, &adopter);
    client.complete_adoption(&pet_id);

    assert_eq!(client.get_current_owner(&pet_id), adopter);
    assert_eq!(client.get_adoption_record(&pet_id).unwrap().pet_id, pet_id);
}

/// Issue #111 – complete_adoption must record a CustodyEntry so the adoption
/// is reflected in the chain-of-custody provenance history.
#[test]
fn adoption_flow_records_custody_chain_entry() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, PetOwnershipContract);
    let client = PetOwnershipContractClient::new(&env, &contract_id);
    let owner = Address::generate(&env);
    let adopter = Address::generate(&env);
    let pet_id = 8;

    client.create_pet(&pet_id, &owner);
    client.set_adoption_config(&0, &owner);
    client.sign_adoption(&pet_id, &adopter, &None);
    client.approve_adoption(&pet_id, &adopter);
    client.complete_adoption(&pet_id);

    // The custody chain must contain exactly one entry reflecting the adoption.
    let chain = client.get_custody_chain(&pet_id);
    assert_eq!(
        chain.len(),
        1,
        "complete_adoption must append exactly one custody entry"
    );

    let entry = chain.get(0).unwrap();
    assert_eq!(entry.from, owner, "custody entry must record the previous owner");
    assert_eq!(entry.to, adopter, "custody entry must record the new owner");
    assert_eq!(
        entry.transfer_type,
        TransferType::Adoption,
        "adoption transfers must be logged with TransferType::Adoption"
    );
}

/// Issue #111 – finalize_transfer must record a CustodyEntry so direct
/// transfers are reflected in the chain-of-custody provenance history.
#[test]
fn finalize_transfer_records_custody_chain_entry() {
    use pet_transfer_adoption::DISPUTE_WINDOW_SECONDS;
    use soroban_sdk::testutils::Ledger;

    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, PetOwnershipContract);
    let client = PetOwnershipContractClient::new(&env, &contract_id);
    let owner = Address::generate(&env);
    let new_owner = Address::generate(&env);
    let pet_id = 9;

    client.create_pet(&pet_id, &owner);
    client.initiate_transfer(&pet_id, &new_owner);
    client.accept_transfer(&pet_id);

    // Advance past the dispute window so finalize_transfer can proceed.
    env.ledger()
        .with_mut(|l| l.timestamp += DISPUTE_WINDOW_SECONDS + 1);
    client.finalize_transfer(&pet_id);

    // The custody chain must contain exactly one entry reflecting the transfer.
    let chain = client.get_custody_chain(&pet_id);
    assert_eq!(
        chain.len(),
        1,
        "finalize_transfer must append exactly one custody entry"
    );

    let entry = chain.get(0).unwrap();
    assert_eq!(entry.from, owner, "custody entry must record the previous owner");
    assert_eq!(entry.to, new_owner, "custody entry must record the new owner");
    assert_eq!(
        entry.transfer_type,
        TransferType::Direct,
        "direct transfers must be logged with TransferType::Direct"
    );
}
