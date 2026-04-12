---
title: "Deepfake Detection API: Centralized vs Decentralized Approaches"
description: "Comparing centralized deepfake detection APIs with DFPN's decentralized protocol: reliability, cost, transparency, and integration options."
publishedAt: 2026-04-10
author: "DFPN Team"
tags: ["API", "deepfake detection", "comparison", "integration"]
---

A deepfake detection API is a programmatic interface that accepts media files (images, video, or audio) and returns a classification indicating whether the content is authentic or synthetically generated. Centralized detection APIs are operated by single companies running proprietary models on their own infrastructure. The Decentralized Fake Proof Network (DFPN) offers an alternative: a decentralized detection protocol where multiple independent workers analyze media through a commit-reveal consensus mechanism on the Solana blockchain, with results that are transparent, verifiable, and resistant to manipulation. DFPN supports four detection modalities with accuracy rates from 96.4% to 99.8% and per-request costs of approximately **0.002-0.008 SOL**.

## What Detection APIs Are Available Today?

The deepfake detection API landscape includes both established centralized services and DFPN's decentralized protocol:

**Centralized services** typically offer a REST API endpoint where clients upload media and receive a JSON response containing a classification score. They run proprietary models on cloud infrastructure and charge through monthly subscription tiers or per-request pricing.

**DFPN** provides the same functional interface -- submit media, receive a classification -- but the underlying architecture distributes analysis across independent workers who stake economic collateral and submit results through a cryptographic commit-reveal protocol. Clients interact through TypeScript and Python SDKs, a REST API gateway, or direct Solana RPC calls.

## How Do Centralized and Decentralized APIs Compare?

| Feature | Centralized API | DFPN Decentralized Protocol |
|---|---|---|
| **Detection models** | Single proprietary model per request | Multiple independent models across workers |
| **Model transparency** | Closed source; model details undisclosed | Open model registry with published benchmarks |
| **Result verification** | Trust the provider | On-chain proof; results independently verifiable |
| **Availability** | Single point of failure; dependent on provider uptime | Distributed; tolerates individual worker failures |
| **Pricing model** | Fixed monthly tiers ($99-$999/mo typical) | Per-request market pricing (~0.002-0.008 SOL) |
| **Minimum commitment** | Monthly subscription | Pay per request; no minimum |
| **Censorship resistance** | Provider can refuse service | Protocol-enforced; permissionless access |
| **Accountability** | Reputational only | Economic staking and slashing |
| **Latency** | 50-200ms typical | 200ms-2s (includes consensus overhead) |
| **Data handling** | Media processed on provider servers | Workers process locally; media not stored on-chain |
| **Multi-modal support** | Varies; many cover only images | 4 modalities: face, AI image, video, voice |
| **Scalability** | Provider provisions capacity | Elastic; workers join when demand rises |

## What Are DFPN's Integration Options?

DFPN offers multiple integration paths to accommodate different development environments and use cases:

### TypeScript SDK

The TypeScript SDK is the recommended integration for Node.js and browser-based applications. It handles connection management, request signing, and result polling.

```typescript
import { DFPNClient, DetectionType } from '@dfpn/sdk';

// Initialize the client with a Solana wallet
const client = new DFPNClient({
  network: 'mainnet-beta',
  wallet: walletAdapter, // Solana wallet adapter
});

// Submit an image for AI-generated image detection
const result = await client.detect({
  media: imageBuffer,
  type: DetectionType.AI_GENERATED_IMAGE,
  consensusThreshold: 3, // Require 3/4 workers to agree
  maxWorkers: 4,
});

console.log(result.classification); // 'authentic' | 'synthetic'
console.log(result.confidence);     // 0.0 - 1.0
console.log(result.workerResults);  // Individual worker scores
console.log(result.txSignature);    // Solana transaction for verification
```

### Python SDK

The Python SDK provides the same functionality for Python applications, with support for NumPy arrays, PIL Images, and file paths as input.

```python
from dfpn import DFPNClient, DetectionType
from solders.keypair import Keypair

# Initialize with a Solana keypair
keypair = Keypair.from_bytes(private_key_bytes)
client = DFPNClient(network="mainnet-beta", keypair=keypair)

# Submit audio for voice cloning detection
result = client.detect(
    media="path/to/audio.wav",
    detection_type=DetectionType.VOICE_CLONE,
    consensus_threshold=3,
    max_workers=4,
)

print(f"Classification: {result.classification}")
print(f"Confidence: {result.confidence:.3f}")
print(f"Worker count: {len(result.worker_results)}")
print(f"Transaction: {result.tx_signature}")
```

### REST API gateway

For applications that cannot use the native SDKs, DFPN provides a REST API gateway that wraps the on-chain protocol. The gateway handles wallet management and transaction signing on behalf of the client.

```bash
curl -X POST https://api.dfpn.network/v1/detect \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: multipart/form-data" \
  -F "media=@image.jpg" \
  -F "type=ai_generated_image" \
  -F "consensus_threshold=3" \
  -F "max_workers=4"
```

Response:

```json
{
  "classification": "synthetic",
  "confidence": 0.967,
  "worker_results": [
    { "worker": "7xKp...3nRm", "classification": "synthetic", "confidence": 0.982 },
    { "worker": "9aFb...7kLp", "classification": "synthetic", "confidence": 0.951 },
    { "worker": "3mNq...8wXr", "classification": "synthetic", "confidence": 0.974 },
    { "worker": "5jRt...2bYs", "classification": "authentic", "confidence": 0.523 }
  ],
  "consensus": "3/4 workers agree: synthetic",
  "tx_signature": "4vJ9...xK2m",
  "processing_time_ms": 1247
}
```

### Direct Solana RPC

Advanced users can interact with DFPN's Solana program directly through RPC calls. This provides maximum control and eliminates dependency on gateway services, but requires handling transaction construction and account management manually.

## What Does DFPN Cost Compared to Centralized APIs?

Cost comparisons depend on usage volume. Centralized services typically offer tiered pricing that favors high-volume users, while DFPN charges per request with no minimum commitment.

| Monthly volume | Centralized API (typical) | DFPN (at 0.003 SOL/request, ~$0.37) |
|---|---|---|
| 100 requests | $99 (starter tier) | ~$37 |
| 1,000 requests | $99-$299 (starter/pro tier) | ~$370 |
| 10,000 requests | $299-$999 (pro/enterprise tier) | ~$3,700 |
| 100,000 requests | $2,000-$5,000 (enterprise) | ~$37,000 |
| 1,000,000 requests | Custom pricing | ~$370,000 |

At low volumes (under ~270 requests/month), DFPN's per-request pricing is more economical than centralized subscription tiers. At high volumes, centralized subscriptions become more cost-effective per request, though DFPN provides additional value through multi-model consensus, on-chain transparency, and censorship resistance that centralized services cannot match.

DFPN fees fluctuate with SOL price and network demand. Workers set their own minimum fees, and the protocol matches requests to workers based on price and performance history.

## What Accuracy Can Each Approach Deliver?

Centralized APIs typically report accuracy figures from their internal benchmarks, which may not reflect production conditions. DFPN publishes benchmark results for each supported model and records all detection outcomes on-chain, allowing independent accuracy auditing.

| Detection type | DFPN model | DFPN benchmark accuracy | Centralized range (reported) |
|---|---|---|---|
| Face manipulation | EfficientNet-B4 SBI | 97.2% (FF++ c23) | 90-97% |
| AI-generated images | CLIP-ViT-L/14 | 99.8% (ProGAN) | 92-99% |
| Video deepfakes | Temporal CNN | 96.4% (FaceForensics++) | 88-95% |
| Voice cloning | wav2vec 2.0 XLSR-53 | 99.2% (ASVspoof 2021) | 90-98% |

The key accuracy advantage of DFPN's approach is not necessarily higher single-model accuracy (centralized services may use comparable models) but rather the **multi-model consensus** that reduces false positives and false negatives. When multiple independent workers analyze the same media and agree, the effective accuracy exceeds any individual model's benchmark.

## When Should You Choose a Centralized API?

Centralized APIs remain the better choice in specific scenarios:

- **Ultra-low latency requirements**: If your application needs sub-100ms detection latency and consensus verification overhead is unacceptable, a centralized API with a single model provides faster responses.
- **Very high volume with cost sensitivity**: At millions of requests per month, centralized enterprise pricing is typically more economical per request than DFPN's per-request model.
- **Simplicity**: If you need a single REST endpoint with no blockchain interaction, centralized APIs require less integration effort (though DFPN's REST gateway largely closes this gap).
- **Regulatory requirements**: Some regulated industries require that data processing occurs in specific geographic jurisdictions. Centralized providers can guarantee data residency; DFPN's distributed workers may process data in multiple jurisdictions.

## When Should You Choose DFPN?

DFPN is the stronger choice when:

- **Transparency matters**: You need verifiable proof that detection was performed correctly, such as for legal proceedings, journalism, or regulatory compliance.
- **Censorship resistance is critical**: You operate in an environment where a centralized provider might be pressured to alter results or refuse service.
- **Multi-model consensus improves your use case**: High-stakes decisions (content moderation at scale, fraud prevention, evidence verification) benefit from multiple independent analyses rather than a single model's opinion.
- **You want no vendor lock-in**: DFPN is an open protocol. You can switch between SDKs, gateways, or direct RPC without changing providers. Workers compete on quality rather than locking you into a proprietary ecosystem.
- **Pay-per-request pricing fits your pattern**: For applications with variable or unpredictable detection volume, DFPN's per-request pricing avoids paying for unused capacity.
- **You need multi-modal detection**: DFPN covers face manipulation, AI-generated images, video, and voice through a single protocol, avoiding the need to integrate with multiple centralized providers.

## How Do You Get Started with DFPN?

Integration follows three steps:

1. **Set up a Solana wallet** -- DFPN transactions require a Solana wallet funded with SOL for detection fees. Any Solana-compatible wallet works.

2. **Install the SDK** -- Choose the TypeScript SDK (`npm install @dfpn/sdk`) or Python SDK (`pip install dfpn`) based on your application's language.

3. **Submit your first detection request** -- Use the code examples above to submit an image, video, or audio file for analysis. The SDK handles worker selection, commit-reveal coordination, and result aggregation.

The [DFPN documentation](/docs) includes quickstart guides, API references, and example applications for common integration patterns including content moderation pipelines, upload verification workflows, and real-time media monitoring systems.

## Learn More About DFPN

- Read the [integration quickstart](/docs/quickstart) to submit your first detection request in under 5 minutes
- Explore the [SDK documentation](/docs/sdk) for TypeScript and Python API references
- Review the [whitepaper](/whitepaper) for the full protocol specification
- Check the [network dashboard](https://dashboard.dfpn.network) for live statistics on worker count, request volume, and average detection latency
