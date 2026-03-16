# Operational Model

This document describes how models and workers join the network, how quality is measured, and how incentives are paid.

## Model Lifecycle

1. **Submission**: Developer registers a model with metadata and a stake.
2. **Benchmarking**: Model is evaluated on a held-out dataset by the evaluation harness.
3. **Activation**: Models meeting minimum thresholds become active for requests.
4. **Versioning**: New versions can be submitted and evaluated independently.
5. **Retirement**: Models with sustained poor performance are retired.

## Worker Lifecycle

1. **Registration**: Workers stake tokens and declare supported modalities.
2. **Task Assignment**: Workers pull jobs from the analysis marketplace.
3. **Submission**: Results are committed then revealed to prevent copying.
4. **Scoring**: Performance updates reputation each epoch.
5. **Slashing**: Fraud, missed deadlines, or invalid results reduce stake.

## Scoring Policy (Baseline)

- **Consensus**: Use multiple workers per request; compare to consensus or benchmarks.
- **Accuracy**: Score by agreement with ground truth or majority vote.
- **Latency**: Modest bonus for faster completion.
- **Reputation**: Weight results by worker reputation to resist sybil attacks.

## Incentives

- **Request Fees**: Paid by clients to fund inference and rewards.
- **Reward Pool**: Splits between workers and model developers.
- **Staking**: Required for both workers and model developers.
- **Penalties**: Slashing for dishonesty or repeated poor results.

## Governance and Policy Updates

- Parameter changes via SPL Governance.
- Community can propose: new benchmark datasets, scoring updates, or model bans.
- Emergency pause to mitigate exploit or fraud risk.
