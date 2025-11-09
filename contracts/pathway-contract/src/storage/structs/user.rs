use soroban_sdk::contracttype;

use crate::storage::types::user_status::UserStatus;

#[derive(Clone)]
#[contracttype]

pub struct User {
    pub user_status: UserStatus,
    pub balance_coin: i128
}