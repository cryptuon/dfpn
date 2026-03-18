//! Commit-reveal protocol implementation

use sha2::{Digest, Sha256};
use solana_sdk::pubkey::Pubkey;

use crate::inference::AnalysisResult;

/// Commit-reveal protocol utilities
pub struct CommitReveal;

impl CommitReveal {
    /// Generate random salt for commitment
    pub fn generate_salt() -> [u8; 16] {
        let mut salt = [0u8; 16];
        getrandom::getrandom(&mut salt).expect("Failed to generate random salt");
        salt
    }

    /// Compute commitment hash
    ///
    /// commitment = SHA256(result_bytes || salt || worker_pubkey || request_pubkey)
    pub fn compute_commitment(
        result: &AnalysisResult,
        salt: &[u8; 16],
        worker: &Pubkey,
        request: &Pubkey,
    ) -> [u8; 32] {
        let result_bytes = Self::encode_result(result);

        let mut hasher = Sha256::new();
        hasher.update(&result_bytes);
        hasher.update(salt);
        hasher.update(worker.as_ref());
        hasher.update(request.as_ref());

        hasher.finalize().into()
    }

    /// Verify that a reveal matches a commitment
    pub fn verify_commitment(
        result: &AnalysisResult,
        salt: &[u8; 16],
        worker: &Pubkey,
        request: &Pubkey,
        commitment: &[u8; 32],
    ) -> bool {
        let computed = Self::compute_commitment(result, salt, worker, request);
        computed == *commitment
    }

    /// Encode result to bytes for hashing
    fn encode_result(result: &AnalysisResult) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(34);
        bytes.push(result.verdict as u8);
        bytes.push(result.confidence);
        bytes.extend_from_slice(&result.detections_hash);
        bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inference::Verdict;

    #[test]
    fn test_salt_generation() {
        let salt1 = CommitReveal::generate_salt();
        let salt2 = CommitReveal::generate_salt();

        // Salts should be different
        assert_ne!(salt1, salt2);

        // Salt should be 16 bytes
        assert_eq!(salt1.len(), 16);
    }

    #[test]
    fn test_commitment_deterministic() {
        let result = AnalysisResult {
            verdict: Verdict::Manipulated,
            confidence: 85,
            detections: vec![],
            detections_hash: [0u8; 32],
            processing_time_ms: 100,
        };

        let salt = [1u8; 16];
        let worker = Pubkey::new_unique();
        let request = Pubkey::new_unique();

        let commitment1 = CommitReveal::compute_commitment(&result, &salt, &worker, &request);
        let commitment2 = CommitReveal::compute_commitment(&result, &salt, &worker, &request);

        assert_eq!(commitment1, commitment2);
    }

    #[test]
    fn test_commitment_varies_with_salt() {
        let result = AnalysisResult {
            verdict: Verdict::Manipulated,
            confidence: 85,
            detections: vec![],
            detections_hash: [0u8; 32],
            processing_time_ms: 100,
        };

        let salt1 = [1u8; 16];
        let salt2 = [2u8; 16];
        let worker = Pubkey::new_unique();
        let request = Pubkey::new_unique();

        let commitment1 = CommitReveal::compute_commitment(&result, &salt1, &worker, &request);
        let commitment2 = CommitReveal::compute_commitment(&result, &salt2, &worker, &request);

        assert_ne!(commitment1, commitment2);
    }

    #[test]
    fn test_verify_commitment() {
        let result = AnalysisResult {
            verdict: Verdict::Authentic,
            confidence: 95,
            detections: vec![],
            detections_hash: [0u8; 32],
            processing_time_ms: 50,
        };

        let salt = CommitReveal::generate_salt();
        let worker = Pubkey::new_unique();
        let request = Pubkey::new_unique();

        let commitment = CommitReveal::compute_commitment(&result, &salt, &worker, &request);

        // Should verify correctly
        assert!(CommitReveal::verify_commitment(
            &result,
            &salt,
            &worker,
            &request,
            &commitment
        ));

        // Should fail with wrong result
        let wrong_result = AnalysisResult {
            verdict: Verdict::Manipulated,
            ..result.clone()
        };
        assert!(!CommitReveal::verify_commitment(
            &wrong_result,
            &salt,
            &worker,
            &request,
            &commitment
        ));
    }
}
