use soroban_sdk::{Address, Env};

use crate::storage::types::storage::DataKey;


pub(crate) fn has_user(env: &Env, user: &Address) -> bool {
    env.storage().instance().has(&DataKey::User(user.clone()))
}