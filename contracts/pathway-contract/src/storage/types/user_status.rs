use soroban_sdk::contracttype;

#[derive(Clone)]
#[contracttype]

pub enum UserStatus {
    Active,
    Blocked,
}