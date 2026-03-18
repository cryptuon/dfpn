# Client Integration Guide

This guide explains how to integrate DFPN into your application to submit media for deepfake analysis and consume results.

## Overview

DFPN provides a decentralized coordination layer for deepfake detection. As a client, you:

1. Upload media to off-chain storage
2. Submit an analysis request with the content hash
3. Wait for workers to analyze (using their own models/infrastructure)
4. Retrieve aggregated results from the network

DFPN does not store your media or run inference. It coordinates independent node operators who bring their own detection capabilities.

## Integration Options

| Method | Best For | Complexity |
|--------|----------|------------|
| SDK (TypeScript/Python) | Applications, backends | Low |
| Direct RPC | Custom implementations | Medium |
| REST API (via indexer) | Quick prototypes, read-heavy | Low |
| Webhook | Event-driven architectures | Medium |

## Quick Start (TypeScript SDK)

### Installation

```bash
npm install @dfpn/sdk @solana/web3.js
```

### Basic Usage

```typescript
import { DFPNClient, Modality } from '@dfpn/sdk';
import { Keypair } from '@solana/web3.js';

// Initialize client
const client = new DFPNClient({
  network: 'devnet',
  wallet: Keypair.fromSecretKey(/* your keypair */),
});

// Submit analysis request
const request = await client.submitRequest({
  mediaPath: './suspicious-video.mp4',
  modalities: [Modality.VideoAuthenticity, Modality.FaceManipulation],
  minWorkers: 3,
  maxFee: 0.01, // SOL
  deadline: Date.now() + 300_000, // 5 minutes
});

console.log('Request ID:', request.id);

// Wait for results
const result = await client.waitForResult(request.id, {
  timeout: 360_000, // 6 minutes
});

console.log('Verdict:', result.verdict);
console.log('Confidence:', result.confidence);
console.log('Worker count:', result.workerResults.length);
```

## Step-by-Step Integration

### Step 1: Set Up Storage

DFPN requires media to be accessible via URI. Workers fetch content from this URI.

**Supported storage options:**
- IPFS (recommended for decentralization)
- Arweave (permanent storage)
- S3-compatible (AWS, GCS, Cloudflare R2)
- HTTP/HTTPS endpoints

```typescript
import { DFPNStorage } from '@dfpn/sdk';

// Option A: Use DFPN's storage helper (IPFS via Pinata)
const storage = new DFPNStorage({
  provider: 'pinata',
  apiKey: process.env.PINATA_API_KEY,
});

const { uri, hash } = await storage.upload('./media.mp4');
// uri: ipfs://Qm...
// hash: SHA256 of content

// Option B: Use your own storage
const uri = 'https://your-cdn.com/media/12345.mp4';
const hash = await computeSha256(mediaBuffer);
```

**Content hash requirement:**

The content hash is critical for integrity. Workers verify the hash matches the downloaded content.

```typescript
import { createHash } from 'crypto';

function computeSha256(buffer: Buffer): string {
  return createHash('sha256').update(buffer).digest('hex');
}
```

### Step 2: Create Analysis Request

```typescript
const request = await client.createRequest({
  // Required
  contentHash: hash,
  storageUri: uri,
  modalities: [Modality.FaceManipulation],

  // Consensus settings
  minWorkers: 3,        // Minimum workers for valid consensus
  maxWorkers: 7,        // Cap on workers (cost control)

  // Pricing
  feeAmount: 0.01,      // Total fee in SOL
  feeCurrency: 'SOL',   // SOL or DFPN

  // Timing
  deadline: Date.now() + 300_000,  // 5 minutes from now

  // Optional
  priority: 'standard',  // standard | high | urgent
  callbackUrl: 'https://your-api.com/webhook/dfpn',
  metadata: {
    source: 'user-upload',
    userId: 'user-123',
  },
});
```

**On-chain transaction:**

```typescript
// The SDK handles this, but here's what happens:
const tx = new Transaction().add(
  createRequestInstruction({
    requester: wallet.publicKey,
    contentHash: Buffer.from(hash, 'hex'),
    storageUri: uri,
    modalities: [1, 4], // Encoded modality flags
    minWorkers: 3,
    feeAmount: 10_000_000, // lamports
    deadline: Math.floor(deadline / 1000),
  })
);

await sendAndConfirmTransaction(connection, tx, [wallet]);
```

### Step 3: Monitor Request Status

```typescript
// Poll for status
const status = await client.getRequestStatus(request.id);
console.log(status);
// {
//   status: 'CommitClosed',
//   commitCount: 5,
//   revealCount: 3,
//   deadline: 1699999999,
//   timeRemaining: 45000,
// }

// Or subscribe to updates
client.subscribeToRequest(request.id, (update) => {
  console.log('Status:', update.status);
  console.log('Commits:', update.commitCount);
  console.log('Reveals:', update.revealCount);

  if (update.status === 'Finalized') {
    console.log('Result ready!');
  }
});
```

### Step 4: Retrieve Results

```typescript
const result = await client.getResult(request.id);

// Aggregated result
console.log('Verdict:', result.verdict);         // Authentic | Manipulated | Inconclusive
console.log('Confidence:', result.confidence);   // 0-100
console.log('Consensus:', result.consensusType); // Unanimous | Majority | Split

// Individual worker results
for (const wr of result.workerResults) {
  console.log(`Worker ${wr.worker}:`);
  console.log(`  Model: ${wr.modelId}`);
  console.log(`  Verdict: ${wr.verdict}`);
  console.log(`  Confidence: ${wr.confidence}`);
  console.log(`  Detections:`, wr.detections);
}

// Audit trail
console.log('Request TX:', result.audit.requestTx);
console.log('Finalize TX:', result.audit.finalizeTx);
console.log('Worker commits:', result.audit.commits);
```

### Step 5: Handle Results

```typescript
// Example: Content moderation flow
async function moderateContent(mediaPath: string): Promise<ModerationDecision> {
  const { uri, hash } = await storage.upload(mediaPath);

  const request = await client.createRequest({
    contentHash: hash,
    storageUri: uri,
    modalities: [Modality.FaceManipulation, Modality.GeneratedContent],
    minWorkers: 3,
    feeAmount: 0.02,
    deadline: Date.now() + 600_000,
  });

  const result = await client.waitForResult(request.id);

  if (result.verdict === 'Manipulated' && result.confidence > 80) {
    return {
      action: 'flag',
      reason: 'High confidence deepfake detection',
      details: result.detections,
      auditTrail: result.audit,
    };
  }

  if (result.verdict === 'Inconclusive') {
    return {
      action: 'manual_review',
      reason: 'Inconclusive automated analysis',
      workerResults: result.workerResults,
    };
  }

  return {
    action: 'approve',
    confidence: result.confidence,
  };
}
```

## Python SDK

```python
from dfpn import DFPNClient, Modality
from solders.keypair import Keypair

# Initialize
client = DFPNClient(
    network="devnet",
    wallet=Keypair.from_bytes(wallet_bytes),
)

# Submit request
request = client.submit_request(
    media_path="./video.mp4",
    modalities=[Modality.VIDEO_AUTHENTICITY],
    min_workers=3,
    max_fee=0.01,
    deadline_seconds=300,
)

# Get result
result = client.wait_for_result(request.id, timeout=360)

print(f"Verdict: {result.verdict}")
print(f"Confidence: {result.confidence}%")
```

## REST API (via Indexer)

For read operations and quick integrations, use the indexer REST API.

**Note:** The indexer mirrors on-chain state but is not authoritative. For critical applications, verify on-chain.

### Submit Request

```bash
# Step 1: Get upload URL
curl -X POST https://api.dfpn.network/v1/requests/prepare \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "content_hash": "abc123...",
    "modalities": ["FaceManipulation"],
    "min_workers": 3,
    "fee_amount": 0.01,
    "deadline_seconds": 300
  }'

# Response:
# {
#   "upload_url": "https://upload.dfpn.network/...",
#   "request_id": "req_abc123",
#   "sign_message": "Sign this to authorize: ..."
# }

# Step 2: Upload media
curl -X PUT "https://upload.dfpn.network/..." \
  -H "Content-Type: video/mp4" \
  --data-binary @video.mp4

# Step 3: Submit signed request
curl -X POST https://api.dfpn.network/v1/requests \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "request_id": "req_abc123",
    "signature": "base64_signature..."
  }'
```

### Get Result

```bash
curl https://api.dfpn.network/v1/results/req_abc123 \
  -H "Authorization: Bearer YOUR_API_KEY"

# Response:
# {
#   "request_id": "req_abc123",
#   "status": "Finalized",
#   "verdict": "Manipulated",
#   "confidence": 87,
#   "consensus_type": "Majority",
#   "worker_results": [...],
#   "audit": {...}
# }
```

### Webhooks

Register a webhook to receive results asynchronously:

```bash
curl -X POST https://api.dfpn.network/v1/webhooks \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "url": "https://your-api.com/webhook/dfpn",
    "events": ["request.finalized", "request.expired"],
    "secret": "your_webhook_secret"
  }'
```

**Webhook payload:**

```json
{
  "event": "request.finalized",
  "timestamp": "2024-01-15T10:30:00Z",
  "data": {
    "request_id": "req_abc123",
    "verdict": "Manipulated",
    "confidence": 87,
    "result_url": "https://api.dfpn.network/v1/results/req_abc123"
  },
  "signature": "hmac_sha256_signature"
}
```

## Pricing and Fees

### Fee Structure

| Component | Recipient | Percentage |
|-----------|-----------|------------|
| Worker rewards | Node operators | 65% |
| Model developers | Algorithm creators | 20% |
| Treasury | Protocol | 10% |
| Insurance | Dispute resolution | 5% |

### Estimating Costs

```typescript
const estimate = await client.estimateFee({
  modalities: [Modality.VideoAuthenticity, Modality.FaceManipulation],
  minWorkers: 5,
  priority: 'high',
  mediaSizeMb: 50,
});

console.log('Estimated fee:', estimate.totalFee, 'SOL');
console.log('Breakdown:', estimate.breakdown);
// {
//   baseFee: 0.005,
//   modalityFee: 0.008,
//   priorityMultiplier: 1.5,
//   workerFee: 0.007,
// }
```

### Fee Tiers by Modality

| Modality | Base Fee (SOL) | Typical Time |
|----------|----------------|--------------|
| ImageAuthenticity | 0.002 | 30-60s |
| AudioAuthenticity | 0.003 | 30-60s |
| VideoAuthenticity | 0.008 | 60-180s |
| FaceManipulation | 0.005 | 45-90s |
| VoiceCloning | 0.004 | 30-60s |
| GeneratedContent | 0.006 | 45-120s |

## Error Handling

```typescript
import { DFPNError, ErrorCode } from '@dfpn/sdk';

try {
  const result = await client.submitRequest({...});
} catch (error) {
  if (error instanceof DFPNError) {
    switch (error.code) {
      case ErrorCode.INSUFFICIENT_FUNDS:
        console.error('Not enough SOL/DFPN for fee');
        break;
      case ErrorCode.INVALID_MODALITY:
        console.error('Requested modality not supported');
        break;
      case ErrorCode.DEADLINE_TOO_SHORT:
        console.error('Deadline must be at least 60 seconds');
        break;
      case ErrorCode.CONTENT_HASH_MISMATCH:
        console.error('Uploaded content does not match hash');
        break;
      case ErrorCode.NO_WORKERS_AVAILABLE:
        console.error('No workers for requested modalities');
        break;
      default:
        console.error('DFPN error:', error.message);
    }
  }
  throw error;
}
```

### Handling Timeouts and Failures

```typescript
const result = await client.waitForResult(request.id, {
  timeout: 360_000,
  onTimeout: async () => {
    // Request didn't complete in time
    const status = await client.getRequestStatus(request.id);

    if (status.revealCount >= status.minWorkers) {
      // Enough reveals, trigger manual finalization
      await client.finalizeRequest(request.id);
      return client.getResult(request.id);
    } else {
      // Not enough workers, consider refund
      console.log('Insufficient workers, eligible for refund');
      return null;
    }
  },
});
```

## Best Practices

### 1. Validate Content Before Submission

```typescript
// Check file type and size
const stats = await fs.stat(mediaPath);
if (stats.size > 500 * 1024 * 1024) {
  throw new Error('File too large (max 500MB)');
}

const mimeType = await detectMimeType(mediaPath);
if (!['video/mp4', 'image/jpeg', 'audio/wav'].includes(mimeType)) {
  throw new Error('Unsupported media type');
}
```

### 2. Choose Appropriate Worker Count

```typescript
// Low stakes: 1-3 workers (faster, cheaper)
// Medium stakes: 3-5 workers (balanced)
// High stakes: 5-7 workers (higher confidence)

const minWorkers = determineWorkerCount(importance);
```

### 3. Set Realistic Deadlines

```typescript
// Consider:
// - Media size (larger = more processing time)
// - Network congestion
// - Worker availability

const deadline = Date.now() + Math.max(
  180_000,  // Minimum 3 minutes
  mediaSizeMb * 3000,  // ~3 seconds per MB
);
```

### 4. Cache Results

```typescript
// Results are immutable once finalized
const cacheKey = `dfpn:${contentHash}:${modalities.join(',')}`;
const cached = await redis.get(cacheKey);

if (cached) {
  return JSON.parse(cached);
}

const result = await client.waitForResult(request.id);
await redis.set(cacheKey, JSON.stringify(result), 'EX', 86400);
return result;
```

### 5. Implement Retry Logic

```typescript
async function submitWithRetry(params, maxRetries = 3) {
  for (let attempt = 0; attempt < maxRetries; attempt++) {
    try {
      return await client.submitRequest(params);
    } catch (error) {
      if (error.code === ErrorCode.NETWORK_ERROR && attempt < maxRetries - 1) {
        await sleep(1000 * Math.pow(2, attempt));
        continue;
      }
      throw error;
    }
  }
}
```

## Security Considerations

1. **Verify on-chain**: For critical decisions, verify results on-chain rather than trusting indexer alone
2. **Protect API keys**: Never expose API keys in client-side code
3. **Validate webhooks**: Always verify webhook signatures
4. **Content privacy**: Consider encryption for sensitive media (DFPN doesn't access content, but storage might)
5. **Rate limiting**: Implement rate limits to prevent abuse and cost overruns

## Support

- SDK Documentation: https://docs.dfpn.network/sdk
- API Reference: https://api.dfpn.network/docs
- Discord: https://discord.gg/dfpn
- GitHub: https://github.com/dfpn/sdk
