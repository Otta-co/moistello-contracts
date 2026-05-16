use soroban_sdk::{Address, Env, Vec, BytesN};
use crate::types::*;
use crate::errors::*;
use crate::storage;
use crate::events::*;

const MAX_VOTE_POWER: i128 = 1_000_000_0000000; // 1M MOI in stroops

pub fn init(env: &Env, admin: &Address) -> Result<(), GovernanceError> {
    admin.require_auth();
    if storage::get_admin(env).is_some() {
        return Err(GovernanceError::AlreadyInitialized);
    }
    storage::set_admin(env, admin);
    storage::set_paused(env, false);
    Ok(())
}

fn require_not_paused(env: &Env) -> Result<(), GovernanceError> {
    if storage::is_paused(env) {
        return Err(GovernanceError::ContractPaused);
    }
    Ok(())
}

pub fn create_proposal(
    env: &Env,
    proposer: &Address,
    deposit_amount: i128,
    action: &ProposalAction,
    description: &BytesN<32>,
) -> Result<u64, GovernanceError> {
    require_not_paused(env)?;
    proposer.require_auth();

    let config = storage::get_config(env);

    if deposit_amount < config.min_proposal_deposit {
        return Err(GovernanceError::InsufficientDeposit);
    }

    let now = env.ledger().timestamp();
    let id = storage::increment_proposal_count(env)?;

    // TODO: Collect deposit via env.invoke_contract() with token transfer to governance contract
    let proposal = Proposal {
        id,
        proposer: proposer.clone(),
        deposit_amount,
        action: action.clone(),
        description: description.clone(),
        status: ProposalStatus::Active,
        created_at: now,
        voting_ends_at: now.checked_add(config.voting_period_seconds).ok_or(GovernanceError::ArithmeticOverflow)?,
        timelock_ends_at: now.checked_add(config.voting_period_seconds).and_then(|v| v.checked_add(config.timelock_seconds)).ok_or(GovernanceError::ArithmeticOverflow)?,
        votes_for: 0,
        votes_against: 0,
        votes_abstain: 0,
    };

    storage::save_proposal(env, &proposal);

    let votes: Vec<VoteRecord> = Vec::new(env);
    storage::save_votes(env, id, &votes);

    ProposalCreated {
        proposal_id: id,
        proposer: proposer.clone(),
        action: action.clone(),
    }.publish(env);

    Ok(id)
}

pub fn cast_vote(
    env: &Env,
    voter: &Address,
    proposal_id: u64,
    vote_type: &VoteType,
    vote_power: i128,
) -> Result<(), GovernanceError> {
    require_not_paused(env)?;
    voter.require_auth();

    // TODO: Replace with governance_token::get_votes(voter) for delegated vote power
    if vote_power <= 0 {
        return Err(GovernanceError::ZeroVotePower);
    }
    if vote_power > MAX_VOTE_POWER {
        return Err(GovernanceError::InvalidVotePower);
    }

    let mut proposal = storage::get_proposal(env, proposal_id)
        .ok_or(GovernanceError::ProposalNotFound)?;

    if proposal.status != ProposalStatus::Active {
        return Err(GovernanceError::ProposalNotActive);
    }

    if env.ledger().timestamp() > proposal.voting_ends_at {
        return Err(GovernanceError::VotingEnded);
    }

    let mut votes = storage::get_votes(env, proposal_id);
    for i in 0..votes.len() {
        let v = votes.get(i).ok_or(GovernanceError::NotInitialized)?;
        if v.voter == *voter {
            return Err(GovernanceError::AlreadyVoted);
        }
    }

    let record = VoteRecord {
        voter: voter.clone(),
        vote: vote_type.clone(),
        vote_power,
        timestamp: env.ledger().timestamp(),
    };
    votes.push_back(record);
    storage::save_votes(env, proposal_id, &votes);

    match vote_type {
        VoteType::For => proposal.votes_for = proposal.votes_for.checked_add(vote_power).ok_or(GovernanceError::ArithmeticOverflow)?,
        VoteType::Against => proposal.votes_against = proposal.votes_against.checked_add(vote_power).ok_or(GovernanceError::ArithmeticOverflow)?,
        VoteType::Abstain => proposal.votes_abstain = proposal.votes_abstain.checked_add(vote_power).ok_or(GovernanceError::ArithmeticOverflow)?,
    }

    storage::save_proposal(env, &proposal);

    VoteCast {
        proposal_id,
        voter: voter.clone(),
        vote: vote_type.clone(),
        power: vote_power,
    }.publish(env);

    Ok(())
}

/// Anyone can call this to finalize a proposal whose voting period has ended.
pub fn finalize_proposal(env: &Env, proposal_id: u64) -> Result<ProposalStatus, GovernanceError> {
    require_not_paused(env)?;

    let mut proposal = storage::get_proposal(env, proposal_id)
        .ok_or(GovernanceError::ProposalNotFound)?;

    if proposal.status != ProposalStatus::Active {
        return Err(GovernanceError::ProposalNotActive);
    }

    if env.ledger().timestamp() < proposal.voting_ends_at {
        return Err(GovernanceError::VotingEnded);
    }

    let config = storage::get_config(env);
    let total_votes = proposal.votes_for.checked_add(proposal.votes_against).and_then(|v| v.checked_add(proposal.votes_abstain)).ok_or(GovernanceError::ArithmeticOverflow)?;

    if total_votes == 0 {
        proposal.status = ProposalStatus::Failed;
        storage::save_proposal(env, &proposal);
        return Ok(ProposalStatus::Failed);
    }

    let for_pct = proposal.votes_for.checked_mul(10000).ok_or(GovernanceError::ArithmeticOverflow)?
        / total_votes;

    if for_pct >= config.pass_threshold_bps as i128 {
        proposal.status = ProposalStatus::Passed;
    } else {
        proposal.status = ProposalStatus::Failed;
    }

    storage::save_proposal(env, &proposal);

    // TODO: Forfeit deposit on failure via env.invoke_contract() for governance treasury

    Ok(proposal.status.clone())
}

/// Anyone can call this to execute a passed proposal after its timelock has expired.
pub fn execute_proposal(env: &Env, proposal_id: u64) -> Result<(), GovernanceError> {
    require_not_paused(env)?;

    let mut proposal = storage::get_proposal(env, proposal_id)
        .ok_or(GovernanceError::ProposalNotFound)?;

    if proposal.status != ProposalStatus::Passed {
        return Err(GovernanceError::ProposalNotActive);
    }

    if env.ledger().timestamp() < proposal.timelock_ends_at {
        return Err(GovernanceError::TimelockNotExpired);
    }

    proposal.status = ProposalStatus::Executed;
    storage::save_proposal(env, &proposal);

    // TODO: Refund deposit via env.invoke_contract() with token transfer back to proposer

    ProposalExecuted {
        proposal_id,
        target: proposal.action.target_contract.clone(),
        method: proposal.action.method.clone(),
    }.publish(env);

    Ok(())
}

pub fn cancel_proposal(env: &Env, proposer: &Address, proposal_id: u64) -> Result<(), GovernanceError> {
    require_not_paused(env)?;
    proposer.require_auth();

    let mut proposal = storage::get_proposal(env, proposal_id)
        .ok_or(GovernanceError::ProposalNotFound)?;

    if proposal.proposer != *proposer {
        return Err(GovernanceError::NotProposer);
    }

    if proposal.status != ProposalStatus::Active {
        return Err(GovernanceError::ProposalNotActive);
    }

    proposal.status = ProposalStatus::Cancelled;
    storage::save_proposal(env, &proposal);

    // TODO: Refund deposit on cancel via env.invoke_contract() with token transfer back to proposer

    ProposalCancelled { proposal_id }.publish(env);

    Ok(())
}

pub fn update_config(env: &Env, admin: &Address, new_config: &GovernanceConfig) -> Result<(), GovernanceError> {
    require_not_paused(env)?;
    admin.require_auth();
    let stored_admin = storage::get_admin(env).ok_or(GovernanceError::NotInitialized)?;
    if admin != &stored_admin {
        return Err(GovernanceError::Unauthorized);
    }
    storage::set_config(env, new_config);

    ConfigUpdated {
        config: new_config.clone(),
    }.publish(env);

    Ok(())
}

pub fn pause(env: &Env, admin: &Address) -> Result<(), GovernanceError> {
    admin.require_auth();
    let stored_admin = storage::get_admin(env).ok_or(GovernanceError::NotInitialized)?;
    if admin != &stored_admin { return Err(GovernanceError::Unauthorized); }
    storage::set_paused(env, true);
    Ok(())
}

pub fn unpause(env: &Env, admin: &Address) -> Result<(), GovernanceError> {
    admin.require_auth();
    let stored_admin = storage::get_admin(env).ok_or(GovernanceError::NotInitialized)?;
    if admin != &stored_admin { return Err(GovernanceError::Unauthorized); }
    storage::set_paused(env, false);
    Ok(())
}

pub fn get_proposal(env: &Env, id: u64) -> Option<Proposal> {
    storage::get_proposal(env, id)
}

pub fn get_proposals(env: &Env) -> Vec<Proposal> {
    let count = storage::get_proposal_count(env);
    let mut result = Vec::new(env);
    for i in 1..=count {
        if let Some(p) = storage::get_proposal(env, i) {
            result.push_back(p);
        }
    }
    result
}

pub fn get_votes_for_proposal(env: &Env, proposal_id: u64) -> Vec<VoteRecord> {
    storage::get_votes(env, proposal_id)
}
