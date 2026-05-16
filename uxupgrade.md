# Moistello — Governance & Reputation UX Upgrade Plan

## Overview

Transform Moistello from a basic savings circle platform into a fully-governed, reputation-driven economy where every action compounds into tangible user benefit. All features are on-chain, verifiable, and self-reinforcing.

---

## 1. MoiScore → Collateral → Circle Access (The Core Loop)

### Current State
- `min_moi_score` exists as a gate but isn't dynamically tied to collateral or circle size
- Collateral is a flat config field — same for all members regardless of reputation
- Circle size limits are hardcoded, not reputation-scaled

### Target State
Every user action feeds into a dynamic, tiered system:

```
On-time Payment  ──→  +10 points  ──→  MoiScore increases
Streak bonus     ──→  +5/level   ──→  Tier upgrades
Circle complete  ──→  +100 pts   ──→  Lower collateral
Default          ──→  -200 pts   ──→  Higher collateral / restricted
```

| Tier | Score | Collateral | Max Circle Size | Max Contribution | Governance Votes |
|---|---|---|---|---|---|
| 🥉 Bronze | 0–200 | 10% | 5 members | 100 USDC | 1x |
| 🥈 Silver | 201–400 | 5% | 10 members | 500 USDC | 1x |
| 🥇 Gold | 401–600 | 3% | 20 members | 2,000 USDC | 2x |
| 💎 Platinum | 601–800 | 1% | 50 members | 10,000 USDC | 3x |
| 👑 Diamond | 801–1000 | 0% | 100 members | 50,000 USDC | 5x |

### Implementation — Smart Contract Changes

**File:** `reputation-registry/src/scoring.rs`

```rust
// NEW: Dynamic collateral calculation
pub fn calculate_collateral(env: &Env, member: &Address) -> u32 {
    let score = get_score(env, member);
    match get_tier(score) {
        TIER_DIAMOND  => 0,     // 0% — trusted, no collateral needed
        TIER_PLATINUM => 100,   // 1% (100 bips)
        TIER_GOLD     => 300,   // 3%
        TIER_SILVER   => 500,   // 5%
        _             => 1000,  // 10% — default for bronze/unscored
    }
}

// NEW: Dynamic circle size by tier
pub fn max_circle_size(env: &Env, member: &Address) -> u32 {
    let score = get_score(env, member);
    match get_tier(score) {
        TIER_DIAMOND  => 100,
        TIER_PLATINUM => 50,
        TIER_GOLD     => 20,
        TIER_SILVER   => 10,
        _             => 5,
    }
}

// NEW: Dynamic max contribution by tier
pub fn max_contribution(env: &Env, member: &Address) -> i128 {
    let score = get_score(env, member);
    match get_tier(score) {
        TIER_DIAMOND  => 50_000_0000000,  // 50,000 USDC
        TIER_PLATINUM => 10_000_0000000,  // 10,000 USDC
        TIER_GOLD     => 2_000_0000000,   // 2,000 USDC
        TIER_SILVER   => 500_0000000,     // 500 USDC
        _             => 100_0000000,     // 100 USDC
    }
}
```

**File:** `reputation-registry/src/scoring.rs` — UPDATED scoring algorithm

```rust
// Current weights preserved, ADD:
pub fn record_on_time_payment(env: &Env, member: &Address, circle_id: &Address) -> u32 {
    let current = get_score(env, member);
    let streak = get_streak(env, member);

    // Base: +10 per on-time payment
    let base = 10;

    // Streak bonus: +5 per consecutive payment (max +50)
    let streak_bonus = if streak <= 10 { streak * 5 } else { 50 };

    // Volume bonus: +1 per 100 USDC contributed (max +20)
    let volume_bonus = min(get_volume(env, member) / 100_0000000, 20);

    let new_score = min(current + base + streak_bonus + volume_bonus, 1000);
    increment_streak(env, member);
    set_score(env, member, new_score);
    new_score
}

pub fn record_circle_completion(env: &Env, member: &Address) -> u32 {
    let score = get_score(env, member);
    let bonus = 100; // One-time bonus for completing a full circle
    let new_score = min(score + bonus, 1000);
    increment_completions(env, member);
    set_score(env, member, new_score);
    new_score
}

pub fn record_default(env: &Env, member: &Address) -> u32 {
    let score = get_score(env, member);
    let penalty = 200;
    let new_score = if score > 200 { score - penalty } else { 0 };
    reset_streak(env, member);
    increment_defaults(env, member);
    set_score(env, member, new_score);
    new_score
}

pub fn apply_inactivity_decay(env: &Env, member: &Address) -> u32 {
    let score = get_score(env, member);
    let days_inactive = days_since_last_activity(env, member);
    // -5 per month of inactivity (30 days)
    let months = days_inactive / 30;
    let decay = min(months * 5, score as u64);
    let new_score = score - decay as u32;
    set_score(env, member, new_score);
    new_score
}
```

---

## 2. Governance System — MOI Token Voting on Protocol Upgrades

### Current State
- Governance token contract exists (mint, transfer, burn, delegate)
- Upgrade proxy exists (`common/upgrade.rs`)
- But voting is NOT wired to upgrades
- No proposal creation mechanism

### Target State
Full on-chain governance lifecycle:

```
Token Holder stakes MOI
        ↓
Creates Proposal (with timelock)
        ↓
Voting Period (7 days, quorum: 20% of supply)
        ↓
Votes counted (weighted by token balance + delegation)
        ↓
Passed? ──No──→ Staked MOI returned
        │
       Yes
        ↓
Timelock Period (48 hours — security delay)
        ↓
Executor calls upgrade proxy with new implementation
        ↓
Contract upgraded. State preserved. Proposal archived.
```

### Implementation — New Contract

**File:** `governance/src/lib.rs` (new contract in existing governance-token package)

```rust
#[contractimpl]
impl Governance {
    // ── Proposal Lifecycle ──

    /// Create a new governance proposal. Stakes MOI tokens as deposit.
    /// deposit_amount: MOI tokens locked until proposal resolves
    /// action: encoded contract call (target, method, args)
    /// description: human-readable proposal text (IPFS hash for long text)
    pub fn create_proposal(
        env: Env,
        proposer: Address,
        deposit_amount: i128,
        action: ProposalAction,
        description: BytesN<32>,
    ) -> Result<u64, GovernanceError>;

    /// Cast a vote on an active proposal
    /// vote_power = token_balance + delegated_balance
    pub fn cast_vote(
        env: Env,
        voter: Address,
        proposal_id: u64,
        vote: VoteType, // For, Against, Abstain
    ) -> Result<(), GovernanceError>;

    /// Execute a passed proposal after timelock expires
    /// Can be called by anyone — permissionless execution
    pub fn execute_proposal(
        env: Env,
        proposal_id: u64,
    ) -> Result<(), GovernanceError>;

    /// Cancel own proposal before voting starts (refunds deposit)
    pub fn cancel_proposal(
        env: Env,
        proposal_id: u64,
    ) -> Result<(), GovernanceError>;

    // ── Configuration (Governable) ──

    /// Update governance parameters (itself governable)
    pub fn update_config(
        env: Env,
        new_config: GovernanceConfig,
    ) -> Result<(), GovernanceError>;

    // ── Queries ──

    pub fn get_proposal(env: Env, id: u64) -> Proposal;
    pub fn get_proposals(env: Env, status: ProposalStatus) -> Vec<Proposal>;
    pub fn get_vote(env: Env, proposal_id: u64, voter: Address) -> VoteRecord;
    pub fn get_vote_power(env: Env, voter: Address) -> i128;
    pub fn get_config(env: Env) -> GovernanceConfig;
}
```

**File:** `governance/src/types.rs`

```rust
#[contracttype]
pub struct Proposal {
    pub id: u64,
    pub proposer: Address,
    pub deposit_amount: i128,        // MOI staked
    pub action: ProposalAction,
    pub description: BytesN<32>,
    pub status: ProposalStatus,      // Active, Passed, Failed, Executed, Cancelled
    pub created_at: u64,
    pub voting_ends_at: u64,
    pub timelock_ends_at: u64,
    pub votes_for: i128,
    pub votes_against: i128,
    pub votes_abstain: i128,
}

#[contracttype]
pub struct ProposalAction {
    pub target_contract: Address,    // Which contract to upgrade/modify
    pub method: Symbol,              // Method to call
    pub args: Vec<Val>,              // Arguments (encoded Soroban values)
}

#[contracttype]
pub struct GovernanceConfig {
    pub proposal_deposit: i128,      // MOI required to create proposal
    pub voting_period_seconds: u64,  // 604800 = 7 days
    pub timelock_seconds: u64,       // 172800 = 48 hours
    pub quorum_bps: u32,             // 2000 = 20% of total supply must vote
    pub pass_threshold_bps: u32,     // 5000 = 50%+ of votes must be For
    pub min_proposal_deposit: i128,  // Floor for deposit
}

#[contracttype]
pub enum ProposalStatus { Active, Passed, Failed, Executed, Cancelled }

#[contracttype]
pub enum VoteType { For, Against, Abstain }

#[contracttype]
pub struct VoteRecord {
    pub voter: Address,
    pub vote: VoteType,
    pub vote_power: i128,
    pub timestamp: u64,
}
```

### Upgradable Parameters (themselves governable)
- Proposal deposit amount
- Voting period length
- Timelock duration
- Quorum percentage
- Pass threshold percentage
- All MoiScore tier thresholds
- All collateral percentages
- All circle size limits
- Platform fee (currently 0.5%)

---

## 3. Circle Access — Dynamic Limits

### Current State
- `max_members` is set at circle creation — flat, static
- `min_moi_score` is a single threshold — pass/fail gate only
- No tier differentiation for circle access

### Target State
When creating or joining a circle, the organizer's tier determines:
- Maximum members allowed
- Maximum contribution per member
- Whether the circle is "verified" (Diamond/Platinum only)

**File:** `circle/src/contract.rs` — create_circle updated

```rust
pub fn create_circle(env: &Env, organizer: &Address, config: &CircleConfig) -> Result<(), CircleError> {
    // Query reputation for organizer's tier
    let tier = reputation_registry::get_tier(env, organizer);
    let score = reputation_registry::get_score(env, organizer);

    // Enforce tier-based limits
    let max_allowed = reputation_registry::max_circle_size(env, organizer);
    if config.max_members > max_allowed {
        return Err(CircleError::CircleSizeExceedsTier);
    }

    let max_amount = reputation_registry::max_contribution(env, organizer);
    if config.contribution_amount > max_amount {
        return Err(CircleError::ContributionExceedsTier);
    }

    // Auto-apply collateral based on organizer's tier
    let collateral = reputation_registry::calculate_collateral(env, organizer);
    let final_config = CircleConfig {
        collateral_percent: collateral,  // Override with tier-based value
        verified: tier >= TIER_PLATINUM, // Premium badge for top tiers
        ..config
    };

    // ... rest of create logic
}
```

---

## 4. Reputation → On-Chain History → Pool Access

### Current State
- Circle completions are recorded but not aggregated into accessible history
- No query endpoint for "completed circles" or "circle history"

### Target State
On-chain history that feeds back into access:

```rust
// reputation-registry/src/scoring.rs

pub fn get_circle_history(env: &Env, member: &Address) -> CircleHistory {
    CircleHistory {
        total_joined: get_total_joined(env, member),
        total_completed: get_total_completed(env, member),
        total_defaulted: get_total_defaulted(env, member),
        total_contributed_usdc: get_total_volume(env, member),
        avg_contribution_size: get_avg_contribution(env, member),
        current_streak: get_streak(env, member),
        best_streak: get_best_streak(env, member),
        joined_circles: get_recent_circles(env, member, 10), // Last 10
    }
}

pub fn qualifies_for_circle(env: &Env, member: &Address, circle_req: u32) -> bool {
    let score = get_score(env, member);
    let completions = get_total_completed(env, member);
    // Qualification: both score AND history matter
    score >= circle_req && completions > 0
}
```

---

## 5. User Experience — The Upward Spiral

Every action the user takes visibly improves their standing:

```
FIRST JOIN
  └─ MoiScore: 0 (Bronze) — Collateral: 10% — Max circle: 5 members

AFTER 5 ON-TIME PAYMENTS
  └─ MoiScore: 50→70 (Bronze) — Streak: 5 — Unlocked: Silver in sight

AFTER 1 COMPLETED CIRCLE  
  └─ MoiScore: 170 (Silver) — Collateral: 5% — Max circle: 10 members

AFTER 3 COMPLETED CIRCLES + 20 STREAK
  └─ MoiScore: 450 (Gold) — Collateral: 3% — Max circle: 20 members — 🔓 Governance: 2x votes

AFTER 10 COMPLETED CIRCLES + 50 STREAK
  └─ MoiScore: 750 (Platinum) — Collateral: 1% — Max circle: 50 members — 🔓 Create verified circles

AFTER 25 COMPLETED CIRCLES + 100 STREAK
  └─ MoiScore: 950 (Diamond) — Collateral: 0% — Max circle: 100 — 🔓 Governance: 5x votes — 🔓 Max contribution: $50K
```

---

## 6. Implementation Phases

### Phase A — Scoring Upgrade (2 days)
- [ ] Update `reputation-registry/src/scoring.rs` with tier-based collateral, circle size, and contribution limits
- [ ] Add `record_on_time_payment`, `record_circle_completion`, `record_default`, `apply_inactivity_decay`
- [ ] Add `get_circle_history` query
- [ ] Run full test suite

### Phase B — Governance Contract (3 days)
- [ ] Create `governance/src/lib.rs` with proposal creation, voting, execution
- [ ] Create `governance/src/types.rs` with Proposal, GovernanceConfig, ProposalAction
- [ ] Wire to existing governance-token for vote power calculation
- [ ] Wire to upgrade proxy for auto-execution
- [ ] Add proposal deposit → treasury flow

### Phase C — Circle Integration (1 day)
- [ ] Update `circle/src/contract.rs` — create_circle enforces tier limits
- [ ] Update `circle/src/contract.rs` — join checks dynamic collateral
- [ ] Update `circle/src/contract.rs` — contribution triggers reputation change

### Phase D — Frontend (2 days)
- [ ] Add "Your Journey" page showing MoiScore progression with step indicators
- [ ] Show tier benefits card (what you unlock at each level)
- [ ] Add governance voting UI to /governance route
- [ ] Show "Next Tier" progress bar on dashboard and reputation page
- [ ] Add proposal creation form with action builder
- [ ] Add proposal listing with live vote counts

### Phase E — Binding Integration + Testing (2 days)
- [ ] Generate Go bindings for new governance contract
- [ ] Integration tests: create proposal → vote → execute
- [ ] Integration tests: score progression → collateral reduction → larger circle
- [ ] End-to-end governance test on testnet

**Total: ~10 days enterprise implementation**

---

## 7. Security Considerations

| Risk | Mitigation |
|---|---|
| Whale domination of governance | Quorum + pass threshold prevent minority rule. Delegation spreads power. |
| Flash loan attacks on voting | Vote power snapshot at proposal creation time, not at vote time |
| Timelock bypass via upgrade | Timelock is immutable per proposal. Cannot be shortened once voting starts. |
| Collateral evasion via score manipulation | Score decay penalizes inactivity. Score from on-chain verifiable actions only. |
| Proposal spam | Deposit requirement + minimum MoiScore to propose |

---

## 8. Frontend UX — Tier Progression Card

```
┌─────────────────────────────────────────────────────┐
│  👑 YOUR REPUTATION                                  │
│                                                     │
│  ┌──────────────────────────────────────┐            │
│  │  MoiScore: 750 ████████████████░░  │            │
│  │  Tier: 💎 Platinum                   │            │
│  │  Next: 👑 Diamond (50 pts away)      │            │
│  └──────────────────────────────────────┘            │
│                                                     │
│  UNLOCKED BENEFITS:                                  │
│  ✅ 1% Collateral (was 3% at Gold)                  │
│  ✅ 50-Member Circles (was 20 at Gold)               │
│  ✅ $10,000 Max Contribution (was $2,000)            │
│  ✅ Verified Circle Creator Badge                     │
│  ✅ 3x Governance Vote Power                          │
│                                                     │
│  AT DIAMOND (801+):                                  │
│  🔒 0% Collateral                                    │
│  🔒 100-Member Circles                               │
│  🔒 $50,000 Max Contribution                         │
│  🔒 5x Governance Vote Power                          │
└─────────────────────────────────────────────────────┘
```
