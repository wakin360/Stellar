use soroban_sdk::{contractimpl, Env, Address, Symbol};

pub struct BookFundContract;

#[contractimpl]
impl BookFundContract {
    // Initialize a pooled fund with target amount
    pub fn init_fund(env: Env, target: i128, bookstore: Address) {
        env.storage().instance().set(&Symbol::short("target"), &target);
        env.storage().instance().set(&Symbol::short("bookstore"), &bookstore);
        env.storage().instance().set(&Symbol::short("total"), &0);
    }

    // Contribute to the pooled fund
    pub fn contribute(env: Env, contributor: Address, amount: i128) {
        contributor.require_auth();
        let total: i128 = env.storage().instance().get(&Symbol::short("total")).unwrap_or(0);
        env.storage().instance().set(&Symbol::short("total"), &(total + amount));
    }

    // Release funds to bookstore once target is met
    pub fn release(env: Env) {
        let total: i128 = env.storage().instance().get(&Symbol::short("total")).unwrap_or(0);
        let target: i128 = env.storage().instance().get(&Symbol::short("target")).unwrap();
        if total < target {
            panic!("Target not yet reached");
        }
        let bookstore: Address = env.storage().instance().get(&Symbol::short("bookstore")).unwrap();
        bookstore.require_auth(); // simulate acceptance
        env.storage().instance().set(&Symbol::short("released"), &target);
    }

    // Check current pooled total
    pub fn check_total(env: Env) -> i128 {
        env.storage().instance().get(&Symbol::short("total")).unwrap_or(0)
    }
}
