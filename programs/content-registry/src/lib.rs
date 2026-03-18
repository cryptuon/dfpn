use anchor_lang::prelude::*;

use dfpn_shared::{constants, seeds, DfpnError, MediaType};

declare_id!("GokivDYuQXPZCWRkwMhdH2h91KpDQXBEmpgBgs55bnpH");

#[program]
pub mod content_registry {
    use super::*;

    /// Register original content for provenance tracking
    pub fn register_content(
        ctx: Context<RegisterContent>,
        content_hash: [u8; 32],
        media_type: MediaType,
        storage_uri: String,
    ) -> Result<()> {
        require!(
            storage_uri.len() <= constants::MAX_URI_LENGTH,
            DfpnError::StringTooLong
        );

        let content = &mut ctx.accounts.content_account;
        content.content_hash = content_hash;
        content.media_type = media_type;
        content.creator = ctx.accounts.creator.key();
        content.created_at = Clock::get()?.unix_timestamp;
        content.storage_uri = storage_uri.clone();
        content.claim_count = 0;
        content.analysis_count = 0;
        content.bump = ctx.bumps.content_account;

        emit!(ContentRegistered {
            content_hash,
            media_type,
            creator: content.creator,
            storage_uri,
        });

        Ok(())
    }

    /// Update storage URI for content
    pub fn update_storage_uri(
        ctx: Context<UpdateStorageUri>,
        new_storage_uri: String,
    ) -> Result<()> {
        require!(
            new_storage_uri.len() <= constants::MAX_URI_LENGTH,
            DfpnError::StringTooLong
        );

        let content = &mut ctx.accounts.content_account;
        let old_uri = content.storage_uri.clone();
        content.storage_uri = new_storage_uri.clone();

        emit!(ContentStorageUpdated {
            content_hash: content.content_hash,
            old_uri,
            new_uri: new_storage_uri,
        });

        Ok(())
    }

    /// Add a provenance claim to content
    pub fn add_provenance_claim(
        ctx: Context<AddProvenanceClaim>,
        claim_type: ClaimType,
        evidence_uri: Option<String>,
        reference_content: Option<[u8; 32]>,
    ) -> Result<()> {
        if let Some(ref uri) = evidence_uri {
            require!(
                uri.len() <= constants::MAX_URI_LENGTH,
                DfpnError::StringTooLong
            );
        }

        let claim = &mut ctx.accounts.claim_account;
        claim.content_hash = ctx.accounts.content_account.content_hash;
        claim.attestor = ctx.accounts.attestor.key();
        claim.claim_type = claim_type;
        claim.evidence_uri = evidence_uri.clone();
        claim.reference_content = reference_content;
        claim.created_at = Clock::get()?.unix_timestamp;
        claim.verified = false;
        claim.bump = ctx.bumps.claim_account;

        // Increment claim count on content
        let content = &mut ctx.accounts.content_account;
        content.claim_count = content.claim_count.saturating_add(1);

        emit!(ProvenanceClaimAdded {
            content_hash: claim.content_hash,
            attestor: claim.attestor,
            claim_type,
            evidence_uri,
        });

        Ok(())
    }

    /// Verify a provenance claim (by authorized verifier)
    pub fn verify_claim(ctx: Context<VerifyClaim>, verified: bool) -> Result<()> {
        let claim = &mut ctx.accounts.claim_account;
        claim.verified = verified;
        claim.verified_at = Some(Clock::get()?.unix_timestamp);
        claim.verified_by = Some(ctx.accounts.verifier.key());

        emit!(ClaimVerified {
            content_hash: claim.content_hash,
            attestor: claim.attestor,
            verified,
            verifier: ctx.accounts.verifier.key(),
        });

        Ok(())
    }

    /// Increment analysis count (called via CPI from marketplace)
    pub fn increment_analysis_count(ctx: Context<IncrementAnalysisCount>) -> Result<()> {
        let content = &mut ctx.accounts.content_account;
        content.analysis_count = content.analysis_count.saturating_add(1);
        Ok(())
    }

    /// Transfer content ownership
    pub fn transfer_ownership(ctx: Context<TransferOwnership>) -> Result<()> {
        let content = &mut ctx.accounts.content_account;
        let old_creator = content.creator;
        content.creator = ctx.accounts.new_owner.key();

        emit!(ContentOwnershipTransferred {
            content_hash: content.content_hash,
            old_owner: old_creator,
            new_owner: content.creator,
        });

        Ok(())
    }
}

// ============================================================================
// Account Structures
// ============================================================================

#[account]
pub struct ContentAccount {
    /// SHA256 hash of the content
    pub content_hash: [u8; 32],
    /// Type of media
    pub media_type: MediaType,
    /// Creator/owner wallet
    pub creator: Pubkey,
    /// Registration timestamp
    pub created_at: i64,
    /// Off-chain storage location
    pub storage_uri: String,
    /// Number of provenance claims
    pub claim_count: u32,
    /// Number of times analyzed
    pub analysis_count: u64,
    /// PDA bump
    pub bump: u8,
}

impl ContentAccount {
    pub const LEN: usize = 8  // discriminator
        + 32  // content_hash
        + 1   // media_type
        + 32  // creator
        + 8   // created_at
        + 4 + constants::MAX_URI_LENGTH  // storage_uri
        + 4   // claim_count
        + 8   // analysis_count
        + 1   // bump
        + 32; // padding
}

#[account]
pub struct ProvenanceClaim {
    /// Content this claim is about
    pub content_hash: [u8; 32],
    /// Who made the claim
    pub attestor: Pubkey,
    /// Type of claim
    pub claim_type: ClaimType,
    /// Optional evidence URI
    pub evidence_uri: Option<String>,
    /// Optional reference to another content (for derived works)
    pub reference_content: Option<[u8; 32]>,
    /// When claim was made
    pub created_at: i64,
    /// Whether claim has been verified
    pub verified: bool,
    /// When claim was verified
    pub verified_at: Option<i64>,
    /// Who verified the claim
    pub verified_by: Option<Pubkey>,
    /// PDA bump
    pub bump: u8,
}

impl ProvenanceClaim {
    pub const LEN: usize = 8  // discriminator
        + 32  // content_hash
        + 32  // attestor
        + 1   // claim_type
        + 1 + 4 + constants::MAX_URI_LENGTH  // evidence_uri (Option<String>)
        + 1 + 32  // reference_content (Option<[u8; 32]>)
        + 8   // created_at
        + 1   // verified
        + 1 + 8  // verified_at (Option<i64>)
        + 1 + 32  // verified_by (Option<Pubkey>)
        + 1   // bump
        + 32; // padding
}

// ============================================================================
// Instruction Contexts
// ============================================================================

#[derive(Accounts)]
#[instruction(content_hash: [u8; 32])]
pub struct RegisterContent<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        init,
        payer = creator,
        space = ContentAccount::LEN,
        seeds = [seeds::CONTENT, &content_hash],
        bump
    )]
    pub content_account: Account<'info, ContentAccount>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateStorageUri<'info> {
    pub creator: Signer<'info>,

    #[account(
        mut,
        constraint = content_account.creator == creator.key() @ DfpnError::InvalidAuthority
    )]
    pub content_account: Account<'info, ContentAccount>,
}

#[derive(Accounts)]
pub struct AddProvenanceClaim<'info> {
    #[account(mut)]
    pub attestor: Signer<'info>,

    #[account(mut)]
    pub content_account: Account<'info, ContentAccount>,

    #[account(
        init,
        payer = attestor,
        space = ProvenanceClaim::LEN,
        seeds = [
            seeds::CONTENT,
            &content_account.content_hash,
            b"claim",
            attestor.key().as_ref()
        ],
        bump
    )]
    pub claim_account: Account<'info, ProvenanceClaim>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VerifyClaim<'info> {
    /// Authorized verifier
    pub verifier: Signer<'info>,

    #[account(mut)]
    pub claim_account: Account<'info, ProvenanceClaim>,
}

#[derive(Accounts)]
pub struct IncrementAnalysisCount<'info> {
    /// Marketplace program authority
    pub authority: Signer<'info>,

    #[account(mut)]
    pub content_account: Account<'info, ContentAccount>,
}

#[derive(Accounts)]
pub struct TransferOwnership<'info> {
    pub current_owner: Signer<'info>,

    /// CHECK: New owner, no validation needed
    pub new_owner: AccountInfo<'info>,

    #[account(
        mut,
        constraint = content_account.creator == current_owner.key() @ DfpnError::InvalidAuthority
    )]
    pub content_account: Account<'info, ContentAccount>,
}

// ============================================================================
// Events
// ============================================================================

#[event]
pub struct ContentRegistered {
    pub content_hash: [u8; 32],
    pub media_type: MediaType,
    pub creator: Pubkey,
    pub storage_uri: String,
}

#[event]
pub struct ContentStorageUpdated {
    pub content_hash: [u8; 32],
    pub old_uri: String,
    pub new_uri: String,
}

#[event]
pub struct ProvenanceClaimAdded {
    pub content_hash: [u8; 32],
    pub attestor: Pubkey,
    pub claim_type: ClaimType,
    pub evidence_uri: Option<String>,
}

#[event]
pub struct ClaimVerified {
    pub content_hash: [u8; 32],
    pub attestor: Pubkey,
    pub verified: bool,
    pub verifier: Pubkey,
}

#[event]
pub struct ContentOwnershipTransferred {
    pub content_hash: [u8; 32],
    pub old_owner: Pubkey,
    pub new_owner: Pubkey,
}

// ============================================================================
// Types
// ============================================================================

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum ClaimType {
    /// Attestor claims to be the original author
    OriginalAuthor,
    /// Content is licensed from another party
    LicensedFrom,
    /// Content is derived from another work
    DerivedFrom,
    /// Content has been verified authentic by a third party
    ThirdPartyVerified,
    /// Content is part of a collection
    CollectionMember,
}
