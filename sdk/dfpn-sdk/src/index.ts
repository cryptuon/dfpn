/**
 * DFPN SDK - TypeScript client for the Decentralized Deepfake Detection Platform Network
 *
 * @packageDocumentation
 */

// Main client
export { DFPNClient, ClientOptions, Wallet, WaitOptions } from './client';

// Types
export {
  // Enums
  Modality,
  Verdict,
  MediaType,
  RequestStatus,
  WorkerStatus,
  ModelStatus,

  // Request/Response types
  AnalysisRequest,
  AnalysisResult,
  WorkerResult,
  Detection,
  AuditTrail,

  // Account types
  ContentAccount,
  WorkerAccount,
  ModelAccount,
  RewardAccount,

  // Params
  CreateRequestParams,
  RegisterContentParams,
  RegisterWorkerParams,
  RegisterModelParams,

  // Filters
  RequestFilters,
  WorkerFilters,
  ModelFilters,
} from './types';

// Constants
export {
  PROGRAM_IDS,
  MODALITY_FLAGS,
  MIN_WORKER_STAKE,
  MIN_MODEL_STAKE,
  BPS_DENOMINATOR,
  DEFAULT_FEE_SHARES,
  DEFAULT_COMMIT_RATIO,
  MIN_DEADLINE_SECONDS,
  MAX_WORKERS_PER_REQUEST,
  SEEDS,
} from './constants';

// Utilities
export {
  computeContentHash,
  computeCommitment,
  generateSalt,
  parseModalities,
  modalitiesToBits,
  modalityToString,
  formatDfpn,
  parseDfpn,
  deriveContentPDA,
  deriveRequestPDA,
  deriveWorkerPDA,
  deriveModelPDA,
  deriveRewardPDA,
} from './utils';

// Instructions
export {
  createRequestInstruction,
  commitResultInstruction,
  revealResultInstruction,
  finalizeRequestInstruction,
  cancelRequestInstruction,
  registerContentInstruction,
  registerWorkerInstruction,
  claimRewardsInstruction,
} from './instructions';

// Account deserialization
export {
  deserializeAnalysisRequest,
  deserializeContent,
  deserializeWorker,
  deserializeModel,
  deserializeReward,
  deserializeReveal,
  fetchAnalysisRequest,
  fetchWorker,
  fetchModel,
  fetchRevealsForRequest,
  fetchOpenRequests,
  fetchWorkers,
} from './accounts';
