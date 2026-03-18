# Operational Model

This document describes how models and node operators join the network, how quality is measured, and how incentives are paid.

## Key Principle: Separation of Concerns

DFPN is a coordination layer. The network tracks and incentivizes, but does not control:

| DFPN Responsibility | Operator Responsibility |
|---------------------|------------------------|
| Register model metadata | Develop and train models |
| Track model performance | Host and serve models |
| Distribute rewards | Run inference infrastructure |
| Apply slashing | Choose hardware and scaling |
| Aggregate results | Decide which requests to serve |

This separation means DFPN has no single point of failure for detection capabilities. If one model underperforms, operators can switch to alternatives without protocol changes.

## Model Lifecycle

Models are developed and hosted by independent parties. DFPN only tracks metadata and performance.

1. **Submission**: Developer registers model metadata (name, version, modalities, download URI) and stakes DFPN.
2. **Distribution**: Model files are hosted by the developer; operators download and run them independently.
3. **Benchmarking**: Protocol evaluation harness tests the model on held-out datasets.
4. **Activation**: Models meeting minimum thresholds become active and visible to operators.
5. **Adoption**: Operators independently choose which models to run on their infrastructure.
6. **Versioning**: New versions are submitted and evaluated without disrupting existing deployments.
7. **Retirement**: Models with sustained poor performance are retired from the registry.

## Node Operator Lifecycle

Node operators run their own infrastructure and choose which models to deploy.

1. **Setup**: Operator provisions GPU/CPU infrastructure and installs chosen detection models.
2. **Registration**: Operator stakes DFPN tokens and declares supported modalities and models.
3. **Task Selection**: Operator's node pulls jobs matching their capabilities from the marketplace.
4. **Inference**: Operator runs models locally on their infrastructure (DFPN never sees the inference).
5. **Submission**: Results are committed then revealed via the commit-reveal protocol.
6. **Scoring**: Performance updates reputation each epoch based on accuracy and reliability.
7. **Rewards**: Operators earn fees proportional to their performance and stake.
8. **Slashing**: Fraud, missed deadlines, or invalid results reduce stake.

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
