import { PublicKey } from '@solana/web3.js';

/**
 * Supported modalities for analysis
 */
export enum Modality {
  ImageAuthenticity = 'ImageAuthenticity',
  VideoAuthenticity = 'VideoAuthenticity',
  AudioAuthenticity = 'AudioAuthenticity',
  FaceManipulation = 'FaceManipulation',
  VoiceCloning = 'VoiceCloning',
  GeneratedContent = 'GeneratedContent',
}

/**
 * Analysis verdict
 */
export enum Verdict {
  Authentic = 'Authentic',
  Manipulated = 'Manipulated',
  Inconclusive = 'Inconclusive',
}

/**
 * Media type for content registration
 */
export enum MediaType {
  Image = 'Image',
  Video = 'Video',
  Audio = 'Audio',
}

/**
 * Request status in the marketplace
 */
export enum RequestStatus {
  Open = 'Open',
  CommitClosed = 'CommitClosed',
  Finalized = 'Finalized',
  Expired = 'Expired',
  Cancelled = 'Cancelled',
  Disputed = 'Disputed',
}

/**
 * Worker status
 */
export enum WorkerStatus {
  Active = 'Active',
  Inactive = 'Inactive',
  Slashed = 'Slashed',
  Unbonding = 'Unbonding',
}

/**
 * Model status
 */
export enum ModelStatus {
  Pending = 'Pending',
  Active = 'Active',
  Retired = 'Retired',
  Suspended = 'Suspended',
}

/**
 * Analysis request on-chain account
 */
export interface AnalysisRequest {
  publicKey: PublicKey;
  requester: PublicKey;
  contentHash: Uint8Array;
  storageUri: string;
  requiredModalities: Modality[];
  minWorkers: number;
  feeAmount: bigint;
  deadline: Date;
  commitDeadline: Date;
  createdAt: Date;
  status: RequestStatus;
  commitCount: number;
  revealCount: number;
  nonce: bigint;
}

/**
 * Aggregated analysis result
 */
export interface AnalysisResult {
  requestId: PublicKey;
  verdict: Verdict;
  confidence: number;
  consensusType: 'Unanimous' | 'Majority' | 'Split';
  workerResults: WorkerResult[];
  finalizedAt: Date;
  audit: AuditTrail;
}

/**
 * Individual worker's result
 */
export interface WorkerResult {
  worker: PublicKey;
  model: PublicKey;
  verdict: Verdict;
  confidence: number;
  detections: Detection[];
  revealSlot: bigint;
}

/**
 * Individual detection finding
 */
export interface Detection {
  detectionType: string;
  confidence: number;
  region?: {
    x?: number;
    y?: number;
    width?: number;
    height?: number;
    startMs?: number;
    endMs?: number;
  };
}

/**
 * Audit trail for verification
 */
export interface AuditTrail {
  requestTx: string;
  finalizeTx: string;
  commits: Array<{
    worker: PublicKey;
    commitment: Uint8Array;
    slot: bigint;
  }>;
  reveals: Array<{
    worker: PublicKey;
    slot: bigint;
  }>;
}

/**
 * Content account
 */
export interface ContentAccount {
  publicKey: PublicKey;
  contentHash: Uint8Array;
  mediaType: MediaType;
  creator: PublicKey;
  createdAt: Date;
  storageUri: string;
  claimCount: number;
  analysisCount: bigint;
}

/**
 * Worker account
 */
export interface WorkerAccount {
  publicKey: PublicKey;
  operator: PublicKey;
  stake: bigint;
  reputationScore: number;
  supportedModalities: Modality[];
  tasksCompleted: bigint;
  tasksFailed: bigint;
  lastActiveSlot: bigint;
  status: WorkerStatus;
}

/**
 * Model account
 */
export interface ModelAccount {
  publicKey: PublicKey;
  developer: PublicKey;
  modelId: Uint8Array;
  name: string;
  version: string;
  modalities: Modality[];
  modelUri: string;
  checksum: Uint8Array;
  stake: bigint;
  score: number;
  status: ModelStatus;
  createdAt: Date;
  updatedAt: Date;
  totalUses: bigint;
}

/**
 * Reward account
 */
export interface RewardAccount {
  publicKey: PublicKey;
  claimant: PublicKey;
  pendingAmount: bigint;
  totalClaimed: bigint;
  lastClaimAt: Date | null;
}

/**
 * Parameters for creating an analysis request
 */
export interface CreateRequestParams {
  /** SHA-256 hash of the content */
  contentHash: Uint8Array;
  /** URI where workers can fetch the media */
  storageUri: string;
  /** Required modalities for analysis */
  modalities: Modality[];
  /** Minimum number of workers for consensus */
  minWorkers: number;
  /** Fee amount in DFPN tokens (in base units) */
  feeAmount: bigint;
  /** Deadline as Date or unix timestamp */
  deadline: Date | number;
  /** Optional priority level */
  priority?: 'standard' | 'high' | 'urgent';
  /** Optional webhook URL for result notification */
  callbackUrl?: string;
  /** Optional metadata */
  metadata?: Record<string, string>;
}

/**
 * Parameters for registering content
 */
export interface RegisterContentParams {
  /** SHA-256 hash of the content */
  contentHash: Uint8Array;
  /** Type of media */
  mediaType: MediaType;
  /** Storage URI */
  storageUri: string;
}

/**
 * Parameters for registering a worker
 */
export interface RegisterWorkerParams {
  /** Supported modalities */
  modalities: Modality[];
  /** Initial stake amount */
  stakeAmount: bigint;
}

/**
 * Parameters for registering a model
 */
export interface RegisterModelParams {
  /** Unique model identifier */
  modelId: string;
  /** Human-readable name */
  name: string;
  /** Semantic version */
  version: string;
  /** Supported modalities */
  modalities: Modality[];
  /** URI where model can be downloaded */
  modelUri: string;
  /** SHA-256 checksum of model file */
  checksum: Uint8Array;
  /** Stake amount */
  stakeAmount: bigint;
}

/**
 * Filters for querying requests
 */
export interface RequestFilters {
  status?: RequestStatus;
  modalities?: Modality[];
  minFee?: bigint;
  requester?: PublicKey;
}

/**
 * Filters for querying workers
 */
export interface WorkerFilters {
  modalities?: Modality[];
  minReputation?: number;
  status?: WorkerStatus;
}

/**
 * Filters for querying models
 */
export interface ModelFilters {
  modalities?: Modality[];
  minScore?: number;
  status?: ModelStatus;
  developer?: PublicKey;
}
