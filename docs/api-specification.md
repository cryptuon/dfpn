# API Specification

This document defines the on-chain instructions and off-chain API endpoints for integrating with DFPN.

## Overview

DFPN is a coordination and tracking layer. It does not provide detection models or inference infrastructure. Node operators bring their own algorithms and GPU resources; DFPN tracks requests, results, reputation, and rewards.

## On-chain Instructions (Solana Programs)

### Content Registry Program

#### `register_content`

Registers original content for provenance tracking.

```
Accounts:
  - [signer] creator          # Content owner
  - [writable] content_account # PDA: seeds = ["content", content_hash]
  - [] system_program

Args:
  - content_hash: [u8; 32]    # SHA-256 hash of the media
  - media_type: MediaType     # Image | Video | Audio
  - storage_uri: Option<String> # Off-chain storage location (IPFS/Arweave/S3)
  - metadata: ContentMetadata # Optional provenance data
```

#### `add_provenance_claim`

Attaches provenance attestations to registered content.

```
Accounts:
  - [signer] attestor
  - [writable] content_account
  - [writable] claim_account  # PDA: seeds = ["claim", content_hash, attestor]

Args:
  - claim_type: ClaimType     # OriginalAuthor | LicensedFrom | DerivedFrom
  - evidence_uri: Option<String>
  - signature: [u8; 64]       # Ed25519 signature over claim data
```

---

### Analysis Marketplace Program

#### `create_request`

Submits a media analysis request.

```
Accounts:
  - [signer] requester
  - [writable] request_account # PDA: seeds = ["request", content_hash, nonce]
  - [writable] fee_vault       # SPL token account for fees
  - [] token_program

Args:
  - content_hash: [u8; 32]
  - storage_uri: String        # Where nodes fetch the media
  - required_modalities: Vec<Modality>  # Image | Video | Audio
  - min_workers: u8            # Minimum workers for consensus (1-10)
  - fee_amount: u64            # Payment in lamports or DFPN
  - deadline: i64              # Unix timestamp
  - priority: Priority         # Standard | High | Urgent
```

#### `commit_result`

Worker commits a hashed result (phase 1 of commit-reveal).

```
Accounts:
  - [signer] worker
  - [writable] request_account
  - [writable] commit_account  # PDA: seeds = ["commit", request, worker]
  - [] worker_account          # Must be registered and staked

Args:
  - commitment: [u8; 32]       # Hash of (result || salt || worker_pubkey)
```

#### `reveal_result`

Worker reveals the actual result (phase 2 of commit-reveal).

```
Accounts:
  - [signer] worker
  - [writable] request_account
  - [writable] commit_account
  - [writable] result_account  # PDA: seeds = ["result", request, worker]

Args:
  - result: AnalysisResult     # See Result Schema below
  - salt: [u8; 16]             # Random salt used in commitment
```

#### `finalize_request`

Aggregates results and triggers reward distribution.

```
Accounts:
  - [signer] anyone            # Permissionless finalization
  - [writable] request_account
  - [writable] fee_vault
  - [writable] treasury
  - [] result_accounts...      # All revealed results for this request

Args: none (reads from result accounts)
```

---

### Model Registry Program

#### `register_model`

Registers a detection model (metadata only; model runs on node infrastructure).

```
Accounts:
  - [signer] developer
  - [writable] model_account   # PDA: seeds = ["model", developer, model_id]
  - [writable] stake_vault
  - [] token_program

Args:
  - model_id: String           # Unique identifier
  - name: String
  - version: String            # Semantic version
  - modalities: Vec<Modality>  # Supported media types
  - model_uri: String          # Where operators download the model
  - checksum: [u8; 32]         # Model file hash for integrity
  - stake_amount: u64          # Required stake in DFPN
```

#### `update_model`

Publishes a new version of an existing model.

```
Accounts:
  - [signer] developer
  - [writable] model_account
  - [writable] version_account # PDA: seeds = ["version", model_id, version]

Args:
  - version: String
  - model_uri: String
  - checksum: [u8; 32]
  - changelog: String
```

#### `retire_model`

Deactivates a model (voluntary or forced via governance).

```
Accounts:
  - [signer] authority         # Developer or governance
  - [writable] model_account

Args:
  - reason: RetirementReason   # Voluntary | LowPerformance | SecurityIssue
```

---

### Worker Registry Program

#### `register_worker`

Registers a node operator to participate in the network.

```
Accounts:
  - [signer] operator
  - [writable] worker_account  # PDA: seeds = ["worker", operator]
  - [writable] stake_vault
  - [] token_program

Args:
  - supported_modalities: Vec<Modality>
  - supported_models: Vec<Pubkey>  # Model accounts this node can run
  - endpoint: Option<String>       # Optional status endpoint
  - stake_amount: u64
```

#### `update_worker`

Updates worker configuration (models, endpoint, stake).

```
Accounts:
  - [signer] operator
  - [writable] worker_account
  - [writable] stake_vault     # If adding stake

Args:
  - supported_models: Option<Vec<Pubkey>>
  - endpoint: Option<String>
  - additional_stake: Option<u64>
```

#### `withdraw_stake`

Withdraws stake after unbonding period.

```
Accounts:
  - [signer] operator
  - [writable] worker_account
  - [writable] stake_vault
  - [writable] destination

Args:
  - amount: u64
```

---

### Rewards Program

#### `claim_rewards`

Claims accumulated rewards for a worker or model developer.

```
Accounts:
  - [signer] claimant
  - [writable] reward_account  # PDA: seeds = ["rewards", claimant]
  - [writable] treasury
  - [writable] destination
  - [] token_program

Args: none (claims full balance)
```

#### `distribute_epoch_rewards`

Permissionless crank to distribute rewards for a completed epoch.

```
Accounts:
  - [signer] anyone
  - [writable] epoch_account   # PDA: seeds = ["epoch", epoch_number]
  - [writable] treasury
  - [] worker_accounts...
  - [] model_accounts...

Args:
  - epoch: u64
```

---

## Data Types

### MediaType
```rust
enum MediaType {
    Image,
    Video,
    Audio,
}
```

### Modality
```rust
enum Modality {
    ImageAuthenticity,
    VideoAuthenticity,
    AudioAuthenticity,
    FaceManipulation,
    VoiceCloning,
    GeneratedContent,
}
```

### AnalysisResult
```rust
struct AnalysisResult {
    verdict: Verdict,           // Authentic | Manipulated | Inconclusive
    confidence: u8,             // 0-100
    model_pubkey: Pubkey,       // Model used for analysis
    model_version: String,
    detections: Vec<Detection>, // Specific findings
    processing_time_ms: u32,
    node_metadata: NodeMetadata,
}

struct Detection {
    detection_type: String,     // e.g., "face_swap", "audio_splice"
    region: Option<Region>,     // Bounding box or time range
    confidence: u8,
}

enum Verdict {
    Authentic,
    Manipulated,
    Inconclusive,
}
```

### RequestStatus
```rust
enum RequestStatus {
    Open,           // Accepting commits
    CommitClosed,   // Accepting reveals only
    Finalized,      // Results aggregated
    Expired,        // Deadline passed, incomplete
    Disputed,       // Under challenge
}
```

---

## Off-chain APIs

Node operators and clients interact with indexers for fast queries. Indexers mirror on-chain state but are not trusted for finality.

### Indexer REST API

Base URL: `https://api.dfpn.network/v1` (example)

#### Requests

```
GET /requests
  Query params: status, modality, min_fee, limit, offset
  Returns: List of open analysis requests

GET /requests/{request_id}
  Returns: Request details, commits, results, status

POST /requests/{request_id}/media-url
  Auth: Signed message from requester
  Returns: Pre-signed URL to upload media
```

#### Results

```
GET /results/{request_id}
  Returns: Aggregated result and individual worker submissions

GET /results/content/{content_hash}
  Returns: All analysis results for a content hash
```

#### Workers

```
GET /workers
  Query params: modality, model, min_reputation, status
  Returns: List of active workers

GET /workers/{worker_pubkey}
  Returns: Worker stats, reputation, supported models

GET /workers/{worker_pubkey}/history
  Returns: Recent submissions and performance
```

#### Models

```
GET /models
  Query params: modality, min_score, status
  Returns: List of registered models

GET /models/{model_pubkey}
  Returns: Model details, versions, benchmark scores

GET /models/{model_pubkey}/leaderboard
  Returns: Performance ranking across workers using this model
```

#### Epochs

```
GET /epochs/current
  Returns: Current epoch number, start time, stats

GET /epochs/{epoch_number}
  Returns: Epoch stats, top performers, reward distribution
```

---

## WebSocket Subscriptions

Real-time updates for workers and clients.

```
WS /ws/requests
  Subscribe to new requests matching filters

WS /ws/requests/{request_id}
  Subscribe to status updates for a specific request

WS /ws/workers/{worker_pubkey}
  Subscribe to assigned tasks and reward notifications
```

---

## Error Codes

| Code | Name | Description |
|------|------|-------------|
| 1001 | InvalidContentHash | Content hash format invalid |
| 1002 | RequestNotFound | Analysis request does not exist |
| 1003 | RequestExpired | Deadline has passed |
| 1004 | CommitWindowClosed | Too late to submit commitment |
| 1005 | RevealWindowClosed | Too late to reveal result |
| 1006 | InvalidCommitment | Revealed data doesn't match commitment |
| 1007 | WorkerNotRegistered | Worker account not found |
| 1008 | InsufficientStake | Worker stake below minimum |
| 1009 | ModelNotActive | Model is retired or suspended |
| 1010 | DuplicateCommit | Worker already committed to this request |
| 2001 | InsufficientFee | Request fee below minimum |
| 2002 | InvalidModality | Unsupported modality requested |
| 3001 | SlashingPending | Cannot withdraw while slash is pending |
| 3002 | UnbondingPeriod | Stake still in unbonding period |

---

## Rate Limits

Indexer APIs enforce rate limits per IP/API key:

- Anonymous: 100 requests/minute
- Authenticated: 1000 requests/minute
- WebSocket: 10 subscriptions per connection

On-chain instructions are limited only by Solana transaction fees and compute limits.
