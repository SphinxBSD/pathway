use soroban_sdk::{Address, contracttype};

#[derive(Clone)]
#[contracttype]

pub enum DataKey {
    Admin,
    Token,
    ContractBalance,
    User(Address)
}