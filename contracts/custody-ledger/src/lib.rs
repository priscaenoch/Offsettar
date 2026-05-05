#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol};

#[contracttype]
pub enum DataKey {
    CustodyBalance(Address),
}

#[contract]
pub struct CustodyLedger;

#[contractimpl]
impl CustodyLedger {
    pub fn credit(env: Env, account: Address, amount: i128) {
        let current: i128 = env.storage().persistent().get(&DataKey::CustodyBalance(account.clone())).unwrap_or(0);
        env.storage().persistent().set(&DataKey::CustodyBalance(account), &(current + amount));
    }

    pub fn debit(env: Env, account: Address, amount: i128) {
        let current: i128 = env.storage().persistent().get(&DataKey::CustodyBalance(account.clone())).unwrap_or(0);
        assert!(current >= amount, "insufficient custody balance");
        env.storage().persistent().set(&DataKey::CustodyBalance(account), &(current - amount));
    }

    pub fn balance(env: Env, account: Address) -> i128 {
        env.storage().persistent().get(&DataKey::CustodyBalance(account)).unwrap_or(0)
    }
}
