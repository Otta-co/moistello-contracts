use soroban_sdk::{Address, Env, contracterror, symbol_short};
#[contracterror] #[derive(Debug)] pub enum UpgradeError { NotAuthorized=1 }
pub fn get_implementation(env: &Env) -> Option<Address> { env.storage().instance().get(&symbol_short!("impl")) }
pub fn set_implementation(env: &Env, admin: &Address, new_impl: &Address) -> Result<(), UpgradeError> { admin.require_auth(); let k = symbol_short!("impl"); env.storage().instance().set(&k, new_impl); /* event: upgraded */ Ok(()) }
