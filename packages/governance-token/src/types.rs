use soroban_sdk::{contracttype,contracterror,contractevent,Address,String};
#[contracttype]#[derive(Clone,Debug)]pub struct TokenMetadata{pub name:String,pub symbol:String,pub decimals:u32}
#[contracttype]#[derive(Clone,Debug)]pub struct AllowanceEntry{pub owner:Address,pub spender:Address,pub amount:i128,pub live_until_ledger:u32}
#[contracttype]#[derive(Clone)]pub enum DataKey{Admin,Metadata,TotalSupply}
#[contracterror]#[derive(Debug,Clone,PartialEq,Eq)]pub enum TokenError{NotInitialized=1,Unauthorized=2,ContractPaused=3,InsufficientBalance=4,InsufficientAllowance=5,InvalidAmount=6,AllowanceExpired=7,SelfTransfer=8,SameDelegate=9}
#[contractevent]#[derive(Clone,Debug)]pub struct TokensMinted{pub to:Address,pub amount:i128}
#[contractevent]#[derive(Clone,Debug)]pub struct TokensTransferred{pub from:Address,pub to:Address,pub amount:i128}
#[contractevent]#[derive(Clone,Debug)]pub struct TokensBurned{pub holder:Address,pub amount:i128}
#[contractevent]#[derive(Clone,Debug)]pub struct VotesDelegated{pub delegator:Address,pub delegate_to:Address}
