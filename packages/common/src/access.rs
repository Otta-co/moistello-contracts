use soroban_sdk::{Address, Env, contracterror, symbol_short};
#[contracterror] #[derive(Debug)] pub enum AccessError { NotInitialized=1, Unauthorized=2, ContractPaused=3 }
pub fn require_admin(env: &Env) -> Result<Address, AccessError> { let admin: Address = env.storage().instance().get(&symbol_short!("admin")).ok_or(AccessError::NotInitialized)?; admin.require_auth(); Ok(admin) }
pub fn require_self_or_admin(env: &Env, addr: &Address) -> Result<(), AccessError> { let caller = env.current_contract_address(); if &caller == addr { return Ok(()); } let admin: Address = env.storage().instance().get(&symbol_short!("admin")).ok_or(AccessError::NotInitialized)?; if &admin == addr { Ok(()) } else { Err(AccessError::Unauthorized) } }
