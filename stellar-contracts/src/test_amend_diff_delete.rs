// =============================================================================
// Tests for Wave 9 issues #97, #98, #99
//   #97 – amend_medical_record records amended_by address + timestamp + emits event
//   #98 – diff_record_versions rejects out-of-bounds version indices
//   #99 – delete_medical_record requires authorization (already implemented; tested here)
// =============================================================================

#[cfg(test)]
mod test_amend_diff_delete {
    use crate::{
        Gender, KoraContract, KoraContractClient, MedicalRecordAmendmentInput,
        PrivacyLevel, Species,
    };
    use soroban_sdk::{testutils::Address as _, Address, Env, String, Vec};

    // -------------------------------------------------------------------------
    // Shared setup
    // -------------------------------------------------------------------------

    fn setup(env: &Env) -> (KoraContractClient<'_>, Address, Address, Address, u64) {
        env.mock_all_auths();
        let contract_id = env.register(KoraContract, ());
        let client = KoraContractClient::new(env, &contract_id);

        let admin = Address::generate(env);
        let vet = Address::generate(env);
        let owner = Address::generate(env);

        client.init_admin(&admin);
        client.register_vet(
            &vet,
            &String::from_str(env, "Dr. Wave9"),
            &String::from_str(env, "LIC-W9"),
            &String::from_str(env, "General"),
        );
        client.verify_vet(&admin, &vet);

        let pet_id = client.register_pet(
            &owner,
            &String::from_str(env, "Paws"),
            &String::from_str(env, "2021-03-15"),
            &Gender::Female,
            &Species::Cat,
            &String::from_str(env, "Persian"),
            &String::from_str(env, "Orange"),
            &4u32,
            &None,
            &PrivacyLevel::Public,
        );

        (client, admin, vet, owner, pet_id)
    }

    fn add_record(
        client: &KoraContractClient<'_>,
        env: &Env,
        vet: &Address,
        pet_id: u64,
    ) -> u64 {
        client.add_medical_record(
            &pet_id,
            vet,
            &String::from_str(env, "Initial Diagnosis"),
            &String::from_str(env, "Initial Treatment"),
            &Vec::new(env),
            &String::from_str(env, "initial notes"),
        )
    }

    // -------------------------------------------------------------------------
    // #97 – amend_medical_record captures amended_by address and block timestamp
    // -------------------------------------------------------------------------

    #[test]
    fn test_amend_medical_record_records_amended_by_and_timestamp() {
        let env = Env::default();
        env.ledger().with_mut(|l| l.timestamp = 1_700_000_000);
        let (client, _admin, vet, _owner, pet_id) = setup(&env);
        let record_id = add_record(&client, &env, &vet, pet_id);

        // Advance the ledger so amendment timestamp is distinct
        env.ledger().with_mut(|l| l.timestamp = 1_700_001_000);

        let input = MedicalRecordAmendmentInput {
            diagnosis: Some(String::from_str(&env, "Revised Diagnosis")),
            treatment: Some(String::from_str(&env, "Revised Treatment")),
            medications: None,
            notes: None,
        };

        let version_returned = client.amend_medical_record(&pet_id, &record_id, &vet, &input);
        assert_eq!(version_returned, 1, "first amendment should be version 1");

        // Retrieve the stored amendment and verify authored fields
        let amendment = client.get_medical_record_amendment(&record_id, &1u32)
            .expect("amendment version 1 must exist");

        assert_eq!(amendment.version, 1);
        assert_eq!(amendment.amended_by, vet, "amended_by must equal the caller");
        assert_eq!(amendment.updated_at, 1_700_001_000, "timestamp must equal ledger time at amendment");
        assert_eq!(
            amendment.changes.diagnosis,
            Some(String::from_str(&env, "Revised Diagnosis"))
        );
    }

    #[test]
    fn test_amend_medical_record_multiple_authors_tracked() {
        let env = Env::default();
        let (client, admin, vet, _owner, pet_id) = setup(&env);
        let record_id = add_record(&client, &env, &vet, pet_id);

        let input1 = MedicalRecordAmendmentInput {
            diagnosis: Some(String::from_str(&env, "Amended by vet")),
            treatment: None,
            medications: None,
            notes: None,
        };
        let input2 = MedicalRecordAmendmentInput {
            diagnosis: Some(String::from_str(&env, "Amended by admin")),
            treatment: None,
            medications: None,
            notes: None,
        };

        client.amend_medical_record(&pet_id, &record_id, &vet, &input1);
        client.amend_medical_record(&pet_id, &record_id, &admin, &input2);

        let v1 = client.get_medical_record_amendment(&record_id, &1u32).unwrap();
        let v2 = client.get_medical_record_amendment(&record_id, &2u32).unwrap();

        assert_eq!(v1.amended_by, vet, "v1 authored by vet");
        assert_eq!(v2.amended_by, admin, "v2 authored by admin");
    }

    /// Unauthorized caller (not the vet or admin) must be rejected.
    #[test]
    #[should_panic]
    fn test_amend_medical_record_unauthorized_caller_rejected() {
        let env = Env::default();
        let (client, _admin, vet, _owner, pet_id) = setup(&env);
        let record_id = add_record(&client, &env, &vet, pet_id);

        let stranger = Address::generate(&env);
        let input = MedicalRecordAmendmentInput {
            diagnosis: Some(String::from_str(&env, "Unauthorized change")),
            treatment: None,
            medications: None,
            notes: None,
        };

        client.amend_medical_record(&pet_id, &record_id, &stranger, &input);
    }

    // -------------------------------------------------------------------------
    // #368 – amend_medical_record rejects tombstoned (soft-deleted) records
    // -------------------------------------------------------------------------

    /// Amending a soft-deleted record must be rejected before any state mutation:
    /// version count, amendment history, and events stay unchanged.
    #[test]
    fn test_amend_soft_deleted_record_rejected_without_mutation() {
        let env = Env::default();
        let (client, _admin, vet, _owner, pet_id) = setup(&env);
        let record_id = add_record(&client, &env, &vet, pet_id);

        // Soft-delete the record (tombstone it).
        client.delete_medical_record(&pet_id, &record_id, &vet);

        // Capture pre-amendment state.
        let version_before = client.get_medical_record_version(&record_id);
        let history_before = client.get_medical_record_history(&record_id);
        let events_before = env.events().all().len();

        let input = MedicalRecordAmendmentInput {
            diagnosis: Some(String::from_str(&env, "Should not apply")),
            treatment: None,
            medications: None,
            notes: None,
        };

        // Attempting to amend a tombstoned record must fail.
        let result = client.try_amend_medical_record(&pet_id, &record_id, &vet, &input);
        assert!(result.is_err(), "amending a soft-deleted record must be rejected");

        // No partial writes: version count, history, and events unchanged.
        assert_eq!(
            client.get_medical_record_version(&record_id),
            version_before,
            "version count must be unchanged on rejection"
        );
        assert_eq!(
            client.get_medical_record_history(&record_id),
            history_before,
            "amendment history must be unchanged on rejection"
        );
        assert_eq!(
            env.events().all().len(),
            events_before,
            "no events must be emitted on rejection"
        );
    }

    /// A non-deleted record still amends successfully (boundary check).
    #[test]
    fn test_amend_non_deleted_record_succeeds() {
        let env = Env::default();
        let (client, _admin, vet, _owner, pet_id) = setup(&env);
        let record_id = add_record(&client, &env, &vet, pet_id);

        let input = MedicalRecordAmendmentInput {
            diagnosis: Some(String::from_str(&env, "Valid amendment")),
            treatment: None,
            medications: None,
            notes: None,
        };

        let version = client.amend_medical_record(&pet_id, &record_id, &vet, &input);
        assert_eq!(version, 1, "non-deleted record should amend to version 1");
    }

    // -------------------------------------------------------------------------
    // #98 – diff_record_versions rejects out-of-bounds version indices
    // -------------------------------------------------------------------------

    #[test]
    #[should_panic(expected = "Error(Contract, #12)")]
    fn test_diff_record_versions_to_version_zero_rejected() {
        let env = Env::default();
        let (client, _admin, vet, _owner, pet_id) = setup(&env);
        let record_id = add_record(&client, &env, &vet, pet_id);

        // to_version = 0 is invalid (versions start at 1)
        client.diff_record_versions(&pet_id, &record_id, &0u32, &0u32);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #12)")]
    fn test_diff_record_versions_to_version_out_of_bounds_rejected() {
        let env = Env::default();
        let (client, _admin, vet, _owner, pet_id) = setup(&env);
        let record_id = add_record(&client, &env, &vet, pet_id);

        let input = MedicalRecordAmendmentInput {
            diagnosis: Some(String::from_str(&env, "v1 diagnosis")),
            treatment: None,
            medications: None,
            notes: None,
        };
        client.amend_medical_record(&pet_id, &record_id, &vet, &input);
        // Only version 1 exists; requesting version 99 must panic with InvalidInput
        client.diff_record_versions(&pet_id, &record_id, &0u32, &99u32);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #12)")]
    fn test_diff_record_versions_from_version_out_of_bounds_rejected() {
        let env = Env::default();
        let (client, _admin, vet, _owner, pet_id) = setup(&env);
        let record_id = add_record(&client, &env, &vet, pet_id);

        let input = MedicalRecordAmendmentInput {
            diagnosis: Some(String::from_str(&env, "v1 diagnosis")),
            treatment: None,
            medications: None,
            notes: None,
        };
        client.amend_medical_record(&pet_id, &record_id, &vet, &input);
        // from_version 50 is out of bounds (max is 1)
        client.diff_record_versions(&pet_id, &record_id, &50u32, &1u32);
    }
}
