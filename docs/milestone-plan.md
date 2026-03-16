# Milestone Plan

This plan turns the roadmap into an implementation schedule with staffing assumptions and concrete deliverables.

## Staffing Assumptions

- 2 protocol engineers (Solana/Anchor)
- 2 ML engineers (model evaluation + inference pipeline)
- 1 backend/indexer engineer
- 1 frontend/UX engineer
- 1 product/ops lead
- Part-time security and legal advisors

Total: 7 full-time equivalents, plus part-time support.

## Milestones and Deliverables

### M0: Discovery and Specification (4-6 weeks)

- Threat model and abuse cases agreed.
- Evaluation harness with baseline datasets.
- Initial tokenomics draft and governance model.
- Acceptance: reproducible benchmarking and scoring policy.
- Owners: product/ops lead, ML engineers, protocol engineers.

### M1: Devnet MVP (6-8 weeks)

- Content registry, analysis marketplace, and model registry programs.
- Commit-reveal flow implemented.
- Single worker reference client (CPU).
- Basic CLI or web UI for submissions.
- Acceptance: end-to-end request -> result -> audit trail on devnet.
- Owners: protocol engineers, backend engineer, ML engineers, frontend/UX.

### M2: Testnet Pilot (8-10 weeks)

- Worker registry, staking, and reputation.
- Epoch scoring and reward distribution.
- Model versioning and retirement.
- Indexer + API for queries and dashboards.
- Acceptance: multiple workers and models with stable scoring across epochs.
- Owners: protocol engineers, backend engineer, ML engineers.

### M3: Mainnet Beta (8-12 weeks)

- SPL token treasury and reward pool live.
- Governance via SPL Governance (Realms).
- Security audit completed and fixes applied.
- Capped public beta with operational runbooks.
- Acceptance: stable economics, low dispute rate, and predictable fees.
- Owners: protocol engineers, product/ops lead, security advisors.

### M4: Scale and Partnerships (ongoing)

- Multi-modal model marketplace and curated model sets.
- Storage integrations (Arweave/IPFS + access controls).
- Dedicated re-recording and low-quality detection tracks.
- Partnerships with media platforms and verification orgs.
- Owners: product/ops lead, ML engineers, partnerships.

## Budget Ranges (Rough Order of Magnitude)

Budgets assume 7 FTE at regional market rates plus infra and audit costs. Adjust for location and scope.

- M0: $90k to $160k
- M1: $160k to $260k
- M2: $200k to $320k
- M3: $280k to $450k (includes security audit)
- M4: $70k to $120k per month (run and scale)

## Week-by-Week Schedule (Weeks 1-34)

- Weeks 1-2: threat model, tokenomics, dataset selection (product/ops, ML)
- Weeks 3-4: evaluation harness baseline, scoring metrics (ML, backend)
- Weeks 5-6: Solana program specs, account design (protocol)
- Weeks 7-8: content registry + request flow (protocol)
- Weeks 9-10: model registry + commit-reveal (protocol)
- Weeks 11-12: worker client MVP + storage integration (ML, backend)
- Weeks 13-14: devnet UI + end-to-end testing (frontend, backend)
- Weeks 15-16: worker registry + staking (protocol)
- Weeks 17-18: epoch scoring + rewards (protocol, backend)
- Weeks 19-20: model versioning + retirement (protocol, ML)
- Weeks 21-22: indexer + dashboards (backend, frontend)
- Weeks 23-24: testnet load testing + tuning (product/ops, protocol)
- Weeks 25-26: treasury + governance setup (protocol, product/ops)
- Weeks 27-28: security audit + fixes (protocol, security advisors)
- Weeks 29-30: mainnet beta readiness + runbooks (product/ops)
- Weeks 31-34: capped beta launch + stability hardening (all)

## Workstreams

- **Protocol**: Solana programs, staking, rewards, governance.
- **ML/Inference**: model registry, evaluation harness, worker stack.
- **Data**: dataset curation, labeling, provenance checks.
- **Product**: UI, API, onboarding, and support.
- **Security/Compliance**: audits, incident response, privacy policies.

## Dependencies

- Reliable datasets with clear licensing.
- Access to Solana testnet resources and RPC providers.
- External storage services with predictable uptime.

## Risks and Contingencies

- **Model performance drift**: schedule frequent benchmark refreshes.
- **Fee volatility**: include dynamic fee caps and priority fee support.
- **Worker churn**: bootstrap with higher rewards and slashing grace periods.
- **Sybil pressure**: tighten stake requirements and reputation weights.
