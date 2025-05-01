#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, Address, String, Vec};

#[contracttype]
#[derive(Clone)]
pub struct NFT {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub owner: Address,
}

#[contracttype]
pub enum StorageKey {
    NFTById(u64),
    NFTsOf(Address),
    NFTCount,
}

#[contract]
pub struct NFTTradingSimulator;

#[contractimpl]
impl NFTTradingSimulator {
    // Mint a new NFT and assign it to an owner
    pub fn mint(env: Env, owner: Address, name: String, description: String) -> u64 {
        let mut count: u64 = env.storage().instance().get(&StorageKey::NFTCount).unwrap_or(0);
        count += 1;

        let nft = NFT {
            id: count,
            name,
            description,
            owner: owner.clone(),
        };

        // Store NFT
        env.storage().instance().set(&StorageKey::NFTById(count), &nft);

        // Add NFT to owner's list
        let mut collection: Vec<u64> = env.storage().instance().get(&StorageKey::NFTsOf(owner.clone())).unwrap_or(Vec::new(&env));
        collection.push_back(count);
        env.storage().instance().set(&StorageKey::NFTsOf(owner), &collection);

        env.storage().instance().set(&StorageKey::NFTCount, &count);
        count
    }



    // View NFTs owned by an address
    pub fn get_owned_nfts(env: Env, owner: Address) -> Vec<u64> {
        env.storage().instance().get(&StorageKey::NFTsOf(owner)).unwrap_or(Vec::new(&env))
    }
}
