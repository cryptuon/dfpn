# DFPN: Deepfake Proof Network

**Decentralized deepfake detection coordination on Solana.**

DFPN is an open coordination layer that connects clients who need media verified with independent workers who run their own detection models and GPU infrastructure. It is not a model provider or a centralized detection service -- it is the protocol that makes decentralized detection work.

---

## What Is DFPN?

DFPN coordinates deepfake detection across a network of independent participants. When a client submits an image, video, or audio file for analysis, the network routes it to multiple workers running different detection models. Their results are aggregated through consensus and recorded on the Solana blockchain.

The protocol handles:

- **Request routing** -- matching media to capable workers
- **Result integrity** -- commit-reveal protocol prevents copying and collusion
- **Quality enforcement** -- reputation scoring and economic penalties
- **Fair compensation** -- automated reward distribution based on performance

DFPN does not train models, run inference, or store media. All of that is handled by the participants themselves.

---

## Why It Matters

Deepfakes are eroding trust in digital media. Current detection solutions are centralized, creating single points of failure, bias, and censorship risk. If one company's API goes down or their model is fooled, there is no fallback.

DFPN addresses this by:

- **Eliminating single points of failure** -- multiple independent workers analyze every request
- **Enabling model diversity** -- different detection algorithms catch different manipulation types
- **Providing transparency** -- all results and audit trails are recorded on-chain
- **Aligning incentives** -- workers earn rewards for accurate, timely results and lose stake for bad behavior

---

## Participant Roles

DFPN has three core roles. You can participate in one or more.

| Role | What You Do | What You Earn |
|------|-------------|---------------|
| **Worker** | Run detection nodes with your own GPU and models | Per-request fees + epoch rewards |
| **Client** | Submit media for deepfake analysis | Verified results with audit trails |
| **Model Developer** | Create and register detection models | 20% of fees when your model is used + epoch rewards |

### Workers

Workers are the backbone of the network. You bring your own hardware and detection models, accept analysis tasks, and submit results through the commit-reveal protocol. Your performance is scored each epoch, and rewards scale with accuracy, availability, and stake.

[Get started as a Worker](getting-started/workers.md){ .md-button }

### Clients

Clients submit media (images, videos, audio) and receive aggregated detection results backed by multi-worker consensus. Integration is available through TypeScript and Python SDKs, a REST API, or direct Solana RPC calls.

[Get started as a Client](getting-started/clients.md){ .md-button }

### Model Developers

Model developers build and register deepfake detection algorithms. When workers adopt your model and use it to process requests, you earn a share of every fee. Models are benchmarked on-chain to ensure quality.

[Get started as a Model Developer](getting-started/model-developers.md){ .md-button }

---

## Key Features

### Multi-Model Consensus

Every analysis request is processed by multiple workers running potentially different models. Results are aggregated using reputation-weighted voting, producing a verdict that is more robust than any single model.

### Commit-Reveal Protocol

Workers first submit a cryptographic hash of their result (commit), then reveal the actual result after all commits are in. This prevents workers from copying each other's answers or front-running.

### On-Chain Transparency

Every request, result, and reward is recorded on the Solana blockchain. Clients get a complete audit trail. Anyone can verify that the process was fair and the results are authentic.

### Economic Incentives

Staking, rewards, and slashing create a system where honest behavior is profitable and dishonesty is expensive. Workers stake DFPN tokens to participate and earn based on their measured performance.

---

## Quick Links

- [Getting Started](getting-started/index.md) -- choose your path and set up
- [How It Works](concepts/how-it-works.md) -- understand the request lifecycle and consensus
- [System Architecture](concepts/architecture.md) -- on-chain and off-chain components
- [Token Economics](concepts/tokenomics.md) -- supply, rewards, fees, and staking
