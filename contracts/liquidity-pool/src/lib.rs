#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[contracttype]
pub enum DataKey {
    Balance(Address),
    TotalLiquidity,
}

#[contract]
pub struct LiquidityPool;

#[contractimpl]
impl LiquidityPool {
    /// Deposit liquidity into the pool.
    pub fn deposit(env: Env, provider: Address, amount: i128) {
        provider.require_auth();
        let current: i128 = env.storage().persistent().get(&DataKey::Balance(provider.clone())).unwrap_or(0);
        env.storage().persistent().set(&DataKey::Balance(provider), &(current + amount));
        let total: i128 = env.storage().persistent().get(&DataKey::TotalLiquidity).unwrap_or(0);
        env.storage().persistent().set(&DataKey::TotalLiquidity, &(total + amount));
    }

    /// Withdraw liquidity from the pool.
    pub fn withdraw(env: Env, provider: Address, amount: i128) {
        provider.require_auth();
        let current: i128 = env.storage().persistent().get(&DataKey::Balance(provider.clone())).unwrap_or(0);
        assert!(current >= amount, "insufficient balance");
        env.storage().persistent().set(&DataKey::Balance(provider), &(current - amount));
        let total: i128 = env.storage().persistent().get(&DataKey::TotalLiquidity).unwrap_or(0);
        env.storage().persistent().set(&DataKey::TotalLiquidity, &(total - amount));
    }

    pub fn balance(env: Env, provider: Address) -> i128 {
        env.storage().persistent().get(&DataKey::Balance(provider)).unwrap_or(0)
    }

    pub fn total_liquidity(env: Env) -> i128 {
        env.storage().persistent().get(&DataKey::TotalLiquidity).unwrap_or(0)
    }
}
