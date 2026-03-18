/**
 * Account deserialization for DFPN programs
 *
 * These functions decode on-chain account data into TypeScript types.
 */

import { PublicKey, AccountInfo, Connection } from '@solana/web3.js';
import {
  AnalysisRequest,
  ContentAccount,
  WorkerAccount,
  ModelAccount,
  RewardAccount,
  WorkerResult,
  RequestStatus,
  WorkerStatus,
  ModelStatus,
  MediaType,
  Verdict,
} from './types';
import { parseModalities } from './utils';
import { PROGRAM_IDS, SEEDS } from './constants';

// Account discriminators (Anchor uses first 8 bytes)
const ACCOUNT_DISCRIMINATORS = {
  analysisRequest: Buffer.from([0x41, 0x4e, 0x41, 0x4c, 0x59, 0x53, 0x49, 0x53]), // Placeholder
  content: Buffer.from([0x43, 0x4f, 0x4e, 0x54, 0x45, 0x4e, 0x54, 0x00]),
  worker: Buffer.from([0x57, 0x4f, 0x52, 0x4b, 0x45, 0x52, 0x00, 0x00]),
  model: Buffer.from([0x4d, 0x4f, 0x44, 0x45, 0x4c, 0x00, 0x00, 0x00]),
  reward: Buffer.from([0x52, 0x45, 0x57, 0x41, 0x52, 0x44, 0x00, 0x00]),
  reveal: Buffer.from([0x52, 0x45, 0x56, 0x45, 0x41, 0x4c, 0x00, 0x00]),
  commit: Buffer.from([0x43, 0x4f, 0x4d, 0x4d, 0x49, 0x54, 0x00, 0x00]),
};

/**
 * Deserialize AnalysisRequest account
 */
export function deserializeAnalysisRequest(
  pubkey: PublicKey,
  data: Buffer
): AnalysisRequest {
  let offset = 8; // Skip discriminator

  const requester = new PublicKey(data.subarray(offset, offset + 32));
  offset += 32;

  const contentHash = new Uint8Array(data.subarray(offset, offset + 32));
  offset += 32;

  const storageUri = decodeString(data, offset);
  offset += 4 + storageUri.length;

  const requiredModalitiesBits = data.readUInt8(offset);
  offset += 1;

  const minWorkers = data.readUInt8(offset);
  offset += 1;

  const feeAmount = data.readBigUInt64LE(offset);
  offset += 8;

  const deadline = data.readBigInt64LE(offset);
  offset += 8;

  const commitDeadline = data.readBigInt64LE(offset);
  offset += 8;

  const createdAt = data.readBigInt64LE(offset);
  offset += 8;

  const statusByte = data.readUInt8(offset);
  offset += 1;

  const commitCount = data.readUInt8(offset);
  offset += 1;

  const revealCount = data.readUInt8(offset);
  offset += 1;

  const nonce = data.readBigUInt64LE(offset);
  offset += 8;

  return {
    publicKey: pubkey,
    requester,
    contentHash,
    storageUri,
    requiredModalities: parseModalities(requiredModalitiesBits),
    minWorkers,
    feeAmount,
    deadline: new Date(Number(deadline) * 1000),
    commitDeadline: new Date(Number(commitDeadline) * 1000),
    createdAt: new Date(Number(createdAt) * 1000),
    status: parseRequestStatus(statusByte),
    commitCount,
    revealCount,
    nonce,
  };
}

/**
 * Deserialize Content account
 */
export function deserializeContent(
  pubkey: PublicKey,
  data: Buffer
): ContentAccount {
  let offset = 8; // Skip discriminator

  const contentHash = new Uint8Array(data.subarray(offset, offset + 32));
  offset += 32;

  const mediaTypeByte = data.readUInt8(offset);
  offset += 1;

  const creator = new PublicKey(data.subarray(offset, offset + 32));
  offset += 32;

  const createdAt = data.readBigInt64LE(offset);
  offset += 8;

  const storageUri = decodeString(data, offset);
  offset += 4 + storageUri.length;

  const claimCount = data.readUInt16LE(offset);
  offset += 2;

  const analysisCount = data.readBigUInt64LE(offset);
  offset += 8;

  return {
    publicKey: pubkey,
    contentHash,
    mediaType: parseMediaType(mediaTypeByte),
    creator,
    createdAt: new Date(Number(createdAt) * 1000),
    storageUri,
    claimCount,
    analysisCount,
  };
}

/**
 * Deserialize Worker account
 */
export function deserializeWorker(
  pubkey: PublicKey,
  data: Buffer
): WorkerAccount {
  let offset = 8; // Skip discriminator

  const operator = new PublicKey(data.subarray(offset, offset + 32));
  offset += 32;

  const stake = data.readBigUInt64LE(offset);
  offset += 8;

  const reputationScore = data.readUInt32LE(offset);
  offset += 4;

  const supportedModalitiesBits = data.readUInt8(offset);
  offset += 1;

  const tasksCompleted = data.readBigUInt64LE(offset);
  offset += 8;

  const tasksFailed = data.readBigUInt64LE(offset);
  offset += 8;

  const lastActiveSlot = data.readBigUInt64LE(offset);
  offset += 8;

  const statusByte = data.readUInt8(offset);
  offset += 1;

  return {
    publicKey: pubkey,
    operator,
    stake,
    reputationScore,
    supportedModalities: parseModalities(supportedModalitiesBits),
    tasksCompleted,
    tasksFailed,
    lastActiveSlot,
    status: parseWorkerStatus(statusByte),
  };
}

/**
 * Deserialize Model account
 */
export function deserializeModel(
  pubkey: PublicKey,
  data: Buffer
): ModelAccount {
  let offset = 8; // Skip discriminator

  const developer = new PublicKey(data.subarray(offset, offset + 32));
  offset += 32;

  const modelId = new Uint8Array(data.subarray(offset, offset + 32));
  offset += 32;

  const name = decodeString(data, offset);
  offset += 4 + name.length;

  const version = decodeString(data, offset);
  offset += 4 + version.length;

  const modalitiesBits = data.readUInt8(offset);
  offset += 1;

  const modelUri = decodeString(data, offset);
  offset += 4 + modelUri.length;

  const checksum = new Uint8Array(data.subarray(offset, offset + 32));
  offset += 32;

  const stake = data.readBigUInt64LE(offset);
  offset += 8;

  const score = data.readUInt32LE(offset);
  offset += 4;

  const statusByte = data.readUInt8(offset);
  offset += 1;

  const createdAt = data.readBigInt64LE(offset);
  offset += 8;

  const updatedAt = data.readBigInt64LE(offset);
  offset += 8;

  const totalUses = data.readBigUInt64LE(offset);
  offset += 8;

  return {
    publicKey: pubkey,
    developer,
    modelId,
    name,
    version,
    modalities: parseModalities(modalitiesBits),
    modelUri,
    checksum,
    stake,
    score,
    status: parseModelStatus(statusByte),
    createdAt: new Date(Number(createdAt) * 1000),
    updatedAt: new Date(Number(updatedAt) * 1000),
    totalUses,
  };
}

/**
 * Deserialize Reward account
 */
export function deserializeReward(
  pubkey: PublicKey,
  data: Buffer
): RewardAccount {
  let offset = 8; // Skip discriminator

  const claimant = new PublicKey(data.subarray(offset, offset + 32));
  offset += 32;

  const pendingAmount = data.readBigUInt64LE(offset);
  offset += 8;

  const totalClaimed = data.readBigUInt64LE(offset);
  offset += 8;

  const lastClaimAtValue = data.readBigInt64LE(offset);
  offset += 8;

  return {
    publicKey: pubkey,
    claimant,
    pendingAmount,
    totalClaimed,
    lastClaimAt: lastClaimAtValue > 0 ? new Date(Number(lastClaimAtValue) * 1000) : null,
  };
}

/**
 * Deserialize Reveal account for worker result
 */
export function deserializeReveal(
  data: Buffer
): WorkerResult {
  let offset = 8; // Skip discriminator

  const worker = new PublicKey(data.subarray(offset, offset + 32));
  offset += 32;

  // Skip request pubkey
  offset += 32;

  const model = new PublicKey(data.subarray(offset, offset + 32));
  offset += 32;

  const verdictByte = data.readUInt8(offset);
  offset += 1;

  const confidence = data.readUInt8(offset);
  offset += 1;

  // Skip detections hash
  offset += 32;

  const revealSlot = data.readBigUInt64LE(offset);
  offset += 8;

  return {
    worker,
    model,
    verdict: parseVerdict(verdictByte),
    confidence,
    detections: [], // Detections are stored off-chain
    revealSlot,
  };
}

// ============================================================================
// Fetch helpers
// ============================================================================

/**
 * Fetch and deserialize an analysis request
 */
export async function fetchAnalysisRequest(
  connection: Connection,
  requestId: PublicKey
): Promise<AnalysisRequest | null> {
  const accountInfo = await connection.getAccountInfo(requestId);
  if (!accountInfo) return null;

  return deserializeAnalysisRequest(requestId, Buffer.from(accountInfo.data));
}

/**
 * Fetch and deserialize a worker account
 */
export async function fetchWorker(
  connection: Connection,
  operator: PublicKey
): Promise<WorkerAccount | null> {
  const [workerPDA] = PublicKey.findProgramAddressSync(
    [SEEDS.worker, operator.toBuffer()],
    PROGRAM_IDS.workerRegistry
  );

  const accountInfo = await connection.getAccountInfo(workerPDA);
  if (!accountInfo) return null;

  return deserializeWorker(workerPDA, Buffer.from(accountInfo.data));
}

/**
 * Fetch and deserialize a model account
 */
export async function fetchModel(
  connection: Connection,
  modelId: PublicKey
): Promise<ModelAccount | null> {
  const accountInfo = await connection.getAccountInfo(modelId);
  if (!accountInfo) return null;

  return deserializeModel(modelId, Buffer.from(accountInfo.data));
}

/**
 * Fetch reveals for a request
 */
export async function fetchRevealsForRequest(
  connection: Connection,
  requestId: PublicKey
): Promise<WorkerResult[]> {
  // Use getProgramAccounts with memcmp filter on request pubkey
  const accounts = await connection.getProgramAccounts(
    PROGRAM_IDS.analysisMarketplace,
    {
      filters: [
        { dataSize: 150 }, // Approximate size of Reveal account
        {
          memcmp: {
            offset: 40, // Offset of request pubkey in Reveal account
            bytes: requestId.toBase58(),
          },
        },
      ],
    }
  );

  return accounts.map(({ pubkey, account }) =>
    deserializeReveal(Buffer.from(account.data))
  );
}

/**
 * Fetch all open requests
 */
export async function fetchOpenRequests(
  connection: Connection,
  limit?: number
): Promise<AnalysisRequest[]> {
  const accounts = await connection.getProgramAccounts(
    PROGRAM_IDS.analysisMarketplace,
    {
      filters: [
        { dataSize: 250 }, // Approximate size of AnalysisRequest account
        {
          memcmp: {
            offset: 8 + 32 + 32 + 204 + 1 + 1 + 8 + 8 + 8 + 8, // Offset of status
            bytes: Buffer.from([0]).toString('base64'), // Open = 0
          },
        },
      ],
    }
  );

  const requests = accounts.map(({ pubkey, account }) =>
    deserializeAnalysisRequest(pubkey, Buffer.from(account.data))
  );

  if (limit) {
    return requests.slice(0, limit);
  }

  return requests;
}

/**
 * Fetch all workers
 */
export async function fetchWorkers(
  connection: Connection
): Promise<WorkerAccount[]> {
  const accounts = await connection.getProgramAccounts(
    PROGRAM_IDS.workerRegistry,
    {
      filters: [
        { dataSize: 130 }, // Approximate size of Worker account
      ],
    }
  );

  return accounts.map(({ pubkey, account }) =>
    deserializeWorker(pubkey, Buffer.from(account.data))
  );
}

// ============================================================================
// Parsing utilities
// ============================================================================

function decodeString(data: Buffer, offset: number): string {
  const length = data.readUInt32LE(offset);
  return data.subarray(offset + 4, offset + 4 + length).toString('utf8');
}

function parseRequestStatus(value: number): RequestStatus {
  switch (value) {
    case 0: return RequestStatus.Open;
    case 1: return RequestStatus.CommitClosed;
    case 2: return RequestStatus.Finalized;
    case 3: return RequestStatus.Expired;
    case 4: return RequestStatus.Cancelled;
    case 5: return RequestStatus.Disputed;
    default: return RequestStatus.Open;
  }
}

function parseWorkerStatus(value: number): WorkerStatus {
  switch (value) {
    case 0: return WorkerStatus.Active;
    case 1: return WorkerStatus.Inactive;
    case 2: return WorkerStatus.Slashed;
    case 3: return WorkerStatus.Unbonding;
    default: return WorkerStatus.Active;
  }
}

function parseModelStatus(value: number): ModelStatus {
  switch (value) {
    case 0: return ModelStatus.Pending;
    case 1: return ModelStatus.Active;
    case 2: return ModelStatus.Retired;
    case 3: return ModelStatus.Suspended;
    default: return ModelStatus.Pending;
  }
}

function parseMediaType(value: number): MediaType {
  switch (value) {
    case 0: return MediaType.Image;
    case 1: return MediaType.Video;
    case 2: return MediaType.Audio;
    default: return MediaType.Image;
  }
}

function parseVerdict(value: number): Verdict {
  switch (value) {
    case 0: return Verdict.Authentic;
    case 1: return Verdict.Manipulated;
    case 2: return Verdict.Inconclusive;
    default: return Verdict.Inconclusive;
  }
}
