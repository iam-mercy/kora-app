use pet_transfer_adoption::{PetOwnershipContract, PetOwnershipContractClient};
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
