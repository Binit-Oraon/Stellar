#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    Env, Address, String
};

#[contract]
pub struct NFTCertificateContract;

// Certificate structure
#[contracttype]
#[derive(Clone)]
pub struct Certificate {
    pub id: u64,
    pub issuer: Address,
    pub recipient: Address,
    pub title: String,
    pub metadata: String,
}

// Storage keys
#[contracttype]
pub enum DataKey {
    Counter,
    Certificate(u64),
}

#[contractimpl]
impl NFTCertificateContract {

    // Mint a new NFT certificate
    pub fn mint(
        env: Env,
        recipient: Address,
        title: String,
        metadata: String,
    ) -> u64 {
        issuer.require_auth();

        // Get and increment counter
        let mut count: u64 = env
            .storage()
            .instance()
            .get(&DataKey::Counter)
            .unwrap_or(0);

        count += 1;

        let cert = Certificate {
            id: count,
            issuer: issuer.clone(),
            recipient,
            title,
            metadata,
        };

        // Store certificate
        env.storage()
            .instance()
            .set(&DataKey::Certificate(count), &cert);

        // Update counter
        env.storage()
            .instance()
            .set(&DataKey::Counter, &count);

        count
    }

    // Retrieve certificate by ID
    pub fn get(env: Env, id: u64) -> Certificate {
        env.storage()
            .instance()
            .get(&DataKey::Certificate(id))
            .unwrap()
    }
}