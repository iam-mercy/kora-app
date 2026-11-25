use crate::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

// Issue #444: unknown offspring index holes must not repeat neighboring IDs.
//
// `get_offspring_ids` walks a fixed ordinal range `[offset+1, offset+limit]`
// and silently skips any sequence number whose `BreedingOffspringIndex` key
// is missing (a "hole"). This test seeds such a hole directly in storage and
// asserts the hole is omitted from the result rather than being filled in
// with a neighboring offspring ID, and that result order and offset
// semantics (offset counts index slots, 1-based) are preserved.
#[test]
fn test_get_offspring_ids_skips_holes_without_repeating_neighbors() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, KoraContract);
    let client = KoraContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.init_admin(&admin);

    let record_id: u64 = 7;

    // Seed offspring at sequence slots 1, 3, 4 (slot 2 is a hole), with a
    // total count of 4, as if slot 2's write never landed.
    env.as_contract(&contract_id, || {
        env.storage().persistent().set(
            &BreedingKey::BreedingOffspringCount(record_id),
            &4u64,
        );
        env.storage().persistent().set(
            &BreedingKey::BreedingOffspringIndex((record_id, 1u64)),
            &101u64,
        );
        // Slot 2 intentionally left unset (the hole).
        env.storage().persistent().set(
            &BreedingKey::BreedingOffspringIndex((record_id, 3u64)),
            &103u64,
        );
        env.storage().persistent().set(
            &BreedingKey::BreedingOffspringIndex((record_id, 4u64)),
            &104u64,
        );
    });

    // Full range: the hole at slot 2 must be omitted, not filled with 101 or 103.
    let full = client.get_offspring_ids(&record_id, &0u32, &10u32);
    assert_eq!(full.len(), 3);
    assert_eq!(full.get(0).unwrap(), 101u64);
    assert_eq!(full.get(1).unwrap(), 103u64);
    assert_eq!(full.get(2).unwrap(), 104u64);

    // Paging directly over the hole (offset=1 => slot 2 only) must return
    // nothing, not a repeat of the neighboring slot 1 or slot 3 value.
    let over_hole = client.get_offspring_ids(&record_id, &1u32, &1u32);
    assert_eq!(over_hole.len(), 0);

    // Page covering slots 2..=3 (offset=1, limit=2) should only surface slot 3's value.
    let adjacent = client.get_offspring_ids(&record_id, &1u32, &2u32);
    assert_eq!(adjacent.len(), 1);
    assert_eq!(adjacent.get(0).unwrap(), 103u64);

    // Page covering slots 1..=2 (offset=0, limit=2) should only surface slot 1's value.
    let adjacent_before = client.get_offspring_ids(&record_id, &0u32, &2u32);
    assert_eq!(adjacent_before.len(), 1);
    assert_eq!(adjacent_before.get(0).unwrap(), 101u64);
}
