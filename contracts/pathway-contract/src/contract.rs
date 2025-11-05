use soroban_sdk::{contract, contractimpl, Address, Env};

use crate::{events, interfaces::contract::PathwayContractTrait, storage::types::errors::Error};

#[contract]
pub struct PathwayContract;

#[contractimpl]
impl PathwayContractTrait for PathwayContract {
    fn __constructor(env: &Env, admin: Address, token: Address) -> Result<(), Error> {
        if admin == token {
            return Err(Error::AdminTokenConflict);
        }

        events::contract::contract_initialized(env, admin, token);

        Ok(())
    }
    
}