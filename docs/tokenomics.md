# Tokenomics

This document calibrates a realistic token design for DFPN on Solana. The numbers assume a small founding team and a 24-36 month runway before meaningful fee volume.

## Launch Assumptions

- Devnet -> testnet -> mainnet beta within 12-18 months.
- Request volume starts low and ramps after partnerships.
- Token incentives are required to bootstrap worker and model supply.

## Token Overview

- **Token**: DFPN (SPL token)
- **Primary use**: staking, rewards, protocol fees, governance weight
- **Secondary use**: fee discounts and optional priority boosts

## Supply and Allocation (Calibrated)

- **Total supply**: 1,000,000,000 DFPN (fixed)
- **Allocations**
  - 38% Network rewards (workers + model developers)
  - 20% Treasury (audits, grants, ops, insurance)
  - 18% Team and advisors (4-year vesting, 1-year cliff)
  - 12% Ecosystem growth (partnerships, integrations, data grants)
  - 7% Strategic backers (2-year lock, linear vesting)
  - 5% Liquidity and market making

Rationale: the reward pool is large enough to bootstrap supply, while treasury + ecosystem allocations sustain ops and integrations.

## Emissions Schedule (Network Rewards)

Emissions are carved from the 38% reward allocation and decline over time.

- Year 1: 12% of total supply
- Year 2: 9% of total supply
- Year 3: 6% of total supply
- Year 4: 4% of total supply
- Year 5: 3% of total supply
- Year 6: 2% of total supply
- Year 7: 1% of total supply
- Year 8: 1% of total supply

Optional tail: governance may enable up to 1% annual inflation if fees do not sustain the worker market.

## Fees

- **Base fee**: paid by requesters to create an analysis job.
- **Fee currency**: SOL by default; DFPN discount optional.
- **Fee split (baseline)**
  - 65% workers
  - 20% model developers
  - 10% treasury
  - 5% insurance pool

Fee tiers can vary by modality (video > image > audio) and SLA.

## Staking Requirements

Stakes scale with demand to reduce sybil pressure and ensure skin in the game.

- **Workers**: stake >= 30x median request fee per epoch (floor: 5,000 DFPN).
- **Model developers**: stake >= 150x median request fee per epoch (floor: 20,000 DFPN).
- **Challengers**: stake >= 5% of the disputed reward.

Example starting values (tunable):

- Worker stake: 5,000 DFPN
- Model stake: 20,000 DFPN per version
- Minimum request fee: 0.02 SOL (or DFPN equivalent)

## Slashing and Penalties

- **Invalid results**: 10% stake slash.
- **Fraud or collusion**: 25% to 50% slash and temporary ban.
- **Missed deadlines**: 1% to 3% slash and reputation decay.
- **Repeated low-quality results**: progressive slashing and retirement.

A short challenge window precedes slashing; governance can override in exceptional cases.

## Reward Calculation (Baseline)

Rewards are distributed per epoch and weighted by measured performance.

- **Accuracy**: agreement with benchmark or consensus.
- **Latency**: small bonus for timeliness.
- **Reputation**: weights results to limit sybil attacks.
- **Stake weight**: capped to avoid pay-to-win dynamics.

## Treasury and Insurance

- **Treasury** funds audits, grants, incident response, and ops.
- **Insurance** covers dispute resolution and catastrophic worker failure.

## Governance Controls

- Adjust fee splits, stake minimums, emission cadence, and slashing ranges.
- Approve new benchmark datasets and rotate hidden test sets.
- Enable or disable inflation tail if the worker market weakens.
