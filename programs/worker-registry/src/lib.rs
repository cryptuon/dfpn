use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use dfpn_shared::{
    constants, seeds, DfpnError, Modalities, WorkerStatus,
};

declare_id!("HmbTLCmaGvZhKnn1Zfa1JVnp7vkMV4DYVxPLWBVoN65L");

#[program]
pub mod worker_registry {
    use super::*;

    /// Register a new worker with initial stake
    pub fn register_worker(
        ctx: Context<RegisterWorker>,
        supported_modalities: u8,
        stake_amount: u64,
    ) -> Result<()> {
        require!(
            stake_amount >= constants::MIN_WORKER_STAKE,
            DfpnError::InsufficientStake
        );

        let modalities = Modalities::from_bits(supported_modalities);
        require!(!modalities.is_empty(), DfpnError::InvalidModality);

        // Transfer stake to vault
        let cpi_accounts = Transfer {
            from: ctx.accounts.stake_source.to_account_info(),
            to: ctx.accounts.stake_vault.to_account_info(),
            authority: ctx.accounts.operator.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, stake_amount)?;

        // Initialize worker account
        let worker = &mut ctx.accounts.worker_account;
        worker.operator = ctx.accounts.operator.key();
        worker.stake = stake_amount;
        worker.reputation_score = constants::INITIAL_REPUTATION;
        worker.supported_modalities = modalities;
        worker.tasks_completed = 0;
        worker.tasks_failed = 0;
        worker.last_active_slot = Clock::get()?.slot;
        worker.status = WorkerStatus::Active;
        worker.bump = ctx.bumps.worker_account;

        emit!(WorkerRegistered {
            operator: worker.operator,
            stake: stake_amount,
            modalities: supported_modalities,
        });

        Ok(())
    }

    /// Update worker configuration (modalities, additional stake)
    pub fn update_worker(
        ctx: Context<UpdateWorker>,
        new_modalities: Option<u8>,
        additional_stake: Option<u64>,
    ) -> Result<()> {
        let worker = &mut ctx.accounts.worker_account;

        // Update modalities if provided
        if let Some(modalities_bits) = new_modalities {
            let modalities = Modalities::from_bits(modalities_bits);
            require!(!modalities.is_empty(), DfpnError::InvalidModality);
            worker.supported_modalities = modalities;
        }

        // Add stake if provided
        if let Some(amount) = additional_stake {
            if amount > 0 {
                let cpi_accounts = Transfer {
                    from: ctx.accounts.stake_source.to_account_info(),
                    to: ctx.accounts.stake_vault.to_account_info(),
                    authority: ctx.accounts.operator.to_account_info(),
                };
                let cpi_program = ctx.accounts.token_program.to_account_info();
                let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
                token::transfer(cpi_ctx, amount)?;

                worker.stake = worker
                    .stake
                    .checked_add(amount)
                    .ok_or(DfpnError::ArithmeticOverflow)?;
            }
        }

        emit!(WorkerUpdated {
            operator: worker.operator,
            stake: worker.stake,
            modalities: worker.supported_modalities.0,
        });

        Ok(())
    }

    /// Set worker status (active/inactive)
    pub fn set_worker_status(ctx: Context<SetWorkerStatus>, active: bool) -> Result<()> {
        let worker = &mut ctx.accounts.worker_account;

        match worker.status {
            WorkerStatus::Active | WorkerStatus::Inactive => {
                worker.status = if active {
                    WorkerStatus::Active
                } else {
                    WorkerStatus::Inactive
                };
            }
            WorkerStatus::Slashed | WorkerStatus::Unbonding { .. } => {
                return Err(DfpnError::WorkerNotActive.into());
            }
        }

        emit!(WorkerStatusChanged {
            operator: worker.operator,
            active,
        });

        Ok(())
    }

    /// Request to unstake (starts unbonding period)
    pub fn request_unstake(ctx: Context<RequestUnstake>, amount: u64) -> Result<()> {
        let worker = &mut ctx.accounts.worker_account;

        // Check worker is not already unbonding
        require!(
            !matches!(worker.status, WorkerStatus::Unbonding { .. }),
            DfpnError::WorkerNotActive
        );

        // Ensure remaining stake meets minimum
        let remaining = worker
            .stake
            .checked_sub(amount)
            .ok_or(DfpnError::InsufficientStake)?;

        // If withdrawing all, set to unbonding; otherwise just reduce stake
        if remaining < constants::MIN_WORKER_STAKE {
            // Full withdrawal - enter unbonding
            let clock = Clock::get()?;
            let unlock_slot = clock
                .slot
                .checked_add(constants::UNBONDING_PERIOD_SLOTS)
                .ok_or(DfpnError::ArithmeticOverflow)?;

            worker.status = WorkerStatus::Unbonding { unlock_slot };
            worker.pending_unstake = amount;

            emit!(WorkerUnbondingStarted {
                operator: worker.operator,
                amount,
                unlock_slot,
            });
        } else {
            // Partial withdrawal - immediate for amounts above minimum
            worker.pending_unstake = amount;

            let clock = Clock::get()?;
            let unlock_slot = clock
                .slot
                .checked_add(constants::UNBONDING_PERIOD_SLOTS)
                .ok_or(DfpnError::ArithmeticOverflow)?;

            worker.unstake_unlock_slot = unlock_slot;

            emit!(WorkerUnbondingStarted {
                operator: worker.operator,
                amount,
                unlock_slot,
            });
        }

        Ok(())
    }

    /// Withdraw stake after unbonding period
    pub fn withdraw_stake(ctx: Context<WithdrawStake>) -> Result<()> {
        let worker = &mut ctx.accounts.worker_account;
        let clock = Clock::get()?;

        // Check if unbonding period is complete
        let can_withdraw = match worker.status {
            WorkerStatus::Unbonding { unlock_slot } => clock.slot >= unlock_slot,
            _ => {
                // Check partial unstake
                worker.pending_unstake > 0 && clock.slot >= worker.unstake_unlock_slot
            }
        };

        require!(can_withdraw, DfpnError::UnbondingNotComplete);

        let amount = worker.pending_unstake;
        require!(amount > 0, DfpnError::InsufficientStake);

        // Transfer from vault to operator
        let seeds = &[
            seeds::STAKE_VAULT,
            seeds::WORKER,
            &[ctx.bumps.stake_vault],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_accounts = Transfer {
            from: ctx.accounts.stake_vault.to_account_info(),
            to: ctx.accounts.stake_destination.to_account_info(),
            authority: ctx.accounts.stake_vault.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        token::transfer(cpi_ctx, amount)?;

        // Update worker state
        worker.stake = worker
            .stake
            .checked_sub(amount)
            .ok_or(DfpnError::ArithmeticOverflow)?;
        worker.pending_unstake = 0;
        worker.unstake_unlock_slot = 0;

        // If fully withdrawn, close account; otherwise set to inactive
        if worker.stake == 0 {
            // Account will be closed by anchor
            emit!(WorkerWithdrawn {
                operator: worker.operator,
                amount,
                closed: true,
            });
        } else {
            worker.status = WorkerStatus::Active;
            emit!(WorkerWithdrawn {
                operator: worker.operator,
                amount,
                closed: false,
            });
        }

        Ok(())
    }

    /// Update worker reputation (called via CPI from marketplace)
    pub fn update_reputation(
        ctx: Context<UpdateReputation>,
        reputation_delta: i32,
        tasks_completed_delta: u64,
        tasks_failed_delta: u64,
    ) -> Result<()> {
        let worker = &mut ctx.accounts.worker_account;

        // Update reputation
        let new_reputation = if reputation_delta >= 0 {
            worker
                .reputation_score
                .saturating_add(reputation_delta as u32)
                .min(constants::MAX_REPUTATION)
        } else {
            worker
                .reputation_score
                .saturating_sub((-reputation_delta) as u32)
        };
        worker.reputation_score = new_reputation;

        // Update task counts
        worker.tasks_completed = worker
            .tasks_completed
            .saturating_add(tasks_completed_delta);
        worker.tasks_failed = worker.tasks_failed.saturating_add(tasks_failed_delta);

        // Update last active
        worker.last_active_slot = Clock::get()?.slot;

        emit!(WorkerReputationUpdated {
            operator: worker.operator,
            reputation: new_reputation,
            tasks_completed: worker.tasks_completed,
            tasks_failed: worker.tasks_failed,
        });

        Ok(())
    }

    /// Slash worker stake (called via CPI from marketplace for violations)
    pub fn slash_worker(ctx: Context<SlashWorker>, amount: u64, reason: SlashReason) -> Result<()> {
        let worker = &mut ctx.accounts.worker_account;

        let slash_amount = amount.min(worker.stake);

        // Transfer slashed amount to treasury
        let seeds = &[
            seeds::STAKE_VAULT,
            seeds::WORKER,
            &[ctx.bumps.stake_vault],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_accounts = Transfer {
            from: ctx.accounts.stake_vault.to_account_info(),
            to: ctx.accounts.treasury_vault.to_account_info(),
            authority: ctx.accounts.stake_vault.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        token::transfer(cpi_ctx, slash_amount)?;

        // Update worker state
        worker.stake = worker
            .stake
            .checked_sub(slash_amount)
            .ok_or(DfpnError::ArithmeticOverflow)?;

        // If stake falls below minimum, suspend worker
        if worker.stake < constants::MIN_WORKER_STAKE {
            worker.status = WorkerStatus::Slashed;
        }

        emit!(WorkerSlashed {
            operator: worker.operator,
            amount: slash_amount,
            reason,
            remaining_stake: worker.stake,
        });

        Ok(())
    }
}

// ============================================================================
// Account Structures
// ============================================================================

#[account]
#[derive(Default)]
pub struct WorkerAccount {
    /// Operator wallet address
    pub operator: Pubkey,
    /// Current stake amount
    pub stake: u64,
    /// Reputation score (0-10000 basis points)
    pub reputation_score: u32,
    /// Supported modalities (bitfield)
    pub supported_modalities: Modalities,
    /// Total tasks completed successfully
    pub tasks_completed: u64,
    /// Total tasks failed
    pub tasks_failed: u64,
    /// Last active slot
    pub last_active_slot: u64,
    /// Current status
    pub status: WorkerStatus,
    /// Pending unstake amount
    pub pending_unstake: u64,
    /// Slot when pending unstake can be withdrawn
    pub unstake_unlock_slot: u64,
    /// PDA bump
    pub bump: u8,
}

impl WorkerAccount {
    pub const LEN: usize = 8  // discriminator
        + 32  // operator
        + 8   // stake
        + 4   // reputation_score
        + 1   // supported_modalities
        + 8   // tasks_completed
        + 8   // tasks_failed
        + 8   // last_active_slot
        + 1 + 8  // status (enum + potential u64)
        + 8   // pending_unstake
        + 8   // unstake_unlock_slot
        + 1   // bump
        + 64; // padding for future fields
}

// ============================================================================
// Instruction Contexts
// ============================================================================

#[derive(Accounts)]
pub struct RegisterWorker<'info> {
    #[account(mut)]
    pub operator: Signer<'info>,

    #[account(
        init,
        payer = operator,
        space = WorkerAccount::LEN,
        seeds = [seeds::WORKER, operator.key().as_ref()],
        bump
    )]
    pub worker_account: Account<'info, WorkerAccount>,

    /// Worker's token account to transfer stake from
    #[account(
        mut,
        constraint = stake_source.owner == operator.key()
    )]
    pub stake_source: Account<'info, TokenAccount>,

    /// Program's stake vault
    #[account(
        mut,
        seeds = [seeds::STAKE_VAULT, seeds::WORKER],
        bump
    )]
    pub stake_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateWorker<'info> {
    #[account(mut)]
    pub operator: Signer<'info>,

    #[account(
        mut,
        seeds = [seeds::WORKER, operator.key().as_ref()],
        bump = worker_account.bump,
        constraint = worker_account.operator == operator.key()
    )]
    pub worker_account: Account<'info, WorkerAccount>,

    /// Worker's token account (optional, for adding stake)
    #[account(
        mut,
        constraint = stake_source.owner == operator.key()
    )]
    pub stake_source: Account<'info, TokenAccount>,

    /// Program's stake vault
    #[account(
        mut,
        seeds = [seeds::STAKE_VAULT, seeds::WORKER],
        bump
    )]
    pub stake_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct SetWorkerStatus<'info> {
    pub operator: Signer<'info>,

    #[account(
        mut,
        seeds = [seeds::WORKER, operator.key().as_ref()],
        bump = worker_account.bump,
        constraint = worker_account.operator == operator.key()
    )]
    pub worker_account: Account<'info, WorkerAccount>,
}

#[derive(Accounts)]
pub struct RequestUnstake<'info> {
    pub operator: Signer<'info>,

    #[account(
        mut,
        seeds = [seeds::WORKER, operator.key().as_ref()],
        bump = worker_account.bump,
        constraint = worker_account.operator == operator.key()
    )]
    pub worker_account: Account<'info, WorkerAccount>,
}

#[derive(Accounts)]
pub struct WithdrawStake<'info> {
    pub operator: Signer<'info>,

    #[account(
        mut,
        seeds = [seeds::WORKER, operator.key().as_ref()],
        bump = worker_account.bump,
        constraint = worker_account.operator == operator.key()
    )]
    pub worker_account: Account<'info, WorkerAccount>,

    /// Destination for withdrawn stake
    #[account(
        mut,
        constraint = stake_destination.owner == operator.key()
    )]
    pub stake_destination: Account<'info, TokenAccount>,

    /// Program's stake vault
    #[account(
        mut,
        seeds = [seeds::STAKE_VAULT, seeds::WORKER],
        bump
    )]
    pub stake_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct UpdateReputation<'info> {
    /// Authority that can update reputation (marketplace program)
    pub authority: Signer<'info>,

    #[account(mut)]
    pub worker_account: Account<'info, WorkerAccount>,
}

#[derive(Accounts)]
pub struct SlashWorker<'info> {
    /// Authority that can slash (marketplace program)
    pub authority: Signer<'info>,

    #[account(mut)]
    pub worker_account: Account<'info, WorkerAccount>,

    /// Program's stake vault
    #[account(
        mut,
        seeds = [seeds::STAKE_VAULT, seeds::WORKER],
        bump
    )]
    pub stake_vault: Account<'info, TokenAccount>,

    /// Treasury vault to receive slashed tokens
    #[account(mut)]
    pub treasury_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

// ============================================================================
// Events
// ============================================================================

#[event]
pub struct WorkerRegistered {
    pub operator: Pubkey,
    pub stake: u64,
    pub modalities: u8,
}

#[event]
pub struct WorkerUpdated {
    pub operator: Pubkey,
    pub stake: u64,
    pub modalities: u8,
}

#[event]
pub struct WorkerStatusChanged {
    pub operator: Pubkey,
    pub active: bool,
}

#[event]
pub struct WorkerUnbondingStarted {
    pub operator: Pubkey,
    pub amount: u64,
    pub unlock_slot: u64,
}

#[event]
pub struct WorkerWithdrawn {
    pub operator: Pubkey,
    pub amount: u64,
    pub closed: bool,
}

#[event]
pub struct WorkerReputationUpdated {
    pub operator: Pubkey,
    pub reputation: u32,
    pub tasks_completed: u64,
    pub tasks_failed: u64,
}

#[event]
pub struct WorkerSlashed {
    pub operator: Pubkey,
    pub amount: u64,
    pub reason: SlashReason,
    pub remaining_stake: u64,
}

// ============================================================================
// Types
// ============================================================================

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum SlashReason {
    InvalidResult,
    MissedDeadline,
    Fraud,
    Collusion,
}
