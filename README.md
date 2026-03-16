# DFPN: Deepfake Proof Network on Solana

DFPN is a roadmap and design for a decentralized deepfake detection network built on Solana. The system uses Solana programs for coordination, reputation, and incentives while keeping heavy media processing off-chain. The result is a trust-minimized, economically secure marketplace where detection models compete on measurable performance.

This repo contains documentation only. It is intentionally independent of any prior paper and focuses on a practical Solana-first implementation.

## Goals

- Build on Solana (no new chain) with minimal on-chain state and predictable fees.
- Support multi-modal analysis (image, video, audio) via off-chain workers.
- Allow dynamic model onboarding, benchmarking, and retirement.
- Reward performance and penalize dishonest or low-quality results.
- Preserve privacy and avoid storing raw media on-chain.

## Non-goals

- On-chain ML inference.
- Storing media content on-chain.
- Replacing existing media provenance tools (we integrate with them).

## High-level Components

- **Clients** submit media for analysis and consume results.
- **Content providers** register originals and metadata for provenance checks.
- **Model developers** publish detection models and updates.
- **Workers** run inference off-chain and post signed results.
- **Solana programs** manage registries, jobs, scoring, and rewards.
- **Indexers/APIs** provide fast queries and UX without trusting them for final state.

## Documentation

- `docs/architecture.md` - System overview and data flow.
- `docs/solana-design.md` - Program/account design and on-chain considerations.
- `docs/operational-model.md` - Model lifecycle, incentives, and governance.
- `docs/roadmap.md` - Realistic delivery plan for Solana.
- `docs/tokenomics.md` - Token design, fees, staking, and rewards.
- `docs/threat-model.md` - Security assumptions, attacks, and mitigations.
- `docs/milestone-plan.md` - Staffing assumptions and implementation milestones.

## Roadmap Snapshot

- Phase 0: Research, threat model, datasets, and evaluation harness.
- Phase 1: Devnet MVP with registry + job marketplace + simple worker.
- Phase 2: Testnet pilot with staking, scoring, and model versioning.
- Phase 3: Mainnet beta with governance and reward treasury.
- Phase 4: Scale-out, UX, and ecosystem integrations.

The milestone plan includes staffing assumptions and delivery checkpoints; happy to tailor it to your team and budget.
