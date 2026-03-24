#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Property {
    pub owner: Address,
    pub physical_address: String,
    pub value: u64,
}

#[contract]
pub struct PropertyRegistryContract;

#[contractimpl]
impl PropertyRegistryContract {
    
    /// Registers a new property on the blockchain.
    pub fn register(env: Env, property_id: u32, owner: Address, physical_address: String, value: u64) {
        // Ensure the transaction is signed by the prospective owner
        owner.require_auth();
        
        // Prevent overwriting an existing property
        if env.storage().persistent().has(&property_id) {
            panic!("Error: Property ID already registered");
        }

        // Create the property struct
        let property = Property {
            owner: owner.clone(),
            physical_address,
            value,
        };

        // Save to the ledger using persistent storage
        env.storage().persistent().set(&property_id, &property);
    }

    /// Transfers ownership of an existing property to a new address.
    pub fn transfer(env: Env, property_id: u32, from: Address, to: Address) {
        // Ensure the transaction is signed by the current owner
        from.require_auth();

        // Retrieve the property from storage
        let mut property: Property = env.storage().persistent().get(&property_id).expect("Error: Property does not exist");
        
        // Verify that the person attempting to transfer actually owns it
        if property.owner != from {
            panic!("Error: Only the current owner can transfer this property");
        }

        // Update ownership and save back to the ledger
        property.owner = to;
        env.storage().persistent().set(&property_id, &property);
    }

    /// Retrieves the details of a registered property.
    pub fn get_property(env: Env, property_id: u32) -> Property {
        env.storage().persistent().get(&property_id).expect("Error: Property does not exist")
    }
}