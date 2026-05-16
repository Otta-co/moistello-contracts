use soroban_sdk::contracterror;

#[contracterror]
#[derive(Debug)]
pub enum GovernanceError {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    Unauthorized = 3,
    ContractPaused = 4,
    InvalidDeposit = 5,
    InsufficientDeposit = 6,
    ProposalNotActive = 7,
    AlreadyVoted = 8,
    QuorumNotMet = 9,
    ThresholdNotMet = 10,
    TimelockNotExpired = 11,
    ExecutionFailed = 12,
    NotProposer = 13,
    VotingEnded = 14,
    ProposalNotFound = 15,
    InvalidVoteType = 16,
    ArithmeticOverflow = 17,
    InvalidVotePower = 18,
    ZeroVotePower = 19,
}
