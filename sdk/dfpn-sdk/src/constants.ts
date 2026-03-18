import { PublicKey } from '@solana/web3.js';

/**
 * Program IDs for DFPN programs
 */
export const PROGRAM_IDS = {
  contentRegistry: new PublicKey('DFPNcontent111111111111111111111111111111111'),
  analysisMarketplace: new PublicKey('DFPNmarket1111111111111111111111111111111111'),
  modelRegistry: new PublicKey('DFPNmodel11111111111111111111111111111111111'),
  workerRegistry: new PublicKey('DFPNworker1111111111111111111111111111111111'),
  rewards: new PublicKey('DFPNrewards111111111111111111111111111111111'),
};

/**
 * Modality bit flags
 */
export const MODALITY_FLAGS = {
  ImageAuthenticity: 1 << 0,
  VideoAuthenticity: 1 << 1,
  AudioAuthenticity: 1 << 2,
  FaceManipulation: 1 << 3,
  VoiceCloning: 1 << 4,
  GeneratedContent: 1 << 5,
} as const;

/**
 * Minimum stake for workers (in base units with 9 decimals)
 */
export const MIN_WORKER_STAKE = BigInt('5000000000000'); // 5,000 DFPN

/**
 * Minimum stake for model developers (in base units with 9 decimals)
 */
export const MIN_MODEL_STAKE = BigInt('20000000000000'); // 20,000 DFPN

/**
 * Basis points denominator (10000 = 100%)
 */
export const BPS_DENOMINATOR = 10000;

/**
 * Default fee shares
 */
export const DEFAULT_FEE_SHARES = {
  worker: 6500,    // 65%
  model: 2000,     // 20%
  treasury: 1000,  // 10%
  insurance: 500,  // 5%
};

/**
 * Default commit window ratio (70% of total time)
 */
export const DEFAULT_COMMIT_RATIO = 0.7;

/**
 * Minimum deadline in seconds
 */
export const MIN_DEADLINE_SECONDS = 60;

/**
 * Maximum workers per request
 */
export const MAX_WORKERS_PER_REQUEST = 10;

/**
 * PDA seeds
 */
export const SEEDS = {
  content: Buffer.from('content'),
  request: Buffer.from('request'),
  commit: Buffer.from('commit'),
  reveal: Buffer.from('reveal'),
  worker: Buffer.from('worker'),
  model: Buffer.from('model'),
  treasury: Buffer.from('treasury'),
  reward: Buffer.from('reward'),
  stakeVault: Buffer.from('stake_vault'),
  feeVault: Buffer.from('fee_vault'),
};
