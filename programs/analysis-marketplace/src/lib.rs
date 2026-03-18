use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use sha2::{Digest, Sha256};

use dfpn_shared::{
    constants, seeds, utils, ConsensusType, DfpnError, DisputeReason, DisputeStatus, Modalities,
    RequestStatus, Verdict, WorkerStatus,
    cpi as rewards_cpi,
};

declare_id!("9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin");

#[program]
pub mod analysis_marketplace {
    use super::*;

    /// Create an analysis request
    pub fn create_request(
        ctx: Context<CreateRequest>,
        content_hash: [u8; 32],
        storage_uri: String,
        required_modalities: u8,
        min_workers: u8,
        fee_amount: u64,
        deadline: i64,
        nonce: u64,
    ) -> Result<()> {
        // Validate inputs
        require!(
            storage_uri.len() <= constants::MAX_URI_LENGTH,
            DfpnError::StringTooLong
        );

        let modalities = Modalities::from_bits(required_modalities);
        require!(!modalities.is_empty(), DfpnError::InvalidModality);

        require!(
            min_workers > 0 && min_workers <= constants::MAX_WORKERS_PER_REQUEST,
            DfpnError::InvalidModality
        );

        let clock = Clock::get()?;
        let time_until_deadline = deadline - clock.unix_timestamp;
        require!(
            time_until_deadline >= constants::MIN_DEADLINE_SECONDS,
            DfpnError::DeadlineTooShort
        );

        require!(fee_amount > 0, DfpnError::InvalidFeeAmount);

        // Transfer fee to escrow
        let cpi_accounts = Transfer {
            from: ctx.accounts.fee_source.to_account_info(),
            to: ctx.accounts.fee_escrow.to_account_info(),
            authority: ctx.accounts.requester.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, fee_amount)?;

        // Calculate commit deadline
        let commit_deadline = utils::calculate_commit_deadline(
            clock.unix_timestamp,
            deadline,
            constants::DEFAULT_COMMIT_RATIO_BPS,
        );

        // Initialize request
        let request = &mut ctx.accounts.request_account;
        request.requester = ctx.accounts.requester.key();
        request.content_hash = content_hash;
        request.storage_uri = storage_uri.clone();
        request.required_modalities = modalities;
        request.min_workers = min_workers;
        request.fee_amount = fee_amount;
        request.deadline = deadline;
        request.commit_deadline = commit_deadline;
        request.created_at = clock.unix_timestamp;
        request.status = RequestStatus::Open;
        request.commit_count = 0;
        request.reveal_count = 0;
        request.nonce = nonce;
        request.final_verdict = None;
        request.final_confidence = None;
        request.consensus_type = None;
        request.dispute_count = 0;
        request.bump = ctx.bumps.request_account;

        emit!(RequestCreated {
            request_id: request.key(),
            requester: request.requester,
            content_hash,
            storage_uri,
            required_modalities,
            min_workers,
            fee_amount,
            deadline,
            commit_deadline,
        });

        Ok(())
    }

    /// Worker commits a result hash (phase 1 of commit-reveal)
    pub fn commit_result(ctx: Context<CommitResultCtx>, commitment: [u8; 32]) -> Result<()> {
        let request = &ctx.accounts.request_account;
        let worker = &ctx.accounts.worker_account;
        let clock = Clock::get()?;

        // Verify request is open and in commit window
        require!(
            request.status == RequestStatus::Open,
            DfpnError::RequestNotOpen
        );
        require!(
            clock.unix_timestamp < request.commit_deadline,
            DfpnError::CommitWindowClosed
        );

        // Verify worker is active and has sufficient stake
        require!(
            matches!(worker.status, WorkerStatus::Active),
            DfpnError::WorkerNotActive
        );
        require!(
            worker.stake >= constants::MIN_WORKER_STAKE,
            DfpnError::InsufficientStake
        );

        // Verify worker supports required modalities
        require!(
            worker
                .supported_modalities
                .supports(&request.required_modalities),
            DfpnError::ModalityMismatch
        );

        // Initialize commit account
        let commit = &mut ctx.accounts.commit_account;
        commit.request = request.key();
        commit.worker = ctx.accounts.operator.key();
        commit.commitment = commitment;
        commit.commit_slot = clock.slot;
        commit.revealed = false;
        commit.bump = ctx.bumps.commit_account;

        // Increment commit count
        let request = &mut ctx.accounts.request_account;
        request.commit_count = request.commit_count.saturating_add(1);

        emit!(ResultCommitted {
            request_id: request.key(),
            worker: commit.worker,
            commitment,
            commit_slot: commit.commit_slot,
        });

        Ok(())
    }

    /// Worker reveals the actual result (phase 2 of commit-reveal)
    pub fn reveal_result(
        ctx: Context<RevealResultCtx>,
        verdict: Verdict,
        confidence: u8,
        detections_hash: [u8; 32],
        salt: [u8; 16],
    ) -> Result<()> {
        let request = &ctx.accounts.request_account;
        let commit = &ctx.accounts.commit_account;
        let clock = Clock::get()?;

        // Verify reveal window is open
        require!(
            clock.unix_timestamp >= request.commit_deadline,
            DfpnError::RevealWindowNotOpen
        );
        require!(
            clock.unix_timestamp < request.deadline,
            DfpnError::RevealWindowClosed
        );

        // Verify commitment exists and hasn't been revealed
        require!(commit.commitment != [0u8; 32], DfpnError::NoCommitment);
        require!(!commit.revealed, DfpnError::AlreadyRevealed);

        // Verify commitment matches
        let result_bytes = encode_result(verdict, confidence, &detections_hash);
        let computed = compute_commitment(
            &result_bytes,
            &salt,
            &commit.worker,
            &commit.request,
        );
        require!(computed == commit.commitment, DfpnError::InvalidCommitment);

        // Mark commitment as revealed
        let commit = &mut ctx.accounts.commit_account;
        commit.revealed = true;

        // Initialize reveal account
        let reveal = &mut ctx.accounts.reveal_account;
        reveal.request = request.key();
        reveal.worker = commit.worker;
        reveal.model = ctx.accounts.model_account.key();
        reveal.verdict = verdict;
        reveal.confidence = confidence;
        reveal.detections_hash = detections_hash;
        reveal.reveal_slot = clock.slot;
        reveal.bump = ctx.bumps.reveal_account;

        // Increment reveal count
        let request = &mut ctx.accounts.request_account;
        request.reveal_count = request.reveal_count.saturating_add(1);

        // If commit window has passed and we have enough reveals, close commits
        if request.status == RequestStatus::Open
            && clock.unix_timestamp >= request.commit_deadline
        {
            request.status = RequestStatus::CommitClosed;
        }

        emit!(ResultRevealed {
            request_id: request.key(),
            worker: reveal.worker,
            model: reveal.model,
            verdict,
            confidence,
            reveal_slot: reveal.reveal_slot,
        });

        Ok(())
    }

    /// Finalize request and distribute rewards
    /// Remaining accounts should be reveal accounts for this request
    pub fn finalize_request(ctx: Context<FinalizeRequest>) -> Result<()> {
        let request = &ctx.accounts.request_account;
        let clock = Clock::get()?;

        // Must be past deadline or have sufficient reveals
        require!(
            clock.unix_timestamp >= request.deadline
                || request.reveal_count >= request.min_workers,
            DfpnError::InsufficientReveals
        );

        // Cannot finalize already finalized requests
        require!(
            request.status != RequestStatus::Finalized,
            DfpnError::RequestAlreadyFinalized
        );

        // Verify rewards program ID
        require!(
            ctx.accounts.rewards_program.key() == rewards_cpi::rewards_program_id(),
            DfpnError::InvalidAuthority
        );

        // Read values we need for transfer before mutable borrow
        let content_hash = request.content_hash;
        let nonce = request.nonce;
        let bump = request.bump;
        let fee_amount = request.fee_amount;
        let reveal_count = request.reveal_count;
        let min_workers = request.min_workers;
        let request_key = request.key();

        // Calculate consensus from reveal accounts in remaining_accounts
        // Also extract worker contributions for reward distribution
        let (final_verdict, final_confidence, consensus_type, worker_contributions) =
            calculate_consensus_with_workers(&ctx.remaining_accounts, &request_key, reveal_count)?;

        // Transfer fees from escrow to fee vault for distribution
        let nonce_bytes = nonce.to_le_bytes();
        let seeds = &[
            seeds::REQUEST,
            content_hash.as_ref(),
            nonce_bytes.as_ref(),
            &[bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_accounts = Transfer {
            from: ctx.accounts.fee_escrow.to_account_info(),
            to: ctx.accounts.fee_vault.to_account_info(),
            authority: ctx.accounts.request_account.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        token::transfer(cpi_ctx, fee_amount)?;

        // Now update status and consensus
        let request = &mut ctx.accounts.request_account;
        let should_distribute_rewards = reveal_count >= min_workers;

        if should_distribute_rewards {
            request.status = RequestStatus::Finalized;
            request.final_verdict = Some(final_verdict);
            request.final_confidence = Some(final_confidence);
            request.consensus_type = Some(consensus_type);
        } else {
            request.status = RequestStatus::Expired;
        }

        // Drop the mutable borrow before CPI
        let _ = request;

        // Call rewards program via CPI to process reward distribution
        if should_distribute_rewards {
            // Extract model developer from first reveal (simplified - could aggregate)
            let model_developer = extract_model_developer(&ctx.remaining_accounts);

            rewards_cpi::process_request_rewards(
                ctx.accounts.payer.to_account_info(),
                ctx.accounts.rewards_treasury.to_account_info(),
                ctx.accounts.rewards_epoch_config.to_account_info(),
                ctx.accounts.fee_vault.to_account_info(),
                ctx.accounts.token_program.to_account_info(),
                ctx.accounts.rewards_program.to_account_info(),
                fee_amount,
                &worker_contributions,
                model_developer,
                None, // No signer seeds needed - payer signs directly
            )?;
        }

        emit!(RequestFinalized {
            request_id: ctx.accounts.request_account.key(),
            status: ctx.accounts.request_account.status,
            reveal_count,
            fee_amount,
        });

        Ok(())
    }

    /// Cancel request before any commits (full refund)
    pub fn cancel_request(ctx: Context<CancelRequest>) -> Result<()> {
        let request = &ctx.accounts.request_account;

        // Can only cancel if no commits yet
        require!(request.commit_count == 0, DfpnError::CannotCancelWithCommits);
        require!(
            request.status == RequestStatus::Open,
            DfpnError::RequestNotOpen
        );

        // Read values before mutable borrow
        let content_hash = request.content_hash;
        let nonce = request.nonce;
        let bump = request.bump;
        let fee_amount = request.fee_amount;

        // Refund fee to requester
        let nonce_bytes = nonce.to_le_bytes();
        let seeds = &[
            seeds::REQUEST,
            content_hash.as_ref(),
            nonce_bytes.as_ref(),
            &[bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_accounts = Transfer {
            from: ctx.accounts.fee_escrow.to_account_info(),
            to: ctx.accounts.fee_destination.to_account_info(),
            authority: ctx.accounts.request_account.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        token::transfer(cpi_ctx, fee_amount)?;

        // Update status
        let request = &mut ctx.accounts.request_account;
        request.status = RequestStatus::Cancelled;

        emit!(RequestCancelled {
            request_id: request.key(),
            fee_refunded: fee_amount,
        });

        Ok(())
    }

    /// Mark request as expired and handle partial refund
    pub fn expire_request(ctx: Context<ExpireRequest>) -> Result<()> {
        let request = &ctx.accounts.request_account;
        let clock = Clock::get()?;

        // Must be past deadline
        require!(
            clock.unix_timestamp >= request.deadline,
            DfpnError::RequestNotOpen
        );

        // Must not have enough reveals
        require!(
            request.reveal_count < request.min_workers,
            DfpnError::RequestAlreadyFinalized
        );

        // Read values before mutable borrow
        let content_hash = request.content_hash;
        let nonce = request.nonce;
        let bump = request.bump;
        let fee_amount = request.fee_amount;
        let reveal_count = request.reveal_count;
        let min_workers = request.min_workers;

        // Calculate refund (partial if some work was done)
        let refund_amount = if reveal_count == 0 {
            fee_amount
        } else {
            // Partial refund: proportion not fulfilled
            let fulfilled_ratio =
                reveal_count as u64 * constants::BPS_DENOMINATOR as u64
                    / min_workers as u64;
            let used = utils::calculate_share(fee_amount, fulfilled_ratio as u16)
                .unwrap_or(0);
            fee_amount.saturating_sub(used)
        };

        if refund_amount > 0 {
            let nonce_bytes = nonce.to_le_bytes();
            let seeds = &[
                seeds::REQUEST,
                content_hash.as_ref(),
                nonce_bytes.as_ref(),
                &[bump],
            ];
            let signer_seeds = &[&seeds[..]];

            let cpi_accounts = Transfer {
                from: ctx.accounts.fee_escrow.to_account_info(),
                to: ctx.accounts.fee_destination.to_account_info(),
                authority: ctx.accounts.request_account.to_account_info(),
            };
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
            token::transfer(cpi_ctx, refund_amount)?;
        }

        // Update status
        let request = &mut ctx.accounts.request_account;
        request.status = RequestStatus::Expired;

        emit!(RequestExpired {
            request_id: request.key(),
            fee_refunded: refund_amount,
            reveals_received: reveal_count,
        });

        Ok(())
    }

    /// Open a dispute against a worker's result
    pub fn open_dispute(
        ctx: Context<OpenDispute>,
        reason: DisputeReason,
        evidence_hash: [u8; 32],
    ) -> Result<()> {
        let request = &ctx.accounts.request_account;
        let reveal = &ctx.accounts.reveal_account;
        let clock = Clock::get()?;

        // Request must be finalized
        require!(
            request.status == RequestStatus::Finalized,
            DfpnError::RequestNotFinalized
        );

        // Cannot dispute own result
        require!(
            ctx.accounts.challenger.key() != reveal.worker,
            DfpnError::CannotDisputeOwnResult
        );

        // Transfer stake from challenger to dispute escrow
        let cpi_accounts = Transfer {
            from: ctx.accounts.challenger_token_account.to_account_info(),
            to: ctx.accounts.dispute_escrow.to_account_info(),
            authority: ctx.accounts.challenger.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, constants::MIN_DISPUTE_STAKE)?;

        // Initialize dispute
        let dispute = &mut ctx.accounts.dispute_account;
        dispute.request = request.key();
        dispute.reveal = reveal.key();
        dispute.challenged_worker = reveal.worker;
        dispute.challenger = ctx.accounts.challenger.key();
        dispute.reason = reason;
        dispute.evidence_hash = evidence_hash;
        dispute.stake_amount = constants::MIN_DISPUTE_STAKE;
        dispute.status = DisputeStatus::Open;
        dispute.created_at = clock.unix_timestamp;
        dispute.resolved_at = None;
        dispute.resolver = None;
        dispute.bump = ctx.bumps.dispute_account;

        // Increment dispute count on request
        let request = &mut ctx.accounts.request_account;
        request.dispute_count = request.dispute_count.saturating_add(1);

        // If this is the first dispute, mark request as disputed
        if request.dispute_count == 1 {
            request.status = RequestStatus::Disputed;
        }

        emit!(DisputeOpened {
            dispute_id: dispute.key(),
            request_id: request.key(),
            reveal_id: reveal.key(),
            challenger: dispute.challenger,
            challenged_worker: dispute.challenged_worker,
            reason,
            evidence_hash,
            stake_amount: dispute.stake_amount,
        });

        Ok(())
    }

    /// Resolve a dispute (authority only)
    pub fn resolve_dispute(
        ctx: Context<ResolveDispute>,
        in_favor_of_challenger: bool,
        slash_amount: u64,
    ) -> Result<()> {
        let clock = Clock::get()?;

        // Read values we need before any mutable borrow
        let dispute_status = ctx.accounts.dispute_account.status;
        let dispute_stake = ctx.accounts.dispute_account.stake_amount;
        let dispute_request = ctx.accounts.dispute_account.request;
        let dispute_reveal = ctx.accounts.dispute_account.reveal;
        let dispute_bump = ctx.accounts.dispute_account.bump;
        let dispute_key = ctx.accounts.dispute_account.key();

        // Dispute must be open
        require!(
            matches!(dispute_status, DisputeStatus::Open | DisputeStatus::UnderReview),
            DfpnError::DisputeAlreadyResolved
        );

        // Build signer seeds
        let seeds = &[
            seeds::DISPUTE,
            dispute_request.as_ref(),
            dispute_reveal.as_ref(),
            &[dispute_bump],
        ];
        let signer_seeds = &[&seeds[..]];

        if in_favor_of_challenger {
            // Return challenger's stake
            let cpi_accounts = Transfer {
                from: ctx.accounts.dispute_escrow.to_account_info(),
                to: ctx.accounts.challenger_token_account.to_account_info(),
                authority: ctx.accounts.dispute_account.to_account_info(),
            };
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
            token::transfer(cpi_ctx, dispute_stake)?;

            // Update dispute status
            let dispute = &mut ctx.accounts.dispute_account;
            dispute.resolved_at = Some(clock.unix_timestamp);
            dispute.resolver = Some(ctx.accounts.resolver.key());
            dispute.status = DisputeStatus::ResolvedForChallenger;

            // Calculate challenger reward from slash
            let challenger_reward = utils::calculate_share(
                slash_amount,
                constants::DISPUTE_CHALLENGER_REWARD_BPS,
            )
            .unwrap_or(0);

            emit!(DisputeResolved {
                dispute_id: dispute_key,
                in_favor_of_challenger: true,
                resolver: ctx.accounts.resolver.key(),
                slash_amount,
                challenger_reward,
            });
        } else {
            // Forfeit challenger's stake to treasury
            let cpi_accounts = Transfer {
                from: ctx.accounts.dispute_escrow.to_account_info(),
                to: ctx.accounts.treasury_vault.to_account_info(),
                authority: ctx.accounts.dispute_account.to_account_info(),
            };
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
            token::transfer(cpi_ctx, dispute_stake)?;

            // Update dispute status
            let dispute = &mut ctx.accounts.dispute_account;
            dispute.resolved_at = Some(clock.unix_timestamp);
            dispute.resolver = Some(ctx.accounts.resolver.key());
            dispute.status = DisputeStatus::ResolvedForWorker;

            emit!(DisputeResolved {
                dispute_id: dispute_key,
                in_favor_of_challenger: false,
                resolver: ctx.accounts.resolver.key(),
                slash_amount: 0,
                challenger_reward: 0,
            });
        }

        // Decrement dispute count
        let request = &mut ctx.accounts.request_account;
        request.dispute_count = request.dispute_count.saturating_sub(1);

        // If no more disputes, restore finalized status
        if request.dispute_count == 0 && request.status == RequestStatus::Disputed {
            request.status = RequestStatus::Finalized;
        }

        Ok(())
    }

    /// Close an expired dispute (anyone can call after timeout)
    pub fn close_dispute(ctx: Context<CloseDispute>) -> Result<()> {
        let clock = Clock::get()?;

        // Read values we need before any mutable borrow
        let dispute_status = ctx.accounts.dispute_account.status;
        let dispute_created_at = ctx.accounts.dispute_account.created_at;
        let dispute_stake = ctx.accounts.dispute_account.stake_amount;
        let dispute_request = ctx.accounts.dispute_account.request;
        let dispute_reveal = ctx.accounts.dispute_account.reveal;
        let dispute_bump = ctx.accounts.dispute_account.bump;
        let dispute_key = ctx.accounts.dispute_account.key();

        // Must be open
        require!(
            matches!(dispute_status, DisputeStatus::Open | DisputeStatus::UnderReview),
            DfpnError::DisputeAlreadyResolved
        );

        // Must be past timeout
        let timeout = dispute_created_at + constants::DISPUTE_TIMEOUT_SECONDS;
        require!(
            clock.unix_timestamp >= timeout,
            DfpnError::DisputeTimeoutNotReached
        );

        // Build signer seeds
        let seeds = &[
            seeds::DISPUTE,
            dispute_request.as_ref(),
            dispute_reveal.as_ref(),
            &[dispute_bump],
        ];
        let signer_seeds = &[&seeds[..]];

        // Return stake to challenger
        let cpi_accounts = Transfer {
            from: ctx.accounts.dispute_escrow.to_account_info(),
            to: ctx.accounts.challenger_token_account.to_account_info(),
            authority: ctx.accounts.dispute_account.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        token::transfer(cpi_ctx, dispute_stake)?;

        // Dismiss the dispute
        let dispute = &mut ctx.accounts.dispute_account;
        dispute.status = DisputeStatus::Dismissed;
        dispute.resolved_at = Some(clock.unix_timestamp);

        // Decrement dispute count
        let request = &mut ctx.accounts.request_account;
        request.dispute_count = request.dispute_count.saturating_sub(1);

        if request.dispute_count == 0 && request.status == RequestStatus::Disputed {
            request.status = RequestStatus::Finalized;
        }

        emit!(DisputeClosed {
            dispute_id: dispute_key,
        });

        Ok(())
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Calculate consensus verdict and extract worker contributions from reveal accounts
fn calculate_consensus_with_workers(
    remaining_accounts: &[AccountInfo],
    _request_key: &Pubkey,
    expected_count: u8,
) -> Result<(Verdict, u8, ConsensusType, Vec<rewards_cpi::WorkerContribution>)> {
    let mut worker_contributions: Vec<rewards_cpi::WorkerContribution> = Vec::new();

    // If no reveals, return inconclusive
    if remaining_accounts.is_empty() || expected_count == 0 {
        return Ok((Verdict::Inconclusive, 0, ConsensusType::Split, worker_contributions));
    }

    // Count votes and aggregate confidence
    let mut authentic_votes: u32 = 0;
    let mut manipulated_votes: u32 = 0;
    let mut inconclusive_votes: u32 = 0;
    let mut total_confidence: u64 = 0;
    let mut valid_reveals: u32 = 0;

    // Deserialize reveal accounts from remaining_accounts
    for account_info in remaining_accounts.iter() {
        // Skip if account doesn't have enough data
        if account_info.data_len() < ResultReveal::LEN {
            continue;
        }

        // Try to deserialize as ResultReveal
        let data = account_info.try_borrow_data()?;

        // Check discriminator (first 8 bytes should match ResultReveal)
        // For simplicity, we'll trust the accounts passed in and just parse the verdict/confidence
        // In production, you'd want to verify the discriminator and that reveal.request == request_key

        // Skip discriminator (8 bytes), request (32), worker (32), model (32)
        // Layout: discriminator[8] + request[32] + worker[32] + model[32] + verdict[1] + confidence[1]
        // Worker is at offset 40 (8 + 32)
        // Verdict is at offset 104 (8 + 32 + 32 + 32)
        if data.len() >= 106 {
            // Extract worker pubkey
            let worker_bytes: [u8; 32] = data[40..72].try_into().unwrap_or([0u8; 32]);
            let worker = Pubkey::new_from_array(worker_bytes);

            let verdict_byte = data[104];
            let confidence = data[105];

            match verdict_byte {
                0 => authentic_votes += 1,    // Verdict::Authentic
                1 => manipulated_votes += 1, // Verdict::Manipulated
                _ => inconclusive_votes += 1, // Verdict::Inconclusive or unknown
            }

            total_confidence += confidence as u64;
            valid_reveals += 1;

            // Add worker contribution
            // Default reputation of 5000 (50%) - in production, would fetch from worker registry
            worker_contributions.push(rewards_cpi::WorkerContribution {
                worker,
                reputation: constants::INITIAL_REPUTATION,
                participated: true,
            });
        }
    }

    // If no valid reveals, return inconclusive
    if valid_reveals == 0 {
        return Ok((Verdict::Inconclusive, 0, ConsensusType::Split, worker_contributions));
    }

    // Determine consensus type and final verdict
    let total_votes = valid_reveals;
    let avg_confidence = (total_confidence / valid_reveals as u64) as u8;

    // Single worker case
    if total_votes == 1 {
        let verdict = if authentic_votes == 1 {
            Verdict::Authentic
        } else if manipulated_votes == 1 {
            Verdict::Manipulated
        } else {
            Verdict::Inconclusive
        };
        return Ok((verdict, avg_confidence, ConsensusType::Single, worker_contributions));
    }

    // Check for unanimous consensus
    if authentic_votes == total_votes {
        return Ok((Verdict::Authentic, avg_confidence, ConsensusType::Unanimous, worker_contributions));
    }
    if manipulated_votes == total_votes {
        return Ok((Verdict::Manipulated, avg_confidence, ConsensusType::Unanimous, worker_contributions));
    }
    if inconclusive_votes == total_votes {
        return Ok((Verdict::Inconclusive, avg_confidence, ConsensusType::Unanimous, worker_contributions));
    }

    // Check for majority (>50%)
    let majority_threshold = total_votes / 2 + 1;

    if authentic_votes >= majority_threshold {
        return Ok((Verdict::Authentic, avg_confidence, ConsensusType::Majority, worker_contributions));
    }
    if manipulated_votes >= majority_threshold {
        return Ok((Verdict::Manipulated, avg_confidence, ConsensusType::Majority, worker_contributions));
    }

    // No clear majority - split result
    // Default to the verdict with most votes, or Inconclusive if tied
    let verdict = if authentic_votes > manipulated_votes && authentic_votes > inconclusive_votes {
        Verdict::Authentic
    } else if manipulated_votes > authentic_votes && manipulated_votes > inconclusive_votes {
        Verdict::Manipulated
    } else {
        Verdict::Inconclusive
    };

    Ok((verdict, avg_confidence, ConsensusType::Split, worker_contributions))
}

/// Extract model developer from reveal accounts (simplified - takes first reveal's model)
fn extract_model_developer(remaining_accounts: &[AccountInfo]) -> Option<Pubkey> {
    // In a full implementation, we'd aggregate all models used and distribute model rewards
    // For now, just extract the first model's developer
    for account_info in remaining_accounts.iter() {
        if account_info.data_len() >= ResultReveal::LEN {
            let data = match account_info.try_borrow_data() {
                Ok(d) => d,
                Err(_) => continue,
            };
            // Model is at offset 72 (8 + 32 + 32)
            if data.len() >= 104 {
                let model_bytes: [u8; 32] = match data[72..104].try_into() {
                    Ok(b) => b,
                    Err(_) => continue,
                };
                let model = Pubkey::new_from_array(model_bytes);
                // Return the model pubkey as "developer" for simplicity
                // In production, you'd fetch the model account and get the actual developer
                return Some(model);
            }
        }
    }
    None
}

fn encode_result(verdict: Verdict, confidence: u8, detections_hash: &[u8; 32]) -> Vec<u8> {
    let mut result = Vec::with_capacity(34);
    result.push(verdict as u8);
    result.push(confidence);
    result.extend_from_slice(detections_hash);
    result
}

fn compute_commitment(
    result_bytes: &[u8],
    salt: &[u8; 16],
    worker: &Pubkey,
    request: &Pubkey,
) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(result_bytes);
    hasher.update(salt);
    hasher.update(worker.as_ref());
    hasher.update(request.as_ref());
    hasher.finalize().into()
}

// ============================================================================
// Account Structures
// ============================================================================

#[account]
pub struct AnalysisRequest {
    /// Who submitted the request
    pub requester: Pubkey,
    /// SHA256 hash of the content
    pub content_hash: [u8; 32],
    /// Where to fetch the content
    pub storage_uri: String,
    /// Required analysis modalities
    pub required_modalities: Modalities,
    /// Minimum workers for consensus
    pub min_workers: u8,
    /// Fee amount in DFPN
    pub fee_amount: u64,
    /// Final deadline
    pub deadline: i64,
    /// Deadline for commits
    pub commit_deadline: i64,
    /// Creation timestamp
    pub created_at: i64,
    /// Current status
    pub status: RequestStatus,
    /// Number of commits received
    pub commit_count: u8,
    /// Number of reveals received
    pub reveal_count: u8,
    /// Nonce for uniqueness
    pub nonce: u64,
    /// Final aggregated verdict (set on finalization)
    pub final_verdict: Option<Verdict>,
    /// Final confidence score (0-100, set on finalization)
    pub final_confidence: Option<u8>,
    /// Consensus type achieved
    pub consensus_type: Option<ConsensusType>,
    /// Number of active disputes
    pub dispute_count: u8,
    /// PDA bump
    pub bump: u8,
}

impl AnalysisRequest {
    pub const LEN: usize = 8  // discriminator
        + 32  // requester
        + 32  // content_hash
        + 4 + constants::MAX_URI_LENGTH  // storage_uri
        + 1   // required_modalities
        + 1   // min_workers
        + 8   // fee_amount
        + 8   // deadline
        + 8   // commit_deadline
        + 8   // created_at
        + 1   // status
        + 1   // commit_count
        + 1   // reveal_count
        + 8   // nonce
        + 2   // final_verdict (Option<u8>)
        + 2   // final_confidence (Option<u8>)
        + 2   // consensus_type (Option<u8>)
        + 1   // dispute_count
        + 1   // bump
        + 32; // padding
}

#[account]
pub struct ResultCommit {
    /// Request this commit is for
    pub request: Pubkey,
    /// Worker who committed
    pub worker: Pubkey,
    /// Commitment hash
    pub commitment: [u8; 32],
    /// Slot when committed
    pub commit_slot: u64,
    /// Whether revealed
    pub revealed: bool,
    /// PDA bump
    pub bump: u8,
}

impl ResultCommit {
    pub const LEN: usize = 8  // discriminator
        + 32  // request
        + 32  // worker
        + 32  // commitment
        + 8   // commit_slot
        + 1   // revealed
        + 1   // bump
        + 16; // padding
}

#[account]
pub struct ResultReveal {
    /// Request this reveal is for
    pub request: Pubkey,
    /// Worker who revealed
    pub worker: Pubkey,
    /// Model used
    pub model: Pubkey,
    /// Analysis verdict
    pub verdict: Verdict,
    /// Confidence score (0-100)
    pub confidence: u8,
    /// Hash of detailed detections (stored off-chain)
    pub detections_hash: [u8; 32],
    /// Slot when revealed
    pub reveal_slot: u64,
    /// PDA bump
    pub bump: u8,
}

impl ResultReveal {
    pub const LEN: usize = 8  // discriminator
        + 32  // request
        + 32  // worker
        + 32  // model
        + 1   // verdict
        + 1   // confidence
        + 32  // detections_hash
        + 8   // reveal_slot
        + 1   // bump
        + 16; // padding
}

#[account]
pub struct Dispute {
    /// Request being disputed
    pub request: Pubkey,
    /// Reveal being challenged
    pub reveal: Pubkey,
    /// Worker being challenged
    pub challenged_worker: Pubkey,
    /// Who opened the dispute
    pub challenger: Pubkey,
    /// Reason for dispute
    pub reason: DisputeReason,
    /// SHA256 hash of evidence (stored off-chain)
    pub evidence_hash: [u8; 32],
    /// Stake locked by challenger
    pub stake_amount: u64,
    /// Current status
    pub status: DisputeStatus,
    /// When dispute was opened
    pub created_at: i64,
    /// When dispute was resolved (if resolved)
    pub resolved_at: Option<i64>,
    /// Who resolved the dispute (if resolved)
    pub resolver: Option<Pubkey>,
    /// PDA bump
    pub bump: u8,
}

impl Dispute {
    pub const LEN: usize = 8    // discriminator
        + 32   // request
        + 32   // reveal
        + 32   // challenged_worker
        + 32   // challenger
        + 1    // reason
        + 32   // evidence_hash
        + 8    // stake_amount
        + 1    // status
        + 8    // created_at
        + 9    // resolved_at (Option<i64>)
        + 33   // resolver (Option<Pubkey>)
        + 1    // bump
        + 16;  // padding
}

// ============================================================================
// Account holding worker state (imported from worker-registry)
// ============================================================================

#[account]
pub struct WorkerAccount {
    pub operator: Pubkey,
    pub stake: u64,
    pub reputation_score: u32,
    pub supported_modalities: Modalities,
    pub tasks_completed: u64,
    pub tasks_failed: u64,
    pub last_active_slot: u64,
    pub status: WorkerStatus,
    pub pending_unstake: u64,
    pub unstake_unlock_slot: u64,
    pub bump: u8,
}

#[account]
pub struct ModelAccount {
    pub developer: Pubkey,
    pub model_id: [u8; 32],
    pub name: String,
    pub version: String,
    pub modalities: Modalities,
    pub model_uri: String,
    pub checksum: [u8; 32],
    pub stake: u64,
    pub score: u32,
    pub status: dfpn_shared::ModelStatus,
    pub created_at: i64,
    pub updated_at: i64,
    pub total_uses: u64,
    pub bump: u8,
}

// ============================================================================
// Instruction Contexts
// ============================================================================

#[derive(Accounts)]
#[instruction(content_hash: [u8; 32], storage_uri: String, required_modalities: u8, min_workers: u8, fee_amount: u64, deadline: i64, nonce: u64)]
pub struct CreateRequest<'info> {
    #[account(mut)]
    pub requester: Signer<'info>,

    #[account(
        init,
        payer = requester,
        space = AnalysisRequest::LEN,
        seeds = [seeds::REQUEST, &content_hash, &nonce.to_le_bytes()],
        bump
    )]
    pub request_account: Account<'info, AnalysisRequest>,

    /// Fee source (requester's token account)
    #[account(
        mut,
        constraint = fee_source.owner == requester.key()
    )]
    pub fee_source: Account<'info, TokenAccount>,

    /// Fee escrow for this request
    #[account(
        init,
        payer = requester,
        token::mint = dfpn_mint,
        token::authority = request_account,
        seeds = [seeds::FEE_VAULT, seeds::REQUEST, &content_hash, &nonce.to_le_bytes()],
        bump
    )]
    pub fee_escrow: Account<'info, TokenAccount>,

    /// DFPN mint
    pub dfpn_mint: Account<'info, anchor_spl::token::Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct CommitResultCtx<'info> {
    #[account(mut)]
    pub operator: Signer<'info>,

    #[account(mut)]
    pub request_account: Account<'info, AnalysisRequest>,

    /// Worker account (from worker-registry)
    #[account(
        constraint = worker_account.operator == operator.key()
    )]
    pub worker_account: Account<'info, WorkerAccount>,

    #[account(
        init,
        payer = operator,
        space = ResultCommit::LEN,
        seeds = [seeds::COMMIT, request_account.key().as_ref(), operator.key().as_ref()],
        bump
    )]
    pub commit_account: Account<'info, ResultCommit>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RevealResultCtx<'info> {
    #[account(mut)]
    pub operator: Signer<'info>,

    #[account(mut)]
    pub request_account: Account<'info, AnalysisRequest>,

    #[account(
        mut,
        seeds = [seeds::COMMIT, request_account.key().as_ref(), operator.key().as_ref()],
        bump = commit_account.bump,
        constraint = commit_account.worker == operator.key()
    )]
    pub commit_account: Account<'info, ResultCommit>,

    #[account(
        init,
        payer = operator,
        space = ResultReveal::LEN,
        seeds = [seeds::REVEAL, request_account.key().as_ref(), operator.key().as_ref()],
        bump
    )]
    pub reveal_account: Account<'info, ResultReveal>,

    /// Model used for this analysis
    pub model_account: Account<'info, ModelAccount>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct FinalizeRequest<'info> {
    /// Anyone can finalize
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub request_account: Account<'info, AnalysisRequest>,

    /// Fee escrow
    #[account(
        mut,
        seeds = [seeds::FEE_VAULT, seeds::REQUEST, request_account.content_hash.as_ref(), &request_account.nonce.to_le_bytes()],
        bump
    )]
    pub fee_escrow: Account<'info, TokenAccount>,

    /// Main fee vault (rewards treasury fee vault)
    #[account(mut)]
    pub fee_vault: Account<'info, TokenAccount>,

    /// Rewards treasury account
    /// CHECK: Validated by rewards program
    #[account(mut)]
    pub rewards_treasury: UncheckedAccount<'info>,

    /// Rewards epoch config account
    /// CHECK: Validated by rewards program
    #[account(mut)]
    pub rewards_epoch_config: UncheckedAccount<'info>,

    /// Rewards program
    /// CHECK: Program ID validated in handler
    pub rewards_program: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct CancelRequest<'info> {
    pub requester: Signer<'info>,

    #[account(
        mut,
        constraint = request_account.requester == requester.key()
    )]
    pub request_account: Account<'info, AnalysisRequest>,

    /// Fee escrow
    #[account(
        mut,
        seeds = [seeds::FEE_VAULT, seeds::REQUEST, request_account.content_hash.as_ref(), &request_account.nonce.to_le_bytes()],
        bump
    )]
    pub fee_escrow: Account<'info, TokenAccount>,

    /// Destination for refund
    #[account(
        mut,
        constraint = fee_destination.owner == requester.key()
    )]
    pub fee_destination: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct ExpireRequest<'info> {
    /// Anyone can trigger expiration
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub request_account: Account<'info, AnalysisRequest>,

    /// Fee escrow
    #[account(
        mut,
        seeds = [seeds::FEE_VAULT, seeds::REQUEST, request_account.content_hash.as_ref(), &request_account.nonce.to_le_bytes()],
        bump
    )]
    pub fee_escrow: Account<'info, TokenAccount>,

    /// Destination for refund (requester's account)
    #[account(
        mut,
        constraint = fee_destination.owner == request_account.requester
    )]
    pub fee_destination: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct OpenDispute<'info> {
    #[account(mut)]
    pub challenger: Signer<'info>,

    #[account(mut)]
    pub request_account: Account<'info, AnalysisRequest>,

    /// Reveal being disputed
    pub reveal_account: Account<'info, ResultReveal>,

    #[account(
        init,
        payer = challenger,
        space = Dispute::LEN,
        seeds = [seeds::DISPUTE, request_account.key().as_ref(), reveal_account.key().as_ref()],
        bump
    )]
    pub dispute_account: Account<'info, Dispute>,

    /// Challenger's token account (for stake)
    #[account(
        mut,
        constraint = challenger_token_account.owner == challenger.key()
    )]
    pub challenger_token_account: Account<'info, TokenAccount>,

    /// Dispute escrow
    #[account(
        init,
        payer = challenger,
        token::mint = dfpn_mint,
        token::authority = dispute_account,
        seeds = [seeds::FEE_VAULT, seeds::DISPUTE, request_account.key().as_ref(), reveal_account.key().as_ref()],
        bump
    )]
    pub dispute_escrow: Account<'info, TokenAccount>,

    pub dfpn_mint: Account<'info, anchor_spl::token::Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct ResolveDispute<'info> {
    /// Authority (governance or designated resolver)
    pub resolver: Signer<'info>,

    #[account(mut)]
    pub request_account: Account<'info, AnalysisRequest>,

    #[account(
        mut,
        constraint = dispute_account.request == request_account.key()
    )]
    pub dispute_account: Account<'info, Dispute>,

    /// Dispute escrow
    #[account(
        mut,
        seeds = [seeds::FEE_VAULT, seeds::DISPUTE, dispute_account.request.as_ref(), dispute_account.reveal.as_ref()],
        bump
    )]
    pub dispute_escrow: Account<'info, TokenAccount>,

    /// Challenger's token account (for returning stake)
    #[account(
        mut,
        constraint = challenger_token_account.owner == dispute_account.challenger
    )]
    pub challenger_token_account: Account<'info, TokenAccount>,

    /// Treasury vault (for forfeited stakes)
    #[account(mut)]
    pub treasury_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct CloseDispute<'info> {
    /// Anyone can close after timeout
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub request_account: Account<'info, AnalysisRequest>,

    #[account(
        mut,
        constraint = dispute_account.request == request_account.key()
    )]
    pub dispute_account: Account<'info, Dispute>,

    /// Dispute escrow
    #[account(
        mut,
        seeds = [seeds::FEE_VAULT, seeds::DISPUTE, dispute_account.request.as_ref(), dispute_account.reveal.as_ref()],
        bump
    )]
    pub dispute_escrow: Account<'info, TokenAccount>,

    /// Challenger's token account (for returning stake)
    #[account(
        mut,
        constraint = challenger_token_account.owner == dispute_account.challenger
    )]
    pub challenger_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

// ============================================================================
// Events
// ============================================================================

#[event]
pub struct RequestCreated {
    pub request_id: Pubkey,
    pub requester: Pubkey,
    pub content_hash: [u8; 32],
    pub storage_uri: String,
    pub required_modalities: u8,
    pub min_workers: u8,
    pub fee_amount: u64,
    pub deadline: i64,
    pub commit_deadline: i64,
}

#[event]
pub struct ResultCommitted {
    pub request_id: Pubkey,
    pub worker: Pubkey,
    pub commitment: [u8; 32],
    pub commit_slot: u64,
}

#[event]
pub struct ResultRevealed {
    pub request_id: Pubkey,
    pub worker: Pubkey,
    pub model: Pubkey,
    pub verdict: Verdict,
    pub confidence: u8,
    pub reveal_slot: u64,
}

#[event]
pub struct RequestFinalized {
    pub request_id: Pubkey,
    pub status: RequestStatus,
    pub reveal_count: u8,
    pub fee_amount: u64,
}

#[event]
pub struct RequestCancelled {
    pub request_id: Pubkey,
    pub fee_refunded: u64,
}

#[event]
pub struct RequestExpired {
    pub request_id: Pubkey,
    pub fee_refunded: u64,
    pub reveals_received: u8,
}

#[event]
pub struct DisputeOpened {
    pub dispute_id: Pubkey,
    pub request_id: Pubkey,
    pub reveal_id: Pubkey,
    pub challenger: Pubkey,
    pub challenged_worker: Pubkey,
    pub reason: DisputeReason,
    pub evidence_hash: [u8; 32],
    pub stake_amount: u64,
}

#[event]
pub struct DisputeResolved {
    pub dispute_id: Pubkey,
    pub in_favor_of_challenger: bool,
    pub resolver: Pubkey,
    pub slash_amount: u64,
    pub challenger_reward: u64,
}

#[event]
pub struct DisputeClosed {
    pub dispute_id: Pubkey,
}
