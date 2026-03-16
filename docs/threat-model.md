# Threat Model

This document outlines a realistic threat model for DFPN and how the Solana-based design mitigates common attacks.

## Assets to Protect

- Integrity of analysis results
- Authenticity of model evaluations
- Funds in the treasury and reward pools
- Privacy of submitted media
- Availability of the marketplace and registries

## Adversaries

- **Malicious worker**: submits incorrect results for profit.
- **Malicious model developer**: ships biased or overfit models to game benchmarks.
- **Sybil attacker**: creates many worker identities to skew outcomes.
- **Colluding cartel**: coordinates workers and models to manipulate consensus.
- **Spammer**: floods the network with low-value requests.
- **External attacker**: targets off-chain storage, indexers, or APIs.

## Trust Assumptions

- Solana provides finality and liveness within expected parameters.
- Off-chain storage links are available but not trusted for integrity.
- Benchmarks are curated and updated via governance.

## Attack Vectors and Mitigations

- **Result copying or front-running**
  - Mitigation: commit-reveal flow and random assignment of requests.

- **Sybil workers**
  - Mitigation: stake requirements, reputation weighting, and per-epoch caps.

- **Collusion to bias outcomes**
  - Mitigation: multi-worker redundancy, diversity constraints, and challenge windows.

- **Benchmark overfitting**
  - Mitigation: rotating datasets, hidden test sets, and periodic refreshes.

- **Data poisoning (training or evaluation)**
  - Mitigation: curated datasets, provenance checks, and community review.

- **Off-chain storage tampering**
  - Mitigation: content hashes on-chain and multi-source retrieval.

- **Replay of old results**
  - Mitigation: request-specific nonces and expiration windows.

- **Censorship of requests**
  - Mitigation: open worker pool, fee incentives, and fallback workers.

- **Oracle or indexer compromise**
  - Mitigation: clients verify on-chain state; indexers are not trusted.

- **Token theft or treasury drain**
  - Mitigation: program audits, multisig governance, and time-locked upgrades.

## Abuse Playbooks (What Attacks Look Like)

- **Sybil swarm**
  - Steps: attacker stakes minimal amounts across many workers -> targets low-value requests -> tries to dominate consensus.
  - Impact: reduced accuracy and distorted rewards.
  - Counter: stake floors, reputation weights, and per-epoch caps.

- **Model overfitting to benchmarks**
  - Steps: developer trains on leaked benchmarks -> scores high -> model underperforms in real traffic.
  - Impact: false confidence and degraded user outcomes.
  - Counter: hidden test sets and periodic benchmark rotation.

- **Colluding cartel**
  - Steps: workers coordinate responses -> converge on a wrong label -> collect rewards.
  - Impact: consensus manipulation.
  - Counter: diversity constraints, challenge windows, and random worker assignment.

- **Request spamming**
  - Steps: adversary floods low-fee requests -> congests marketplace -> discourages honest workers.
  - Impact: reduced availability.
  - Counter: dynamic fees, rate limits, and priority fees.

- **Storage substitution**
  - Steps: attacker swaps off-chain media at the URI -> workers analyze different content.
  - Impact: incorrect results and disputes.
  - Counter: content hashes, multi-source retrieval, and signed storage attestations.

## Monitoring and Detection Signals

- Sudden drops in accuracy or spikes in disagreement rates.
- High variance in worker performance across epochs.
- Unusual concentration of rewards to a small set of workers/models.
- Repeated disputes from the same requester or model developer.
- Storage fetch failures or hash mismatches.

## Incident Response Workflow

- **Detect**: monitor alerts and on-chain metrics; validate with spot checks.
- **Triage**: classify severity (low/medium/high) and isolate scope.
- **Contain**: pause affected programs, freeze rewards, or quarantine models.
- **Eradicate**: update policies, rotate benchmarks, or slash bad actors.
- **Recover**: resume rewards and re-enable models with stricter thresholds.
- **Postmortem**: document root cause and policy changes via governance.

## Response Levers

- Emergency pause and fee caps via governance.
- Temporary increase of stake floors and slashing rates.
- Model retirement or quarantine flags.
- Disable commit-reveal if it is being exploited.

## Residual Risks

- Adversarial examples may bypass detection in specific contexts.
- Deepfake generation improves faster than model updates.
- Privacy risks if media is shared insecurely off-chain.

## Out of Scope

- Fully private on-chain inference
- Content takedown enforcement
- Legal attribution or prosecution
