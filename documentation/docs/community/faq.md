# Frequently Asked Questions

---

## General

??? question "What is DFPN?"
    DFPN (Deepfake Proof Network) is a decentralized coordination layer for deepfake detection built on Solana. It connects clients who need media verified with independent node operators who run detection algorithms on their own hardware. Results are aggregated through a reputation-weighted consensus mechanism, and economic incentives (staking, rewards, slashing) keep participants honest.

??? question "How is DFPN different from centralized deepfake detection services?"
    Centralized services rely on a single provider -- if that provider is compromised, biased, or goes offline, all users are affected. DFPN distributes analysis across multiple independent workers:

    - **No single point of failure** -- the network continues operating even if individual workers go offline
    - **On-chain transparency** -- every request, result, and reputation score is recorded on Solana and publicly auditable
    - **Model diversity** -- workers can run different detection models, reducing the risk of a single model's blind spots
    - **Economic accountability** -- workers stake tokens and risk slashing for bad results, creating a financial incentive for accuracy

??? question "What blockchain does DFPN use?"
    DFPN runs on **Solana**. Solana's sub-second finality and low transaction costs (fractions of a cent) make it practical to coordinate real-time detection tasks on-chain without prohibitive fees.

??? question "What types of deepfakes can DFPN detect?"
    The network currently supports four detection categories:

    - **Face manipulation** -- face swaps and reenactment forgeries in images
    - **AI-generated images** -- synthetic images from diffusion models, GANs, and similar generators
    - **Video manipulation** -- temporal inconsistencies that reveal video-level tampering
    - **Voice cloning** -- synthetic or cloned voices in audio recordings

    See [Detection Models](../concepts/detection-models.md) for details on each model's accuracy and speed.

??? question "Is DFPN live?"
    DFPN is currently in the **Testnet Pilot** phase. The core on-chain programs and worker infrastructure are functional on devnet. Public testnet and mainnet launches are upcoming. See the [Roadmap](roadmap.md) for the full timeline.

---

## Workers

??? question "How do I become a worker?"
    1. Install the DFPN worker client and detection models
    2. Generate or import a Solana wallet and fund it with SOL (for transaction fees) and DFPN tokens (for staking)
    3. Stake a minimum of **5,000 DFPN** tokens
    4. Configure your `config.yaml` with your wallet path, supported modalities, and model paths
    5. Register your worker on-chain
    6. Start the worker process

    See the [For Workers](../getting-started/workers.md) guide for step-by-step instructions.

??? question "What hardware do I need to run a worker?"
    **Minimum requirements:**

    | Component | Minimum | Recommended |
    |-----------|---------|-------------|
    | GPU | NVIDIA RTX 3080 (10 GB VRAM) | NVIDIA RTX 4090+ (24 GB VRAM) |
    | RAM | 32 GB | 64 GB |
    | CPU | 8 cores | 16+ cores |
    | Storage | 50 GB SSD | 200 GB NVMe SSD |
    | Network | 100 Mbps | 1 Gbps |

    GPU is strongly recommended for competitive latencies. CPU-only nodes can participate but will be significantly slower.

??? question "How much can I earn as a worker?"
    Earnings depend on three factors:

    - **Accuracy** -- workers with higher accuracy scores receive a larger share of fees
    - **Volume** -- more tasks processed means more fees earned
    - **Stake** -- higher stake increases your weight in the reward distribution

    The fee split allocates **65% of each request's fee to workers**. Additionally, workers receive epoch-based rewards from the treasury based on their scoring (accuracy 50%, availability 25%, latency 15%, consistency 10%).

    Actual earnings vary with network demand and competition from other workers.

??? question "What happens if I submit bad results?"
    Workers who submit inaccurate or fraudulent results face **slashing** -- a penalty that removes a portion of their staked tokens:

    | Offense | Slash Percentage |
    |---------|-----------------|
    | Invalid result (disagrees with consensus) | 10% of stake |
    | Missed deadline | 1--3% of stake |
    | Proven fraud (deliberate manipulation) | 25--50% of stake |

    Repeated offenses also lower your reputation score, which reduces your share of future rewards and may lead to deregistration.

??? question "Can I run a CPU-only node?"
    Yes. DFPN provides a `config-cpu.yaml` template for nodes without a GPU. CPU-only nodes can process all modalities except video (which is impractically slow on CPU).

    Expect significantly higher latencies:

    | Modality | GPU | CPU |
    |----------|-----|-----|
    | Face manipulation | 50 ms | 500 ms |
    | AI-generated image | 100 ms | 800 ms |
    | Voice cloning | 200 ms | 2 s |
    | Video | 2 s | ~30 s |

    See [CPU-only Configuration](../reference/configuration.md#cpu-only-configuration) for setup details.

---

## Tokens & Staking

??? question "What is the DFPN token used for?"
    The DFPN token serves four purposes:

    1. **Staking** -- workers and model developers stake tokens as a security deposit
    2. **Fees** -- clients pay for analysis requests in DFPN tokens
    3. **Rewards** -- workers and model developers earn DFPN tokens for honest participation
    4. **Governance** -- token holders vote on protocol parameters and upgrades (via Realms DAO, coming in Phase 3)

??? question "How does staking work?"
    Two roles require staking:

    | Role | Minimum Stake | Purpose |
    |------|--------------|---------|
    | Worker | 5,000 DFPN | Guarantees honest analysis; at risk of slashing |
    | Model Developer | 20,000 DFPN | Guarantees model quality; at risk of slashing |

    Staked tokens earn a share of network rewards. When you want to withdraw, there is an **unbonding period** (~3 days at current slot times) during which your stake cannot be used and you cannot accept new tasks.

??? question "What is slashing?"
    Slashing is an economic penalty that removes a portion of a participant's staked tokens. It discourages dishonest or negligent behavior.

    | Offense | Penalty |
    |---------|---------|
    | Invalid result | 10% of stake |
    | Missed deadline | 1--3% of stake |
    | Proven fraud | 25--50% of stake |

    Slashed tokens are sent to the protocol treasury and insurance fund. In dispute cases, a portion (20%) goes to the challenger who identified the bad result.

---

## Security

??? question "How does commit-reveal prevent cheating?"
    The commit-reveal protocol prevents workers from copying each other's answers:

    1. **Commit phase** -- Each worker analyzes the media independently, then submits a *hash* of their result (not the result itself). The hash includes a random salt so it cannot be reverse-engineered.
    2. **Reveal phase** -- After the commit deadline passes, workers reveal their actual results along with the salt. The protocol verifies that each reveal matches the previously committed hash.

    Because commits are opaque hashes, a worker cannot see what others submitted before locking in their own answer. Any attempt to reveal a result that does not match the original commit is rejected on-chain.

??? question "What happens if workers disagree on a result?"
    DFPN uses **reputation-weighted consensus** to resolve disagreements:

    - Each worker's result is weighted by their reputation score (0--10,000)
    - Workers with a longer track record of accurate results carry more influence
    - The consensus verdict is determined by the reputation-weighted majority

    If the disagreement is significant, any participant can open a **dispute** by staking tokens. Disputes are resolved through additional review, and the losing party's stake is slashed while the winner receives a portion of the slashed amount.
