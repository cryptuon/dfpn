# DFPN: Deepfake Proof Network on Solana

DFPN is a decentralized coordination layer for deepfake detection built on Solana. The network connects clients who need media verified with independent node operators who run their own detection algorithms and inference infrastructure.

**DFPN provides:** Request routing, result aggregation, reputation tracking, and economic incentives.

**Node operators provide:** Their own detection models, GPU/CPU infrastructure, and operational expertise.

The result is a trust-minimized marketplace where independent detection capabilities are coordinated through on-chain incentives, without DFPN controlling or providing the actual detection models.

This repo contains documentation only. It is intentionally independent of any prior paper and focuses on a practical Solana-first implementation.

## Goals

- Build on Solana (no new chain) with minimal on-chain state and predictable fees.
- Coordinate independent node operators who bring their own models and infrastructure.
- Support multi-modal analysis (image, video, audio) via off-chain workers.
- Track model performance and node reputation without controlling the models.
- Reward performance and penalize dishonest or low-quality results.
- Preserve privacy and avoid storing raw media on-chain.

## Non-goals

- Providing or hosting detection models (operators bring their own).
- Running inference infrastructure (operators manage their own GPUs).
- On-chain ML inference.
- Storing media content on-chain.
- Replacing existing media provenance tools (we integrate with them).

## High-level Components

- **Clients** submit media for analysis and consume results.
- **Content providers** register originals and metadata for provenance checks.
- **Model developers** publish detection model metadata (models run on operator infrastructure).
- **Node operators** run their own models and GPUs, post signed results to DFPN.
- **Solana programs** manage coordination, tracking, scoring, and rewards.
- **Indexers/APIs** provide fast queries and UX without trusting them for final state.

## Documentation

### Core Design
- `docs/architecture.md` - System overview and data flow.
- `docs/solana-design.md` - Program/account design and on-chain considerations.
- `docs/operational-model.md` - Model lifecycle, incentives, and governance.
- `docs/commit-reveal-protocol.md` - Anti-copying mechanism for result submission.

### Integration
- `docs/api-specification.md` - On-chain instructions and off-chain APIs.
- `docs/node-operator-guide.md` - How to run a node with your own models/GPUs.
- `docs/client-integration-guide.md` - How to submit requests and consume results.

### Planning
- `docs/roadmap.md` - Realistic delivery plan for Solana.
- `docs/milestone-plan.md` - Staffing assumptions and implementation milestones.
- `docs/tokenomics.md` - Token design, fees, staking, and rewards.
- `docs/threat-model.md` - Security assumptions, attacks, and mitigations.

## Roadmap Snapshot

- Phase 0: Research, threat model, datasets, and evaluation harness.
- Phase 1: Devnet MVP with registry + job marketplace + simple worker.
- Phase 2: Testnet pilot with staking, scoring, and model versioning.
- Phase 3: Mainnet beta with governance and reward treasury.
- Phase 4: Scale-out, UX, and ecosystem integrations.

The milestone plan includes staffing assumptions and delivery checkpoints; happy to tailor it to your team and budget.
