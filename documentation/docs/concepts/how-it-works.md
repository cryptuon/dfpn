# How DFPN Works

This page explains the core mechanisms that make DFPN function: the request lifecycle, commit-reveal protocol, consensus mechanism, scoring system, and reputation.

---

## Request Lifecycle

Every analysis request goes through nine steps from submission to reward distribution.

### Step 1: Submit

A client creates an analysis request on-chain. The request includes:

- Content hash (SHA-256 of the media file)
- Storage URI (where workers download the media)
- Required modalities (e.g., FaceManipulation, VideoAuthenticity)
- Fee amount (in SOL)
- Deadline (when the request must be finalized)
- Minimum number of workers required for valid consensus

### Step 2: Route

The network matches the request to workers based on their declared modalities, availability, and stake. Workers discover new requests by polling the indexer or monitoring on-chain state.

### Step 3: Download

Workers that accept the request download the media file from the client's storage URI. They verify the content hash matches the hash declared in the request.

### Step 4: Analyze

Each worker runs their detection model(s) on the downloaded media. This happens entirely on the worker's own hardware. DFPN has no visibility into the inference process.

### Step 5: Commit

Workers submit a cryptographic commitment -- a SHA-256 hash of their result combined with a random salt. This locks in their answer without revealing it. See [Commit-Reveal Protocol](#commit-reveal-protocol) below for details.

### Step 6: Reveal

After the commit window closes, workers reveal their actual results along with the salt used to generate the commitment. The protocol verifies that each reveal matches the earlier commitment.

### Step 7: Aggregate

The network combines all revealed results using reputation-weighted voting to produce a consensus verdict, confidence score, and consensus type. See [Consensus Mechanism](#consensus-mechanism) below.

### Step 8: Finalize

The aggregated result is recorded on-chain. The request status changes to `Finalized`. The client can now read the result.

### Step 9: Reward

Fees are distributed to workers, model developers, treasury, and the insurance pool. Worker reputation scores are updated based on how well their individual results aligned with the consensus.

---

## Commit-Reveal Protocol

### Why It Exists

Without commit-reveal, a dishonest worker could wait for other workers to submit results, copy the most common answer, and earn rewards without doing any real work. This is called free-riding, and it undermines the entire purpose of multi-worker consensus.

The commit-reveal protocol prevents this by hiding all results until every worker has locked in their answer.

### How It Works

The process has two phases:

**Phase 1 -- Commit:** Each worker computes their analysis result, generates a random 16-byte salt, and submits a SHA-256 hash of the result, salt, their public key, and the request ID. This commitment is stored on-chain but reveals nothing about the actual result.

```
commitment = SHA256(result_bytes + salt + worker_pubkey + request_id)
```

**Phase 2 -- Reveal:** After the commit window closes, workers submit their actual result and salt. The protocol recalculates the hash and verifies it matches the earlier commitment. If it matches, the result is accepted. If not, the reveal is rejected.

### Timing Windows

Each request's total time is split between the commit and reveal phases:

| Phase | Typical Duration | What Happens |
|-------|-----------------|--------------|
| **Commit window** | 60-70% of total time | Workers submit commitments |
| **Reveal window** | 30-40% of total time | Workers reveal actual results |
| **Finalization** | After deadline | Anyone can trigger aggregation |

!!! info "Workers who commit but do not reveal are penalized"
    If a worker submits a commitment but fails to reveal before the deadline, they receive a reputation penalty and a 1-3% stake slash. This incentivizes workers to stay online through the full lifecycle.

---

## Consensus Mechanism

DFPN uses multiple independent workers per request to produce results more reliable than any single model.

### How Consensus Is Reached

1. All revealed results are collected after the reveal window closes
2. Each result is weighted by the submitting worker's reputation score
3. The weighted results are aggregated to produce a final verdict

### Consensus Outcomes

| Outcome | Condition | Meaning |
|---------|-----------|---------|
| **Unanimous** | All workers agree on the verdict | Highest confidence in the result |
| **Majority** | More than half of weighted votes agree | Strong result with some disagreement |
| **Split** | No clear majority | Result is marked `Inconclusive` |

### Reputation-Weighted Voting

Not all workers' votes count equally. Workers with higher reputation scores (built from a history of accurate results) have more influence on the final verdict. This prevents newly joined or low-quality workers from diluting consensus quality.

The weight of each worker's vote is:

```
vote_weight = worker_reputation * stake_factor
```

Where `stake_factor` is a capped multiplier that gives a modest bonus to higher-staked workers without allowing pay-to-win dynamics.

---

## Scoring System

Worker performance is measured each epoch and determines reward distribution.

### Scoring Factors

| Factor | Weight | What It Measures |
|--------|--------|------------------|
| **Accuracy** | 50% | How often results agree with consensus or benchmarks |
| **Availability** | 25% | Uptime and task acceptance rate over the epoch |
| **Latency** | 15% | Average time to complete tasks relative to deadlines |
| **Consistency** | 10% | Stability of results across similar inputs |

### Epoch-Based Calculation

Scores are calculated at the end of each epoch (a fixed time period). The process:

1. Collect all completed tasks for each worker during the epoch
2. Calculate each scoring factor independently
3. Combine factors using the weights above
4. Normalize scores across all active workers

### Score Formula

```
worker_score = (accuracy * 0.50)
             + (availability * 0.25)
             + (latency * 0.15)
             + (consistency * 0.10)
```

Each factor is normalized to a 0-100 scale before weighting.

!!! tip "Focus on accuracy"
    Accuracy is worth more than all other factors combined. Running a well-tuned model on reliable hardware is the most important thing you can do to maximize your score.

---

## Reputation

Reputation is a persistent score that reflects a worker's long-term track record. It affects both reward distribution and consensus weight.

### How Reputation Works

- **Starting reputation:** New workers begin at **50%** (neutral)
- **Reputation increases** with each epoch of good performance (accurate results, high availability)
- **Reputation decreases** from failures (inaccurate results, missed deadlines, slashing events)
- **Maximum reputation:** 100%
- **Minimum reputation:** 0% (effectively banned from receiving tasks)

### Reputation Effects

| Reputation Range | Effect |
|-----------------|--------|
| **80-100%** | Full reward weight, high consensus influence, priority task access |
| **50-79%** | Standard reward weight, normal task routing |
| **20-49%** | Reduced reward weight, fewer tasks routed to you |
| **Below 20%** | Minimal tasks, at risk of removal from active worker pool |

### Reputation Recovery

If your reputation drops due to operational issues (hardware failure, network problems), it can be recovered by:

1. Fixing the underlying issue
2. Processing tasks successfully over subsequent epochs
3. Reputation increases gradually -- there is no shortcut

!!! warning "Reputation drops faster than it recovers"
    A single epoch of poor performance can undo several epochs of good work. This asymmetry is intentional: it protects the network from workers who alternate between honest and dishonest behavior.
