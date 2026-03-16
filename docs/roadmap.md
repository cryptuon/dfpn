# Roadmap (Solana)

A realistic, phased roadmap to launch DFPN on Solana without building a new chain. Timelines depend on team size and funding; each phase is defined by measurable deliverables.

## Phase 0: Research and Foundations (4-6 weeks)

- Define threat model and abuse cases.
- Select baseline datasets for images, video, and audio.
- Build evaluation harness for repeatable benchmarking.
- Finalize scoring metrics and minimum thresholds.

## Phase 1: Devnet MVP (6-8 weeks)

- Solana programs: content registry, analysis marketplace, model registry.
- Single worker reference implementation (CPU-only).
- Commit-reveal for results.
- CLI or basic web UI for submissions and results.
- End-to-end flow on devnet with a small test dataset.

## Phase 2: Testnet Pilot (8-10 weeks)

- Worker registry, staking, and reputation.
- Epoch-based scoring and reward distribution.
- Model versioning and retirement.
- Indexer + API for read access and dashboards.
- Testnet load testing with multiple workers.

## Phase 3: Mainnet Beta (8-12 weeks)

- SPL token treasury and reward pool.
- Governance via SPL Governance (Realms).
- Security audit of all programs.
- Public beta with capped volume and clear SLAs.

## Phase 4: Scale and Ecosystem (ongoing)

- Model marketplace and curated model sets.
- Advanced storage integrations (Arweave/IPFS + access controls).
- Specialized detection for re-recording and low-quality manipulations.
- Partnerships with content platforms and fact-checking services.

## Definition of "Working"

- Requests are posted and settled on Solana.
- Workers submit results with provable incentives.
- Models can be added, evaluated, and retired.
- Users can audit results and provenance from on-chain state.
