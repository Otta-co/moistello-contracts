use soroban_sdk::{contractevent, Address, Symbol};
use crate::types::{ProposalAction, VoteType, GovernanceConfig};

#[contractevent]
#[derive(Clone, Debug)]
pub struct ProposalCreated {
    pub proposal_id: u64,
    pub proposer: Address,
    pub action: ProposalAction,
}

#[contractevent]
#[derive(Clone, Debug)]
pub struct VoteCast {
    pub proposal_id: u64,
    pub voter: Address,
    pub vote: VoteType,
    pub power: i128,
}

#[contractevent]
#[derive(Clone, Debug)]
pub struct ProposalExecuted {
    pub proposal_id: u64,
    pub target: Address,
    pub method: Symbol,
}

#[contractevent]
#[derive(Clone, Debug)]
pub struct ProposalCancelled {
    pub proposal_id: u64,
}

#[contractevent]
#[derive(Clone, Debug)]
pub struct ConfigUpdated {
    pub config: GovernanceConfig,
}
