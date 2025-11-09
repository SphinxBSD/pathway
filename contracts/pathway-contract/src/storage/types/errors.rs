use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    AdminTokenConflict = 0,
    AdminNotFound = 1,
    UserNotFound = 2,
    
}