# Solana Program IDs

DFPN is composed of five on-chain programs deployed to Solana. This page lists the program IDs, PDA derivation seeds, and the Anchor framework version used.

---

## Anchor Version

All programs are built with **Anchor 0.30.1**.

---

## Devnet Program IDs

These addresses are used on both localnet and devnet during development.

| Program | ID |
|---------|----|
| Content Registry | `GokivDYuQXPZCWRkwMhdH2h91KpDQXBEmpgBgs55bnpH` |
| Analysis Marketplace | `9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin` |
| Model Registry | `Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS` |
| Worker Registry | `HmbTLCmaGvZhKnn1Zfa1JVnp7vkMV4DYVxPLWBVoN65L` |
| Rewards | `4uQeVj5tqViQh7yWWGStvkEG1Zmhx6uasJtWCJziofM` |

!!! note "Testnet and Mainnet"
    Program IDs will differ on testnet and mainnet once those deployments are live. Check the [Roadmap](../community/roadmap.md) for deployment timelines.

---

## PDA Seeds

DFPN uses **Program Derived Addresses (PDAs)** to create deterministic, program-owned accounts. Each PDA is derived from a set of seeds and the owning program's ID.

### Content Registry

| Account Type | Seeds | Program |
|-------------|-------|---------|
| Content | `"content"` + `content_hash` | Content Registry |
| Provenance Claim | `"content"` + `content_hash` + `attestor` | Content Registry |

### Analysis Marketplace

| Account Type | Seeds | Program |
|-------------|-------|---------|
| Request | `"request"` + `content_hash` + `nonce` | Analysis Marketplace |
| Fee Vault (Request) | `"fee_vault"` + `"request"` + `content_hash` + `nonce` | Analysis Marketplace |
| Commit | `"commit"` + `request_key` + `operator_key` | Analysis Marketplace |
| Reveal | `"reveal"` + `request_key` + `operator_key` | Analysis Marketplace |
| Dispute | `"dispute"` + `request_key` + `reveal_key` | Analysis Marketplace |
| Fee Vault (Dispute) | `"fee_vault"` + `"dispute"` + `request_key` + `reveal_key` | Analysis Marketplace |

### Worker Registry

| Account Type | Seeds | Program |
|-------------|-------|---------|
| Worker | `"worker"` + `operator_key` | Worker Registry |
| Stake Vault | `"stake_vault"` + `"worker"` | Worker Registry |

### Model Registry

| Account Type | Seeds | Program |
|-------------|-------|---------|
| Model | `"model"` + `developer_key` + `model_id` | Model Registry |
| Stake Vault | `"stake_vault"` + `"model"` | Model Registry |

### Rewards

| Account Type | Seeds | Program |
|-------------|-------|---------|
| Treasury | `"treasury"` | Rewards |
| Fee Vault | `"fee_vault"` | Rewards |
| Reward Account | `"reward"` + `claimant_key` | Rewards |
| Epoch Config | `"epoch"` + `treasury_key` | Rewards |

---

## Deriving PDAs in Code

### TypeScript (SDK)

```typescript
import { PublicKey } from '@solana/web3.js';

const PROGRAM_ID = new PublicKey('HmbTLCmaGvZhKnn1Zfa1JVnp7vkMV4DYVxPLWBVoN65L');

// Derive a worker PDA
const [workerPda, bump] = PublicKey.findProgramAddressSync(
  [Buffer.from('worker'), operatorPublicKey.toBuffer()],
  PROGRAM_ID,
);
```

### Rust (Anchor)

```rust
use anchor_lang::prelude::*;
use dfpn_shared::seeds;

// In an Anchor account constraint:
#[account(
    seeds = [seeds::WORKER, operator.key().as_ref()],
    bump = worker_account.bump,
)]
pub worker_account: Account<'info, WorkerAccount>,
```

---

## Seed Constants

All seed strings are defined in the `dfpn-shared` crate:

| Constant | Value | Used By |
|----------|-------|---------|
| `CONTENT` | `b"content"` | Content Registry |
| `REQUEST` | `b"request"` | Analysis Marketplace |
| `COMMIT` | `b"commit"` | Analysis Marketplace |
| `REVEAL` | `b"reveal"` | Analysis Marketplace |
| `WORKER` | `b"worker"` | Worker Registry |
| `MODEL` | `b"model"` | Model Registry |
| `TREASURY` | `b"treasury"` | Rewards |
| `REWARD` | `b"reward"` | Rewards |
| `STAKE_VAULT` | `b"stake_vault"` | Worker Registry, Model Registry |
| `FEE_VAULT` | `b"fee_vault"` | Analysis Marketplace, Rewards |
| `DISPUTE` | `b"dispute"` | Analysis Marketplace |
| `EPOCH` | `b"epoch"` | Rewards |
