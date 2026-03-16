# Solana Program Design

This document outlines a Solana-first design using standard tools (Anchor, SPL Token, SPL Governance) with minimal custom primitives.

## Program Set

1. **Content Registry**
   - PDA: `content/{hash}`
   - Stores: hash, media type, creator wallet, timestamp, storage URI (optional), provenance claims.

2. **Analysis Marketplace**
   - PDA: `request/{hash}/{nonce}`
   - Stores: requester, fee, deadline, required modalities, status.
   - Records worker submissions and result commitments.

3. **Model Registry**
   - PDA: `model/{developer}/{model_id}`
   - Stores: model metadata, version, supported modalities, evaluation score, active flag, stake.

4. **Worker Registry**
   - PDA: `worker/{pubkey}`
   - Stores: stake, reputation score, supported modalities, last-active epoch.

5. **Rewards + Treasury**
   - Treasury is an SPL token account with program authority.
   - Rewards are distributed in epochs based on scoring records.

## Account Types (Sketch)

- `ContentAccount`: hash, creator, created_at, storage_ref.
- `AnalysisRequest`: hash, fee, deadline, status, required_modalities.
- `ResultCommit`: request, worker, model, commit_hash, timestamp.
- `ResultReveal`: request, worker, model, result_hash, metrics.
- `ModelAccount`: metadata, version, stake, score, status.
- `WorkerAccount`: stake, reputation, slashes, status.
- `EpochStats`: epoch, aggregate scores, payouts.

## Instruction Flow (Simplified)

- `register_content`
- `create_request`
- `commit_result`
- `reveal_result`
- `finalize_request`
- `register_model`
- `update_model`
- `register_worker`
- `update_reputation`
- `distribute_rewards`

## Key Solana Considerations

- **Compute limits**: avoid loops; use epoch aggregation.
- **Account size**: keep accounts small; store hashes and URIs.
- **Fees**: dynamic fees to discourage spam; allow priority fees for urgent requests.
- **Upgrades**: use upgrade authority with governance controls.
- **Time**: use `Clock` sysvar for deadlines and epoch windows.

## Storage Strategy

- Media stays off-chain.
- Store content hash and optional storage URI.
- Optional content attestations use separate PDAs to avoid bloat.

## Governance

- Use SPL Governance (Realms) for parameter changes.
- Proposals can manage: reward rates, stake requirements, scoring policy, and model retirement.
