#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol, Address, Map};

#[contract]
pub struct LoyaltyProgram;

#[contractimpl]
impl LoyaltyProgram {
    // Initialize storage for an admin
    pub fn init(env: Env, admin: Address) {
        admin.require_auth();
        env.storage().instance().set(&symbol_short!("ADMIN"), &admin);
    }

    // Add points to a user
    pub fn add_points(env: Env, to: Address, amount: i128) {
        let admin: Address = env
            .storage()
            .instance()
            .get(&symbol_short!("ADMIN"))
            .unwrap();

        admin.require_auth();

        let mut points: i128 = env
            .storage()
            .persistent()
            .get(&to)
            .unwrap_or(0);

        points += amount;

        env.storage().persistent().set(&to, &points);
    }

    // Redeem points
    pub fn redeem_points(env: Env, user: Address, amount: i128) {
        user.require_auth();

        let mut points: i128 = env
            .storage()
            .persistent()
            .get(&user)
            .unwrap_or(0);

        if points < amount {
            panic!("Not enough points");
        }

        points -= amount;

        env.storage().persistent().set(&user, &points);
    }

    // Check balance
    pub fn get_points(env: Env, user: Address) -> i128 {
        env.storage().persistent().get(&user).unwrap_or(0)
    }
}