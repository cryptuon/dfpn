# Node Operator Guide

This guide explains how to run a DFPN node using your own detection models and inference infrastructure.

## Overview

DFPN is a coordination layer, not a model provider. As a node operator, you:

- **Provide**: Your own detection algorithms, GPU/CPU infrastructure, and operational expertise
- **Receive from DFPN**: Task assignments, reputation tracking, and reward distribution

You choose which models to run, how to scale your infrastructure, and which requests to serve.

## Prerequisites

### Infrastructure Requirements

| Component | Minimum | Recommended |
|-----------|---------|-------------|
| CPU | 8 cores | 16+ cores |
| RAM | 32 GB | 64+ GB |
| GPU | RTX 3080 (10GB VRAM) | RTX 4090 / A100 |
| Storage | 500 GB SSD | 2 TB NVMe |
| Network | 100 Mbps | 1 Gbps |

GPU requirements depend on the models you run. CPU-only operation is possible but limits throughput and model options.

### Software Requirements

- Linux (Ubuntu 22.04+ recommended)
- Docker and Docker Compose
- Solana CLI tools
- Node.js 18+ (for worker client)
- CUDA 12.x (if using NVIDIA GPUs)

### Token Requirements

- **Minimum stake**: 5,000 DFPN (adjustable by governance)
- **SOL for fees**: ~0.1 SOL for registration and ongoing transactions
- **Recommended buffer**: 10,000+ DFPN to handle slashing without falling below minimum

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    Your Infrastructure                   │
├─────────────────────────────────────────────────────────┤
│                                                          │
│  ┌──────────────┐    ┌──────────────┐    ┌───────────┐ │
│  │ Worker Client │───▶│ Task Queue   │───▶│ Inference │ │
│  │  (dfpn-node)  │    │              │    │  Workers  │ │
│  └──────────────┘    └──────────────┘    └───────────┘ │
│         │                                      │        │
│         │                                      ▼        │
│         │                              ┌───────────┐    │
│         │                              │ Your Models│   │
│         │                              │ (GPU/CPU)  │   │
│         │                              └───────────┘    │
│         │                                               │
└─────────┼───────────────────────────────────────────────┘
          │
          ▼
┌─────────────────────┐
│   DFPN Network      │
│  (Solana + Indexer) │
└─────────────────────┘
```

## Setup

### 1. Install the Worker Client

```bash
# Clone the worker client repository
git clone https://github.com/dfpn/dfpn-node.git
cd dfpn-node

# Install dependencies
npm install

# Build
npm run build
```

### 2. Configure Your Wallet

```bash
# Generate a new keypair for your worker (or use existing)
solana-keygen new -o ~/.config/solana/worker.json

# Fund with SOL (devnet example)
solana airdrop 2 --keypair ~/.config/solana/worker.json --url devnet

# Note your public key
solana address --keypair ~/.config/solana/worker.json
```

### 3. Acquire DFPN Tokens

For mainnet, acquire DFPN through exchanges or the ecosystem.

For devnet/testnet:
```bash
# Request test tokens from faucet
curl -X POST https://faucet.dfpn.network/request \
  -H "Content-Type: application/json" \
  -d '{"address": "YOUR_WALLET_ADDRESS", "amount": 10000}'
```

### 4. Configure the Worker

Create `config.yaml`:

```yaml
# Network configuration
network: devnet  # devnet | testnet | mainnet
rpc_url: https://api.devnet.solana.com
indexer_url: https://indexer.devnet.dfpn.network

# Wallet
wallet_path: ~/.config/solana/worker.json

# Worker settings
worker:
  # Modalities this node supports
  modalities:
    - ImageAuthenticity
    - VideoAuthenticity
    - FaceManipulation

  # Minimum fee to accept (in lamports)
  min_fee: 1000000  # 0.001 SOL

  # Maximum concurrent tasks
  max_concurrent: 4

  # Task timeout (seconds)
  task_timeout: 300

# Model configuration
models:
  - id: "face-forensics-v2"
    path: ./models/face_forensics_v2
    modalities: [FaceManipulation]
    gpu_required: true

  - id: "image-ela-detector"
    path: ./models/ela_detector
    modalities: [ImageAuthenticity]
    gpu_required: false

# Inference settings
inference:
  device: cuda  # cuda | cpu
  batch_size: 1
  precision: fp16  # fp32 | fp16 | int8

# Storage for fetching media
storage:
  temp_dir: /tmp/dfpn
  max_file_size_mb: 500
  cleanup_after_seconds: 3600

# Monitoring
monitoring:
  metrics_port: 9090
  health_port: 8080
```

### 5. Set Up Your Models

DFPN provides pre-configured detection models or you can supply your own.

#### Option A: Use Pre-Configured Models (Recommended)

DFPN includes ready-to-use detection models for all modalities:

```bash
# Setup all pre-configured models
./scripts/setup-models.sh

# Test models
./scripts/test-models.sh
```

**Available Models:**

| Model | Modality | Description |
|-------|----------|-------------|
| `face-forensics` | Face Manipulation | SBI/EfficientNet-B4 |
| `universal-fake-detect` | AI-Generated Images | CLIP-ViT-L/14 |
| `video-ftcn` | Video Authenticity | Docker microservice |
| `ssl-antispoofing` | Voice Cloning | wav2vec 2.0/XLSR |

See [Detection Models Guide](detection-models.md) for detailed documentation.

#### Option B: Use Other Open-Source Models

```bash
# Example: Download a public deepfake detection model
mkdir -p models/my_model
cd models/my_model

# Create inference.py with standard output format
# Download model weights
```

#### Option C: Use Your Proprietary Models

Package your model with a standard interface:

```python
#!/usr/bin/env python3
# models/your_model/inference.py
import sys
import json

def analyze(media_path):
    # Load and run your model
    prediction = your_model.predict(media_path)

    return {
        "verdict": "manipulated" if prediction > 0.5 else "authentic",
        "confidence": int(prediction * 100),
        "detections": [{
            "detection_type": "face_swap",
            "confidence": int(prediction * 100),
            "region": {"x": 100, "y": 100, "width": 200, "height": 200}
        }]
    }

if __name__ == "__main__":
    result = analyze(sys.argv[1])
    print(json.dumps(result))
```

#### Option D: Use Commercial APIs

Wrap external APIs (with appropriate licensing):

```python
#!/usr/bin/env python3
import sys
import json
import requests

def analyze(media_path):
    response = requests.post(
        "https://api.detection-service.com/analyze",
        files={"media": open(media_path, "rb")},
        headers={"Authorization": f"Bearer {API_KEY}"}
    )
    data = response.json()
    return {
        "verdict": data["result"],
        "confidence": data["confidence"],
        "detections": data.get("detections", [])
    }

if __name__ == "__main__":
    result = analyze(sys.argv[1])
    print(json.dumps(result))
```

### 6. Register Your Worker

```bash
# Register on-chain
dfpn-node register \
  --stake 5000 \
  --modalities ImageAuthenticity,VideoAuthenticity,FaceManipulation \
  --config config.yaml

# Verify registration
dfpn-node status
```

### 7. Start the Worker

```bash
# Start in foreground
dfpn-node start --config config.yaml

# Or run as a service
dfpn-node service install
dfpn-node service start
```

## Operations

### Task Flow

1. **Poll**: Worker client polls indexer for matching requests
2. **Fetch**: Download media from provided storage URI
3. **Analyze**: Run configured models on the media
4. **Commit**: Submit hash of result to chain
5. **Wait**: Wait for commit window to close
6. **Reveal**: Submit actual result to chain
7. **Cleanup**: Delete temporary media files

### Monitoring

```bash
# Check worker status
dfpn-node status

# View recent tasks
dfpn-node tasks --limit 20

# Check earnings
dfpn-node rewards

# View reputation
dfpn-node reputation
```

#### Prometheus Metrics

The worker exposes metrics at `http://localhost:9090/metrics`:

```
dfpn_tasks_total{status="completed|failed|timeout"}
dfpn_task_duration_seconds
dfpn_inference_duration_seconds{model="..."}
dfpn_rewards_earned_total
dfpn_reputation_score
dfpn_stake_balance
```

#### Health Endpoint

```bash
curl http://localhost:8080/health
# {"status": "healthy", "tasks_in_progress": 2, "models_loaded": 3}
```

### Updating Models

```bash
# Update model files
cd models/face_forensics_v2
wget https://example.com/models/efficientnet_dfdc_v2.pth

# Update config.yaml with new model version

# Restart worker (graceful - finishes current tasks)
dfpn-node restart --graceful
```

### Managing Stake

```bash
# Add stake
dfpn-node stake add 2000

# Request withdrawal (starts unbonding period)
dfpn-node stake withdraw 1000

# Check unbonding status
dfpn-node stake status
```

## Scoring and Rewards

### How You're Scored

Each epoch (24 hours), your performance is evaluated:

| Factor | Weight | Description |
|--------|--------|-------------|
| Accuracy | 50% | Agreement with consensus or ground truth |
| Availability | 25% | Percentage of assigned tasks completed |
| Latency | 15% | Speed relative to deadline |
| Consistency | 10% | Variance in result quality |

### Reward Calculation

```
epoch_reward = (your_score / total_scores) * epoch_reward_pool * stake_weight
```

Where:
- `your_score` = weighted combination of factors above
- `stake_weight` = min(your_stake / median_stake, 2.0) — capped to prevent pay-to-win

### Slashing Events

| Event | Penalty | Description |
|-------|---------|-------------|
| Invalid result | 10% stake | Result doesn't match commitment |
| Missed deadline | 1-3% stake | Failed to reveal before deadline |
| Repeated failures | Progressive | >20% failure rate triggers escalation |
| Fraud/collusion | 25-50% stake | Detected coordinated manipulation |

## Troubleshooting

### Common Issues

**Worker not receiving tasks**
```bash
# Check registration
dfpn-node status

# Verify modalities match available requests
dfpn-node requests --available

# Check stake is above minimum
dfpn-node stake status
```

**Model loading failures**
```bash
# Test model directly
dfpn-node test-model --model face-forensics-v2 --input test.jpg

# Check GPU availability
nvidia-smi

# Verify CUDA version
nvcc --version
```

**Commitment failures**
```bash
# Check wallet balance
solana balance --keypair ~/.config/solana/worker.json

# Verify RPC endpoint
curl -X POST -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"getHealth"}' \
  https://api.devnet.solana.com
```

### Logs

```bash
# View worker logs
dfpn-node logs --tail 100

# Filter by level
dfpn-node logs --level error

# Export for support
dfpn-node logs --export support-bundle.tar.gz
```

## Security Best Practices

1. **Isolate inference**: Run models in containers with limited permissions
2. **Validate inputs**: Check file types and sizes before processing
3. **Secure keys**: Use hardware wallets or HSMs for production stakes
4. **Monitor resources**: Set alerts for unusual CPU/GPU/network usage
5. **Update regularly**: Keep models and worker client updated
6. **Backup configs**: Store configuration securely, separate from keys

## Cost Considerations

### Infrastructure Costs (Estimated Monthly)

| Setup | Cost | Throughput |
|-------|------|------------|
| Entry (RTX 3080) | $200-400 | ~500 tasks/day |
| Mid (RTX 4090) | $400-800 | ~1500 tasks/day |
| Pro (A100 cloud) | $2000-4000 | ~5000 tasks/day |

### Break-Even Analysis

```
monthly_revenue = tasks_completed * avg_reward_per_task
monthly_cost = infrastructure + electricity + maintenance

profit = monthly_revenue - monthly_cost
```

Factors affecting profitability:
- Model accuracy (higher accuracy = higher rewards)
- Task selection (match your model strengths)
- Infrastructure efficiency (optimize batch processing)
- Network demand (more requests = more opportunities)

## Support

- Documentation: https://docs.dfpn.network
- Discord: https://discord.gg/dfpn
- GitHub Issues: https://github.com/dfpn/dfpn-node/issues
- Email: operators@dfpn.network
