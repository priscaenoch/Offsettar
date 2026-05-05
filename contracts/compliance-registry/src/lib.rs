#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[contracttype]
pub enum DataKey {
    Approved(Address),
    Blocked(Address),
}

#[contract]
pub struct ComplianceRegistry;

#[contractimpl]
impl ComplianceRegistry {
    pub fn approve(env: Env, admin: Address, account: Address) {
        admin.require_auth();
        env.storage().persistent().set(&DataKey::Approved(account), &true);
    }

    pub fn block(env: Env, admin: Address, account: Address) {
        admin.require_auth();
        env.storage().persistent().set(&DataKey::Blocked(account), &true);
    }

    pub fn is_approved(env: Env, account: Address) -> bool {
        env.storage().persistent().get(&DataKey::Approved(account)).unwrap_or(false)
    }

    pub fn is_blocked(env: Env, account: Address) -> bool {
        env.storage().persistent().get(&DataKey::Blocked(account)).unwrap_or(false)
    }
}
