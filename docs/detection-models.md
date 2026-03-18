# DFPN Detection Models Guide

This guide documents the pre-configured detection models available for DFPN worker nodes. These models cover all supported modalities and provide state-of-the-art deepfake detection capabilities.

## Quick Start

```bash
# 1. Setup all models
./scripts/setup-models.sh

# 2. Test models
./scripts/test-models.sh

# 3. Start worker
./scripts/start-worker.sh
```

## Available Models

| Model | Modality | Architecture | Source |
|-------|----------|--------------|--------|
| [face-forensics](#face-forensics) | Face Manipulation | SBI/EfficientNet-B4 | [Self-Blended Images](https://github.com/mapooon/SelfBlendedImages) |
| [universal-fake-detect](#universal-fake-detect) | AI-Generated Images | CLIP-ViT-L/14 | [UniversalFakeDetect](https://github.com/WisconsinAIVision/UniversalFakeDetect) |
| [video-ftcn](#video-ftcn) | Video Authenticity | Xception + Temporal | [DeepfakeBench](https://github.com/SCLBD/DeepfakeBench) |
| [ssl-antispoofing](#ssl-antispoofing) | Voice Cloning | wav2vec 2.0/XLSR | [SSL Anti-spoofing](https://github.com/TakHemlata/SSL_Anti-spoofing) |

---

## Face Forensics

### Overview

Detects face manipulation including face swaps, reenactment, expression transfer, and other facial deepfakes.

**Location**: `models/face-forensics/`

**Modalities**: `face_manipulation`

**Architecture**: Self-Blended Images (SBI) approach with EfficientNet-B4 backbone

### How It Works

1. **Face Detection**: Uses MTCNN to locate faces in the image
2. **Face Extraction**: Crops face region with 30% margin for context
3. **Classification**: EfficientNet-B4 classifies as real or fake
4. **Output**: Returns verdict with confidence and face bounding box

### Performance

| Dataset | Accuracy | AUC |
|---------|----------|-----|
| FaceForensics++ (c23) | 97.2% | 0.99 |
| FaceForensics++ (c40) | 93.1% | 0.96 |
| Celeb-DF | 91.4% | 0.94 |
| DFDC | 88.7% | 0.92 |

### Usage

```bash
# Direct inference
python models/face-forensics/inference.py /path/to/image.jpg

# Output
{
  "verdict": "manipulated",
  "confidence": 94,
  "detections": [{
    "detection_type": "face_manipulation",
    "confidence": 94,
    "region": {"x": 120, "y": 80, "width": 200, "height": 250}
  }]
}
```

### Configuration

```yaml
models:
  - id: "face-forensics-sbi"
    path: ./models/face-forensics
    modalities: [face_manipulation]
    gpu_required: true
    runtime: external
```

### Dependencies

- PyTorch 2.0+
- EfficientNet-PyTorch
- FaceNet-PyTorch (MTCNN)
- OpenCV, Pillow

### Model Weights

Download with:
```bash
cd models/face-forensics
./download_weights.sh
```

Weights source: [SBI Official Release](https://github.com/mapooon/SelfBlendedImages#pretrained-models)

---

## Universal Fake Detect

### Overview

Detects AI-generated images from various generators including Stable Diffusion, DALL-E, Midjourney, StyleGAN, and other GANs/diffusion models.

**Location**: `models/universal-fake-detect/`

**Modalities**: `image_authenticity`, `generated_content`

**Architecture**: CLIP-ViT-L/14 feature extractor with trained linear classifier

### How It Works

1. **Feature Extraction**: Uses frozen CLIP-ViT-L/14 to extract 768-dim features
2. **Classification**: Linear layer trained on ProGAN data generalizes to unseen generators
3. **Output**: Binary classification with confidence score

### Key Feature

Uses **CLIP features** which capture semantic content, making it highly generalizable to unseen generation methods without retraining.

### Performance

| Generator | Accuracy | AUC |
|-----------|----------|-----|
| ProGAN | 99.8% | 1.00 |
| StyleGAN | 99.2% | 1.00 |
| StyleGAN2 | 98.5% | 0.99 |
| BigGAN | 97.1% | 0.99 |
| GauGAN | 96.3% | 0.98 |
| Stable Diffusion | 89.4% | 0.95 |
| DALL-E 2 | 85.7% | 0.92 |
| Midjourney | 83.2% | 0.90 |

### Usage

```bash
# Direct inference
python models/universal-fake-detect/inference.py /path/to/image.png

# Output
{
  "verdict": "manipulated",
  "confidence": 87,
  "detections": [{
    "detection_type": "ai_generated_image",
    "confidence": 87,
    "region": null
  }]
}
```

### Configuration

```yaml
models:
  - id: "universal-fake-detect"
    path: ./models/universal-fake-detect
    modalities: [image_authenticity, generated_content]
    gpu_required: true
    runtime: external
```

### Dependencies

- PyTorch 2.0+
- OpenAI CLIP
- Pillow

### Model Weights

Download with:
```bash
cd models/universal-fake-detect
./download_weights.sh
```

Weights source: [UniversalFakeDetect GitHub](https://github.com/WisconsinAIVision/UniversalFakeDetect)

---

## Video FTCN

### Overview

Detects video-level manipulation by analyzing temporal consistency across frames. Runs as a Docker microservice for isolation and scalability.

**Location**: `models/video-ftcn/`

**Modalities**: `video_authenticity`

**Architecture**: Frame-level face detection + Xception classification + temporal aggregation

### How It Works

1. **Frame Extraction**: Samples up to 32 frames uniformly from video
2. **Face Detection**: Detects faces in each frame using Haar cascades
3. **Frame Classification**: Xception network classifies each face crop
4. **Temporal Aggregation**: Aggregates predictions with outlier rejection
5. **Output**: Overall verdict with confidence

### Deployment

Runs as HTTP microservice via Docker:

```bash
# Build and start
cd models/video-ftcn
docker-compose up -d

# Service available at http://localhost:8001
```

### Performance

| Dataset | Accuracy | AUC |
|---------|----------|-----|
| FaceForensics++ | 96.4% | 0.98 |
| DFDC | 85.2% | 0.91 |
| Celeb-DF | 88.9% | 0.93 |

### Usage

```bash
# HTTP API call
curl -X POST -F "file=@/path/to/video.mp4" http://localhost:8001/analyze

# Output
{
  "verdict": "manipulated",
  "confidence": 82,
  "detections": [{
    "detection_type": "temporal_inconsistency",
    "confidence": 82,
    "region": null
  }],
  "metadata": {
    "frames_analyzed": 32,
    "faces_found": 28
  }
}
```

### API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/analyze` | POST | Upload video for analysis (multipart form) |
| `/health` | GET | Health check |
| `/` | GET | API information |

### Configuration

```yaml
models:
  - id: "video-ftcn"
    path: http://localhost:8001/analyze
    modalities: [video_authenticity]
    gpu_required: true
    runtime: http
```

### Docker Configuration

```yaml
# docker-compose.yml
services:
  video-ftcn:
    build: .
    ports:
      - "8001:8000"
    deploy:
      resources:
        reservations:
          devices:
            - driver: nvidia
              count: 1
              capabilities: [gpu]
```

### Model Weights

Download with:
```bash
cd models/video-ftcn
./download_weights.sh
```

---

## SSL Anti-spoofing

### Overview

Detects synthetic/cloned audio using self-supervised learning features from wav2vec 2.0 / XLSR.

**Location**: `models/ssl-antispoofing/`

**Modalities**: `voice_cloning`

**Architecture**: XLSR-53 (wav2vec 2.0) feature extractor with classification head

### How It Works

1. **Audio Loading**: Loads audio and resamples to 16kHz mono
2. **Feature Extraction**: XLSR extracts self-supervised representations
3. **Pooling**: Average pooling over time dimension
4. **Classification**: 2-layer MLP classifies as bonafide or spoofed
5. **Output**: Verdict with confidence and audio region

### Supported Audio Formats

- WAV (recommended)
- MP3
- FLAC
- OGG
- M4A

### Performance

| Dataset | EER | Accuracy |
|---------|-----|----------|
| ASVspoof 2021 LA | 0.82% | 99.2% |
| ASVspoof 2021 DF | 2.85% | 97.1% |
| ASVspoof 2019 LA | 1.14% | 98.9% |

### Usage

```bash
# Direct inference
python models/ssl-antispoofing/inference.py /path/to/audio.wav

# Output
{
  "verdict": "manipulated",
  "confidence": 91,
  "detections": [{
    "detection_type": "voice_cloning",
    "confidence": 91,
    "region": {"start_ms": 0, "end_ms": 5000}
  }]
}
```

### Configuration

```yaml
models:
  - id: "ssl-antispoofing"
    path: ./models/ssl-antispoofing
    modalities: [voice_cloning]
    gpu_required: true
    runtime: external
```

### Dependencies

- PyTorch 1.8+
- torchaudio
- fairseq (for XLSR model)
- scipy, librosa (optional)

### Model Weights

Download with:
```bash
cd models/ssl-antispoofing
./download_weights.sh
```

This downloads:
- `xlsr2_300m.pt` - XLSR base model (~1.2GB)
- `classifier_weights.pth` - Trained classification head

---

## Output Format

All models output JSON in a standardized format:

```json
{
  "verdict": "authentic" | "manipulated" | "inconclusive",
  "confidence": 0-100,
  "detections": [
    {
      "detection_type": "face_manipulation" | "ai_generated_image" | "temporal_inconsistency" | "voice_cloning",
      "confidence": 0-100,
      "region": {
        "x": 100,
        "y": 200,
        "width": 150,
        "height": 200,
        "start_ms": 0,
        "end_ms": 5000
      }
    }
  ]
}
```

### Verdict Values

| Verdict | Meaning | Confidence Range |
|---------|---------|------------------|
| `authentic` | Media appears genuine | >70% confidence real |
| `manipulated` | Media appears fake/generated | >70% confidence fake |
| `inconclusive` | Cannot determine | 30-70% either way |

### Detection Types

| Type | Modality | Description |
|------|----------|-------------|
| `face_manipulation` | Face | Face swap, reenactment, expression transfer |
| `ai_generated_image` | Image | DALL-E, Stable Diffusion, GANs |
| `temporal_inconsistency` | Video | Frame-level manipulation, unnatural motion |
| `voice_cloning` | Audio | TTS, voice conversion, audio splicing |

---

## GPU vs CPU Performance

| Model | GPU (RTX 3080) | CPU (16 cores) |
|-------|----------------|----------------|
| face-forensics | ~50ms/image | ~500ms/image |
| universal-fake-detect | ~100ms/image | ~800ms/image |
| video-ftcn | ~2s/video | ~30s/video |
| ssl-antispoofing | ~200ms/audio | ~2s/audio |

All models automatically detect and use GPU if available.

---

## Adding Custom Models

### 1. Create Model Directory

```bash
mkdir -p models/my-model
```

### 2. Create inference.py

```python
#!/usr/bin/env python3
import sys
import json

def analyze(media_path):
    # Your detection logic here
    result = your_model.predict(media_path)

    return {
        "verdict": "manipulated" if result > 0.5 else "authentic",
        "confidence": int(result * 100),
        "detections": [{
            "detection_type": "my_detection_type",
            "confidence": int(result * 100),
            "region": None
        }]
    }

if __name__ == "__main__":
    result = analyze(sys.argv[1])
    print(json.dumps(result))
```

### 3. Create requirements.txt

```
torch>=2.0.0
# your dependencies
```

### 4. Add to config.yaml

```yaml
models:
  - id: "my-model"
    path: ./models/my-model
    modalities: [image_authenticity]
    runtime: external
```

### 5. Test

```bash
python models/my-model/inference.py test_samples/image.png
```

---

## Troubleshooting

### Model Loading Errors

```bash
# Test model directly
python models/face-forensics/inference.py test_samples/face.jpg

# Check GPU
nvidia-smi

# Check CUDA version
nvcc --version
```

### Missing Dependencies

```bash
# Reinstall model dependencies
pip install -r models/face-forensics/requirements.txt
```

### Weights Not Found

```bash
# Re-download weights
cd models/face-forensics
./download_weights.sh
```

### Video Service Not Running

```bash
# Check Docker status
docker-compose -f models/video-ftcn/docker-compose.yml ps

# View logs
docker-compose -f models/video-ftcn/docker-compose.yml logs

# Restart
docker-compose -f models/video-ftcn/docker-compose.yml restart
```

---

## References

### Papers

- **SBI**: Shiohara & Yamasaki. "Detecting Deepfakes with Self-Blended Images" (CVPR 2022)
- **UniversalFakeDetect**: Ojha et al. "Towards Universal Fake Image Detectors" (CVPR 2023)
- **DeepfakeBench**: Yan et al. "DeepfakeBench: A Comprehensive Benchmark" (arXiv 2023)
- **SSL Anti-spoofing**: Tak et al. "End-to-End Spectro-Temporal Graph Attention Networks" (ASVspoof 2021)

### Repositories

- [Self-Blended Images](https://github.com/mapooon/SelfBlendedImages)
- [UniversalFakeDetect](https://github.com/WisconsinAIVision/UniversalFakeDetect)
- [DeepfakeBench](https://github.com/SCLBD/DeepfakeBench)
- [SSL Anti-spoofing](https://github.com/TakHemlata/SSL_Anti-spoofing)
