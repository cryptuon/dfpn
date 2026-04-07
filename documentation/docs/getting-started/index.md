# Getting Started

DFPN has three participation paths. Choose the one that matches your goals.

---

## Choose Your Path

### Workers -- Earn by Running Detection Nodes

Workers provide the compute power behind the network. You run GPU-equipped nodes with detection models and earn fees for every analysis request you complete.

**What you need:**

- A machine with a modern NVIDIA GPU (RTX 3080 or better)
- 5,000 DFPN tokens for staking
- SOL for transaction fees
- Basic command-line experience

**What you earn:**

- Per-request fees (65% of client payment)
- Epoch-based reward pool distributions
- Higher earnings with better hardware, accuracy, and uptime

[Worker Setup Guide](workers.md){ .md-button .md-button--primary }

---

### Clients -- Submit Media for Analysis

Clients integrate DFPN into their applications to verify media authenticity. Submit an image, video, or audio file and receive a consensus-backed detection result.

**What you need:**

- A Solana wallet with SOL for fees
- The TypeScript SDK, Python SDK, or REST API access
- Media files to analyze

**What it costs:**

- Per-request fees starting at ~0.002 SOL for images
- No staking or token holdings required

[Client Integration Guide](clients.md){ .md-button .md-button--primary }

---

### Model Developers -- Build and Register Detection Models

Model developers create deepfake detection algorithms and register them on the network. When workers adopt your model, you earn a share of every fee it helps generate.

**What you need:**

- A trained detection model with standard output format
- 20,000 DFPN tokens for staking (per model version)
- A hosting location for model files (IPFS, S3, or HTTP)

**What you earn:**

- 20% of fees from every request processed with your model
- Epoch reward distributions based on model performance

[Model Developer Guide](model-developers.md){ .md-button .md-button--primary }

---

## Prerequisites

Before you begin with any path, you will need these common tools.

### Solana CLI

Install the Solana command-line tools:

```bash
sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"
```

Verify the installation:

```bash
solana --version
```

### Solana Wallet

Create a new wallet or use an existing one:

```bash
# Create a new wallet
solana-keygen new --outfile ~/.config/solana/id.json

# Or use an existing keypair file
solana config set --keypair ~/.config/solana/id.json
```

### Network Configuration

Connect to devnet for testing:

```bash
solana config set --url https://api.devnet.solana.com
```

!!! tip "Start on Devnet"
    Always start on devnet to test your setup before moving to mainnet. You can get free devnet SOL with `solana airdrop 2`.

### SOL for Transactions

Every on-chain action (registering, submitting requests, committing results) requires a small amount of SOL for Solana transaction fees. Keep at least 0.1 SOL in your wallet.

---

## Next Steps

Once you have the prerequisites set up, follow the guide for your chosen role:

- [Running a Worker Node](workers.md)
- [Submitting Media for Analysis](clients.md)
- [Registering Detection Models](model-developers.md)
