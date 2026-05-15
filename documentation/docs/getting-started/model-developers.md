# Registering Detection Models

Model developers build deepfake detection algorithms and register them on the DFPN network. When workers adopt your model and use it to process requests, you earn a share of every fee.

---

## What Model Developers Do

As a model developer, you:

1. Train a detection model for one or more supported modalities
2. Package it with a standard output format
3. Host the model files at a publicly accessible URI
4. Register the model on-chain with metadata and a DFPN stake
5. Pass automated benchmarks to activate the model
6. Earn revenue when workers use your model to process requests

You retain full ownership of your model. DFPN only stores metadata and performance records on-chain.

---

## Requirements

Before registering a model, make sure you meet these requirements:

| Requirement | Details |
|-------------|---------|
| **Stake** | 20,000 DFPN per model version |
| **Output format** | Standard JSON result format (see below) |
| **Benchmarks** | Model must pass minimum accuracy thresholds on held-out test sets |
| **Hosting** | Model files available at a stable URI (IPFS, S3, HTTP) |
| **Modality support** | At least one supported modality declared |

!!! info "Stake protects the network"
    The 20,000 DFPN stake ensures model developers have skin in the game. If your model consistently produces poor results, stake can be slashed. The stake is returned when you retire a model version in good standing.

---

## Model Lifecycle

Your model goes through these stages after registration:

```
Submit          Benchmark       Activation      Adoption        Revenue
metadata   -->  on held-out -->  model goes -->  workers    -->  you earn
and stake       test sets       live             adopt it        fees
```

### 1. Submit Metadata

You register your model on-chain with its name, version, supported modalities, and a URI where workers can download it.

### 2. Benchmark

The protocol evaluation harness tests your model on held-out datasets. These datasets are rotated periodically to prevent overfitting. Your model must meet minimum accuracy thresholds for each declared modality.

### 3. Activation

Models that pass benchmarks become active in the registry. They are now visible to all workers on the network.

### 4. Adoption

Workers independently choose which models to download and run. Better-performing models attract more workers, which means more request volume flowing through your model.

### 5. Revenue

You earn a share of fees from every request processed using your model, plus epoch-based reward distributions.

---

## Registration Steps

### Step 1: Prepare Your Model

Ensure your model:

- Accepts standard media input (image, video, or audio files)
- Produces output in the required JSON format (see [Output Format](#output-format))
- Can be downloaded and run by workers independently
- Includes a setup script or Docker configuration for easy deployment

### Step 2: Host Model Files

Upload your model weights and configuration to a stable, publicly accessible location:

- **IPFS** (recommended for decentralization): `ipfs://Qm...`
- **S3-compatible storage**: `https://your-bucket.s3.amazonaws.com/model-v1.tar.gz`
- **HTTP endpoint**: `https://your-site.com/models/detector-v1.tar.gz`

!!! warning "URI must remain accessible"
    Workers need to download your model files at any time. If the URI goes down, workers cannot adopt your model and your revenue stops. Use a reliable hosting solution.

### Step 3: Register On-Chain

The `model-registry` program exposes a `register_model` instruction that takes the following arguments:

| Argument | Type | Description |
|----------|------|-------------|
| `model_id` | `[u8; 32]` | Caller-chosen unique ID (typically a hash of name + version) |
| `name` | `string` | Human-readable model name (max 64 chars) |
| `version` | `string` | Semantic version string (max 32 chars) |
| `modalities` | `u8` | Bitfield of supported modalities (see [Detection Models](../concepts/detection-models.md#supported-modalities)) |
| `model_uri` | `string` | Stable URI where workers can download model artefacts |
| `checksum` | `[u8; 32]` | SHA-256 of the model archive at `model_uri` |
| `stake_amount` | `u64` | DFPN base units to stake (>= `MIN_MODEL_STAKE` from the SDK constants) |

You can build the instruction directly with Anchor IDLs, or with the lower-level helpers in `@dfpn/sdk` (`SEEDS`, `PROGRAM_IDS`, `deriveModelPDA`, `MIN_MODEL_STAKE`). A high-level `client.registerModel` helper on `DFPNClient` is on the roadmap but not yet shipped -- track the [Roadmap](../community/roadmap.md).

!!! note "Anchor client"
    Until the helper lands, the most reliable path is to consume the program's Anchor IDL (`anchor build` writes it to `target/idl/model_registry.json`) and call `program.methods.registerModel(...)` with `@coral-xyz/anchor`.

### Step 4: Stake DFPN

The stake is moved into a program-owned `stake_vault` PDA as part of the `register_model` transaction. Make sure the developer wallet's DFPN associated token account holds at least `MIN_MODEL_STAKE` (20,000 DFPN, in base units) and enough SOL to pay the rent and transaction fee.

### Step 5: Benchmarking and Activation

Today, the model becomes visible in the registry as soon as the `register_model` transaction confirms. Automated on-chain benchmark scoring and an explicit `Active` activation step are part of the Testnet Pilot phase -- see the [Roadmap](../community/roadmap.md). Until then, model quality is signalled by the model's `score` field, which is updated as workers report results.

!!! tip "Test locally first"
    Run your model against public benchmark datasets before registering. Re-registering a new version requires a separate stake.

---

## Output Format

All models must produce results in this standard JSON format:

```json
{
  "verdict": "Manipulated",
  "confidence": 92,
  "detections": [
    {
      "type": "face_swap",
      "confidence": 94,
      "region": {
        "x": 120,
        "y": 80,
        "width": 200,
        "height": 200
      },
      "metadata": {
        "technique": "first-order-motion",
        "source_model": "FaceForensics-SBI-v1"
      }
    }
  ]
}
```

| Field | Type | Description |
|-------|------|-------------|
| `verdict` | string | `Authentic`, `Manipulated`, or `Inconclusive` |
| `confidence` | integer | 0-100 confidence score |
| `detections` | array | List of specific findings (can be empty for `Authentic`) |
| `detections[].type` | string | Detection type (e.g., `face_swap`, `voice_clone`, `ai_generated`) |
| `detections[].confidence` | integer | 0-100 confidence for this specific detection |
| `detections[].region` | object | Spatial location in image/video (optional) |
| `detections[].metadata` | object | Additional model-specific details (optional) |

!!! danger "Non-standard output will fail verification"
    Workers serialize your model's output for the commit-reveal protocol. If the output does not match the standard format, the result will fail verification and the worker (not you) will be penalized -- which means workers will stop using your model.

---

## Revenue

Model developers earn from two sources:

### Per-Request Fees

When a worker processes a request using your model, you receive **20% of the request fee**. This is distributed automatically when the request is finalized on-chain.

### Epoch Rewards

A portion of each epoch's reward pool is allocated to model developers based on:

- Number of requests processed with your model
- Accuracy of results produced by your model
- Number of workers running your model

### Revenue Example

If your model is used for 1,000 requests per day at an average fee of 0.005 SOL:

- Daily fees: 1,000 x 0.005 x 0.20 = **1.0 SOL per day**
- Plus epoch reward distributions

---

## Supported Modalities

Register your model for one or more of these modalities:

| Modality | Description | Input Type |
|----------|-------------|------------|
| `ImageAuthenticity` | Detect tampered or spliced images | JPEG, PNG, WebP |
| `VideoAuthenticity` | Detect manipulated video frames or sequences | MP4, AVI, MOV |
| `AudioAuthenticity` | Detect tampered or spliced audio | WAV, MP3, FLAC |
| `FaceManipulation` | Detect face swaps, reenactment, morphing | Image or video |
| `VoiceCloning` | Detect synthetic or cloned speech | Audio files |
| `GeneratedContent` | Detect fully AI-generated media (GAN, diffusion) | Image, video, or audio |

A single model can declare multiple modalities if it supports them. Each modality is benchmarked independently.

---

## Versioning

When you improve your model, register a new version rather than updating the existing one. Bump the `version` string, derive a new `model_id`, and call `register_model` again with a fresh `stake_amount`:

- Each version requires a separate 20,000 DFPN stake
- Old versions remain active until you retire them
- Workers choose independently when to upgrade to new versions
- Performance metrics are tracked per version
