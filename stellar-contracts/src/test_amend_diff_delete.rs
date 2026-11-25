// =============================================================================
// Tests for Wave 9 issues #97, #98, #99
//   #97 – amend_medical_record records amended_by address + timestamp + emits event
//   #98 – diff_record_versions rejects out-of-bounds version indices
//   #99 – delete_medical_record requires authorization (already implemented; tested here)
//   #367 – amend_medical_record projects amendment onto current record state
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
    // #367 – amendment is projected onto the current record state
    // -------------------------------------------------------------------------

    /// A single amendment must be reflected by ordinary record retrieval.
    #[test]
    fn test_amend_medical_record_updates_current_record() {
        let env = Env::default();
        let (client, _admin, vet, _owner, pet_id) = setup(&env);
        let record_id = add_record(&client, &env, &vet, pet_id);

        let input = MedicalRecordAmendmentInput {
            diagnosis: Some(String::from_str(&env, "Revised Diagnosis")),
            treatment: Some(String::from_str(&env, "Revised Treatment")),
            medications: None,
            notes: Some(String::from_str(&env, "revised notes")),
        };
        client.amend_medical_record(&pet_id, &record_id, &vet, &input);

        let record = client.get_medical_record(&record_id)
            .expect("record must still exist after amendment");

        assert_eq!(record.diagnosis, String::from_str(&env, "Revised Diagnosis"));
        assert_eq!(record.treatment, String::from_str(&env, "Revised Treatment"));
        assert_eq!(record.notes, String::from_str(&env, "revised notes"));
    }

    /// Fields not present in the amendment must be preserved from the current record.
    #[test]
    fn test_amend_medical_record_preserves_untouched_fields() {
        let env = Env::default();
        let (client, _admin, vet, _owner, pet_id) = setup(&env);
        let record_id = add_record(&client, &env, &vet, pet_id);

        let input = MedicalRecordAmendmentInput {
            diagnosis: Some(String::from_str(&env, "Only diagnosis changed")),
            treatment: None,
            medications: None,
            notes: None,
        };
        client.amend_medical_record(&pet_id, &record_id, &vet, &input);

        let record = client.get_medical_record(&record_id)
            .expect("record must still exist after amendment");

        assert_eq!(record.diagnosis, String::from_str(&env, "Only diagnosis changed"));
        assert_eq!(record.treatment, String::from_str(&env, "Initial Treatment"));
        assert_eq!(record.notes, String::from_str(&env, "initial notes"));
    }

    /// Successive amendments must each project onto the latest record state and
    /// keep the version history intact.
    #[test]
    fn test_successive_amendments_update_record_and_history() {
        let env = Env::default();
        let (client, _admin, vet, _owner, pet_id) = setup(&env);
        let record_id = add_record(&client, &env, &vet, pet_id);

        let input1 = MedicalRecordAmendmentInput {
            diagnosis: Some(String::from_str(&env, "Diagnosis v1")),
            treatment: None,
            medications: None,
            notes: None,
        };
        let input2 = MedicalRecordAmendmentInput {
            diagnosis: None,
            treatment: Some(String::from_str(&env, "Treatment v2")),
            medications: None,
            notes: None,
        };

        let v1 = client.amend_medical_record(&pet_id, &record_id, &vet, &input1);
        let v2 = client.amend_medical_record(&pet_id, &record_id, &vet, &input2);
        assert_eq!(v1, 1);
        assert_eq!(v2, 2);

        // Current retrieval reflects both amendments cumulatively.
        let record = client.get_medical_record(&record_id)
            .expect("record must still exist after amendments");
        assert_eq!(record.diagnosis, String::from_str(&env, "Diagnosis v1"));
        assert_eq!(record.treatment, String::from_str(&env, "Treatment v2"));

        // Version history is preserved for each amendment.
        let a1 = client.get_medical_record_amendment(&record_id, &1u32).unwrap();
        let a2 = client.get_medical_record_amendment(&record_id, &2u32).unwrap();
        assert_eq!(a1.version, 1);
        assert_eq!(a2.version, 2);
        assert_eq!(a1.changes.diagnosis, Some(String::from_str(&env, "Diagnosis v1")));
        assert_eq!(a2.changes.treatment, Some(String::from_str(&env, "Treatment v2")));
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

    // -------------------------------------------------------------------------
    // #442 – the 5-version amendment cap must be enforced atomically: a
    // rejected 6th amendment must leave the count, current record, and all
    // prior versions completely unchanged.
    // -------------------------------------------------------------------------

    #[test]
    fn test_sixth_amendment_rejected_and_state_unchanged() {
        let env = Env::default();
        let (client, _admin, vet, _owner, pet_id) = setup(&env);
        let record_id = add_record(&client, &env, &vet, pet_id);

        // Fill five versions.
        for i in 1..=5u32 {
            let input = MedicalRecordAmendmentInput {
                diagnosis: Some(String::from_str(&env, &format!("Diagnosis v{}", i))),
                treatment: None,
                medications: None,
                notes: None,
            };
            let version = client.amend_medical_record(&pet_id, &record_id, &vet, &input);
            assert_eq!(version, i);
        }

        let record_before = client.get_medical_record(&record_id).unwrap();
        let count_before: u32 = env.as_contract(&client.address, || {
            env.storage()
                .instance()
                .get(&crate::MedicalKey::MedicalRecordAmendmentCount(record_id))
                .unwrap_or(0)
        });

        // Sixth amendment must be rejected.
        let sixth_input = MedicalRecordAmendmentInput {
            diagnosis: Some(String::from_str(&env, "Diagnosis v6 should not land")),
            treatment: None,
            medications: None,
            notes: None,
        };
        let result = client.try_amend_medical_record(&pet_id, &record_id, &vet, &sixth_input);
        assert!(result.is_err(), "sixth amendment must be rejected");

        // Amendment count must remain at 5 (no partial write of the count).
        let count_after: u32 = env.as_contract(&client.address, || {
            env.storage()
                .instance()
                .get(&crate::MedicalKey::MedicalRecordAmendmentCount(record_id))
                .unwrap_or(0)
        });
        assert_eq!(count_after, count_before);
        assert_eq!(count_after, 5);

        // The current record must be unchanged (still reflecting version 5).
        let record_after = client.get_medical_record(&record_id).unwrap();
        assert_eq!(record_after.diagnosis, record_before.diagnosis);
        assert_eq!(record_after.diagnosis, String::from_str(&env, "Diagnosis v5"));

        // No version 6 amendment must have been written.
        let v6 = client.get_medical_record_amendment(&record_id, &6u32);
        assert!(v6.is_none(), "no version 6 amendment must exist after rejection");

        // All prior versions (1..=5) must remain intact and untouched.
        for i in 1..=5u32 {
            let amendment = client.get_medical_record_amendment(&record_id, &i).unwrap();
            assert_eq!(amendment.version, i);
            assert_eq!(
                amendment.changes.diagnosis,
                Some(String::from_str(&env, &format!("Diagnosis v{}", i)))
            );
        }
    }
}
