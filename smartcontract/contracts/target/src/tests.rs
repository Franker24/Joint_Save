#![cfg(test)]

use super::{TargetPool, TargetPoolClient};
use soroban_sdk::{
    testutils::{Address as _, Ledger as _},
    token, Address, Env, Vec,
};

#[test]
fn test_unlock_on_target() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, TargetPool);
    let client = TargetPoolClient::new(&env, &contract_id);

    let token_admin = Address::generate(&env);
    let token_contract = env.register_stellar_asset_contract_v2(token_admin.clone());
    let token_address = token_contract.address();
    let token_client = token::StellarAssetClient::new(&env, &token_address);

    let admin = Address::generate(&env);
    let member_a = Address::generate(&env);
    let member_b = Address::generate(&env);

    let mut members = Vec::new(&env);
    members.push_back(member_a.clone());
    members.push_back(member_b.clone());

    let target_amount = 100i128;
    let deadline = 1000u32;

    client.initialize(
        &token_address,
        &admin,
        &members,
        &target_amount,
        &deadline,
    );

    assert!(!client.is_unlocked());
    assert_eq!(client.total_deposited(), 0);

    token_client.mint(&member_a, &100i128);
    token_client.mint(&member_b, &100i128);

    // Deposit 40 from member A
    client.deposit(&member_a, &40i128);
    assert_eq!(client.total_deposited(), 40);
    assert!(!client.is_unlocked());

    // Deposit 60 from member B (target is 100)
    client.deposit(&member_b, &60i128);
    assert_eq!(client.total_deposited(), 100);
    assert!(client.is_unlocked());
}
