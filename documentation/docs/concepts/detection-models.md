# Detection Models

DFPN ships with four pre-configured detection models that cover the most common deepfake and synthetic-media threats. Each model is maintained as a standalone module under the `models/` directory and can be run by any worker node.

---

## Pre-configured Models

### face-forensics -- Face Manipulation Detection

Detects face-swap and reenactment forgeries in still images using the **Self-Blended Images (SBI)** training strategy on an **EfficientNet-B4** backbone.

| Benchmark | Accuracy |
|-----------|----------|
| FaceForensics++ (c23) | **97.2%** |
| Celeb-DF v2 | 91.4% |

**Processing speed:** ~50 ms on GPU / ~500 ms on CPU

!!! tip "Best for"
    Profile photos, identity documents, social-media headshots -- any image where a face is the primary subject.

---

### universal-fake-detect -- AI-Generated Image Detection

A CLIP-based classifier (**CLIP-ViT-L/14**) fine-tuned to separate real photographs from outputs of modern generative models.

| Generator | Accuracy |
|-----------|----------|
| ProGAN | **99.8%** |
| Stable Diffusion | 89.4% |
| DALL-E | 85.7% |

**Processing speed:** ~100 ms on GPU / ~800 ms on CPU

!!! tip "Best for"
    Identifying fully synthetic images produced by diffusion models, GANs, or other generative pipelines.

---

### video-ftcn -- Video Authenticity Detection

Combines an **Xception** frame-level feature extractor with a **Temporal CNN** to capture inter-frame inconsistencies that reveal video-level manipulation.

| Benchmark | Accuracy |
|-----------|----------|
| FaceForensics++ | **96.4%** |
| Celeb-DF v2 | 88.9% |

**Processing speed:** ~2 s on GPU / ~30 s on CPU

!!! warning "CPU note"
    Video analysis is extremely slow on CPU-only nodes. The default CPU configuration disables this modality. See [CPU-only configuration](../reference/configuration.md#cpu-only-configuration) for details.

---

### ssl-antispoofing -- Voice Cloning Detection

Leverages **wav2vec 2.0 / XLSR-53** self-supervised speech representations to detect synthetic and cloned voices.

| Benchmark | Accuracy |
|-----------|----------|
| ASVspoof 2021 (LA) | **99.2%** |

**Processing speed:** ~200 ms on GPU / ~2 s on CPU

!!! tip "Best for"
    Voice messages, phone-call recordings, podcast clips -- any audio where speaker authenticity matters.

---

## Performance Comparison

### Accuracy by model

| Model | Primary Benchmark | Accuracy | Secondary Benchmark | Accuracy |
|-------|-------------------|----------|---------------------|----------|
| face-forensics | FF++ (c23) | 97.2% | Celeb-DF v2 | 91.4% |
| universal-fake-detect | ProGAN | 99.8% | Stable Diffusion | 89.4% |
| video-ftcn | FF++ | 96.4% | Celeb-DF v2 | 88.9% |
| ssl-antispoofing | ASVspoof 2021 | 99.2% | -- | -- |

### Processing speed

| Model | GPU Latency | CPU Latency | GPU Required? |
|-------|-------------|-------------|---------------|
| face-forensics | 50 ms | 500 ms | Recommended |
| universal-fake-detect | 100 ms | 800 ms | Recommended |
| video-ftcn | 2 s | 30 s | Strongly recommended |
| ssl-antispoofing | 200 ms | 2 s | Recommended |

---

## Supported Modalities

Every model, worker, and analysis request declares which **modalities** it supports using a bitfield. This allows efficient on-chain filtering without string comparison.

| Modality | Description | Bit Value |
|----------|-------------|-----------|
| `ImageAuthenticity` | Real vs. AI-generated image classification | `1` (bit 0) |
| `VideoAuthenticity` | Temporal forgery detection in video | `2` (bit 1) |
| `AudioAuthenticity` | General audio manipulation detection | `4` (bit 2) |
| `FaceManipulation` | Face-swap and reenactment detection | `8` (bit 3) |
| `VoiceCloning` | Synthetic / cloned voice detection | `16` (bit 4) |
| `GeneratedContent` | Fully AI-generated media detection | `32` (bit 5) |

Combine values with bitwise OR. For example, a worker that handles face manipulation and AI-generated images advertises modalities = `8 | 1 | 32` = **41**.

??? example "Model-to-modality mapping"
    | Model | Modalities |
    |-------|------------|
    | face-forensics | `FaceManipulation` (8) |
    | universal-fake-detect | `ImageAuthenticity` (1) + `GeneratedContent` (32) = 33 |
    | video-ftcn | `VideoAuthenticity` (2) |
    | ssl-antispoofing | `VoiceCloning` (16) |

---

## Standardized Output Format

All models produce a JSON result with the same envelope so that workers and on-chain aggregation logic can treat them uniformly.

```json
{
  "verdict": "manipulated",
  "confidence": 0.973,
  "detections": [
    {
      "type": "face_swap",
      "confidence": 0.973,
      "region": {
        "x": 120,
        "y": 80,
        "width": 256,
        "height": 256
      },
      "metadata": {
        "model": "face-forensics-sbi",
        "version": "1.0.0"
      }
    }
  ]
}
```

| Field | Type | Description |
|-------|------|-------------|
| `verdict` | string | One of `authentic`, `manipulated`, or `inconclusive` |
| `confidence` | float | Overall confidence score between 0.0 and 1.0 |
| `detections` | array | Individual findings, each with its own confidence and optional spatial region |
| `detections[].type` | string | Detection category (e.g. `face_swap`, `generated_image`, `voice_clone`) |
| `detections[].region` | object | Bounding box for spatial detections (images/video); omitted for audio |
| `detections[].metadata` | object | Model identifier and version that produced this detection |
