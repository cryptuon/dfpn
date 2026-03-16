# Architecture

This document describes a practical architecture for a decentralized deepfake detection network that runs on Solana without building a new chain.

## System Components

- **Clients**: Submit media, check authenticity, and retrieve audit trails.
- **Content Providers**: Register originals and proofs of authorship.
- **Model Developers**: Publish detection models and updates.
- **Workers**: Execute inference off-chain and post signed results.
- **Solana Programs**: Registries, jobs, scoring, staking, and rewards.
- **Storage**: Off-chain media and datasets (IPFS/Arweave/S3), with hashes on-chain.
- **Indexers**: Read Solana state and provide APIs for fast search.

## Core On-chain Programs

- **Content Registry Program**
  - Stores hashes and metadata of original content.
  - Supports provenance attestations and ownership claims.

- **Analysis Marketplace Program**
  - Creates analysis requests with fees and deadlines.
  - Tracks assigned workers and their submissions.

- **Model Registry Program**
  - Registers models with versioning and metadata.
  - Supports model staking and retirement.

- **Scoring and Rewards Program**
  - Computes performance-based rewards in epochs.
  - Applies slashing or reputation decay for invalid results.

- **Treasury Program**
  - Holds fees and reward pools (SPL tokens).
  - Streams rewards to workers and model developers.

## Off-chain Services

- **Inference Workers**
  - Pull tasks, run models, and submit results.
  - Sign outputs with registered worker keys.

- **Evaluation Harness**
  - Benchmarks models on held-out datasets.
  - Produces score reports for on-chain updates.

- **Indexer + API**
  - Builds fast queries for clients and dashboards.
  - Mirrors on-chain data without being a source of truth.

## Data Flow

1. **Register Original**: Content providers upload media off-chain and store content hashes on-chain.
2. **Submit Analysis Request**: Clients post a hash and metadata, pay a fee, and set a deadline.
3. **Worker Inference**: Workers retrieve the media from storage, run models, and submit signed results.
4. **Consensus/Scoring**: Results are aggregated using a scoring policy and worker reputation.
5. **Reward Distribution**: Treasury sends rewards based on accuracy and stake commitments.
6. **Audit Trail**: Anyone can inspect the request, results, and model versions used.

## Security Model

- **No on-chain inference**; trust comes from economic incentives and redundancy.
- **Commit-reveal** can reduce result copying and collusion.
- **Staking + slashing** enforces honest behavior for workers and model developers.
- **Epoch scoring** limits on-chain compute and keeps fees predictable.

## Scalability Notes

- Keep program state small: store hashes, not content.
- Use Solana account compression or off-chain indexing for large registries.
- Prefer batch settlement for rewards and scoring updates.
