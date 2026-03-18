# DFPN Detection Models

This directory contains deepfake detection models for DFPN worker nodes.

## Available Models

| Model | Modality | Type | Description |
|-------|----------|------|-------------|
| `face-forensics` | Face Manipulation | ExternalDetector | SBI/EfficientNet-B4 for face swap detection |
| `universal-fake-detect` | AI-Generated Images | ExternalDetector | CLIP-based detector for diffusion models |
| `video-ftcn` | Video Authenticity | HttpDetector | Temporal consistency analysis |
| `ssl-antispoofing` | Voice Cloning | ExternalDetector | wav2vec 2.0 based audio spoofing detection |

## Quick Start

```bash
# Setup all models
./scripts/setup-models.sh

# Test models
./scripts/test-models.sh

# Start worker
./scripts/start-worker.sh
```

## Directory Structure

```
models/
├── face-forensics/
│   ├── inference.py         # Detection script
│   ├── requirements.txt     # Python dependencies
│   ├── download_weights.sh  # Weight download script
│   └── model.pt             # Model weights (after download)
│
├── universal-fake-detect/
│   ├── inference.py
│   ├── requirements.txt
│   ├── download_weights.sh
│   └── fc_weights.pth       # Classification head weights
│
├── video-ftcn/
│   ├── Dockerfile           # Container definition
│   ├── docker-compose.yml   # Service configuration
│   ├── server.py            # Flask API server
│   ├── download_weights.sh
│   └── weights/             # Model weights directory
│
└── ssl-antispoofing/
    ├── inference.py
    ├── requirements.txt
    ├── download_weights.sh
    ├── xlsr2_300m.pt        # XLSR base model (1.2GB)
    └── classifier_weights.pth
```

## Model Output Format

All models output JSON in the following format:

```json
{
  "verdict": "authentic" | "manipulated" | "inconclusive",
  "confidence": 0-100,
  "detections": [
    {
      "detection_type": "face_manipulation" | "ai_generated_image" | ...,
      "confidence": 0-100,
      "region": {
        "x": 100,
        "y": 200,
        "width": 50,
        "height": 50
      }
    }
  ]
}
```

## Adding New Models

1. Create a directory for your model: `models/my-model/`

2. Create `inference.py` that:
   - Accepts a file path as command line argument
   - Outputs JSON to stdout in the format above
   - Handles errors gracefully

3. Create `requirements.txt` with Python dependencies

4. Create `download_weights.sh` to download model weights

5. Add model to `config.yaml`:
   ```yaml
   models:
     - id: "my-model"
       path: ./models/my-model
       modalities: [image_authenticity]
       runtime: external
   ```

## Testing Individual Models

```bash
# Test face forensics
python models/face-forensics/inference.py test_samples/face.jpg

# Test image detector
python models/universal-fake-detect/inference.py test_samples/image.png

# Test audio detector
python models/ssl-antispoofing/inference.py test_samples/audio.wav

# Test video service (requires Docker)
curl -X POST -F "file=@test_samples/video.mp4" http://localhost:8001/analyze
```

## GPU vs CPU

All models support both GPU (CUDA) and CPU inference:

- **GPU**: Faster inference, recommended for production
- **CPU**: Slower but works on any machine

The models automatically detect available hardware and use GPU if available.

## Model Sources

- **Face Forensics**: [Self-Blended Images](https://github.com/mapooon/SelfBlendedImages)
- **Universal Fake Detect**: [UniversalFakeDetect](https://github.com/WisconsinAIVision/UniversalFakeDetect)
- **Video FTCN**: [DeepfakeBench](https://github.com/SCLBD/DeepfakeBench)
- **SSL Anti-spoofing**: [SSL_Anti-spoofing](https://github.com/TakHemlata/SSL_Anti-spoofing)
