# Submitting Media for Analysis

Clients submit images, videos, and audio files to DFPN for deepfake detection. The network routes your request to multiple independent workers, aggregates their results through consensus, and returns a verified verdict.

---

## What Clients Do

As a client, you:

1. Upload media to off-chain storage (IPFS, S3, or any HTTP endpoint)
2. Submit an analysis request to the network with the content hash and storage URI
3. Pay a fee in SOL
4. Receive an aggregated result backed by multi-worker consensus
5. Get a full on-chain audit trail for every analysis

You do not need to stake tokens or run any infrastructure.

---

## Integration Options

| Method | Best For | Complexity |
|--------|----------|------------|
| **TypeScript SDK** | Node.js backends, web apps | Low |
| **Python SDK** | ML pipelines, data processing | Low |
| **REST API** | Quick prototypes, language-agnostic | Low |
| **Direct RPC** | Custom Solana integrations | Medium |

---

## Quick Start with TypeScript SDK

### Install

```bash
npm install @dfpn/sdk @solana/web3.js
```

### Connect

```typescript
import { DFPNClient, Modality } from '@dfpn/sdk';
import { Keypair } from '@solana/web3.js';

const client = new DFPNClient({
  network: 'devnet',
  wallet: Keypair.fromSecretKey(/* your keypair bytes */),
});
```

### Submit a Request

```typescript
const request = await client.submitRequest({
  mediaPath: './photo.jpg',
  modalities: [Modality.FaceManipulation, Modality.ImageAuthenticity],
  minWorkers: 3,
  maxFee: 0.01,  // SOL
  deadline: Date.now() + 300_000,  // 5 minutes
});

console.log('Request ID:', request.id);
```

### Wait for the Result

```typescript
const result = await client.waitForResult(request.id, {
  timeout: 360_000,  // 6 minutes
});

console.log('Verdict:', result.verdict);
console.log('Confidence:', result.confidence);
console.log('Workers:', result.workerResults.length);
```

---

## Python SDK

```python
from dfpn import DFPNClient, Modality
from solders.keypair import Keypair

client = DFPNClient(
    network="devnet",
    wallet=Keypair.from_bytes(wallet_bytes),
)

request = client.submit_request(
    media_path="./photo.jpg",
    modalities=[Modality.FACE_MANIPULATION],
    min_workers=3,
    max_fee=0.01,
    deadline_seconds=300,
)

result = client.wait_for_result(request.id, timeout=360)

print(f"Verdict: {result.verdict}")
print(f"Confidence: {result.confidence}%")
```

---

## Fee Tiers

Fees vary by media type and complexity. These are baseline per-request fees:

| Modality | Base Fee (SOL) | Typical Processing Time |
|----------|----------------|------------------------|
| Image Authenticity | ~0.002 | 30-60 seconds |
| Face Manipulation | ~0.003 | 45-90 seconds |
| AI-Generated Image | ~0.003 | 45-90 seconds |
| Video Authenticity | ~0.008 | 60-180 seconds |
| Voice Cloning | ~0.004 | 30-60 seconds |

!!! info "Fees are dynamic"
    Actual fees depend on worker availability, request priority, and the number of workers you require. Use `client.estimateFee()` to get a current estimate before submitting.

Fees are split across network participants:

| Recipient | Share |
|-----------|-------|
| Workers | 65% |
| Model Developers | 20% |
| Treasury | 10% |
| Insurance Pool | 5% |

---

## Request Lifecycle

Here is what happens after you submit a request:

```
You submit       Workers         Workers         Network         You receive
a request   -->  analyze    -->  commit     -->  aggregates -->  a result
                 the media       & reveal        consensus
```

1. **Submit** -- Your request is recorded on-chain with the content hash, storage URI, fee, and deadline.
2. **Route** -- The network matches your request to workers who support the requested modalities.
3. **Analyze** -- Workers download your media and run their detection models.
4. **Commit** -- Each worker submits a cryptographic hash of their result (no one can see others' answers).
5. **Reveal** -- After all commits are in, workers reveal their actual results.
6. **Aggregate** -- The network combines results using reputation-weighted voting.
7. **Finalize** -- The consensus result is recorded on-chain and fees are distributed.

---

## Result Format

Every completed request returns a structured result:

```json
{
  "verdict": "Manipulated",
  "confidence": 87,
  "consensusType": "Majority",
  "workerResults": [
    {
      "worker": "7xKw...3nPq",
      "modelId": "face-forensics-sbi",
      "verdict": "Manipulated",
      "confidence": 92,
      "detections": [
        {
          "type": "face_swap",
          "region": { "x": 120, "y": 80, "w": 200, "h": 200 },
          "confidence": 94
        }
      ]
    }
  ],
  "audit": {
    "requestTx": "5Uj2...kLm9",
    "finalizeTx": "8Pq1...wNx4",
    "workerCount": 5,
    "commitCount": 5,
    "revealCount": 5
  }
}
```

### Verdict Values

| Verdict | Meaning |
|---------|---------|
| `Authentic` | No manipulation detected; media appears genuine |
| `Manipulated` | Manipulation detected with supporting evidence |
| `Inconclusive` | Workers could not reach consensus or confidence is low |

### Confidence Score

The confidence score ranges from **0 to 100**:

- **80-100**: High confidence in the verdict
- **50-79**: Moderate confidence; consider manual review
- **Below 50**: Low confidence; treat as inconclusive

### Detections Array

The `detections` array contains specific findings from each worker, including manipulation type, affected region (for images/video), and per-detection confidence.

---

## Use Cases

### Content Moderation

Automatically screen user-uploaded media before publication. Flag manipulated content for human review.

```typescript
if (result.verdict === 'Manipulated' && result.confidence > 80) {
  flagForReview(mediaId, result);
}
```

### Journalism and Fact-Checking

Verify the authenticity of photos and videos before publication. Use the on-chain audit trail as evidence of verification.

### Identity Verification

Detect face-swapped or AI-generated photos in identity documents and selfie verification flows.

### Social Media Platforms

Integrate detection into upload pipelines to label or restrict synthetic media, giving users transparency about content authenticity.

---

## Error Handling

```typescript
import { DFPNError, ErrorCode } from '@dfpn/sdk';

try {
  const result = await client.submitRequest({ /* ... */ });
} catch (error) {
  if (error instanceof DFPNError) {
    switch (error.code) {
      case ErrorCode.INSUFFICIENT_FUNDS:
        // Not enough SOL for the fee
        break;
      case ErrorCode.NO_WORKERS_AVAILABLE:
        // No workers online for requested modalities
        break;
      case ErrorCode.DEADLINE_TOO_SHORT:
        // Deadline must be at least 60 seconds
        break;
      case ErrorCode.CONTENT_HASH_MISMATCH:
        // Uploaded file does not match the provided hash
        break;
    }
  }
}
```

!!! tip "Set realistic deadlines"
    Video analysis can take several minutes. Set deadlines of at least 3 minutes for images and 5-10 minutes for video to give workers enough time to process and go through the commit-reveal cycle.
