use soroban_sdk::{Address, Env};

use crate::storage::types::errors::Error;

pub trait PathwayContractTrait {
    fn __constructor(env: &Env, admin: Address, token: Address) -> Result<(), Error>;

}