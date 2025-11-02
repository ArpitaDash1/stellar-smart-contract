#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol};

#[contract]
pub struct Counter;

#[contractimpl]
impl Counter {
    pub fn increment(env: Env) -> u32 {
        let key = Symbol::new(&env, "count");
        let mut count: u32 = env.storage().persistent().get(&key).unwrap_or(0);
        count += 1;
        env.storage().persistent().set(&key, &count);
        count
    }

    pub fn reset(env: Env) {
        let key = Symbol::new(&env, "count");
        env.storage().persistent().set(&key, &0u32);
    }

    pub fn get(env: Env) -> u32 {
        let key = Symbol::new(&env, "count");
        env.storage().persistent().get(&key).unwrap_or(0)
    }
}
