#![no_std]
use soroban_sdk::{contract, contractimpl, Env};

/// Fee is expressed in basis points (1 bp = 0.01%)
const DEFAULT_FEE_BPS: i128 = 30; // 0.30%

#[contract]
pub struct FeeEngine;

#[contractimpl]
impl FeeEngine {
    /// Calculate the fee for a given amount.
    pub fn calculate_fee(_env: Env, amount: i128) -> i128 {
        amount * DEFAULT_FEE_BPS / 10_000
    }

    /// Return the amount after fee deduction.
    pub fn amount_after_fee(env: Env, amount: i128) -> i128 {
        amount - Self::calculate_fee(env, amount)
    }
}
