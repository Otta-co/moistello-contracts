use soroban_sdk::{Address, Env, contracterror, symbol_short};
#[contracterror] #[derive(Debug)] pub enum PauseError { ContractPaused=1 }
pub fn is_paused(env: &Env) -> bool { env.storage().instance().get(&symbol_short!("paused")).unwrap_or(false) }
pub fn pause(env: &Env, admin: &Address) -> Result<(), PauseError> { admin.require_auth(); let k = symbol_short!("paused"); env.storage().instance().set(&k, &true); /* event: paused */ Ok(()) }
pub fn unpause(env: &Env, admin: &Address) -> Result<(), PauseError> { admin.require_auth(); let k = symbol_short!("paused"); env.storage().instance().set(&k, &false); /* event: unpaused */ Ok(()) }
pub fn when_not_paused(env: &Env) -> Result<(), PauseError> { if is_paused(env) { Err(PauseError::ContractPaused) } else { Ok(()) } }
