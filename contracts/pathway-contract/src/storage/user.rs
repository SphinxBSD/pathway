use soroban_sdk::{Address, Env};

use crate::storage::{structs::user::User, types::{errors::Error, storage::DataKey}};


pub(crate) fn has_user(env: &Env, user_address: &Address) -> bool {
    env.storage().instance().has(&DataKey::User(user_address.clone()))
}

pub(crate) fn read_user(env: &Env, user_address: &Address) -> Result<User, Error> {
    env.storage().instance().get(&DataKey::User(user_address.clone())).ok_or(Error::UserNotFound)
}

pub(crate) fn write_user(env: &Env, user_address: &Address, user: &User) {
    env.storage().instance().set(&DataKey::User(user_address.clone()), user);
}

pub(crate) fn remove_user(env: &Env, user_address: &Address) {
    env.storage().instance().remove(&DataKey::User(user_address.clone()));
}