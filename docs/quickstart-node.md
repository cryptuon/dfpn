# DFPN Node Quick Start Guide

Get a DFPN detection node running in under 10 minutes.

## Prerequisites

- **OS**: Linux (Ubuntu 22.04+ recommended)
- **Python**: 3.9+
- **GPU**: NVIDIA with CUDA 11.7+ (optional but recommended)
- **Storage**: 10GB free space for model weights
- **Network**: Stable internet connection

## Step 1: Clone and Setup

```bash
# Clone the repository
git clone https://github.com/dfpn/dfpn.git
cd dfpn

# Install system dependencies (Ubuntu)
sudo apt update
sudo apt install -y python3-pip python3-venv ffmpeg

# Optional: Install Docker for video detection
sudo apt install -y docker.io docker-compose
sudo usermod -aG docker $USER
```

## Step 2: Setup Detection Models

```bash
# Run the setup script
./scripts/setup-models.sh

# This will:
# - Create Python virtual environment
# - Install PyTorch and dependencies
# - Download model weights (~2GB)
# - Build Docker container for video detection
```

**What gets installed:**

| Model | Size | Modality |
|-------|------|----------|
| face-forensics | ~100MB | Face swaps, deepfakes |
| universal-fake-detect | ~2GB | AI-generated images |
| ssl-antispoofing | ~1.3GB | Voice cloning |
| video-ftcn | ~500MB | Video manipulation |

## Step 3: Test Models

```bash
# Verify all models work
./scripts/test-models.sh
```

Expected output:
```
Testing Face Forensics...
{"verdict": "authentic", "confidence": 78, "detections": [...]}
✓ Test passed

Testing Universal Fake Detect...
{"verdict": "authentic", "confidence": 85, "detections": [...]}
✓ Test passed

Testing SSL Anti-spoofing...
{"verdict": "authentic", "confidence": 72, "detections": [...]}
✓ Test passed

Results: 3 passed, 0 failed
```

## Step 4: Configure Wallet

```bash
# Generate Solana keypair (or use existing)
solana-keygen new -o ~/.config/solana/worker.json

# Fund with SOL (devnet)
solana airdrop 2 --keypair ~/.config/solana/worker.json --url devnet

# Verify balance
solana balance --keypair ~/.config/solana/worker.json --url devnet
```

## Step 5: Review Configuration

The default configuration is in `config.yaml`. Key settings:

```yaml
# Network
network: devnet
rpc_url: https://api.devnet.solana.com

# What this node detects
worker:
  modalities:
    - image_authenticity
    - video_authenticity
    - face_manipulation
    - voice_cloning
    - generated_content

# Models to use
models:
  - id: "face-forensics-sbi"
    path: ./models/face-forensics
    modalities: [face_manipulation]

  - id: "universal-fake-detect"
    path: ./models/universal-fake-detect
    modalities: [image_authenticity, generated_content]

  # ... more models
```

For CPU-only systems, use `config-cpu.yaml` instead.

## Step 6: Start the Worker

```bash
# Start all services and worker
./scripts/start-worker.sh

# Or with specific config
./scripts/start-worker.sh --config config-cpu.yaml
```

## Step 7: Verify It's Running

```bash
# Check health endpoint
curl http://localhost:8080/health

# View metrics
curl http://localhost:9090/metrics

# Check worker status (once dfpn-worker is built)
dfpn-worker status
```

## What Happens Next

Once running, your node will:

1. **Poll** the DFPN indexer for matching analysis requests
2. **Download** media files from storage URIs
3. **Analyze** using configured detection models
4. **Submit** results to the Solana blockchain
5. **Earn** DFPN tokens based on accuracy and availability

## Common Issues

### GPU Not Detected

```bash
# Check NVIDIA driver
nvidia-smi

# Check CUDA
nvcc --version

# Use CPU config if no GPU
./scripts/start-worker.sh --config config-cpu.yaml
```

### Model Download Failed

```bash
# Retry specific model
cd models/face-forensics
./download_weights.sh

# Or manually download from source
# See docs/detection-models.md for links
```

### Video Service Not Starting

```bash
# Check Docker
docker --version
docker-compose --version

# Restart service
cd models/video-ftcn
docker-compose down
docker-compose up -d

# View logs
docker-compose logs -f
```

### Insufficient Funds

```bash
# Airdrop more SOL (devnet only)
solana airdrop 2 --keypair ~/.config/solana/worker.json --url devnet
```

## Configuration Options

### GPU Configuration (Default)

```yaml
inference:
  device: cuda
  precision: fp16
```

### CPU Configuration

```yaml
inference:
  device: cpu
  precision: fp32
```

### Selective Modalities

Only run specific detection types:

```yaml
worker:
  modalities:
    - face_manipulation  # Only face detection

models:
  - id: "face-forensics-sbi"
    path: ./models/face-forensics
    modalities: [face_manipulation]
```

## Next Steps

- Read [Node Operator Guide](node-operator-guide.md) for detailed operations
- Review [Detection Models Guide](detection-models.md) for model specifics
- Join [Discord](https://discord.gg/dfpn) for support
- Register on mainnet when ready

## File Structure

After setup, your directory should look like:

```
dfpn/
├── config.yaml              # GPU configuration
├── config-cpu.yaml          # CPU configuration
├── scripts/
│   ├── setup-models.sh      # Model setup
│   ├── test-models.sh       # Model testing
│   ├── start-worker.sh      # Start worker
│   └── stop-worker.sh       # Stop worker
├── models/
│   ├── face-forensics/      # Face detection
│   ├── universal-fake-detect/  # Image detection
│   ├── video-ftcn/          # Video detection
│   └── ssl-antispoofing/    # Audio detection
└── venv/                    # Python environment
```
