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
| **TypeScript SDK (`@dfpn/sdk`)** | Node.js backends, web apps | Low |
| **REST API** (via the indexer) | Quick prototypes, language-agnostic reads | Low |
| **Direct Solana RPC** | Custom integrations, other languages | Medium |

!!! note "Language support today"
    Only the TypeScript SDK ships in this repository (`sdk/dfpn-sdk`). For other languages, build instructions and discriminators in the `instructions.ts` module are a reasonable starting point for porting -- or talk to the indexer's REST API directly.

---

## Quick Start with the TypeScript SDK

### Install

```bash
npm install @dfpn/sdk @solana/web3.js
```

### Connect

The `DFPNClient` constructor takes a `Connection`, a `Wallet`, and optional `ClientOptions` (including overrides for program IDs). The `Wallet` shape matches the `@solana/wallet-adapter` interface: a `publicKey` plus `signTransaction` / `signAllTransactions`.

```typescript
import { Connection, Keypair } from '@solana/web3.js';
import { DFPNClient } from '@dfpn/sdk';

const connection = new Connection('https://api.devnet.solana.com', 'confirmed');
const payer = Keypair.generate(); // or load your real keypair

const wallet = {
  publicKey: payer.publicKey,
  signTransaction: async (tx) => { tx.partialSign(payer); return tx; },
  signAllTransactions: async (txs) => txs.map((tx) => { tx.partialSign(payer); return tx; }),
};

const client = new DFPNClient(connection, wallet);
```

### Create a Request

`createRequest` takes a `CreateRequestParams` object. The content hash is a 32-byte SHA-256 of the media bytes; helpers like `computeContentHash` and `modalitiesToBits` are re-exported from the SDK.

```typescript
import { Modality, computeContentHash } from '@dfpn/sdk';
import { readFile } from 'node:fs/promises';

const mediaBytes = await readFile('./photo.jpg');
const contentHash = computeContentHash(mediaBytes); // Uint8Array of length 32

const { requestId, signature } = await client.createRequest({
  contentHash,
  storageUri: 'https://your-bucket.example/photo.jpg',
  modalities: [Modality.FaceManipulation, Modality.ImageAuthenticity],
  minWorkers: 3,
  feeAmount: 5_000_000_000n,           // DFPN base units (9 decimals)
  deadline: new Date(Date.now() + 5 * 60_000),
  priority: 'standard',                 // optional: 'standard' | 'high' | 'urgent'
});

console.log('Request ID:', requestId.toBase58());
console.log('Tx signature:', signature);
```

### Wait for the Result

`waitForResult` polls until the request status becomes `Finalized`, then returns an `AnalysisResult` assembled from the on-chain reveals.

```typescript
import { RequestStatus } from '@dfpn/sdk';

const result = await client.waitForResult(requestId, {
  timeout: 6 * 60_000,
  pollInterval: 5_000,
  onStatusUpdate: (status) => console.log('Status:', RequestStatus[status]),
});

console.log('Verdict:', result.verdict);
console.log('Confidence:', result.confidence);
console.log('Worker results:', result.workerResults.length);
```

Other useful methods on `DFPNClient`: `getRequest`, `getRequestStatus`, `getResult`, `cancelRequest`, `listOpenRequests`, `registerContent`, `getContent`, `listWorkers`, `getWorker`, `listModels`, `getModel`.

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

The TypeScript SDK throws plain `Error` instances; failures from on-chain programs surface with the program error name and Anchor error code. Wrap calls in `try`/`catch` and inspect the message and the failing transaction signature:

```typescript
try {
  const { requestId } = await client.createRequest(params);
  const result = await client.waitForResult(requestId, { timeout: 6 * 60_000 });
  console.log(result.verdict);
} catch (err) {
  // Common cases:
  //   "Timeout waiting for result"      -> increase deadline / timeout
  //   "Request expired" / "cancelled"   -> request finalised without consensus
  //   Anchor program errors             -> surface from the marketplace program
  console.error('DFPN request failed:', err);
}
```

The `analysis-marketplace` program defines the structured error codes (insufficient fee, deadline-too-short, modality mismatch, etc.) -- see [`programs/analysis-marketplace/src/lib.rs`](https://github.com/cryptuon/dfpn/blob/main/programs/analysis-marketplace/src/lib.rs) for the authoritative list.

!!! tip "Set realistic deadlines"
    Video analysis can take several minutes. Set deadlines of at least 3 minutes for images and 5-10 minutes for video to give workers enough time to process and go through the commit-reveal cycle.
