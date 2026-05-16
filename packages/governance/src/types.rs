use soroban_sdk::{contracttype, Address, Symbol, Vec, BytesN};

#[contracttype]
#[derive(Clone, Debug)]
pub struct Proposal {
    pub id: u64,
    pub proposer: Address,
    pub deposit_amount: i128,
    pub action: ProposalAction,
    pub description: BytesN<32>,
    pub status: ProposalStatus,
    pub created_at: u64,
    pub voting_ends_at: u64,
    pub timelock_ends_at: u64,
    pub votes_for: i128,
    pub votes_against: i128,
    pub votes_abstain: i128,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct ProposalAction {
    pub target_contract: Address,
    pub method: Symbol,
    pub args: Vec<i128>,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct GovernanceConfig {
    pub proposal_deposit: i128,
    pub voting_period_seconds: u64,
    pub timelock_seconds: u64,
    pub quorum_bps: u32,
    pub pass_threshold_bps: u32,
    pub min_proposal_deposit: i128,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum ProposalStatus {
    Active = 0,
    Passed = 1,
    Failed = 2,
    Executed = 3,
    Cancelled = 4,
}

#[contracttype]
#[derive(Clone, Debug)]
pub enum VoteType {
    For = 0,
    Against = 1,
    Abstain = 2,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct VoteRecord {
    pub voter: Address,
    pub vote: VoteType,
    pub vote_power: i128,
    pub timestamp: u64,
}

pub const STATUS_ACTIVE: u32 = 0;
pub const STATUS_PASSED: u32 = 1;
pub const STATUS_FAILED: u32 = 2;
pub const STATUS_EXECUTED: u32 = 3;
pub const STATUS_CANCELLED: u32 = 4;

pub const VOTE_FOR: u32 = 0;
pub const VOTE_AGAINST: u32 = 1;
pub const VOTE_ABSTAIN: u32 = 2;
