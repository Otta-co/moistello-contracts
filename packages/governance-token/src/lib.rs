#![cfg_attr(not(test), no_std)]
mod types;mod contract;#[cfg(test)]mod test;
use soroban_sdk::{contract,contractimpl,Address,Env,String};
#[contract]pub struct GovernanceToken;
#[contractimpl]impl GovernanceToken{
    pub fn init(env:Env,admin:Address,name:String,symbol:String,decimals:u32){contract::init(&env,&admin,name,symbol,decimals);}
    pub fn mint(env:Env,admin:Address,to:Address,amount:i128)->Result<(),types::TokenError>{contract::mint(&env,&admin,&to,amount)}
    pub fn transfer(env:Env,from:Address,to:Address,amount:i128)->Result<(),types::TokenError>{contract::transfer(&env,&from,&to,amount)}
    pub fn burn(env:Env,holder:Address,amount:i128)->Result<(),types::TokenError>{contract::burn(&env,&holder,amount)}
    pub fn delegate(env:Env,from:Address,to:Address)->Result<(),types::TokenError>{contract::delegate(&env,&from,&to)}
    pub fn approve(env:Env,owner:Address,spender:Address,amount:i128,live_until_ledger:u32)->Result<(),types::TokenError>{contract::approve(&env,&owner,&spender,amount,live_until_ledger)}
    pub fn transfer_from(env:Env,spender:Address,from:Address,to:Address,amount:i128)->Result<(),types::TokenError>{contract::transfer_from(&env,&spender,&from,&to,amount)}
    pub fn balance(env:Env,addr:Address)->i128{contract::balance(&env,&addr)}
    pub fn allowance(env:Env,owner:Address,spender:Address)->i128{contract::allowance(&env,&owner,&spender)}
    pub fn get_votes(env:Env,addr:Address)->i128{contract::get_votes(&env,&addr)}
    pub fn decimals(env:Env)->u32{contract::decimals(&env)}
    pub fn name(env:Env)->String{contract::name(&env)}
    pub fn symbol(env:Env)->String{contract::symbol(&env)}
    pub fn pause(env:Env,admin:Address)->Result<(),types::TokenError>{contract::pause(&env,&admin)}
    pub fn unpause(env:Env,admin:Address)->Result<(),types::TokenError>{contract::unpause(&env,&admin)}
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

