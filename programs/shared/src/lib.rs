use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111111");

/// Media type for content registration
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum MediaType {
    Image = 0,
    Video = 1,
    Audio = 2,
}

/// Modality flags for detection capabilities (bitfield)
/// Each bit represents a supported modality
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Modalities(pub u8);

impl Modalities {
    pub const IMAGE_AUTHENTICITY: u8 = 1 << 0;
    pub const VIDEO_AUTHENTICITY: u8 = 1 << 1;
    pub const AUDIO_AUTHENTICITY: u8 = 1 << 2;
    pub const FACE_MANIPULATION: u8 = 1 << 3;
    pub const VOICE_CLONING: u8 = 1 << 4;
    pub const GENERATED_CONTENT: u8 = 1 << 5;

    pub fn new() -> Self {
        Self(0)
    }

    pub fn from_bits(bits: u8) -> Self {
        Self(bits)
    }

    pub fn has(&self, flag: u8) -> bool {
        self.0 & flag != 0
    }

    pub fn set(&mut self, flag: u8) {
        self.0 |= flag;
    }

    pub fn unset(&mut self, flag: u8) {
        self.0 &= !flag;
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    /// Check if self contains all modalities required by other
    pub fn supports(&self, required: &Modalities) -> bool {
        (self.0 & required.0) == required.0
    }
}

/// Verdict from analysis
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Verdict {
    Authentic = 0,
    Manipulated = 1,
    Inconclusive = 2,
}

/// Worker status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum WorkerStatus {
    /// Worker is active and can accept tasks
    Active,
    /// Worker is inactive (voluntary pause)
    Inactive,
    /// Worker has been slashed and is suspended
    Slashed,
    /// Worker is unbonding stake
    Unbonding {
        /// Slot when stake can be withdrawn
        unlock_slot: u64,
    },
}

impl Default for WorkerStatus {
    fn default() -> Self {
        Self::Active
    }
}

/// Model status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum ModelStatus {
    /// Model is pending evaluation
    Pending,
    /// Model is active and can be used
    Active,
    /// Model is retired (voluntary or forced)
    Retired,
    /// Model is suspended due to issues
    Suspended,
}

impl Default for ModelStatus {
    fn default() -> Self {
        Self::Pending
    }
}

/// Request status in the analysis marketplace
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum RequestStatus {
    /// Request is open, accepting commits
    Open,
    /// Commit window closed, accepting reveals
    CommitClosed,
    /// Request finalized with results
    Finalized,
    /// Request expired without sufficient results
    Expired,
    /// Request cancelled by requester
    Cancelled,
    /// Request under dispute
    Disputed,
}

impl Default for RequestStatus {
    fn default() -> Self {
        Self::Open
    }
}

/// Reason for opening a dispute
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum DisputeReason {
    /// Worker submitted invalid/fabricated result
    InvalidResult = 0,
    /// Worker colluded with others
    Collusion = 1,
    /// Worker copied another's result
    ResultCopying = 2,
    /// Worker used incorrect model
    IncorrectModel = 3,
    /// Result does not match content
    ContentMismatch = 4,
    /// Other reason (requires evidence)
    Other = 5,
}

/// Status of a dispute
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum DisputeStatus {
    /// Dispute is open and under review
    Open,
    /// Under investigation by resolver
    UnderReview,
    /// Resolved in favor of challenger
    ResolvedForChallenger,
    /// Resolved in favor of challenged worker
    ResolvedForWorker,
    /// Dismissed (invalid dispute)
    Dismissed,
}

impl Default for DisputeStatus {
    fn default() -> Self {
        Self::Open
    }
}

/// Consensus type for finalized requests
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum ConsensusType {
    /// All workers agreed
    Unanimous = 0,
    /// Majority (>50%) agreed
    Majority = 1,
    /// Workers were split (no clear majority)
    Split = 2,
    /// Only one worker participated
    Single = 3,
}

/// Constants for the protocol
pub mod constants {
    /// Maximum length for storage URIs (IPFS, Arweave, etc.)
    pub const MAX_URI_LENGTH: usize = 200;

    /// Maximum length for model/content names
    pub const MAX_NAME_LENGTH: usize = 64;

    /// Maximum length for version strings
    pub const MAX_VERSION_LENGTH: usize = 16;

    /// Minimum stake for workers (in token base units)
    pub const MIN_WORKER_STAKE: u64 = 5_000_000_000_000; // 5,000 DFPN (9 decimals)

    /// Minimum stake for model developers (in token base units)
    pub const MIN_MODEL_STAKE: u64 = 20_000_000_000_000; // 20,000 DFPN (9 decimals)

    /// Unbonding period in slots (~2 days at 400ms/slot)
    pub const UNBONDING_PERIOD_SLOTS: u64 = 432_000;

    /// Default commit window ratio (70% of total time)
    pub const DEFAULT_COMMIT_RATIO_BPS: u16 = 7000;

    /// Minimum deadline duration in seconds
    pub const MIN_DEADLINE_SECONDS: i64 = 60;

    /// Maximum workers per request
    pub const MAX_WORKERS_PER_REQUEST: u8 = 10;

    /// Basis points denominator (10000 = 100%)
    pub const BPS_DENOMINATOR: u16 = 10_000;

    /// Default fee splits (in basis points)
    pub const DEFAULT_WORKER_SHARE_BPS: u16 = 6_500; // 65%
    pub const DEFAULT_MODEL_SHARE_BPS: u16 = 2_000;  // 20%
    pub const DEFAULT_TREASURY_SHARE_BPS: u16 = 1_000; // 10%
    pub const DEFAULT_INSURANCE_SHARE_BPS: u16 = 500;  // 5%

    /// Slashing rates (in basis points)
    pub const SLASH_INVALID_RESULT_BPS: u16 = 1_000;   // 10%
    pub const SLASH_MISSED_DEADLINE_BPS: u16 = 300;    // 3%
    pub const SLASH_FRAUD_MIN_BPS: u16 = 2_500;        // 25%
    pub const SLASH_FRAUD_MAX_BPS: u16 = 5_000;        // 50%

    /// Initial reputation score for new workers
    pub const INITIAL_REPUTATION: u32 = 5_000; // 50%

    /// Maximum reputation score
    pub const MAX_REPUTATION: u32 = 10_000; // 100%

    /// Salt length for commit-reveal
    pub const SALT_LENGTH: usize = 16;

    /// Minimum stake to open a dispute (in token base units)
    pub const MIN_DISPUTE_STAKE: u64 = 1_000_000_000_000; // 1,000 DFPN

    /// Dispute resolution timeout in seconds (7 days)
    pub const DISPUTE_TIMEOUT_SECONDS: i64 = 604_800;

    /// Dispute challenger reward when resolved in their favor (basis points of challenged worker's slash)
    pub const DISPUTE_CHALLENGER_REWARD_BPS: u16 = 2_000; // 20%
}

/// Error codes shared across programs
#[error_code]
pub enum DfpnError {
    #[msg("Invalid modality configuration")]
    InvalidModality,

    #[msg("Insufficient stake amount")]
    InsufficientStake,

    #[msg("Worker is not active")]
    WorkerNotActive,

    #[msg("Model is not active")]
    ModelNotActive,

    #[msg("Request has expired")]
    RequestExpired,

    #[msg("Commit window has closed")]
    CommitWindowClosed,

    #[msg("Reveal window has closed")]
    RevealWindowClosed,

    #[msg("Reveal window not yet open")]
    RevealWindowNotOpen,

    #[msg("Invalid commitment hash")]
    InvalidCommitment,

    #[msg("Already committed to this request")]
    DuplicateCommit,

    #[msg("Already revealed for this request")]
    AlreadyRevealed,

    #[msg("No commitment found")]
    NoCommitment,

    #[msg("Deadline too short")]
    DeadlineTooShort,

    #[msg("Invalid fee amount")]
    InvalidFeeAmount,

    #[msg("Request not open")]
    RequestNotOpen,

    #[msg("Request already finalized")]
    RequestAlreadyFinalized,

    #[msg("Insufficient reveals for finalization")]
    InsufficientReveals,

    #[msg("Unbonding period not complete")]
    UnbondingNotComplete,

    #[msg("Invalid authority")]
    InvalidAuthority,

    #[msg("String too long")]
    StringTooLong,

    #[msg("Arithmetic overflow")]
    ArithmeticOverflow,

    #[msg("Worker does not support required modalities")]
    ModalityMismatch,

    #[msg("Cannot cancel request with existing commits")]
    CannotCancelWithCommits,

    #[msg("Invalid share configuration")]
    InvalidShareConfig,

    #[msg("Dispute already exists for this result")]
    DuplicateDispute,

    #[msg("Dispute not found")]
    DisputeNotFound,

    #[msg("Dispute already resolved")]
    DisputeAlreadyResolved,

    #[msg("Dispute timeout not reached")]
    DisputeTimeoutNotReached,

    #[msg("Invalid dispute reason")]
    InvalidDisputeReason,

    #[msg("Request not finalized")]
    RequestNotFinalized,

    #[msg("Cannot dispute own result")]
    CannotDisputeOwnResult,
}

/// Seeds for PDA derivation
pub mod seeds {
    pub const CONTENT: &[u8] = b"content";
    pub const REQUEST: &[u8] = b"request";
    pub const COMMIT: &[u8] = b"commit";
    pub const REVEAL: &[u8] = b"reveal";
    pub const WORKER: &[u8] = b"worker";
    pub const MODEL: &[u8] = b"model";
    pub const TREASURY: &[u8] = b"treasury";
    pub const REWARD: &[u8] = b"reward";
    pub const STAKE_VAULT: &[u8] = b"stake_vault";
    pub const FEE_VAULT: &[u8] = b"fee_vault";
    pub const DISPUTE: &[u8] = b"dispute";
    pub const EPOCH: &[u8] = b"epoch";
}

/// CPI interface for cross-program invocations
#[cfg(feature = "cpi")]
pub mod cpi {
    use super::*;
    use anchor_lang::solana_program::{
        account_info::AccountInfo,
        instruction::{AccountMeta, Instruction},
        program::invoke,
        pubkey::Pubkey,
    };

    /// Rewards program ID (should match the deployed program)
    pub const REWARDS_PROGRAM_ID: &str = "4uQeVj5tqViQh7yWWGStvkEG1Zmhx6uasJtWCJziofM";

    /// Get the rewards program pubkey
    pub fn rewards_program_id() -> Pubkey {
        REWARDS_PROGRAM_ID.parse().unwrap()
    }

    /// Worker contribution data for reward distribution
    #[derive(AnchorSerialize, AnchorDeserialize, Clone)]
    pub struct WorkerContribution {
        pub worker: Pubkey,
        pub reputation: u32,
        pub participated: bool,
    }

    /// Anchor discriminator for process_request_rewards
    /// Computed as: sha256("global:process_request_rewards")[0..8]
    pub const PROCESS_REQUEST_REWARDS_DISCRIMINATOR: [u8; 8] = [0x9d, 0x4b, 0x3e, 0x1c, 0x8a, 0x7f, 0x2d, 0x5e];

    /// Build instruction data for process_request_rewards
    pub fn build_process_request_rewards_data(
        fee_amount: u64,
        workers: &[WorkerContribution],
        model_developer: Option<Pubkey>,
    ) -> Result<Vec<u8>> {
        let mut data = Vec::new();

        // Discriminator
        data.extend_from_slice(&PROCESS_REQUEST_REWARDS_DISCRIMINATOR);

        // fee_amount (u64, little-endian)
        data.extend_from_slice(&fee_amount.to_le_bytes());

        // workers vec (length + items)
        data.extend_from_slice(&(workers.len() as u32).to_le_bytes());
        for worker in workers {
            // Pubkey (32 bytes)
            data.extend_from_slice(worker.worker.as_ref());
            // reputation (u32, little-endian)
            data.extend_from_slice(&worker.reputation.to_le_bytes());
            // participated (bool as u8)
            data.push(if worker.participated { 1 } else { 0 });
        }

        // model_developer Option<Pubkey>
        match model_developer {
            Some(pubkey) => {
                data.push(1); // Some
                data.extend_from_slice(pubkey.as_ref());
            }
            None => {
                data.push(0); // None
            }
        }

        Ok(data)
    }

    /// Invoke process_request_rewards on the rewards program
    /// Takes owned AccountInfo values to avoid lifetime issues
    pub fn process_request_rewards<'a>(
        authority: AccountInfo<'a>,
        treasury: AccountInfo<'a>,
        epoch_config: AccountInfo<'a>,
        fee_vault: AccountInfo<'a>,
        token_program: AccountInfo<'a>,
        rewards_program: AccountInfo<'a>,
        fee_amount: u64,
        workers: &[WorkerContribution],
        model_developer: Option<Pubkey>,
        signer_seeds: Option<&[&[&[u8]]]>,
    ) -> Result<()> {
        let rewards_program_id = rewards_program_id();

        // Build instruction data
        let data = build_process_request_rewards_data(fee_amount, workers, model_developer)?;

        // Build account metas
        let account_metas = vec![
            AccountMeta::new_readonly(*authority.key, true),
            AccountMeta::new(*treasury.key, false),
            AccountMeta::new(*epoch_config.key, false),
            AccountMeta::new_readonly(*fee_vault.key, false),
            AccountMeta::new_readonly(*token_program.key, false),
        ];

        let instruction = Instruction {
            program_id: rewards_program_id,
            accounts: account_metas,
            data,
        };

        let account_infos = &[
            authority,
            treasury,
            epoch_config,
            fee_vault,
            token_program,
            rewards_program,
        ];

        if let Some(seeds) = signer_seeds {
            anchor_lang::solana_program::program::invoke_signed(
                &instruction,
                account_infos,
                seeds,
            )?;
        } else {
            invoke(&instruction, account_infos)?;
        }

        Ok(())
    }

    /// Derive treasury PDA
    pub fn derive_treasury_pda() -> (Pubkey, u8) {
        Pubkey::find_program_address(&[seeds::TREASURY], &rewards_program_id())
    }

    /// Derive epoch config PDA
    pub fn derive_epoch_config_pda(treasury: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[seeds::EPOCH, treasury.as_ref()],
            &rewards_program_id(),
        )
    }

    /// Derive fee vault PDA
    pub fn derive_fee_vault_pda() -> (Pubkey, u8) {
        Pubkey::find_program_address(&[seeds::FEE_VAULT], &rewards_program_id())
    }
}

/// Utility functions
pub mod utils {
    use super::*;
    use sha2::{Digest, Sha256};

    /// Compute commitment hash for commit-reveal
    pub fn compute_commitment(
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

    /// Calculate commit deadline from request deadline
    pub fn calculate_commit_deadline(
        created_at: i64,
        deadline: i64,
        commit_ratio_bps: u16,
    ) -> i64 {
        let total_duration = deadline - created_at;
        let commit_duration = (total_duration as u64 * commit_ratio_bps as u64
            / constants::BPS_DENOMINATOR as u64) as i64;
        created_at + commit_duration
    }

    /// Calculate reward share
    pub fn calculate_share(amount: u64, share_bps: u16) -> Option<u64> {
        amount
            .checked_mul(share_bps as u64)?
            .checked_div(constants::BPS_DENOMINATOR as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modalities() {
        let mut m = Modalities::new();
        assert!(m.is_empty());

        m.set(Modalities::IMAGE_AUTHENTICITY);
        m.set(Modalities::FACE_MANIPULATION);

        assert!(m.has(Modalities::IMAGE_AUTHENTICITY));
        assert!(m.has(Modalities::FACE_MANIPULATION));
        assert!(!m.has(Modalities::VIDEO_AUTHENTICITY));

        let required = Modalities::from_bits(Modalities::IMAGE_AUTHENTICITY);
        assert!(m.supports(&required));

        let required2 = Modalities::from_bits(
            Modalities::IMAGE_AUTHENTICITY | Modalities::VIDEO_AUTHENTICITY,
        );
        assert!(!m.supports(&required2));
    }

    #[test]
    fn test_calculate_share() {
        assert_eq!(utils::calculate_share(10_000, 6_500), Some(6_500));
        assert_eq!(utils::calculate_share(10_000, 10_000), Some(10_000));
        assert_eq!(utils::calculate_share(100, 500), Some(5));
    }

    #[test]
    fn test_commit_deadline() {
        let created = 1000;
        let deadline = 2000;
        let commit_deadline = utils::calculate_commit_deadline(created, deadline, 7000);
        assert_eq!(commit_deadline, 1700); // 70% of 1000 second window
    }
}
