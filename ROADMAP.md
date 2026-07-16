# DFPN Roadmap

**Vision:** make deepfake detection a *verifiable, decentralized public utility* rather than a proprietary black box. DFPN is verifiable-AI-as-DePIN: many independent operators run detection models on their own GPUs, a commit-reveal protocol stops collusion, reputation-weighted consensus produces a verdict, outliers are slashed, and every result is anchored on Solana with a reconstructable audit trail. The goal is a network where *nobody* — not Cryptuon, not any single vendor — owns the definition of "real," and where the honest strategy is always the profitable one.

This document is the north-star plan. It complements the more granular [`docs/roadmap.md`](docs/roadmap.md) and [`docs/milestone-plan.md`](docs/milestone-plan.md), which hold the phase-by-phase engineering schedule and budget ranges. Timelines here are directional and depend on team size and funding; each milestone is defined by measurable deliverables, not dates.

> **Where DFPN fits in the 2026 stack:** on-chain & verifiable AI + DePIN. As AI outputs drive more consequential decisions, "trust the model" stops scaling. DFPN's contribution is a concrete, narrow, checkable instance of verifiable AI — a media-authenticity verdict backed by staked, independent compute — rather than a general-purpose "AI oracle." Staying narrow is a feature: one checkable claim, real economic stakes, no privileged party who should own the answer.

---

## Guiding Principles

- **Coordination, not inference.** DFPN never runs the model. It tracks, incentivizes, and settles. Detection capability is provided permissionlessly by operators and model developers. (See [`docs/operational-model.md`](docs/operational-model.md).)
- **Economic security over trusted parties.** Stake, rewards, and slashing replace admin privilege. Diversity of models and operators is a security primitive, not a nice-to-have.
- **Auditability is the product.** A verdict without a reconstructable on-chain trail is worth less than no verdict. Indexers are untrusted; clients verify against on-chain state.
- **Honest scope.** DFPN is not a takedown service, not legal attribution, not private on-chain inference. Detection is a strong *signal*, never proof. (See [`docs/threat-model.md`](docs/threat-model.md).)

---

## Milestones

### Near term — Devnet MVP → Testnet pilot

**Objective: prove the loop end to end and stabilize scoring across epochs.**

- Solana programs live on devnet: content registry, analysis marketplace, model registry, worker registry, rewards. (Program set: [`docs/solana-design.md`](docs/solana-design.md).)
- Commit-reveal flow working end to end: submit → route → analyze → commit → reveal → consensus → reward, with the audit trail queryable on-chain.
- Reference worker daemon (Rust · Tokio) processing real requests; CPU and GPU configs (`config.yaml` / `config-cpu.yaml`).
- Worker registry, staking, and reputation; epoch-based scoring and reward distribution.
- Indexer + REST API for read access and dashboards.
- Multi-worker testnet load testing; stable scoring across epochs; model versioning and retirement.
- **Exit criteria:** multiple independent workers and models, low disagreement variance, reproducible benchmarking harness with rotating hidden test sets.

### Mid term — Mainnet beta

**Objective: turn on real economics under a security audit, with capped volume.**

- SPL token treasury and reward pool live; fee splits enforced on-chain (65% workers / 20% model developers / 10% treasury / 5% insurance).
- Governance via SPL Governance (Realms): fee splits, stake floors, slashing ranges, emission cadence, benchmark rotation.
- Security audit of all programs completed and fixes applied; time-locked upgrade authority under multisig.
- Capped public beta with clear SLAs, operational runbooks, and an incident-response workflow (detect → triage → contain → eradicate → recover → postmortem).
- **Exit criteria:** stable economics, low dispute rate, predictable fees, no unresolved audit findings.

### Long term — Scale & ecosystem

**Objective: become the default neutral verdict layer for media authenticity.**

- Curated model sets and a broader multi-modal model marketplace; permissionless registration remains the default.
- Advanced storage integrations (Arweave/IPFS + access controls) with signed storage attestations to close the storage-substitution vector.
- Specialized detection tracks: re-recording, low-quality/compressed manipulations, novel-generator adaptation.
- Verifiable-AI adjacencies to evaluate (honestly, only if they strengthen the core): restaking-style shared security for operator stake, prediction-market-style challenge markets on contested verdicts, and provenance interop (C2PA-signed inputs as an additional signal, not a replacement).
- Partnerships with content platforms, newsrooms, and fact-checking / verification organizations.

### Definition of "working"

- Requests are posted and settled on Solana with provable incentives.
- Models can be added, evaluated against hidden test sets, and retired.
- Users can audit any verdict and its provenance purely from on-chain state.

---

## Cheapest path to production

The fastest, lowest-cost way to a *credible, verifiable* production network — not just a demo. DFPN is Solana/Anchor, so the analysis is about **which Solana environment** and **what minimum viable set of changes** get us to a network people can actually rely on.

### Cheapest viable chain / infra target: Solana mainnet-beta (thin), not a new devnet cycle

For a Solana/Anchor protocol, **Solana mainnet-beta is already the cheapest production-grade L1 available** — there is no cheaper "real" chain to move to, and the whole point (auditable, tamper-evident verdicts) requires a chain with real finality and real economic weight behind its state. Devnet/testnet are free but produce *no* production credibility: state is periodically reset, tokens are valueless, and no external party will trust a verdict anchored there. So the cheapest path is **not** to keep iterating on devnet indefinitely; it is to keep *development* on devnet and get a **thin, audited slice** onto mainnet-beta as early as possible.

Concretely, Solana is the right cost target because:

- **Per-transaction cost is negligible.** Base fees are a fraction of a cent; DFPN's on-chain footprint is hashes, commitments, reveals, and metadata — never media — so account rent and compute stay small (the design explicitly keeps accounts small and avoids on-chain loops via epoch aggregation).
- **No custom chain, no rollup infra to operate.** Standard primitives only: Anchor programs, SPL Token, SPL Governance (Realms). Nothing to run, sequence, or bridge.
- **The expensive line item is the audit, not the chain.** Budget reality (from [`docs/milestone-plan.md`](docs/milestone-plan.md)) is that mainnet-beta cost is dominated by the security audit, not gas or infra.

**Cost-minimizing sequencing:** ship a *reduced-surface* mainnet-beta first — the fewest programs and instructions that let a real client submit a request and reconstruct a real verdict — so the audit scope (and therefore the largest cost) is as small as possible. Expand the on-chain surface only after the thin slice is live and audited.

### Production-viability changes (the checklist that turns devnet into a network people trust)

1. **Operator onboarding** — Make it possible to bootstrap real GPU supply cheaply. Ship a one-command worker install path, publish a plain-language stake/reward/slashing explainer, and run a **higher-reward, slashing-grace bootstrap epoch** so early operators aren't punished for network-wide teething issues (an explicit contingency in the milestone plan). Target: a new operator goes from bare hardware to a first paid verdict in under an hour.

2. **Detection-model accuracy benchmarks** — Stand up the evaluation harness with **rotating, hidden test sets** *before* mainnet economics go live. Publish reproducible per-modality accuracy and false-positive/false-negative rates for the reference models (face manipulation, AI-generated images, video authenticity, voice cloning). No mainnet rewards should flow against a benchmark a developer could overfit. This is the single biggest driver of whether verdicts are trustworthy.

3. **Staking / slashing parameter audit** — Before real value is at stake, pin and stress-test the economic parameters: worker stake floor (≥ 5,000 DFPN, scaling ≥ 30× median request fee/epoch), model-developer stake (≥ 20,000 DFPN/version), challenger escrow (5% of disputed reward), and slashing bands (10% invalid / 25–50% fraud / 1–3% missed deadline). Verify with a small adversarial simulation that a sybil swarm or a colluding cartel is *net-unprofitable* at these settings, and wire the parameters to governance so they can be tightened without a redeploy.

4. **Media-hash data availability** — The chain stores only the content hash + storage URI; the media lives off-chain (IPFS / Arweave / S3). For production, close the **storage-substitution** vector: workers must fetch bytes and verify them against the on-chain hash before inference (already the design intent), and requests should support **multi-source retrieval** plus optional signed storage attestations so a swapped URI can't quietly change what gets analyzed. Recommend Arweave (or IPFS with a paid pinning guarantee) for anything that must remain auditable long after the verdict.

5. **Dispute handling** — Ship the **challenge window before slashing** and the challenger-escrow path as first-class, not future work. A verdict that can be contested — with a staked challenger, a resolution rule, and the insurance pool backstopping catastrophic worker failure — is what makes the audit trail meaningful. Keep an emergency governance pause available for exploit conditions.

6. **Monitoring** — Production needs the detection signals from the threat model wired to alerts *day one*: sudden accuracy drops or disagreement-rate spikes, high per-epoch worker variance, reward concentration to a small operator set, repeated disputes from the same party, and storage fetch failures / hash mismatches. The Rust indexer already mirrors on-chain state; add health metrics (`/health`, `/metrics`) and dashboards so the network is observable before, not after, the first incident.

**Bottom line:** the cheapest path to production is *not* a cheaper chain — Solana mainnet-beta already is the cheap, credible target — it is **minimizing audited on-chain surface** and **front-loading the six changes above** (benchmarks, parameter audit, and dispute handling first, since they gate trust) so that a thin, honest slice can go live and be relied upon at the lowest possible spend.

---

*DFPN is one of the open-source blockchain-infrastructure projects from [Cryptuon Research](https://www.cryptuon.com). Site: [dfpn.cryptuon.com](https://dfpn.cryptuon.com/) · Docs: [docs.cryptuon.com/dfpn](https://docs.cryptuon.com/dfpn/) · MIT License.*
