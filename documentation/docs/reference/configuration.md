# Configuration Reference

This page documents every configuration option for the worker node, indexer, and dashboard.

---

## Worker Configuration

The worker reads a YAML configuration file at startup:

```bash
dfpn-worker --config config.yaml
```

Two example files are provided in the repository root:

- **`config.yaml`** -- Full GPU node with all modalities
- **`config-cpu.yaml`** -- CPU-only node (reduced concurrency, no video)

### Network

```yaml
network: devnet          # devnet | testnet | mainnet
rpc_url: https://api.devnet.solana.com
indexer_url: https://indexer.devnet.dfpn.network
```

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `network` | string | `devnet` | Solana cluster to connect to |
| `rpc_url` | string | -- | Solana JSON-RPC endpoint |
| `indexer_url` | string | -- | DFPN indexer base URL |

### Wallet

```yaml
wallet_path: ~/.config/solana/id.json
```

| Key | Type | Description |
|-----|------|-------------|
| `wallet_path` | string | Path to the Solana keypair JSON file used to sign transactions |

### Worker Settings

```yaml
worker:
  modalities:
    - image_authenticity
    - video_authenticity
    - face_manipulation
    - voice_cloning
    - generated_content
  min_fee: 1000000
  max_concurrent: 4
  task_timeout: 300
  poll_interval_ms: 5000
```

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `worker.modalities` | list | -- | Detection modalities this node can process. Must match the models configured below. |
| `worker.min_fee` | integer | `1000000` | Minimum fee in lamports to accept a task (1,000,000 = 0.001 SOL) |
| `worker.max_concurrent` | integer | `4` | Maximum tasks processed in parallel. Reduce for CPU or low-VRAM GPUs. |
| `worker.task_timeout` | integer | `300` | Seconds before a task is considered timed out |
| `worker.poll_interval_ms` | integer | `5000` | How often to poll for new tasks (milliseconds) |

### Models

Each entry in the `models` list configures one detection model.

```yaml
models:
  - id: "face-forensics-sbi"
    path: ./models/face-forensics
    modalities:
      - face_manipulation
    gpu_required: true
    runtime: external        # external (Python subprocess) | http (microservice)
    on_chain_id: null        # Set after registering model on-chain
```

| Key | Type | Description |
|-----|------|-------------|
| `models[].id` | string | Unique identifier for this model |
| `models[].path` | string | Local filesystem path or HTTP URL for the model runtime |
| `models[].modalities` | list | Modalities this model handles |
| `models[].gpu_required` | bool | Whether the model needs a GPU |
| `models[].runtime` | string | `external` for Python subprocess, `http` for a sidecar microservice |
| `models[].on_chain_id` | string | On-chain model account public key (set after registration) |

### Inference

```yaml
inference:
  device: cuda       # cuda | cpu
  batch_size: 1
  precision: fp16    # fp32 | fp16 | int8
```

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `inference.device` | string | `cuda` | PyTorch device. Use `cpu` for CPU-only nodes. |
| `inference.batch_size` | integer | `1` | Inference batch size. Keep at 1 for single-item processing. |
| `inference.precision` | string | `fp16` | Floating-point precision. `fp16` saves VRAM on GPU; use `fp32` on CPU. |

### Storage

```yaml
storage:
  temp_dir: /tmp/dfpn
  max_file_size_mb: 500
  cleanup_after_seconds: 3600
```

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `storage.temp_dir` | string | `/tmp/dfpn` | Directory for downloaded media files |
| `storage.max_file_size_mb` | integer | `500` | Maximum file size to process (MB) |
| `storage.cleanup_after_seconds` | integer | `3600` | Remove temporary files after this many seconds |

### Monitoring

```yaml
monitoring:
  metrics_port: 9090
  health_port: 8080
  log_level: info
```

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `monitoring.metrics_port` | integer | `9090` | Prometheus metrics endpoint port |
| `monitoring.health_port` | integer | `8080` | Health check endpoint port |
| `monitoring.log_level` | string | `info` | Log verbosity: `trace`, `debug`, `info`, `warn`, `error` |

---

## CPU-only Configuration

The `config-cpu.yaml` file adjusts several defaults for nodes without a GPU:

| Setting | GPU Value | CPU Value | Reason |
|---------|-----------|-----------|--------|
| `inference.device` | `cuda` | `cpu` | No GPU available |
| `inference.precision` | `fp16` | `fp32` | Half-precision has no benefit on CPU |
| `worker.max_concurrent` | `4` | `2` | CPU inference is slower; fewer parallel tasks avoid overload |
| `worker.task_timeout` | `300` | `600` | Longer timeout accounts for slower processing |
| `worker.poll_interval_ms` | `5000` | `10000` | Reduced polling frequency |
| `storage.max_file_size_mb` | `500` | `200` | Smaller cap to limit CPU processing time |
| Video modality | Enabled | **Disabled** | Video analysis on CPU takes ~30 s per clip and is impractical |

!!! warning "Video on CPU"
    The `video_authenticity` modality is commented out in the CPU config and the `video-ftcn` model is removed entirely. You *can* re-enable it, but expect latencies of 30 seconds or more per video.

---

## Indexer Configuration

The indexer is configured entirely through CLI flags. It does not use a config file.

```bash
dfpn-indexer \
  --rpc-url https://api.devnet.solana.com \
  --index-path ./data/indexes \
  --bind 127.0.0.1:3030 \
  --log-level info \
  --content-registry GokivDYuQXPZCWRkwMhdH2h91KpDQXBEmpgBgs55bnpH \
  --analysis-marketplace 9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin \
  --model-registry Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS \
  --worker-registry HmbTLCmaGvZhKnn1Zfa1JVnp7vkMV4DYVxPLWBVoN65L \
  --rewards 4uQeVj5tqViQh7yWWGStvkEG1Zmhx6uasJtWCJziofM
```

### CLI Flags

| Flag | Default | Description |
|------|---------|-------------|
| `--rpc-url`, `-r` | `http://localhost:8899` | Solana JSON-RPC endpoint |
| `--index-path`, `-i` | `./data/indexes` | Directory for Tantivy search indexes |
| `--bind`, `-b` | `127.0.0.1:3030` | Address and port for the REST API server |
| `--log-level` | `info` | Log verbosity: `trace`, `debug`, `info`, `warn`, `error` |
| `--content-registry` | -- | Content Registry program ID |
| `--analysis-marketplace` | -- | Analysis Marketplace program ID |
| `--model-registry` | -- | Model Registry program ID |
| `--worker-registry` | -- | Worker Registry program ID |
| `--rewards` | -- | Rewards program ID |

---

## Docker Environment Variables

When deploying the dashboard + indexer via Docker, program IDs and the RPC URL are passed as environment variables.

| Variable | Description |
|----------|-------------|
| `SOLANA_RPC_URL` | Solana JSON-RPC endpoint |
| `CONTENT_REGISTRY` | Content Registry program ID |
| `ANALYSIS_MARKETPLACE` | Analysis Marketplace program ID |
| `MODEL_REGISTRY` | Model Registry program ID |
| `WORKER_REGISTRY` | Worker Registry program ID |
| `REWARDS` | Rewards program ID |

??? example "Docker Compose snippet"
    ```yaml
    services:
      dfpn:
        image: dfpn-dashboard:latest
        ports:
          - "80:80"
        environment:
          SOLANA_RPC_URL: https://api.devnet.solana.com
          CONTENT_REGISTRY: GokivDYuQXPZCWRkwMhdH2h91KpDQXBEmpgBgs55bnpH
          ANALYSIS_MARKETPLACE: 9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin
          MODEL_REGISTRY: Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS
          WORKER_REGISTRY: HmbTLCmaGvZhKnn1Zfa1JVnp7vkMV4DYVxPLWBVoN65L
          REWARDS: 4uQeVj5tqViQh7yWWGStvkEG1Zmhx6uasJtWCJziofM
    ```

---

## Dashboard Configuration

The Vue.js dashboard uses Vite for development and nginx in production.

### Development Proxy

During local development, Vite proxies API calls to the indexer:

```typescript
// vite.config.ts
server: {
  proxy: {
    '/api': {
      target: 'http://127.0.0.1:3030',
      changeOrigin: true,
      rewrite: (path) => path.replace(/^\/api/, ''),
    },
  },
},
```

In production, nginx handles the same proxy rule (`/api/ -> http://127.0.0.1:3030/`).

### Network Selector

The dashboard includes a network selector that switches between Solana clusters:

| Network | RPC Endpoint | Description |
|---------|-------------|-------------|
| Devnet | `https://api.devnet.solana.com` | Development and testing |
| Testnet | `https://api.testnet.solana.com` | Pre-production pilot |
| Mainnet | `https://api.mainnet-beta.solana.com` | Production (when available) |

Switching networks updates the RPC connection and the set of program IDs used for on-chain queries.
