# API Reference

The DFPN indexer exposes a read-only REST API for querying on-chain state. The indexer subscribes to Solana program events and maintains a full-text search index (Tantivy) that powers these endpoints.

---

## Base URL

In development the indexer listens on `http://127.0.0.1:3030`. The dashboard proxies requests through nginx:

```
/api/*  ->  http://127.0.0.1:3030/*
```

All endpoint paths below are relative to `/api`.

---

## Endpoints

### Health

#### `GET /health`

Returns the service status and document counts.

```json
{
  "status": "healthy",
  "request_count": 1842,
  "worker_count": 37,
  "model_count": 12
}
```

| Field | Type | Description |
|-------|------|-------------|
| `status` | string | Always `"healthy"` when the service is up |
| `request_count` | integer | Total indexed analysis requests |
| `worker_count` | integer | Total indexed workers |
| `model_count` | integer | Total indexed models |

---

### Requests

#### `GET /requests`

List analysis requests with optional filters.

**Query parameters**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `status` | string | -- | Filter by status (see [Request statuses](#request-statuses)) |
| `requester` | string | -- | Filter by requester public key |
| `modalities` | integer | -- | Filter by modality bitfield |
| `limit` | integer | `100` | Max results (capped at 1000) |
| `offset` | integer | `0` | Number of results to skip |

**Response:** Array of [Request objects](#request-object).

---

#### `GET /requests/:id`

Get a single analysis request by its on-chain ID.

**Path parameters**

| Parameter | Description |
|-----------|-------------|
| `id` | The request account public key |

**Response:** A single [Request object](#request-object), or `404` if not found.

---

#### `GET /requests/search`

Full-text search across request storage URIs.

**Query parameters**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `q` | string | **required** | Search query |
| `limit` | integer | `100` | Max results (capped at 1000) |

**Response:** Array of [Request objects](#request-object).

---

### Workers

#### `GET /workers`

List registered workers with optional filters.

**Query parameters**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `status` | string | -- | Filter by status (see [Worker statuses](#worker-statuses)) |
| `operator` | string | -- | Filter by operator public key |
| `min_reputation` | integer | -- | Minimum reputation score (0--10000) |
| `modalities` | integer | -- | Filter by modality bitfield |
| `limit` | integer | `100` | Max results (capped at 1000) |
| `offset` | integer | `0` | Number of results to skip |

**Response:** Array of [Worker objects](#worker-object).

---

#### `GET /workers/:operator`

Get a single worker by operator public key.

**Path parameters**

| Parameter | Description |
|-----------|-------------|
| `operator` | The operator's Solana public key |

**Response:** A single [Worker object](#worker-object), or `404` if not found.

---

### Models

#### `GET /models`

List registered detection models with optional filters.

**Query parameters**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `status` | string | -- | Filter by status (see [Model statuses](#model-statuses)) |
| `developer` | string | -- | Filter by developer public key |
| `modalities` | integer | -- | Filter by modality bitfield |
| `limit` | integer | `100` | Max results (capped at 1000) |
| `offset` | integer | `0` | Number of results to skip |

**Response:** Array of [Model objects](#model-object).

---

#### `GET /models/:id`

Get a single model by its on-chain ID.

**Path parameters**

| Parameter | Description |
|-----------|-------------|
| `id` | The model account public key |

**Response:** A single [Model object](#model-object), or `404` if not found.

---

#### `GET /models/search`

Full-text search across model names and URIs.

**Query parameters**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `q` | string | **required** | Search query |
| `limit` | integer | `100` | Max results (capped at 1000) |

**Response:** Array of [Model objects](#model-object).

---

## Response Shapes

### Request object

```json
{
  "id": "7xKXtg2CW87d95...",
  "requester": "9WzDXwBbmkg8ZTb...",
  "content_hash": "a1b2c3d4e5f6...",
  "storage_uri": "https://arweave.net/abc123",
  "modalities": 9,
  "status": "Committed",
  "fee_amount": 5000000000,
  "deadline": 1720000000,
  "commit_deadline": 1719999000,
  "created_at": 1719990000,
  "commit_count": 3,
  "reveal_count": 1
}
```

| Field | Type | Description |
|-------|------|-------------|
| `id` | string | Request account public key |
| `requester` | string | Public key of the client who submitted the request |
| `content_hash` | string | SHA-256 hash of the submitted media |
| `storage_uri` | string | Location of the media file (Arweave, IPFS, etc.) |
| `modalities` | integer | Bitfield of requested detection modalities |
| `status` | string | Current request status |
| `fee_amount` | integer | Fee deposited in DFPN token base units |
| `deadline` | integer | Unix timestamp -- reveal deadline |
| `commit_deadline` | integer | Unix timestamp -- commit deadline |
| `created_at` | integer | Unix timestamp -- creation time |
| `commit_count` | integer | Number of worker commits received |
| `reveal_count` | integer | Number of worker reveals received |

---

### Worker object

```json
{
  "id": "HmbTLC...",
  "operator": "4uQeVj...",
  "stake": 5000000000000,
  "reputation": 8500,
  "modalities": 41,
  "status": "Active",
  "tasks_completed": 312,
  "tasks_failed": 2,
  "last_active_slot": 265400000
}
```

| Field | Type | Description |
|-------|------|-------------|
| `id` | string | Worker account public key |
| `operator` | string | Operator's Solana public key |
| `stake` | integer | Staked DFPN tokens (base units, 9 decimals) |
| `reputation` | integer | Reputation score, 0--10000 (divide by 100 for percentage) |
| `modalities` | integer | Bitfield of supported modalities |
| `status` | string | Current worker status |
| `tasks_completed` | integer | Lifetime successful tasks |
| `tasks_failed` | integer | Lifetime failed tasks |
| `last_active_slot` | integer | Solana slot of last activity |

---

### Model object

```json
{
  "id": "Fg6PaF...",
  "developer": "9WzDXw...",
  "name": "face-forensics-sbi",
  "version": "1.0.0",
  "modalities": 8,
  "model_uri": "https://huggingface.co/dfpn/face-forensics-sbi",
  "status": "Active",
  "score": 9720,
  "total_uses": 4821,
  "created_at": 1719000000
}
```

| Field | Type | Description |
|-------|------|-------------|
| `id` | string | Model account public key |
| `developer` | string | Developer's Solana public key |
| `name` | string | Human-readable model name |
| `version` | string | Semantic version string |
| `modalities` | integer | Bitfield of supported modalities |
| `model_uri` | string | URL to model weights/code |
| `status` | string | Current model status |
| `score` | integer | Performance score, 0--10000 |
| `total_uses` | integer | Lifetime analysis count |
| `created_at` | integer | Unix timestamp -- registration time |

---

## Status Enums

### Request statuses

| Value | Description |
|-------|-------------|
| `Created` | Request submitted, awaiting worker commits |
| `Committed` | At least one worker has committed a result hash |
| `Revealed` | Workers have revealed their results |
| `Finalized` | Consensus reached, result is final |
| `Cancelled` | Request cancelled by the requester |
| `Expired` | Deadline passed without sufficient results |
| `Disputed` | Result is under dispute |

### Worker statuses

| Value | Description |
|-------|-------------|
| `Active` | Accepting tasks |
| `Inactive` | Registered but not currently accepting tasks |
| `Unbonding` | Stake withdrawal in progress (unbonding period) |
| `Slashed` | Penalized for misbehavior |

### Model statuses

| Value | Description |
|-------|-------------|
| `Active` | Available for use by workers |
| `Inactive` | Registered but disabled |
| `Deprecated` | Superseded by a newer version |

---

## Rate Limits

| Tier | Limit |
|------|-------|
| Anonymous | 100 requests per minute |
| Authenticated | 1,000 requests per minute |

Rate limit headers are included in every response:

```
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 87
X-RateLimit-Reset: 1720000060
```

When the limit is exceeded the API returns `429 Too Many Requests`.

---

## Error Responses

All errors follow a consistent shape:

```json
{
  "error": "Not Found",
  "message": "No request found with the given ID",
  "status": 404
}
```

| HTTP Status | Meaning |
|-------------|---------|
| `400` | Bad request -- invalid query parameters or search syntax |
| `404` | Resource not found |
| `429` | Rate limit exceeded |
| `500` | Internal server error |
