use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use dfpn_shared::{constants, seeds, DfpnError, Modalities, ModelStatus};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod model_registry {
    use super::*;

    /// Register a new detection model
    pub fn register_model(
        ctx: Context<RegisterModel>,
        model_id: [u8; 32],
        name: String,
        version: String,
        modalities: u8,
        model_uri: String,
        checksum: [u8; 32],
        stake_amount: u64,
    ) -> Result<()> {
        // Validate inputs
        require!(
            name.len() <= constants::MAX_NAME_LENGTH,
            DfpnError::StringTooLong
        );
        require!(
            version.len() <= constants::MAX_VERSION_LENGTH,
            DfpnError::StringTooLong
        );
        require!(
            model_uri.len() <= constants::MAX_URI_LENGTH,
            DfpnError::StringTooLong
        );
        require!(
            stake_amount >= constants::MIN_MODEL_STAKE,
            DfpnError::InsufficientStake
        );

        let modalities_field = Modalities::from_bits(modalities);
        require!(!modalities_field.is_empty(), DfpnError::InvalidModality);

        // Transfer stake to vault
        let cpi_accounts = Transfer {
            from: ctx.accounts.stake_source.to_account_info(),
            to: ctx.accounts.stake_vault.to_account_info(),
            authority: ctx.accounts.developer.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, stake_amount)?;

        // Initialize model account
        let model = &mut ctx.accounts.model_account;
        model.developer = ctx.accounts.developer.key();
        model.model_id = model_id;
        model.name = name.clone();
        model.version = version.clone();
        model.modalities = modalities_field;
        model.model_uri = model_uri.clone();
        model.checksum = checksum;
        model.stake = stake_amount;
        model.score = 0; // Will be set after evaluation
        model.status = ModelStatus::Pending;
        model.created_at = Clock::get()?.unix_timestamp;
        model.updated_at = model.created_at;
        model.total_uses = 0;
        model.bump = ctx.bumps.model_account;

        emit!(ModelRegistered {
            developer: model.developer,
            model_id,
            name,
            version,
            modalities,
            model_uri,
            stake: stake_amount,
        });

        Ok(())
    }

    /// Update model to a new version
    pub fn update_model(
        ctx: Context<UpdateModel>,
        new_version: String,
        new_model_uri: String,
        new_checksum: [u8; 32],
    ) -> Result<()> {
        require!(
            new_version.len() <= constants::MAX_VERSION_LENGTH,
            DfpnError::StringTooLong
        );
        require!(
            new_model_uri.len() <= constants::MAX_URI_LENGTH,
            DfpnError::StringTooLong
        );

        let model = &mut ctx.accounts.model_account;

        // Store previous version info
        let old_version = model.version.clone();

        // Update model
        model.version = new_version.clone();
        model.model_uri = new_model_uri.clone();
        model.checksum = new_checksum;
        model.updated_at = Clock::get()?.unix_timestamp;
        // Reset to pending for re-evaluation
        model.status = ModelStatus::Pending;

        emit!(ModelUpdated {
            developer: model.developer,
            model_id: model.model_id,
            old_version,
            new_version,
            new_model_uri,
        });

        Ok(())
    }

    /// Activate a model after successful evaluation
    pub fn activate_model(ctx: Context<ActivateModel>, score: u32) -> Result<()> {
        let model = &mut ctx.accounts.model_account;

        require!(
            model.status == ModelStatus::Pending,
            DfpnError::ModelNotActive
        );
        require!(score <= constants::MAX_REPUTATION, DfpnError::InvalidModality);

        model.status = ModelStatus::Active;
        model.score = score;
        model.updated_at = Clock::get()?.unix_timestamp;

        emit!(ModelActivated {
            developer: model.developer,
            model_id: model.model_id,
            score,
        });

        Ok(())
    }

    /// Retire a model (voluntary by developer)
    pub fn retire_model(ctx: Context<RetireModel>) -> Result<()> {
        let model = &mut ctx.accounts.model_account;

        model.status = ModelStatus::Retired;
        model.updated_at = Clock::get()?.unix_timestamp;

        emit!(ModelRetired {
            developer: model.developer,
            model_id: model.model_id,
            reason: RetirementReason::Voluntary,
        });

        Ok(())
    }

    /// Force retire a model (governance action)
    pub fn force_retire_model(
        ctx: Context<ForceRetireModel>,
        reason: RetirementReason,
    ) -> Result<()> {
        let model = &mut ctx.accounts.model_account;

        model.status = ModelStatus::Retired;
        model.updated_at = Clock::get()?.unix_timestamp;

        emit!(ModelRetired {
            developer: model.developer,
            model_id: model.model_id,
            reason,
        });

        Ok(())
    }

    /// Suspend a model (temporary, for investigation)
    pub fn suspend_model(ctx: Context<SuspendModel>) -> Result<()> {
        let model = &mut ctx.accounts.model_account;

        model.status = ModelStatus::Suspended;
        model.updated_at = Clock::get()?.unix_timestamp;

        emit!(ModelSuspended {
            developer: model.developer,
            model_id: model.model_id,
        });

        Ok(())
    }

    /// Reactivate a suspended model
    pub fn reactivate_model(ctx: Context<ReactivateModel>) -> Result<()> {
        let model = &mut ctx.accounts.model_account;

        require!(
            model.status == ModelStatus::Suspended,
            DfpnError::ModelNotActive
        );

        model.status = ModelStatus::Active;
        model.updated_at = Clock::get()?.unix_timestamp;

        emit!(ModelReactivated {
            developer: model.developer,
            model_id: model.model_id,
        });

        Ok(())
    }

    /// Update model score (called via CPI from evaluation harness)
    pub fn update_score(ctx: Context<UpdateScore>, new_score: u32) -> Result<()> {
        let model = &mut ctx.accounts.model_account;

        require!(
            new_score <= constants::MAX_REPUTATION,
            DfpnError::InvalidModality
        );

        let old_score = model.score;
        model.score = new_score;
        model.updated_at = Clock::get()?.unix_timestamp;

        emit!(ModelScoreUpdated {
            developer: model.developer,
            model_id: model.model_id,
            old_score,
            new_score,
        });

        Ok(())
    }

    /// Increment model usage counter (called via CPI from marketplace)
    pub fn increment_usage(ctx: Context<IncrementUsage>) -> Result<()> {
        let model = &mut ctx.accounts.model_account;

        model.total_uses = model.total_uses.saturating_add(1);

        Ok(())
    }

    /// Withdraw stake after model retirement
    pub fn withdraw_model_stake(ctx: Context<WithdrawModelStake>) -> Result<()> {
        let model = &ctx.accounts.model_account;

        require!(
            model.status == ModelStatus::Retired,
            DfpnError::ModelNotActive
        );

        let amount = model.stake;

        // Transfer from vault to developer
        let seeds = &[seeds::STAKE_VAULT, seeds::MODEL, &[ctx.bumps.stake_vault]];
        let signer_seeds = &[&seeds[..]];

        let cpi_accounts = Transfer {
            from: ctx.accounts.stake_vault.to_account_info(),
            to: ctx.accounts.stake_destination.to_account_info(),
            authority: ctx.accounts.stake_vault.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        token::transfer(cpi_ctx, amount)?;

        emit!(ModelStakeWithdrawn {
            developer: model.developer,
            model_id: model.model_id,
            amount,
        });

        Ok(())
    }
}

// ============================================================================
// Account Structures
// ============================================================================

#[account]
pub struct ModelAccount {
    /// Model developer wallet
    pub developer: Pubkey,
    /// Unique model identifier (hash of name)
    pub model_id: [u8; 32],
    /// Human-readable name
    pub name: String,
    /// Semantic version
    pub version: String,
    /// Supported modalities
    pub modalities: Modalities,
    /// URI where model can be downloaded
    pub model_uri: String,
    /// SHA256 checksum of model file
    pub checksum: [u8; 32],
    /// Staked amount
    pub stake: u64,
    /// Evaluation score (0-10000)
    pub score: u32,
    /// Current status
    pub status: ModelStatus,
    /// Creation timestamp
    pub created_at: i64,
    /// Last update timestamp
    pub updated_at: i64,
    /// Total times this model was used
    pub total_uses: u64,
    /// PDA bump
    pub bump: u8,
}

impl ModelAccount {
    pub const LEN: usize = 8  // discriminator
        + 32  // developer
        + 32  // model_id
        + 4 + constants::MAX_NAME_LENGTH  // name (string)
        + 4 + constants::MAX_VERSION_LENGTH  // version (string)
        + 1   // modalities
        + 4 + constants::MAX_URI_LENGTH  // model_uri (string)
        + 32  // checksum
        + 8   // stake
        + 4   // score
        + 1   // status
        + 8   // created_at
        + 8   // updated_at
        + 8   // total_uses
        + 1   // bump
        + 64; // padding
}

// ============================================================================
// Instruction Contexts
// ============================================================================

#[derive(Accounts)]
#[instruction(model_id: [u8; 32])]
pub struct RegisterModel<'info> {
    #[account(mut)]
    pub developer: Signer<'info>,

    #[account(
        init,
        payer = developer,
        space = ModelAccount::LEN,
        seeds = [seeds::MODEL, developer.key().as_ref(), &model_id],
        bump
    )]
    pub model_account: Account<'info, ModelAccount>,

    /// Developer's token account to transfer stake from
    #[account(
        mut,
        constraint = stake_source.owner == developer.key()
    )]
    pub stake_source: Account<'info, TokenAccount>,

    /// Program's stake vault for models
    #[account(
        mut,
        seeds = [seeds::STAKE_VAULT, seeds::MODEL],
        bump
    )]
    pub stake_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateModel<'info> {
    pub developer: Signer<'info>,

    #[account(
        mut,
        constraint = model_account.developer == developer.key(),
        constraint = model_account.status != ModelStatus::Retired @ DfpnError::ModelNotActive
    )]
    pub model_account: Account<'info, ModelAccount>,
}

#[derive(Accounts)]
pub struct ActivateModel<'info> {
    /// Evaluation authority (governance or designated evaluator)
    pub authority: Signer<'info>,

    #[account(mut)]
    pub model_account: Account<'info, ModelAccount>,
}

#[derive(Accounts)]
pub struct RetireModel<'info> {
    pub developer: Signer<'info>,

    #[account(
        mut,
        constraint = model_account.developer == developer.key()
    )]
    pub model_account: Account<'info, ModelAccount>,
}

#[derive(Accounts)]
pub struct ForceRetireModel<'info> {
    /// Governance authority
    pub authority: Signer<'info>,

    #[account(mut)]
    pub model_account: Account<'info, ModelAccount>,
}

#[derive(Accounts)]
pub struct SuspendModel<'info> {
    /// Governance authority
    pub authority: Signer<'info>,

    #[account(mut)]
    pub model_account: Account<'info, ModelAccount>,
}

#[derive(Accounts)]
pub struct ReactivateModel<'info> {
    /// Governance authority
    pub authority: Signer<'info>,

    #[account(mut)]
    pub model_account: Account<'info, ModelAccount>,
}

#[derive(Accounts)]
pub struct UpdateScore<'info> {
    /// Evaluation authority
    pub authority: Signer<'info>,

    #[account(mut)]
    pub model_account: Account<'info, ModelAccount>,
}

#[derive(Accounts)]
pub struct IncrementUsage<'info> {
    /// Marketplace program authority
    pub authority: Signer<'info>,

    #[account(mut)]
    pub model_account: Account<'info, ModelAccount>,
}

#[derive(Accounts)]
pub struct WithdrawModelStake<'info> {
    pub developer: Signer<'info>,

    #[account(
        mut,
        constraint = model_account.developer == developer.key(),
        constraint = model_account.status == ModelStatus::Retired @ DfpnError::ModelNotActive,
        close = developer
    )]
    pub model_account: Account<'info, ModelAccount>,

    /// Destination for withdrawn stake
    #[account(
        mut,
        constraint = stake_destination.owner == developer.key()
    )]
    pub stake_destination: Account<'info, TokenAccount>,

    /// Program's stake vault
    #[account(
        mut,
        seeds = [seeds::STAKE_VAULT, seeds::MODEL],
        bump
    )]
    pub stake_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

// ============================================================================
// Events
// ============================================================================

#[event]
pub struct ModelRegistered {
    pub developer: Pubkey,
    pub model_id: [u8; 32],
    pub name: String,
    pub version: String,
    pub modalities: u8,
    pub model_uri: String,
    pub stake: u64,
}

#[event]
pub struct ModelUpdated {
    pub developer: Pubkey,
    pub model_id: [u8; 32],
    pub old_version: String,
    pub new_version: String,
    pub new_model_uri: String,
}

#[event]
pub struct ModelActivated {
    pub developer: Pubkey,
    pub model_id: [u8; 32],
    pub score: u32,
}

#[event]
pub struct ModelRetired {
    pub developer: Pubkey,
    pub model_id: [u8; 32],
    pub reason: RetirementReason,
}

#[event]
pub struct ModelSuspended {
    pub developer: Pubkey,
    pub model_id: [u8; 32],
}

#[event]
pub struct ModelReactivated {
    pub developer: Pubkey,
    pub model_id: [u8; 32],
}

#[event]
pub struct ModelScoreUpdated {
    pub developer: Pubkey,
    pub model_id: [u8; 32],
    pub old_score: u32,
    pub new_score: u32,
}

#[event]
pub struct ModelStakeWithdrawn {
    pub developer: Pubkey,
    pub model_id: [u8; 32],
    pub amount: u64,
}

// ============================================================================
// Types
// ============================================================================

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum RetirementReason {
    Voluntary,
    LowPerformance,
    SecurityIssue,
    Governance,
}
