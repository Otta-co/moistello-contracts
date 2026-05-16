#![cfg_attr(not(test), no_std)]

mod types;
mod errors;
mod events;
mod storage;
mod contract;

pub use types::*;
pub use errors::*;

use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, Vec};

#[contract]
pub struct Governance;

#[contractimpl]
impl Governance {
    pub fn init(env: Env, admin: Address) -> Result<(), GovernanceError> {
        contract::init(&env, &admin)
    }

    pub fn create_proposal(
        env: Env,
        proposer: Address,
        deposit_amount: i128,
        action: ProposalAction,
        description: BytesN<32>,
    ) -> Result<u64, GovernanceError> {
        contract::create_proposal(&env, &proposer, deposit_amount, &action, &description)
    }

    pub fn cast_vote(
        env: Env,
        voter: Address,
        proposal_id: u64,
        vote_type: VoteType,
        vote_power: i128,
    ) -> Result<(), GovernanceError> {
        contract::cast_vote(&env, &voter, proposal_id, &vote_type, vote_power)
    }

    pub fn finalize_proposal(env: Env, proposal_id: u64) -> Result<ProposalStatus, GovernanceError> {
        contract::finalize_proposal(&env, proposal_id)
    }

    pub fn execute_proposal(env: Env, proposal_id: u64) -> Result<(), GovernanceError> {
        contract::execute_proposal(&env, proposal_id)
    }

    pub fn cancel_proposal(env: Env, proposer: Address, proposal_id: u64) -> Result<(), GovernanceError> {
        contract::cancel_proposal(&env, &proposer, proposal_id)
    }

    pub fn update_config(env: Env, admin: Address, config: GovernanceConfig) -> Result<(), GovernanceError> {
        contract::update_config(&env, &admin, &config)
    }

    pub fn pause(env: Env, admin: Address) -> Result<(), GovernanceError> {
        contract::pause(&env, &admin)
    }

    pub fn unpause(env: Env, admin: Address) -> Result<(), GovernanceError> {
        contract::unpause(&env, &admin)
    }

    pub fn get_proposal(env: Env, id: u64) -> Option<Proposal> {
        contract::get_proposal(&env, id)
    }

    pub fn get_proposals(env: Env) -> Vec<Proposal> {
        contract::get_proposals(&env)
    }

    pub fn get_votes_for_proposal(env: Env, proposal_id: u64) -> Vec<VoteRecord> {
        contract::get_votes_for_proposal(&env, proposal_id)
    }
}
