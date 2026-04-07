# Contributing

Thank you for your interest in improving DFPN. This guide covers how to report issues, set up a development environment, and submit pull requests.

---

## Ways to Contribute

- **Report bugs** -- Open a GitHub issue with steps to reproduce
- **Suggest features** -- Start a discussion or open a feature-request issue
- **Submit pull requests** -- Fix a bug, improve docs, or add a feature
- **Run a worker node** -- Help test the network on devnet/testnet
- **Write detection models** -- Build and register new detection algorithms

---

## Development Setup

### Prerequisites

| Tool | Version | Purpose |
|------|---------|---------|
| Rust | 1.75+ | Worker, indexer, on-chain programs |
| Node.js | 18+ | SDK, dashboard, tests |
| Anchor | 0.30.1 | Solana program framework |
| Solana CLI | 1.18+ | Cluster management and key generation |
| Python | 3.10+ | Detection model runtimes |

### Clone and Build

```bash
# Clone the repository
git clone https://github.com/anthropics/dfpn.git
cd dfpn

# Install Node.js dependencies
npm install

# Build Solana programs
anchor build

# Build the worker
cargo build --release -p dfpn-worker

# Build the indexer
cargo build --release -p dfpn-indexer

# Set up the dashboard
cd dashboard && npm install && cd ..
```

### Run Tests

```bash
# Start a local Solana validator and run Anchor tests
anchor test

# Run Rust unit tests
cargo test

# Run dashboard dev server
cd dashboard && npm run dev
```

---

## Project Structure

```
programs/                  # Solana smart contracts (Anchor)
  shared/                  # Shared types, constants, seeds
  content-registry/        # Media hash and provenance
  analysis-marketplace/    # Request lifecycle and commit-reveal
  model-registry/          # Model metadata and versioning
  worker-registry/         # Worker staking and reputation
  rewards/                 # Fee distribution and treasury
worker/                    # Node operator client (Rust + Tokio)
indexer/                   # REST API indexer (Axum + Tantivy)
sdk/                       # TypeScript SDK (@solana/web3.js)
models/                    # Pre-configured detection models
dashboard/                 # Vue 3 + TypeScript + Tailwind CSS 4
documentation/             # MkDocs user-facing documentation
docs/                      # Technical design documents
scripts/                   # Deployment and setup scripts
tests/                     # Integration tests (TypeScript)
```

---

## Code Style

### Rust

- Format with `cargo fmt` before committing
- Run `cargo clippy` and fix all warnings
- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `///` doc comments for public items

### TypeScript

- Lint with ESLint (`npx eslint .`)
- Use strict TypeScript (`strict: true` in tsconfig)
- Prefer `const` over `let`

### Vue (Dashboard)

- Use `<script setup lang="ts">` with the Composition API
- One component per file
- Props and emits must be typed

---

## Testing

### Anchor Tests

Integration tests live in `tests/` and run against a local Solana validator.

```bash
# Run all program tests
anchor test

# Run a specific test file
anchor test -- --grep "content registry"
```

### Local Validator

For manual testing you can start a persistent local validator:

```bash
solana-test-validator --reset

# In another terminal
anchor deploy
```

### Worker Tests

```bash
cargo test -p dfpn-worker
```

---

## Pull Request Guidelines

1. **Create a branch** from `main` with a descriptive name (`fix/slash-calculation`, `feat/model-search`, `docs/api-reference`)
2. **Write a clear title** that summarizes the change in one line
3. **Include a test plan** -- describe how to verify the change works
4. **Link related issues** using `Closes #123` or `Relates to #456`
5. **Keep PRs focused** -- one logical change per PR
6. **Ensure CI passes** -- all tests, lints, and builds must succeed

??? example "Good PR description"
    ```
    ## Summary
    Fix incorrect slash percentage calculation in worker-registry.
    The BPS denominator was applied twice, resulting in 0.01x the intended penalty.

    ## Test plan
    - [ ] `anchor test` passes
    - [ ] New test case: `test_slash_correct_amount`
    - [ ] Manual verification on localnet

    Closes #42
    ```

---

## Communication

- **GitHub Issues** -- Bug reports, feature requests, and task tracking
- **GitHub Discussions** -- Questions, ideas, and general conversation
- **Discord** -- Real-time chat with the community (link coming soon)

---

## License

DFPN is released under the [MIT License](https://github.com/anthropics/dfpn/blob/main/LICENSE). By contributing, you agree that your contributions will be licensed under the same terms.
