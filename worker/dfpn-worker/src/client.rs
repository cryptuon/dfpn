//! Solana RPC client for DFPN operations

use anyhow::{anyhow, Result};
use borsh::BorshDeserialize;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_program,
    transaction::Transaction,
};
use std::str::FromStr;

use crate::config::Config;

// Program IDs (from Anchor.toml)
const WORKER_REGISTRY_ID: &str = "EoXhjMqN6hzzEejkCzDv4qMrj4y1ZxdTsmWRZL9WDbQr";
const ANALYSIS_MARKETPLACE_ID: &str = "9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin";
const REWARDS_ID: &str = "4uQeVj5tqViQh7yWWGStvkEG1Zmhx6uasJtWCJziofM";

// PDA seeds
const SEED_WORKER: &[u8] = b"worker";
const SEED_COMMIT: &[u8] = b"commit";
const SEED_REVEAL: &[u8] = b"reveal";
const SEED_REWARD: &[u8] = b"reward";

/// Worker status from on-chain account
#[derive(Debug)]
pub struct WorkerStatus {
    pub operator: String,
    pub stake: u64,
    pub reputation_score: u32,
    pub status: WorkerStatusEnum,
    pub tasks_completed: u64,
    pub tasks_failed: u64,
}

#[derive(Debug, Clone, Copy)]
pub enum WorkerStatusEnum {
    Active,
    Inactive,
    Slashed,
    Unbonding { unlock_slot: u64 },
}

/// Available task from the marketplace
#[derive(Debug, Clone, serde::Deserialize)]
pub struct AvailableTask {
    pub request_id: String,
    pub content_hash: String, // Hex-encoded hash from indexer
    pub storage_uri: String,
    pub modalities: u8,
    pub fee_amount: u64,
    pub deadline: i64,
    pub commit_deadline: i64,
}

impl AvailableTask {
    /// Parse content hash from hex string to bytes
    pub fn content_hash_bytes(&self) -> [u8; 32] {
        let mut arr = [0u8; 32];
        // Simple hex decode without external crate
        let clean = self.content_hash.trim_start_matches("0x");
        if clean.len() == 64 {
            for (i, chunk) in clean.as_bytes().chunks(2).enumerate() {
                if i >= 32 {
                    break;
                }
                let s = std::str::from_utf8(chunk).unwrap_or("00");
                arr[i] = u8::from_str_radix(s, 16).unwrap_or(0);
            }
        }
        arr
    }
}

/// Pending rewards
#[derive(Debug)]
pub struct PendingRewards {
    pub pending: u64,
    pub total_claimed: u64,
}

/// On-chain worker account data (for deserialization)
#[derive(BorshDeserialize, Debug)]
struct WorkerAccountData {
    pub discriminator: [u8; 8],
    pub operator: [u8; 32],
    pub stake: u64,
    pub reputation_score: u32,
    pub supported_modalities: u8,
    pub tasks_completed: u64,
    pub tasks_failed: u64,
    pub last_active_slot: u64,
    pub status: u8,
    pub pending_unstake: u64,
    pub unstake_unlock_slot: u64,
    pub bump: u8,
}

/// On-chain reward account data
#[derive(BorshDeserialize, Debug)]
struct RewardAccountData {
    pub discriminator: [u8; 8],
    pub claimant: [u8; 32],
    pub pending_amount: u64,
    pub total_claimed: u64,
    pub last_claim_at: i64,
    pub bump: u8,
}

/// DFPN client for interacting with Solana programs
pub struct DfpnClient {
    rpc_client: RpcClient,
    wallet: Keypair,
    config: Config,
    worker_registry_id: Pubkey,
    marketplace_id: Pubkey,
    rewards_id: Pubkey,
}

impl DfpnClient {
    /// Create a new DFPN client
    pub fn new(config: &Config) -> Result<Self> {
        let rpc_client = RpcClient::new_with_commitment(
            config.rpc_url.clone(),
            CommitmentConfig::confirmed(),
        );

        // Load wallet
        let wallet_path = shellexpand::tilde(&config.wallet_path).to_string();
        let wallet_bytes = std::fs::read(&wallet_path)?;
        let wallet = Keypair::from_bytes(&serde_json::from_slice::<Vec<u8>>(&wallet_bytes)?)?;

        // Parse program IDs
        let worker_registry_id = Pubkey::from_str(WORKER_REGISTRY_ID)?;
        let marketplace_id = Pubkey::from_str(ANALYSIS_MARKETPLACE_ID)?;
        let rewards_id = Pubkey::from_str(REWARDS_ID)?;

        Ok(Self {
            rpc_client,
            wallet,
            config: config.clone(),
            worker_registry_id,
            marketplace_id,
            rewards_id,
        })
    }

    /// Get the worker's public key
    pub fn pubkey(&self) -> Pubkey {
        self.wallet.pubkey()
    }

    /// Derive worker PDA
    pub fn derive_worker_pda(&self, operator: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[SEED_WORKER, operator.as_ref()],
            &self.worker_registry_id,
        )
    }

    /// Derive commit PDA
    pub fn derive_commit_pda(&self, request: &Pubkey, worker: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[SEED_COMMIT, request.as_ref(), worker.as_ref()],
            &self.marketplace_id,
        )
    }

    /// Derive reveal PDA
    pub fn derive_reveal_pda(&self, request: &Pubkey, worker: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[SEED_REVEAL, request.as_ref(), worker.as_ref()],
            &self.marketplace_id,
        )
    }

    /// Derive reward PDA
    pub fn derive_reward_pda(&self, claimant: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[SEED_REWARD, claimant.as_ref()],
            &self.rewards_id,
        )
    }

    /// Register as a new worker
    pub async fn register_worker(&self, stake: u64, modalities: u8) -> Result<String> {
        let (worker_pda, _bump) = self.derive_worker_pda(&self.pubkey());

        tracing::info!(
            "Registering worker {} with stake {} and modalities {}",
            self.pubkey(),
            stake,
            modalities
        );

        // Build instruction data (Anchor format: 8-byte discriminator + args)
        // register_worker discriminator: sha256("global:register_worker")[..8]
        let mut data = vec![0x01, 0x8c, 0x43, 0x4d, 0x65, 0x8e, 0x7f, 0xa6]; // discriminator
        data.extend_from_slice(&stake.to_le_bytes());
        data.push(modalities);

        let accounts = vec![
            AccountMeta::new(self.pubkey(), true),          // operator (signer)
            AccountMeta::new(worker_pda, false),            // worker_account
            AccountMeta::new_readonly(system_program::id(), false),
        ];

        let instruction = Instruction {
            program_id: self.worker_registry_id,
            accounts,
            data,
        };

        let recent_blockhash = self.rpc_client.get_latest_blockhash()?;
        let tx = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&self.pubkey()),
            &[&self.wallet],
            recent_blockhash,
        );

        let signature = self.rpc_client.send_and_confirm_transaction(&tx)?;
        tracing::info!("Worker registered: {}", signature);

        Ok(signature.to_string())
    }

    /// Get worker status from on-chain account
    pub async fn get_worker_status(&self) -> Result<WorkerStatus> {
        let (worker_pda, _) = self.derive_worker_pda(&self.pubkey());

        let account = self.rpc_client.get_account(&worker_pda)
            .map_err(|e| anyhow!("Failed to fetch worker account: {}", e))?;

        // Deserialize using Borsh
        let worker_data: WorkerAccountData = BorshDeserialize::try_from_slice(&account.data)
            .map_err(|e| anyhow!("Failed to deserialize worker account: {}", e))?;

        let status = match worker_data.status {
            0 => WorkerStatusEnum::Active,
            1 => WorkerStatusEnum::Inactive,
            2 => WorkerStatusEnum::Slashed,
            3 => WorkerStatusEnum::Unbonding {
                unlock_slot: worker_data.unstake_unlock_slot,
            },
            _ => WorkerStatusEnum::Inactive,
        };

        Ok(WorkerStatus {
            operator: Pubkey::new_from_array(worker_data.operator).to_string(),
            stake: worker_data.stake,
            reputation_score: worker_data.reputation_score,
            status,
            tasks_completed: worker_data.tasks_completed,
            tasks_failed: worker_data.tasks_failed,
        })
    }

    /// Get available tasks matching worker's modalities
    pub async fn get_available_tasks(&self, limit: usize) -> Result<Vec<AvailableTask>> {
        // Query indexer for open requests
        if !self.config.indexer_url.is_empty() {
            let modalities_param = self.config.worker.modalities.join(",");
            let url = format!(
                "{}/requests?status=Open&modalities={}&limit={}",
                self.config.indexer_url,
                modalities_param,
                limit
            );

            let response = reqwest::get(&url).await;
            if let Ok(resp) = response {
                if let Ok(tasks) = resp.json::<Vec<AvailableTask>>().await {
                    return Ok(tasks);
                }
            }
        }

        // Fallback: return empty (would need getProgramAccounts for direct query)
        tracing::debug!("No indexer available or no tasks found");
        Ok(vec![])
    }

    /// Get pending rewards for this worker
    pub async fn get_pending_rewards(&self) -> Result<PendingRewards> {
        let (reward_pda, _) = self.derive_reward_pda(&self.pubkey());

        match self.rpc_client.get_account(&reward_pda) {
            Ok(account) => {
                let reward_data: RewardAccountData = BorshDeserialize::try_from_slice(&account.data)
                    .map_err(|e| anyhow!("Failed to deserialize reward account: {}", e))?;

                Ok(PendingRewards {
                    pending: reward_data.pending_amount,
                    total_claimed: reward_data.total_claimed,
                })
            }
            Err(_) => {
                // Account doesn't exist yet
                Ok(PendingRewards {
                    pending: 0,
                    total_claimed: 0,
                })
            }
        }
    }

    /// Claim accumulated rewards
    pub async fn claim_rewards(&self) -> Result<u64> {
        let rewards = self.get_pending_rewards().await?;
        if rewards.pending == 0 {
            return Ok(0);
        }

        let (reward_pda, _) = self.derive_reward_pda(&self.pubkey());

        // Build claim instruction
        // claim_rewards discriminator
        let data = vec![0x95, 0x42, 0x6a, 0x14, 0x9d, 0x4c, 0x83, 0xae]; // discriminator

        let accounts = vec![
            AccountMeta::new_readonly(self.pubkey(), true), // claimant
            AccountMeta::new(reward_pda, false),            // reward_account
            // Additional accounts would be needed (treasury, destination, etc.)
        ];

        let instruction = Instruction {
            program_id: self.rewards_id,
            accounts,
            data,
        };

        let recent_blockhash = self.rpc_client.get_latest_blockhash()?;
        let tx = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&self.pubkey()),
            &[&self.wallet],
            recent_blockhash,
        );

        let signature = self.rpc_client.send_and_confirm_transaction(&tx)?;
        tracing::info!("Rewards claimed: {}", signature);

        Ok(rewards.pending)
    }

    /// Update worker configuration
    pub async fn update_worker(
        &self,
        modalities: Option<u8>,
        add_stake: Option<u64>,
    ) -> Result<String> {
        let (worker_pda, _) = self.derive_worker_pda(&self.pubkey());

        // Build update instruction
        let mut data = vec![0xd2, 0xf5, 0x32, 0x4a, 0x87, 0x6c, 0x99, 0x1b]; // discriminator

        // Encode optional modalities
        if let Some(m) = modalities {
            data.push(1); // Some
            data.push(m);
        } else {
            data.push(0); // None
        }

        // Encode optional stake
        if let Some(s) = add_stake {
            data.push(1); // Some
            data.extend_from_slice(&s.to_le_bytes());
        } else {
            data.push(0); // None
        }

        let accounts = vec![
            AccountMeta::new(self.pubkey(), true),
            AccountMeta::new(worker_pda, false),
        ];

        let instruction = Instruction {
            program_id: self.worker_registry_id,
            accounts,
            data,
        };

        let recent_blockhash = self.rpc_client.get_latest_blockhash()?;
        let tx = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&self.pubkey()),
            &[&self.wallet],
            recent_blockhash,
        );

        let signature = self.rpc_client.send_and_confirm_transaction(&tx)?;
        Ok(signature.to_string())
    }

    /// Request stake withdrawal
    pub async fn request_unstake(&self, amount: u64) -> Result<String> {
        let (worker_pda, _) = self.derive_worker_pda(&self.pubkey());

        // Build unstake instruction
        let mut data = vec![0xa7, 0xb3, 0xc8, 0x21, 0x54, 0x67, 0x8a, 0x9f]; // discriminator
        data.extend_from_slice(&amount.to_le_bytes());

        let accounts = vec![
            AccountMeta::new(self.pubkey(), true),
            AccountMeta::new(worker_pda, false),
        ];

        let instruction = Instruction {
            program_id: self.worker_registry_id,
            accounts,
            data,
        };

        let recent_blockhash = self.rpc_client.get_latest_blockhash()?;
        let tx = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&self.pubkey()),
            &[&self.wallet],
            recent_blockhash,
        );

        let signature = self.rpc_client.send_and_confirm_transaction(&tx)?;
        Ok(signature.to_string())
    }

    /// Withdraw unstaked tokens
    pub async fn withdraw_stake(&self) -> Result<u64> {
        let (worker_pda, _) = self.derive_worker_pda(&self.pubkey());

        // Build withdraw instruction
        let data = vec![0xb5, 0x12, 0x43, 0x76, 0xa9, 0x8c, 0xdf, 0x02]; // discriminator

        let accounts = vec![
            AccountMeta::new(self.pubkey(), true),
            AccountMeta::new(worker_pda, false),
        ];

        let instruction = Instruction {
            program_id: self.worker_registry_id,
            accounts,
            data,
        };

        let recent_blockhash = self.rpc_client.get_latest_blockhash()?;
        let tx = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&self.pubkey()),
            &[&self.wallet],
            recent_blockhash,
        );

        let signature = self.rpc_client.send_and_confirm_transaction(&tx)?;
        tracing::info!("Stake withdrawn: {}", signature);

        // Would need to track actual amount
        Ok(0)
    }

    /// Submit a commit for a request
    pub async fn submit_commit(
        &self,
        request: &Pubkey,
        commitment: [u8; 32],
    ) -> Result<String> {
        let (worker_pda, _) = self.derive_worker_pda(&self.pubkey());
        let (commit_pda, _) = self.derive_commit_pda(request, &self.pubkey());

        // Build commit instruction
        // commit_result discriminator + commitment
        let mut data = vec![0xc4, 0x23, 0x87, 0x5a, 0xb1, 0xde, 0x6f, 0x09]; // discriminator
        data.extend_from_slice(&commitment);

        let accounts = vec![
            AccountMeta::new(self.pubkey(), true),     // operator
            AccountMeta::new(*request, false),          // request_account
            AccountMeta::new_readonly(worker_pda, false), // worker_account
            AccountMeta::new(commit_pda, false),        // commit_account
            AccountMeta::new_readonly(system_program::id(), false),
        ];

        let instruction = Instruction {
            program_id: self.marketplace_id,
            accounts,
            data,
        };

        let recent_blockhash = self.rpc_client.get_latest_blockhash()?;
        let tx = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&self.pubkey()),
            &[&self.wallet],
            recent_blockhash,
        );

        let signature = self.rpc_client.send_and_confirm_transaction(&tx)?;
        tracing::info!("Commit submitted: {}", signature);

        Ok(signature.to_string())
    }

    /// Submit a reveal for a request
    pub async fn submit_reveal(
        &self,
        request: &Pubkey,
        verdict: u8,
        confidence: u8,
        detections_hash: [u8; 32],
        salt: [u8; 16],
        model: &Pubkey,
    ) -> Result<String> {
        let (commit_pda, _) = self.derive_commit_pda(request, &self.pubkey());
        let (reveal_pda, _) = self.derive_reveal_pda(request, &self.pubkey());

        // Build reveal instruction
        // reveal_result discriminator + verdict + confidence + detections_hash + salt
        let mut data = vec![0xe8, 0x79, 0x2c, 0x41, 0x6b, 0xa3, 0xf7, 0x5d]; // discriminator
        data.push(verdict);
        data.push(confidence);
        data.extend_from_slice(&detections_hash);
        data.extend_from_slice(&salt);

        let accounts = vec![
            AccountMeta::new(self.pubkey(), true),     // operator
            AccountMeta::new(*request, false),          // request_account
            AccountMeta::new(commit_pda, false),        // commit_account
            AccountMeta::new(reveal_pda, false),        // reveal_account
            AccountMeta::new_readonly(*model, false),   // model_account
            AccountMeta::new_readonly(system_program::id(), false),
        ];

        let instruction = Instruction {
            program_id: self.marketplace_id,
            accounts,
            data,
        };

        let recent_blockhash = self.rpc_client.get_latest_blockhash()?;
        let tx = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&self.pubkey()),
            &[&self.wallet],
            recent_blockhash,
        );

        let signature = self.rpc_client.send_and_confirm_transaction(&tx)?;
        tracing::info!("Reveal submitted: {}", signature);

        Ok(signature.to_string())
    }

    /// Get RPC client reference
    pub fn rpc(&self) -> &RpcClient {
        &self.rpc_client
    }

    /// Get current slot
    pub async fn get_slot(&self) -> Result<u64> {
        Ok(self.rpc_client.get_slot()?)
    }

    /// Get current timestamp (approximation based on slot)
    pub async fn get_timestamp(&self) -> Result<i64> {
        // For simplicity, just use current system time
        // In production, would parse clock sysvar for exact on-chain time
        Ok(std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs() as i64)
    }
}
