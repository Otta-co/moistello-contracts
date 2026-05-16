use soroban_sdk::{contracttype, Env, Address, Vec};
use crate::types::{Proposal, VoteRecord, GovernanceConfig};
use crate::errors::GovernanceError;

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Config,
    ProposalCount,
    Proposal(u64),
    Votes(u64),
    Paused,
}

pub fn get_admin(env: &Env) -> Option<Address> {
    env.storage().instance().get(&DataKey::Admin)
}

pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&DataKey::Admin, admin);
}

pub fn get_config(env: &Env) -> GovernanceConfig {
    env.storage().instance().get(&DataKey::Config)
        .unwrap_or(GovernanceConfig {
            proposal_deposit: 1000_0000000,
            voting_period_seconds: 604800,
            timelock_seconds: 172800,
            quorum_bps: 2000,
            pass_threshold_bps: 5000,
            min_proposal_deposit: 100_0000000,
        })
}

pub fn set_config(env: &Env, config: &GovernanceConfig) {
    env.storage().instance().set(&DataKey::Config, config);
}

pub fn get_proposal_count(env: &Env) -> u64 {
    env.storage().instance().get(&DataKey::ProposalCount).unwrap_or(0u64)
}

pub fn increment_proposal_count(env: &Env) -> Result<u64, GovernanceError> {
    let count = get_proposal_count(env).checked_add(1).ok_or(GovernanceError::ArithmeticOverflow)?;
    env.storage().instance().set(&DataKey::ProposalCount, &count);
    Ok(count)
}

pub fn get_proposal(env: &Env, id: u64) -> Option<Proposal> {
    env.storage().persistent().get(&DataKey::Proposal(id))
}

pub fn save_proposal(env: &Env, proposal: &Proposal) {
    env.storage().persistent().set(&DataKey::Proposal(proposal.id), proposal);
}

pub fn get_votes(env: &Env, proposal_id: u64) -> Vec<VoteRecord> {
    env.storage().persistent().get(&DataKey::Votes(proposal_id))
        .unwrap_or_else(|| Vec::new(env))
}

pub fn save_votes(env: &Env, proposal_id: u64, votes: &Vec<VoteRecord>) {
    env.storage().persistent().set(&DataKey::Votes(proposal_id), votes);
}

pub fn is_paused(env: &Env) -> bool {
    env.storage().instance().get(&DataKey::Paused).unwrap_or(false)
}

pub fn set_paused(env: &Env, paused: bool) {
    env.storage().instance().set(&DataKey::Paused, &paused);
}
