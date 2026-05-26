use soroban_sdk::{contractimpl, Env, Address, Symbol, Map, Vec};

pub struct AntiCrocodile;

#[contractimpl]
impl AntiCrocodile {
    // Storage key for approved recipients
    const APPROVED: Symbol = Symbol::short("APPROVED");

    // Add an approved recipient (government sets this once)
    pub fn add_recipient(env: Env, recipient: Address) {
        let mut list: Vec<Address> = env.storage().persistent().get(&Self::APPROVED).unwrap_or(Vec::new(&env));
        if !list.contains(&recipient) {
            list.push_back(recipient.clone());
            env.storage().persistent().set(&Self::APPROVED, &list);
        }
    }

    // Release funds only if recipient is approved
    pub fn release(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth(); // Only government wallet can call
        let list: Vec<Address> = env.storage().persistent().get(&Self::APPROVED).unwrap_or(Vec::new(&env));
        if !list.contains(&to) {
            panic!("Recipient not approved");
        }
        env.pay(&from, &to, amount); // Transfer USDC
    }

    // View approved recipients
    pub fn get_recipients(env: Env) -> Vec<Address> {
        env.storage().persistent().get(&Self::APPROVED).unwrap_or(Vec::new(&env))
    }
}
