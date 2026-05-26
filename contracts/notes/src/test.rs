#![cfg(test)]

use soroban_sdk::{Env, testutils::Address as TestAddress, Address};
use crate::BookFundContract;

#[test]
fn test_happy_path() {
    let env = Env::default();
    let bookstore = TestAddress::random(&env);
    BookFundContract::init_fund(env.clone(), 3000, bookstore.clone());
    let friend1 = TestAddress::random(&env);
    BookFundContract::contribute(env.clone(), friend1.clone(), 1000);
    let friend2 = TestAddress::random(&env);
    BookFundContract::contribute(env.clone(), friend2.clone(), 2000);
    BookFundContract::release(env.clone());
    assert_eq!(BookFundContract::check_total(env.clone()), 3000);
}

#[test]
fn test_target_not_reached() {
    let env = Env::default();
    let bookstore = TestAddress::random(&env);
    BookFundContract::init_fund(env.clone(), 3000, bookstore.clone());
    let friend = TestAddress::random(&env);
    BookFundContract::contribute(env.clone(), friend.clone(), 1000);
    assert!(std::panic::catch_unwind(|| {
        BookFundContract::release(env.clone());
    }).is_err());
}

#[test]
fn test_state_after_init() {
    let env = Env::default();
    let bookstore = TestAddress::random(&env);
    BookFundContract::init_fund(env.clone(), 3000, bookstore.clone());
    assert_eq!(BookFundContract::check_total(env.clone()), 0);
}

#[test]
fn test_multiple_contributions() {
    let env = Env::default();
    let bookstore = TestAddress::random(&env);
    BookFundContract::init_fund(env.clone(), 3000, bookstore.clone());
    let friend1 = TestAddress::random(&env);
    BookFundContract::contribute(env.clone(), friend1.clone(), 500);
    let friend2 = TestAddress::random(&env);
    BookFundContract::contribute(env.clone(), friend2.clone(), 700);
    assert_eq!(BookFundContract::check_total(env.clone()), 1200);
}

#[test]
fn test_release_logs_payment() {
    let env = Env::default();
    let bookstore = TestAddress::random(&env);
    BookFundContract::init_fund(env.clone(), 3000, bookstore.clone());
    let friend1 = TestAddress::random(&env);
    BookFundContract::contribute(env.clone(), friend1.clone(), 3000);
    BookFundContract::release(env.clone());
    let released: i128 = env.storage().instance().get(&soroban_sdk::Symbol::short("released")).unwrap();
    assert_eq!(released, 3000);
}
