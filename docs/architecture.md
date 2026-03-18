# Architecture

This document describes a practical architecture for a decentralized deepfake detection coordination layer that runs on Solana.

## Design Philosophy

DFPN is a **tracking and coordination system**, not a model provider or inference platform. The network:

- **Coordinates** independent node operators who bring their own detection models and GPU infrastructure
- **Tracks** request lifecycle, result submissions, and worker reputation
- **Incentivizes** honest behavior through staking, rewards, and slashing
- **Aggregates** results from multiple independent workers into consensus verdicts

Node operators choose their own algorithms, manage their own hardware, and compete on detection quality. DFPN provides the economic rails that make this coordination trustworthy.

## System Components

- **Clients**: Submit media, check authenticity, and retrieve audit trails.
- **Content Providers**: Register originals and proofs of authorship.
- **Model Developers**: Publish detection model metadata; models run on operator infrastructure.
- **Node Operators**: Run their own models and GPUs, execute inference, and post signed results.
- **Solana Programs**: Coordination, tracking, scoring, staking, and rewards.
- **Storage**: Off-chain media and datasets (IPFS/Arweave/S3), with hashes on-chain.
- **Indexers**: Read Solana state and provide APIs for fast search.

### What DFPN Controls vs. What Operators Control

| DFPN (On-chain) | Node Operators (Off-chain) |
|-----------------|---------------------------|
| Request routing | Model selection |
| Result tracking | Inference execution |
| Reputation scores | GPU/CPU infrastructure |
| Reward distribution | Algorithm updates |
| Stake management | Operational decisions |

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

- **Node Operator Infrastructure** (operator-managed)
  - Operators run their own detection models on their own hardware.
  - Pull tasks from DFPN, run inference locally, submit signed results.
  - Choose which models to run, how to scale, and which requests to serve.

- **Evaluation Harness** (protocol-managed)
  - Benchmarks registered models on held-out datasets.
  - Produces score reports for on-chain reputation updates.
  - Does not run production inference (only periodic evaluation).

- **Indexer + API** (protocol-managed)
  - Builds fast queries for clients and dashboards.
  - Mirrors on-chain data without being a source of truth.

## Pre-configured Detection Models

DFPN provides ready-to-use detection models covering all supported modalities:

| Model | Modality | Architecture | Backend |
|-------|----------|--------------|---------|
| `face-forensics` | Face Manipulation | SBI/EfficientNet-B4 | Python subprocess |
| `universal-fake-detect` | AI-Generated Images | CLIP-ViT-L/14 | Python subprocess |
| `video-ftcn` | Video Authenticity | Xception + Temporal | Docker HTTP service |
| `ssl-antispoofing` | Voice Cloning | wav2vec 2.0/XLSR | Python subprocess |

### Model Integration Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    DFPN Worker Daemon                        │
│  ┌─────────────────────────────────────────────────────────┐│
│  │                   Task Manager                           ││
│  │  - Polls indexer for tasks                              ││
│  │  - Selects model by modality                            ││
│  │  - Manages commit-reveal protocol                       ││
│  └───────────────────┬─────────────────────────────────────┘│
│                      │                                       │
│  ┌───────────────────┴───────────────────────────────────┐  │
│  │               Detector Interface                       │  │
│  │  async fn analyze(&self, path: &Path) -> AnalysisResult│  │
│  └───┬─────────────┬─────────────┬─────────────┬─────────┘  │
│      │             │             │             │             │
│  ┌───┴───┐    ┌────┴───┐   ┌────┴───┐   ┌────┴───┐        │
│  │External│    │  HTTP  │   │ ONNX  │   │ Candle │        │
│  │Detector│    │Detector│   │Detector│   │Detector│        │
│  │(Python)│    │ (API)  │   │(native)│   │(native)│        │
│  └───┬───┘    └────┬───┘   └────────┘   └────────┘        │
│      │             │        (disabled)   (disabled)         │
└──────┼─────────────┼────────────────────────────────────────┘
       │             │
       ▼             ▼
┌──────────┐  ┌─────────────┐
│ Python   │  │   Docker    │
│inference │  │  Container  │
│  .py     │  │  (video)    │
└──────────┘  └─────────────┘
```

### Model Output Format

All models output standardized JSON:

```json
{
  "verdict": "authentic" | "manipulated" | "inconclusive",
  "confidence": 0-100,
  "detections": [{
    "detection_type": "face_manipulation" | "ai_generated_image" | ...,
    "confidence": 0-100,
    "region": {"x": 0, "y": 0, "width": 100, "height": 100}
  }]
}
```

See [Detection Models Guide](detection-models.md) for detailed documentation.

## Data Flow

1. **Register Original**: Content providers upload media off-chain and store content hashes on-chain.
2. **Submit Analysis Request**: Clients post a hash and metadata, pay a fee, and set a deadline.
3. **Worker Inference**: Workers retrieve the media from storage, run models, and submit signed results.
4. **Consensus/Scoring**: Results are aggregated using a scoring policy and worker reputation.
5. **Reward Distribution**: Treasury sends rewards based on accuracy and stake commitments.
6. **Audit Trail**: Anyone can inspect the request, results, and model versions used.

## Security Model

- **No on-chain inference**; trust comes from economic incentives and redundancy.
- **Operator independence**; DFPN doesn't control models, so no single point of failure.
- **Commit-reveal** prevents result copying and front-running between workers.
- **Staking + slashing** enforces honest behavior for workers and model developers.
- **Epoch scoring** limits on-chain compute and keeps fees predictable.
- **Multi-worker consensus** aggregates independent analyses for reliability.

## Scalability Notes

- Keep program state small: store hashes, not content.
- Use Solana account compression or off-chain indexing for large registries.
- Prefer batch settlement for rewards and scoring updates.
