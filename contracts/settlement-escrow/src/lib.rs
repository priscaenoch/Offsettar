#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol};

#[contracttype]
pub enum DataKey {
    Escrow(Symbol),
}

#[contracttype]
#[derive(Clone)]
pub struct EscrowRecord {
    pub sender: Address,
    pub recipient: Address,
    pub amount: i128,
    pub released: bool,
}

#[contract]
pub struct SettlementEscrow;

#[contractimpl]
impl SettlementEscrow {
    /// Lock funds in escrow for a settlement.
    pub fn lock(env: Env, id: Symbol, sender: Address, recipient: Address, amount: i128) {
        sender.require_auth();
        let record = EscrowRecord { sender, recipient, amount, released: false };
        env.storage().persistent().set(&DataKey::Escrow(id), &record);
    }

    /// Release escrowed funds to the recipient.
    pub fn release(env: Env, id: Symbol) {
        let mut record: EscrowRecord = env
            .storage()
            .persistent()
            .get(&DataKey::Escrow(id.clone()))
            .expect("escrow not found");
        record.sender.require_auth();
        record.released = true;
        env.storage().persistent().set(&DataKey::Escrow(id), &record);
    }

    /// Query an escrow record.
    pub fn get(env: Env, id: Symbol) -> EscrowRecord {
        env.storage()
            .persistent()
            .get(&DataKey::Escrow(id))
            .expect("escrow not found")
    }
}
