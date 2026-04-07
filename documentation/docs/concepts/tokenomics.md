# Token Economics

DFPN uses a native token to align incentives across all network participants. This page covers the token supply, allocation, emissions, fees, staking, slashing, and reward calculations.

---

## Token Overview

| Property | Value |
|----------|-------|
| **Token name** | DFPN |
| **Token standard** | SPL Token (Solana) |
| **Total supply** | 1,000,000,000 (1 billion) |
| **Decimals** | 9 |
| **Supply type** | Fixed (no minting beyond initial supply) |
| **Primary uses** | Staking, rewards, governance weight |
| **Secondary uses** | Fee discounts, priority boosts |

---

## Token Allocation

The total supply is allocated across six categories:

| Allocation | Percentage | Tokens | Details |
|-----------|-----------|--------|---------|
| **Network Rewards** | 38% | 380,000,000 | Workers and model developers, released over 8 years |
| **Treasury** | 20% | 200,000,000 | Audits, grants, operations, insurance |
| **Team & Advisors** | 18% | 180,000,000 | 4-year vesting with 1-year cliff |
| **Ecosystem Growth** | 12% | 120,000,000 | Partnerships, integrations, data grants |
| **Strategic Backers** | 7% | 70,000,000 | 2-year lock with linear vesting |
| **Liquidity** | 5% | 50,000,000 | Market making and exchange liquidity |

!!! info "Reward pool is the largest allocation"
    At 38%, the network reward pool is deliberately the largest allocation. It needs to be big enough to bootstrap worker and model supply during the early years when request volume is still growing.

---

## Emissions Schedule

Network rewards (the 38% allocation) are released on a declining annual schedule:

| Year | Emission Rate | Tokens Released | Cumulative |
|------|--------------|----------------|------------|
| Year 1 | 12% of total supply | 120,000,000 | 120,000,000 |
| Year 2 | 9% of total supply | 90,000,000 | 210,000,000 |
| Year 3 | 6% of total supply | 60,000,000 | 270,000,000 |
| Year 4 | 4% of total supply | 40,000,000 | 310,000,000 |
| Year 5 | 3% of total supply | 30,000,000 | 340,000,000 |
| Year 6 | 2% of total supply | 20,000,000 | 360,000,000 |
| Year 7 | 1% of total supply | 10,000,000 | 370,000,000 |
| Year 8 | 1% of total supply | 10,000,000 | 380,000,000 |

Emissions are highest in Year 1 to attract early workers and model developers when request volume is low. As fee revenue grows, the protocol becomes self-sustaining and emissions decline.

!!! note "Governance may enable tail inflation"
    If fees alone cannot sustain the worker market after Year 8, governance can vote to enable up to 1% annual inflation to maintain network security.

---

## Fee Structure

Clients pay a per-request fee in SOL when submitting media for analysis. Fees vary by modality and request parameters.

### Fee Split

Every fee is split across four recipients:

| Recipient | Share | Purpose |
|-----------|-------|---------|
| **Workers** | 65% | Compensation for running inference |
| **Model Developers** | 20% | Revenue for models used in analysis |
| **Treasury** | 10% | Protocol operations, audits, grants |
| **Insurance Pool** | 5% | Dispute resolution, catastrophic failure coverage |

### Fee Tiers by Modality

| Modality | Approximate Base Fee |
|----------|---------------------|
| Image Authenticity | ~0.002 SOL |
| Face Manipulation | ~0.003 SOL |
| AI-Generated Content | ~0.003 SOL |
| Voice Cloning | ~0.004 SOL |
| Video Authenticity | ~0.008 SOL |

Actual fees are dynamic. They depend on worker availability, request priority (standard, high, urgent), and the number of workers requested. Clients can use the SDK's `estimateFee()` method to get current pricing.

---

## Staking

Staking is required for workers and model developers. It serves as economic commitment -- participants with tokens at risk are incentivized to behave honestly.

| Role | Minimum Stake | Purpose |
|------|--------------|---------|
| **Workers** | 5,000 DFPN | Ensures workers have skin in the game; slashable for bad behavior |
| **Model Developers** | 20,000 DFPN per version | Ensures quality; slashable if model consistently underperforms |

### How Staking Works

- Tokens are locked in a program-controlled account when you register
- Staked tokens remain yours but cannot be transferred or sold while staked
- You can unstake by deregistering (subject to a cooldown period)
- Higher stakes give a modest bonus to reward calculations (capped to prevent pay-to-win)

!!! warning "Staked tokens are at risk"
    Slashing events reduce your staked balance. If your stake falls below the minimum, you are removed from the active pool until you top it up.

---

## Slashing

Slashing is the economic penalty for bad behavior. It protects the network by making dishonesty expensive.

| Offense | Slash Amount | Additional Consequences |
|---------|-------------|------------------------|
| **Invalid results** | 10% of stake | Reputation penalty |
| **Missed deadlines** | 1-3% of stake | Reputation decay, escalates with frequency |
| **Repeated failures** | Progressive | Increasing slash percentages, potential removal |
| **Fraud or collusion** | 25-50% of stake | Temporary ban from the network |

### How Slashing Works

1. A potential violation is detected (e.g., result that contradicts consensus)
2. A short challenge window opens where the worker can dispute
3. If the violation is confirmed, the slash is applied to the worker's staked balance
4. Slashed tokens go to the insurance pool
5. Governance can override slashing in exceptional cases

!!! danger "Fraud slashing is designed to be painful"
    A worker caught colluding or submitting fabricated results loses 25-50% of their stake in a single event, plus receives a temporary ban. The severity is intentional: it must be cheaper to behave honestly than to cheat.

---

## Reward Formula

Rewards are distributed at the end of each epoch. Your share is based on your performance score relative to all other workers.

### The Formula

```
epoch_reward = (your_score / total_scores) * epoch_pool * stake_weight
```

Where:

- **your_score** -- your performance score for the epoch (based on accuracy, availability, latency, and consistency)
- **total_scores** -- sum of all active workers' scores
- **epoch_pool** -- total rewards available for the epoch (from emissions + fee accumulation)
- **stake_weight** -- a capped multiplier based on your stake amount (diminishing returns above the minimum)

### Example

Suppose in a given epoch:

- Your performance score: 85
- Total of all workers' scores: 10,000
- Epoch reward pool: 500,000 DFPN
- Your stake weight: 1.1x (slightly above minimum stake)

```
epoch_reward = (85 / 10,000) * 500,000 * 1.1
             = 0.0085 * 500,000 * 1.1
             = 4,675 DFPN
```

This is in addition to the per-request fees you earn directly from each completed analysis.

### Maximizing Rewards

The most effective strategies for maximizing rewards:

1. **Run accurate models** -- accuracy is 50% of your score
2. **Maintain high uptime** -- availability is 25% of your score
3. **Use fast hardware** -- latency is 15% of your score
4. **Be consistent** -- consistency is 10% of your score
5. **Stake above the minimum** -- modest bonus from stake weight (but diminishing returns)
