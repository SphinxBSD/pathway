use soroban_sdk::{contractevent, Address, Env};

/// Define the event using the `contractevent` macro
#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContractInitialized {
    /// Marked as topic — will appear in the indexed part of the event
    #[topic]
    pub admin: Address,
    /// Also a topic or can be left as data if preferred
    #[topic]
    pub token: Address,
}

/// Emits the contract initialization event
pub(crate) fn contract_initialized(env: &Env, admin: Address, token: Address) {
    ContractInitialized { admin, token }.publish(env);
}
