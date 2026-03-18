use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

use dfpn_shared::{constants, seeds, utils, DfpnError};

declare_id!("4uQeVj5tqViQh7yWWGStvkEG1Zmhx6uasJtWCJziofM");

#[program]
pub mod rewards {
    use super::*;

    /// Initialize the treasury (one-time setup)
    pub fn initialize_treasury(
        ctx: Context<InitializeTreasury>,
        worker_share_bps: u16,
        model_share_bps: u16,
        treasury_share_bps: u16,
        insurance_share_bps: u16,
    ) -> Result<()> {
        // Validate shares sum to 100%
        let total = worker_share_bps as u32
            + model_share_bps as u32
            + treasury_share_bps as u32
            + insurance_share_bps as u32;
        require!(
            total == constants::BPS_DENOMINATOR as u32,
            DfpnError::InvalidShareConfig
        );

        let treasury = &mut ctx.accounts.treasury;
        treasury.authority = ctx.accounts.authority.key();
        treasury.dfpn_mint = ctx.accounts.dfpn_mint.key();
        treasury.total_fees_collected = 0;
        treasury.total_rewards_distributed = 0;
        treasury.worker_share_bps = worker_share_bps;
        treasury.model_share_bps = model_share_bps;
        treasury.treasury_share_bps = treasury_share_bps;
        treasury.insurance_share_bps = insurance_share_bps;
        treasury.paused = false;
        treasury.bump = ctx.bumps.treasury;

        emit!(TreasuryInitialized {
            authority: treasury.authority,
            dfpn_mint: treasury.dfpn_mint,
            worker_share_bps,
            model_share_bps,
            treasury_share_bps,
            insurance_share_bps,
        });

        Ok(())
    }

    /// Deposit fees from a request (called via CPI from marketplace)
    pub fn deposit_fees(ctx: Context<DepositFees>, amount: u64) -> Result<()> {
        require!(!ctx.accounts.treasury.paused, DfpnError::RequestNotOpen);

        // Transfer tokens to fee vault
        let cpi_accounts = Transfer {
            from: ctx.accounts.fee_source.to_account_info(),
            to: ctx.accounts.fee_vault.to_account_info(),
            authority: ctx.accounts.depositor.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount)?;

        // Update treasury stats
        let treasury = &mut ctx.accounts.treasury;
        treasury.total_fees_collected = treasury
            .total_fees_collected
            .checked_add(amount)
            .ok_or(DfpnError::ArithmeticOverflow)?;

        emit!(FeesDeposited {
            amount,
            total_collected: treasury.total_fees_collected,
        });

        Ok(())
    }

    /// Distribute rewards for a finalized request
    pub fn distribute_rewards(
        ctx: Context<DistributeRewards>,
        fee_amount: u64,
        worker_rewards: Vec<WorkerReward>,
        _model_reward: Option<ModelReward>,
    ) -> Result<()> {
        let treasury = &ctx.accounts.treasury;
        require!(!treasury.paused, DfpnError::RequestNotOpen);

        // Calculate shares
        let worker_pool = utils::calculate_share(fee_amount, treasury.worker_share_bps)
            .ok_or(DfpnError::ArithmeticOverflow)?;
        let model_pool = utils::calculate_share(fee_amount, treasury.model_share_bps)
            .ok_or(DfpnError::ArithmeticOverflow)?;
        let treasury_share = utils::calculate_share(fee_amount, treasury.treasury_share_bps)
            .ok_or(DfpnError::ArithmeticOverflow)?;
        let insurance_share = utils::calculate_share(fee_amount, treasury.insurance_share_bps)
            .ok_or(DfpnError::ArithmeticOverflow)?;

        // Distribute to worker reward accounts
        let mut total_worker_distributed: u64 = 0;
        for worker_reward in &worker_rewards {
            // Find or create reward account for this worker
            // This is simplified - in practice would use remaining accounts
            total_worker_distributed = total_worker_distributed
                .checked_add(worker_reward.amount)
                .ok_or(DfpnError::ArithmeticOverflow)?;
        }

        // Emit distribution event
        emit!(RewardsDistributed {
            fee_amount,
            worker_pool,
            model_pool,
            treasury_share,
            insurance_share,
            worker_count: worker_rewards.len() as u8,
        });

        Ok(())
    }

    /// Allocate rewards to a specific claimant (worker or model developer)
    pub fn allocate_reward(ctx: Context<AllocateReward>, amount: u64) -> Result<()> {
        let reward = &mut ctx.accounts.reward_account;

        reward.pending_amount = reward
            .pending_amount
            .checked_add(amount)
            .ok_or(DfpnError::ArithmeticOverflow)?;

        emit!(RewardAllocated {
            claimant: reward.claimant,
            amount,
            total_pending: reward.pending_amount,
        });

        Ok(())
    }

    /// Claim accumulated rewards
    pub fn claim_rewards(ctx: Context<ClaimRewards>) -> Result<()> {
        let reward = &mut ctx.accounts.reward_account;
        let amount = reward.pending_amount;

        require!(amount > 0, DfpnError::InsufficientStake);

        // Transfer from fee vault to claimant
        let seeds = &[seeds::FEE_VAULT, &[ctx.bumps.fee_vault]];
        let signer_seeds = &[&seeds[..]];

        let cpi_accounts = Transfer {
            from: ctx.accounts.fee_vault.to_account_info(),
            to: ctx.accounts.destination.to_account_info(),
            authority: ctx.accounts.fee_vault.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        token::transfer(cpi_ctx, amount)?;

        // Update reward account
        reward.pending_amount = 0;
        reward.total_claimed = reward
            .total_claimed
            .checked_add(amount)
            .ok_or(DfpnError::ArithmeticOverflow)?;
        reward.last_claim_at = Clock::get()?.unix_timestamp;

        // Update treasury stats
        let treasury = &mut ctx.accounts.treasury;
        treasury.total_rewards_distributed = treasury
            .total_rewards_distributed
            .checked_add(amount)
            .ok_or(DfpnError::ArithmeticOverflow)?;

        emit!(RewardsClaimed {
            claimant: reward.claimant,
            amount,
            total_claimed: reward.total_claimed,
        });

        Ok(())
    }

    /// Update fee share configuration (governance)
    pub fn update_shares(
        ctx: Context<UpdateShares>,
        worker_share_bps: u16,
        model_share_bps: u16,
        treasury_share_bps: u16,
        insurance_share_bps: u16,
    ) -> Result<()> {
        // Validate shares sum to 100%
        let total = worker_share_bps as u32
            + model_share_bps as u32
            + treasury_share_bps as u32
            + insurance_share_bps as u32;
        require!(
            total == constants::BPS_DENOMINATOR as u32,
            DfpnError::InvalidShareConfig
        );

        let treasury = &mut ctx.accounts.treasury;
        let old_shares = (
            treasury.worker_share_bps,
            treasury.model_share_bps,
            treasury.treasury_share_bps,
            treasury.insurance_share_bps,
        );

        treasury.worker_share_bps = worker_share_bps;
        treasury.model_share_bps = model_share_bps;
        treasury.treasury_share_bps = treasury_share_bps;
        treasury.insurance_share_bps = insurance_share_bps;

        emit!(SharesUpdated {
            old_worker: old_shares.0,
            old_model: old_shares.1,
            old_treasury: old_shares.2,
            old_insurance: old_shares.3,
            new_worker: worker_share_bps,
            new_model: model_share_bps,
            new_treasury: treasury_share_bps,
            new_insurance: insurance_share_bps,
        });

        Ok(())
    }

    /// Pause/unpause the treasury (emergency)
    pub fn set_paused(ctx: Context<SetPaused>, paused: bool) -> Result<()> {
        let treasury = &mut ctx.accounts.treasury;
        treasury.paused = paused;

        emit!(TreasuryPauseToggled { paused });

        Ok(())
    }

    /// Transfer treasury authority
    pub fn transfer_authority(ctx: Context<TransferAuthority>) -> Result<()> {
        let treasury = &mut ctx.accounts.treasury;
        let old_authority = treasury.authority;
        treasury.authority = ctx.accounts.new_authority.key();

        emit!(AuthorityTransferred {
            old_authority,
            new_authority: treasury.authority,
        });

        Ok(())
    }

    /// Initialize a reward account for a claimant
    pub fn initialize_reward_account(ctx: Context<InitializeRewardAccount>) -> Result<()> {
        let reward = &mut ctx.accounts.reward_account;
        reward.claimant = ctx.accounts.claimant.key();
        reward.pending_amount = 0;
        reward.total_claimed = 0;
        reward.last_claim_at = 0;
        reward.bump = ctx.bumps.reward_account;

        emit!(RewardAccountInitialized {
            claimant: reward.claimant,
        });

        Ok(())
    }

    /// Initialize epoch tracking
    pub fn initialize_epoch(
        ctx: Context<InitializeEpoch>,
        epoch_duration_slots: u64,
    ) -> Result<()> {
        let clock = Clock::get()?;

        let epoch = &mut ctx.accounts.epoch_config;
        epoch.current_epoch = 0;
        epoch.epoch_start_slot = clock.slot;
        epoch.epoch_duration_slots = epoch_duration_slots;
        epoch.epoch_fees = 0;
        epoch.epoch_distributed = 0;
        epoch.epoch_requests = 0;
        epoch.treasury = ctx.accounts.treasury.key();
        epoch.bump = ctx.bumps.epoch_config;

        emit!(EpochInitialized {
            epoch: 0,
            start_slot: clock.slot,
            duration_slots: epoch_duration_slots,
        });

        Ok(())
    }

    /// Advance to next epoch (can be called by anyone after epoch duration)
    pub fn advance_epoch(ctx: Context<AdvanceEpoch>) -> Result<()> {
        let clock = Clock::get()?;
        let epoch = &ctx.accounts.epoch_config;

        // Check if epoch duration has passed
        let epoch_end = epoch
            .epoch_start_slot
            .checked_add(epoch.epoch_duration_slots)
            .ok_or(DfpnError::ArithmeticOverflow)?;

        require!(
            clock.slot >= epoch_end,
            DfpnError::RequestNotOpen // Epoch not yet complete
        );

        // Capture stats before advancing
        let old_epoch = epoch.current_epoch;
        let old_fees = epoch.epoch_fees;
        let old_distributed = epoch.epoch_distributed;
        let old_requests = epoch.epoch_requests;

        // Advance epoch
        let epoch = &mut ctx.accounts.epoch_config;
        epoch.current_epoch = epoch.current_epoch.saturating_add(1);
        epoch.epoch_start_slot = clock.slot;
        epoch.epoch_fees = 0;
        epoch.epoch_distributed = 0;
        epoch.epoch_requests = 0;

        emit!(EpochAdvanced {
            old_epoch,
            new_epoch: epoch.current_epoch,
            old_fees,
            old_distributed,
            old_requests,
            new_start_slot: clock.slot,
        });

        Ok(())
    }

    /// Process rewards for a finalized request
    /// Allocates rewards to workers and model developers based on their contributions
    pub fn process_request_rewards(
        ctx: Context<ProcessRequestRewards>,
        fee_amount: u64,
        workers: Vec<WorkerContribution>,
        model_developer: Option<Pubkey>,
    ) -> Result<()> {
        let treasury = &ctx.accounts.treasury;
        require!(!treasury.paused, DfpnError::RequestNotOpen);

        // Calculate pool amounts
        let worker_pool = utils::calculate_share(fee_amount, treasury.worker_share_bps)
            .ok_or(DfpnError::ArithmeticOverflow)?;
        let model_pool = utils::calculate_share(fee_amount, treasury.model_share_bps)
            .ok_or(DfpnError::ArithmeticOverflow)?;
        let treasury_share = utils::calculate_share(fee_amount, treasury.treasury_share_bps)
            .ok_or(DfpnError::ArithmeticOverflow)?;
        let insurance_share = utils::calculate_share(fee_amount, treasury.insurance_share_bps)
            .ok_or(DfpnError::ArithmeticOverflow)?;

        // Calculate total reputation for weighted distribution
        let total_reputation: u64 = workers
            .iter()
            .map(|w| w.reputation as u64)
            .sum();

        // Allocate worker rewards based on reputation weight
        let mut worker_allocations: Vec<(Pubkey, u64)> = Vec::new();

        if total_reputation > 0 {
            for worker in &workers {
                let reputation_weight = (worker.reputation as u64)
                    .checked_mul(constants::BPS_DENOMINATOR as u64)
                    .and_then(|v| v.checked_div(total_reputation))
                    .unwrap_or(0) as u16;

                let worker_reward = utils::calculate_share(worker_pool, reputation_weight)
                    .unwrap_or(0);

                if worker_reward > 0 {
                    worker_allocations.push((worker.worker, worker_reward));
                }
            }
        } else if !workers.is_empty() {
            // Equal distribution if no reputation data
            let per_worker = worker_pool / workers.len() as u64;
            for worker in &workers {
                worker_allocations.push((worker.worker, per_worker));
            }
        }

        // Update epoch stats
        let epoch = &mut ctx.accounts.epoch_config;
        epoch.epoch_fees = epoch
            .epoch_fees
            .checked_add(fee_amount)
            .ok_or(DfpnError::ArithmeticOverflow)?;
        epoch.epoch_requests = epoch.epoch_requests.saturating_add(1);

        let total_allocated: u64 = worker_allocations.iter().map(|(_, a)| a).sum();
        epoch.epoch_distributed = epoch
            .epoch_distributed
            .checked_add(total_allocated)
            .ok_or(DfpnError::ArithmeticOverflow)?;

        emit!(RequestRewardsProcessed {
            fee_amount,
            worker_pool,
            model_pool,
            treasury_share,
            insurance_share,
            worker_count: workers.len() as u8,
            model_developer,
            total_allocated,
        });

        Ok(())
    }
}

// ============================================================================
// Account Structures
// ============================================================================

#[account]
pub struct Treasury {
    /// Authority that can update treasury settings
    pub authority: Pubkey,
    /// DFPN token mint
    pub dfpn_mint: Pubkey,
    /// Total fees collected (lifetime)
    pub total_fees_collected: u64,
    /// Total rewards distributed (lifetime)
    pub total_rewards_distributed: u64,
    /// Worker share in basis points
    pub worker_share_bps: u16,
    /// Model developer share in basis points
    pub model_share_bps: u16,
    /// Treasury share in basis points
    pub treasury_share_bps: u16,
    /// Insurance pool share in basis points
    pub insurance_share_bps: u16,
    /// Whether treasury operations are paused
    pub paused: bool,
    /// PDA bump
    pub bump: u8,
}

impl Treasury {
    pub const LEN: usize = 8  // discriminator
        + 32  // authority
        + 32  // dfpn_mint
        + 8   // total_fees_collected
        + 8   // total_rewards_distributed
        + 2   // worker_share_bps
        + 2   // model_share_bps
        + 2   // treasury_share_bps
        + 2   // insurance_share_bps
        + 1   // paused
        + 1   // bump
        + 32; // padding
}

#[account]
pub struct RewardAccount {
    /// Claimant (worker or model developer)
    pub claimant: Pubkey,
    /// Pending amount to claim
    pub pending_amount: u64,
    /// Total claimed (lifetime)
    pub total_claimed: u64,
    /// Last claim timestamp
    pub last_claim_at: i64,
    /// PDA bump
    pub bump: u8,
}

impl RewardAccount {
    pub const LEN: usize = 8  // discriminator
        + 32  // claimant
        + 8   // pending_amount
        + 8   // total_claimed
        + 8   // last_claim_at
        + 1   // bump
        + 16; // padding
}

#[account]
pub struct EpochConfig {
    /// Current epoch number
    pub current_epoch: u64,
    /// Start slot of current epoch
    pub epoch_start_slot: u64,
    /// Duration of each epoch in slots (~24 hours at 400ms/slot = 216000 slots)
    pub epoch_duration_slots: u64,
    /// Total fees collected this epoch
    pub epoch_fees: u64,
    /// Total rewards distributed this epoch
    pub epoch_distributed: u64,
    /// Number of requests finalized this epoch
    pub epoch_requests: u32,
    /// Treasury reference
    pub treasury: Pubkey,
    /// PDA bump
    pub bump: u8,
}

impl EpochConfig {
    pub const LEN: usize = 8    // discriminator
        + 8    // current_epoch
        + 8    // epoch_start_slot
        + 8    // epoch_duration_slots
        + 8    // epoch_fees
        + 8    // epoch_distributed
        + 4    // epoch_requests
        + 32   // treasury
        + 1    // bump
        + 16;  // padding
}

// ============================================================================
// Instruction Contexts
// ============================================================================

#[derive(Accounts)]
pub struct InitializeTreasury<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = Treasury::LEN,
        seeds = [seeds::TREASURY],
        bump
    )]
    pub treasury: Account<'info, Treasury>,

    /// DFPN token mint
    pub dfpn_mint: Account<'info, Mint>,

    /// Fee vault token account
    #[account(
        init,
        payer = authority,
        token::mint = dfpn_mint,
        token::authority = fee_vault,
        seeds = [seeds::FEE_VAULT],
        bump
    )]
    pub fee_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct DepositFees<'info> {
    pub depositor: Signer<'info>,

    #[account(mut)]
    pub treasury: Account<'info, Treasury>,

    /// Source of fee tokens
    #[account(mut)]
    pub fee_source: Account<'info, TokenAccount>,

    /// Fee vault
    #[account(
        mut,
        seeds = [seeds::FEE_VAULT],
        bump
    )]
    pub fee_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct DistributeRewards<'info> {
    /// Authority (marketplace program)
    pub authority: Signer<'info>,

    #[account(mut)]
    pub treasury: Account<'info, Treasury>,

    /// Fee vault
    #[account(
        mut,
        seeds = [seeds::FEE_VAULT],
        bump
    )]
    pub fee_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct AllocateReward<'info> {
    /// Authority (marketplace program)
    pub authority: Signer<'info>,

    #[account(mut)]
    pub reward_account: Account<'info, RewardAccount>,
}

#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    pub claimant: Signer<'info>,

    #[account(mut)]
    pub treasury: Account<'info, Treasury>,

    #[account(
        mut,
        seeds = [seeds::REWARD, claimant.key().as_ref()],
        bump = reward_account.bump,
        constraint = reward_account.claimant == claimant.key()
    )]
    pub reward_account: Account<'info, RewardAccount>,

    /// Destination for claimed tokens
    #[account(
        mut,
        constraint = destination.owner == claimant.key()
    )]
    pub destination: Account<'info, TokenAccount>,

    /// Fee vault
    #[account(
        mut,
        seeds = [seeds::FEE_VAULT],
        bump
    )]
    pub fee_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct UpdateShares<'info> {
    pub authority: Signer<'info>,

    #[account(
        mut,
        constraint = treasury.authority == authority.key() @ DfpnError::InvalidAuthority
    )]
    pub treasury: Account<'info, Treasury>,
}

#[derive(Accounts)]
pub struct SetPaused<'info> {
    pub authority: Signer<'info>,

    #[account(
        mut,
        constraint = treasury.authority == authority.key() @ DfpnError::InvalidAuthority
    )]
    pub treasury: Account<'info, Treasury>,
}

#[derive(Accounts)]
pub struct TransferAuthority<'info> {
    pub authority: Signer<'info>,

    /// CHECK: New authority
    pub new_authority: AccountInfo<'info>,

    #[account(
        mut,
        constraint = treasury.authority == authority.key() @ DfpnError::InvalidAuthority
    )]
    pub treasury: Account<'info, Treasury>,
}

#[derive(Accounts)]
pub struct InitializeRewardAccount<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: Claimant for the reward account
    pub claimant: AccountInfo<'info>,

    #[account(
        init,
        payer = payer,
        space = RewardAccount::LEN,
        seeds = [seeds::REWARD, claimant.key().as_ref()],
        bump
    )]
    pub reward_account: Account<'info, RewardAccount>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeEpoch<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        constraint = treasury.authority == authority.key() @ DfpnError::InvalidAuthority
    )]
    pub treasury: Account<'info, Treasury>,

    #[account(
        init,
        payer = authority,
        space = EpochConfig::LEN,
        seeds = [seeds::EPOCH, treasury.key().as_ref()],
        bump
    )]
    pub epoch_config: Account<'info, EpochConfig>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AdvanceEpoch<'info> {
    /// Anyone can advance the epoch after duration passes
    pub payer: Signer<'info>,

    pub treasury: Account<'info, Treasury>,

    #[account(
        mut,
        seeds = [seeds::EPOCH, treasury.key().as_ref()],
        bump = epoch_config.bump,
        constraint = epoch_config.treasury == treasury.key()
    )]
    pub epoch_config: Account<'info, EpochConfig>,
}

#[derive(Accounts)]
pub struct ProcessRequestRewards<'info> {
    /// Authority (marketplace program or designated caller)
    pub authority: Signer<'info>,

    #[account(mut)]
    pub treasury: Account<'info, Treasury>,

    #[account(
        mut,
        seeds = [seeds::EPOCH, treasury.key().as_ref()],
        bump = epoch_config.bump,
        constraint = epoch_config.treasury == treasury.key()
    )]
    pub epoch_config: Account<'info, EpochConfig>,

    /// Fee vault
    #[account(
        seeds = [seeds::FEE_VAULT],
        bump
    )]
    pub fee_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

// ============================================================================
// Events
// ============================================================================

#[event]
pub struct TreasuryInitialized {
    pub authority: Pubkey,
    pub dfpn_mint: Pubkey,
    pub worker_share_bps: u16,
    pub model_share_bps: u16,
    pub treasury_share_bps: u16,
    pub insurance_share_bps: u16,
}

#[event]
pub struct FeesDeposited {
    pub amount: u64,
    pub total_collected: u64,
}

#[event]
pub struct RewardsDistributed {
    pub fee_amount: u64,
    pub worker_pool: u64,
    pub model_pool: u64,
    pub treasury_share: u64,
    pub insurance_share: u64,
    pub worker_count: u8,
}

#[event]
pub struct RewardAllocated {
    pub claimant: Pubkey,
    pub amount: u64,
    pub total_pending: u64,
}

#[event]
pub struct RewardsClaimed {
    pub claimant: Pubkey,
    pub amount: u64,
    pub total_claimed: u64,
}

#[event]
pub struct SharesUpdated {
    pub old_worker: u16,
    pub old_model: u16,
    pub old_treasury: u16,
    pub old_insurance: u16,
    pub new_worker: u16,
    pub new_model: u16,
    pub new_treasury: u16,
    pub new_insurance: u16,
}

#[event]
pub struct TreasuryPauseToggled {
    pub paused: bool,
}

#[event]
pub struct AuthorityTransferred {
    pub old_authority: Pubkey,
    pub new_authority: Pubkey,
}

#[event]
pub struct RewardAccountInitialized {
    pub claimant: Pubkey,
}

#[event]
pub struct EpochInitialized {
    pub epoch: u64,
    pub start_slot: u64,
    pub duration_slots: u64,
}

#[event]
pub struct EpochAdvanced {
    pub old_epoch: u64,
    pub new_epoch: u64,
    pub old_fees: u64,
    pub old_distributed: u64,
    pub old_requests: u32,
    pub new_start_slot: u64,
}

#[event]
pub struct RequestRewardsProcessed {
    pub fee_amount: u64,
    pub worker_pool: u64,
    pub model_pool: u64,
    pub treasury_share: u64,
    pub insurance_share: u64,
    pub worker_count: u8,
    pub model_developer: Option<Pubkey>,
    pub total_allocated: u64,
}

// ============================================================================
// Types
// ============================================================================

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct WorkerReward {
    pub worker: Pubkey,
    pub amount: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ModelReward {
    pub model: Pubkey,
    pub developer: Pubkey,
    pub amount: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct WorkerContribution {
    pub worker: Pubkey,
    pub reputation: u32,
    pub participated: bool,
}
