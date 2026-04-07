# Running a Worker Node

Workers are the backbone of DFPN. You provide GPU hardware, run detection models, and earn rewards for analyzing media submitted by clients.

---

## What Workers Do

As a worker, your node:

1. Polls the network for new analysis requests
2. Downloads the media file from the client's storage URI
3. Runs one or more detection models on the media
4. Submits a cryptographic commitment of the result (commit phase)
5. Reveals the actual result after all workers have committed (reveal phase)
6. Earns fees and reputation based on accuracy and timeliness

You control your own hardware, choose which models to run, and decide which requests to accept based on fee and modality.

---

## Hardware Requirements

| Component | Entry Tier | Mid Tier | Pro Tier |
|-----------|-----------|----------|----------|
| **CPU** | 8 cores | 16 cores | 32 cores |
| **RAM** | 32 GB | 64 GB | 128 GB |
| **GPU** | NVIDIA RTX 3080 (10 GB) | NVIDIA RTX 4090 (24 GB) | NVIDIA A100 (40/80 GB) |
| **Storage** | 500 GB SSD | 1 TB NVMe | 2 TB NVMe |
| **Network** | 100 Mbps | 500 Mbps | 1 Gbps |

!!! info "Entry tier is sufficient to start"
    You can run a profitable worker node with entry-tier hardware. Higher tiers allow you to process more concurrent requests and handle larger video files, which increases your earning potential.

---

## Software Prerequisites

- **Operating System**: Linux (Ubuntu 22.04+ recommended)
- **Docker**: 24.x or later
- **NVIDIA Drivers**: 535+ with CUDA 12.x
- **NVIDIA Container Toolkit**: For GPU access in Docker
- **Node.js**: 18+ (for SDK tooling)
- **Solana CLI**: Latest stable release

Verify your GPU setup:

```bash
nvidia-smi
```

You should see your GPU listed with driver and CUDA versions.

---

## Step-by-Step Setup

### Step 1: Clone the Repository

```bash
git clone https://github.com/dfpn/dfpn.git
cd dfpn
```

### Step 2: Install Dependencies

```bash
npm install
```

### Step 3: Generate a Worker Wallet

Create a dedicated wallet for your worker node. Do not reuse your personal wallet.

```bash
solana-keygen new --outfile ~/.config/solana/dfpn-worker.json
solana config set --keypair ~/.config/solana/dfpn-worker.json
```

!!! warning "Back up your keypair"
    Your keypair file controls your staked tokens. Store a backup in a secure, offline location. If you lose this file, you lose access to your stake.

### Step 4: Fund Your Wallet

Your worker wallet needs two types of funds:

- **SOL** for Solana transaction fees (at least 0.5 SOL recommended)
- **5,000 DFPN** tokens for staking

On devnet, you can get test SOL:

```bash
solana airdrop 2
```

### Step 5: Set Up Detection Models

Run the model setup script to download and configure the pre-packaged detection models:

```bash
./scripts/setup-models.sh
```

This downloads model weights, sets up Python environments, and verifies GPU access. The process takes 10-30 minutes depending on your internet connection.

### Step 6: Configure Your Node

Copy and edit the configuration file:

```bash
cp config.yaml config-worker.yaml
```

Edit `config-worker.yaml` with your preferences. See the [Configuration Reference](#configuration-reference) below for all options.

### Step 7: Register and Start

Register your worker on-chain and start the node:

```bash
# Register as a worker (stakes your DFPN tokens)
dfpn-worker register --config config-worker.yaml

# Start the worker daemon
dfpn-worker start --config config-worker.yaml
```

!!! tip "Use a process manager"
    For production, run the worker under `systemd` or a similar process manager so it restarts automatically after crashes or reboots.

---

## Configuration Reference

The `config.yaml` file controls your worker's behavior. Here are the key settings:

```yaml
# Network
network: devnet
rpc_url: https://api.devnet.solana.com
indexer_url: https://indexer.devnet.dfpn.network

# Wallet
wallet_path: ~/.config/solana/dfpn-worker.json

# Worker settings
worker:
  modalities:
    - image_authenticity
    - video_authenticity
    - face_manipulation
    - voice_cloning
    - generated_content
  min_fee: 1000000          # Minimum fee in lamports (0.001 SOL)
  max_concurrent: 4         # Concurrent tasks (adjust for GPU memory)
  task_timeout: 300         # Seconds before a task times out
  poll_interval_ms: 5000    # How often to check for new tasks

# Inference
inference:
  device: cuda              # "cuda" for GPU, "cpu" for CPU-only
  precision: fp16           # fp32, fp16, or int8
  batch_size: 1

# Monitoring
monitoring:
  metrics_port: 9090
  health_port: 8080
  log_level: info
```

| Setting | Description | Default |
|---------|-------------|---------|
| `worker.modalities` | Which media types you accept | All supported |
| `worker.min_fee` | Reject tasks below this fee (lamports) | 1,000,000 |
| `worker.max_concurrent` | Parallel tasks (limited by GPU memory) | 4 |
| `worker.poll_interval_ms` | Task polling frequency in ms | 5,000 |
| `inference.device` | `cuda` for GPU, `cpu` for CPU-only | `cuda` |
| `inference.precision` | Model precision: `fp32`, `fp16`, `int8` | `fp16` |

---

## Monitoring Your Node

### Health Endpoint

Your worker exposes a health check at `http://localhost:8080/health`:

```bash
curl http://localhost:8080/health
```

```json
{
  "status": "healthy",
  "uptime_seconds": 86400,
  "tasks_completed": 142,
  "active_tasks": 2,
  "gpu_utilization": 45
}
```

### Metrics

Prometheus metrics are available at `http://localhost:9090/metrics`. Key metrics to watch:

- `dfpn_tasks_completed_total` -- total tasks processed
- `dfpn_tasks_failed_total` -- total task failures
- `dfpn_inference_duration_seconds` -- model inference time
- `dfpn_gpu_memory_used_bytes` -- GPU memory consumption

### Logs

```bash
# Follow worker logs
tail -f /var/log/dfpn-worker/worker.log

# Filter for errors
grep ERROR /var/log/dfpn-worker/worker.log
```

---

## Earning Rewards

Your earnings come from two sources:

1. **Per-request fees** -- 65% of the client's fee for each request you process
2. **Epoch rewards** -- share of the epoch reward pool based on your performance score

### Scoring Factors

Your performance score determines your share of epoch rewards. It is calculated from four factors:

| Factor | Weight | What It Measures |
|--------|--------|------------------|
| **Accuracy** | 50% | Agreement with consensus or benchmarks |
| **Availability** | 25% | Uptime and task acceptance rate |
| **Latency** | 15% | How quickly you complete tasks |
| **Consistency** | 10% | Stability of results across similar inputs |

!!! tip "Accuracy is king"
    Accuracy accounts for half your score. Running high-quality, well-maintained models matters more than having the fastest hardware.

### Reward Formula

Your share of each epoch's reward pool is:

```
epoch_reward = (your_score / total_scores) * epoch_pool * stake_weight
```

Where `stake_weight` is a capped multiplier based on your staked DFPN.

---

## Slashing Risks

Staked tokens can be slashed for bad behavior. Understand these risks before you start.

| Offense | Slash Amount | Details |
|---------|-------------|---------|
| **Invalid results** | 10% of stake | Submitting results that fail verification |
| **Missed deadlines** | 1-3% of stake | Committing but not revealing before deadline |
| **Repeated failures** | Progressive | Escalating penalties for consistent poor quality |
| **Fraud or collusion** | 25-50% of stake | Coordinated dishonesty or result manipulation |

!!! danger "Fraud slashing is severe"
    Detected fraud or collusion results in 25-50% stake loss plus a temporary ban from the network. Ensure your node is configured correctly and your models produce genuine results.

!!! warning "Monitor your node"
    Most slashing events happen because of operational failures, not malice. A crashed node that commits but fails to reveal will be penalized. Use a process manager and set up alerts for downtime.
