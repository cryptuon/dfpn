# DFPN: Deepfake Proof Network

A decentralized coordination layer for deepfake detection built on Solana.

DFPN connects clients who need media verified with independent node operators who run their own detection algorithms and GPU infrastructure. The network uses economic incentives (staking, rewards, slashing) to ensure honest, accurate results.

## Architecture

```
Client -> Submit Request -> Solana Programs -> Workers Analyze -> Consensus Result
```

- **Clients** submit media for deepfake analysis
- **Workers** run detection models on their own GPU/CPU hardware
- **Model Developers** register detection algorithms
- **Solana Programs** coordinate requests, aggregate results, manage reputation and rewards

## Repository Structure

```
programs/               # Solana smart contracts (Anchor)
  shared/               # Shared types and constants
  content-registry/     # Media hash and provenance storage
  analysis-marketplace/ # Request creation and result tracking
  model-registry/       # Model metadata and versioning
  worker-registry/      # Worker staking and reputation
  rewards/              # Reward distribution and treasury
worker/                 # Node operator client (Rust)
indexer/                # REST API indexer (Axum + Tantivy)
sdk/                    # TypeScript SDK for integration
models/                 # Pre-configured detection models
dashboard/              # Vue.js web dashboard
documentation/          # MkDocs user-facing documentation
docs/                   # Technical design documents
scripts/                # Deployment and setup scripts
```

## Quick Start

### Run the Dashboard

```bash
cd dashboard
npm install
npm run dev
```

### Run a Worker Node

```bash
# Install and configure
./scripts/setup-models.sh
cp config.yaml.example config.yaml
# Edit config.yaml with your wallet and preferences

# Start
cargo run --release -p dfpn-worker -- --config config.yaml
```

### Deploy to CapRover

The repository includes a `captain-definition` and `Dockerfile` for one-click deployment to CapRover. The Docker image bundles the Vue.js dashboard with the indexer service.

```bash
# Build locally
docker build -f dashboard/Dockerfile -t dfpn-dashboard .

# Or push to CapRover
# Configure captain-definition to point at your CapRover instance
```

## Detection Models

| Model | Modality | Accuracy | Speed (GPU) |
|-------|----------|----------|-------------|
| face-forensics | Face Manipulation | 97.2% | 50ms |
| universal-fake-detect | AI-Generated Images | 99.8% | 100ms |
| video-ftcn | Video Authenticity | 96.4% | 2s |
| ssl-antispoofing | Voice Cloning | 99.2% | 200ms |

## Token Economics

- **Total Supply**: 1,000,000,000 DFPN
- **Worker Stake**: 5,000 DFPN minimum
- **Fee Split**: 65% Workers / 20% Model Devs / 10% Treasury / 5% Insurance
- **Scoring**: Accuracy (50%), Availability (25%), Latency (15%), Consistency (10%)

## Documentation

- **User Docs**: See `documentation/` (MkDocs) for comprehensive user-facing documentation
- **Technical Docs**: See `docs/` for architecture, protocol, and API specifications

## Technology Stack

- **Blockchain**: Solana (Anchor 0.30.1)
- **Worker**: Rust + Tokio
- **Indexer**: Rust + Axum + Tantivy
- **Dashboard**: Vue 3 + TypeScript + Tailwind CSS 4
- **SDK**: TypeScript (@solana/web3.js)
- **Models**: Python + PyTorch

## License

MIT - see [LICENSE](LICENSE)
