#![cfg(test)]
use soroban_sdk::{Env, Address, testutils::Address as TestAddress};
use crate::AntiCrocodile;

#[test]
fn test_happy_path() {
    let env = Env::default();
    let gov = TestAddress::random(&env);
    let barangay = TestAddress::random(&env);

    AntiCrocodile::add_recipient(env.clone(), barangay.clone());
    AntiCrocodile::release(env.clone(), gov.clone(), barangay.clone(), 1000);
}

#[test]
#[should_panic(expected = "Recipient not approved")]
fn test_unapproved_recipient() {
    let env = Env::default();
    let gov = TestAddress::random(&env);
    let fake = TestAddress::random(&env);

    AntiCrocodile::release(env.clone(), gov.clone(), fake.clone(), 500);
}

#[test]
fn test_storage_state() {
    let env = Env::default();
    let barangay = TestAddress::random(&env);

    AntiCrocodile::add_recipient(env.clone(), barangay.clone());
    let recipients = AntiCrocodile::get_recipients(env.clone());
    assert!(recipients.contains(&barangay));
}

#[test]
#[should_panic]
fn test_unauthorized_caller() {
    let env = Env::default();
    let gov = TestAddress::random(&env);
    let barangay = TestAddress::random(&env);
    let hacker = TestAddress::random(&env);

    AntiCrocodile::add_recipient(env.clone(), barangay.clone());
    AntiCrocodile::release(env.clone(), hacker.clone(), barangay.clone(), 1000);
}

#[test]
fn test_duplicate_recipient() {
    let env = Env::default();
    let barangay = TestAddress::random(&env);

    AntiCrocodile::add_recipient(env.clone(), barangay.clone());
    AntiCrocodile::add_recipient(env.clone(), barangay.clone()); // should not duplicate
    let recipients = AntiCrocodile::get_recipients(env.clone());
    assert_eq!(recipients.len(), 1);
}
