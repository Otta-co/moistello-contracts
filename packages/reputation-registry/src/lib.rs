#![cfg_attr(not(test), no_std)]
mod types;mod contract;#[cfg(test)]mod test;
use soroban_sdk::{contract,contractimpl,Address,Env};
#[contract]pub struct ReputationRegistry;
#[contractimpl]impl ReputationRegistry{
    pub fn init(env:Env,admin:Address){contract::init(&env,&admin);}
    pub fn record_activity(env:Env,user:Address,activity_type:u32,score_impact:u32)->Result<(),types::ReputationError>{contract::record(&env,&user,activity_type,score_impact)}
    pub fn get_score(env:Env,user:Address)->types::MoiScore{contract::get_score(&env,&user)}
    pub fn get_history(env:Env,user:Address)->soroban_sdk::Vec<types::Activity>{contract::get_history(&env,&user)}
    pub fn pause(env:Env,admin:Address)->Result<(),types::ReputationError>{contract::pause(&env,&admin)}
    pub fn unpause(env:Env,admin:Address)->Result<(),types::ReputationError>{contract::unpause(&env,&admin)}
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_smoke_compile() { assert!(true); }
    #[test]
    fn test_types_compile() {
        // Verify contract types compile correctly
        assert!(true);
    }
}

