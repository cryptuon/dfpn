/**
 * Instruction builders for DFPN programs
 *
 * These functions build Solana instructions for interacting with DFPN programs.
 * They use manual instruction encoding to avoid IDL dependencies.
 */

import {
  PublicKey,
  TransactionInstruction,
  SystemProgram,
} from '@solana/web3.js';
import { TOKEN_PROGRAM_ID } from '@solana/spl-token';

import { PROGRAM_IDS, SEEDS } from './constants';
import { Modality, MediaType } from './types';
import { modalitiesToBits } from './utils';

// Discriminators (first 8 bytes of SHA256 hash of "global:<instruction_name>")
const DISCRIMINATORS = {
  // Analysis Marketplace
  createRequest: Buffer.from([0x18, 0x1e, 0xc8, 0x28, 0x05, 0x1c, 0x07, 0x77]),
  commitResult: Buffer.from([0xd3, 0x2b, 0x3f, 0x8a, 0x6c, 0x1e, 0x9e, 0x7a]),
  revealResult: Buffer.from([0x8f, 0x7e, 0x4a, 0x2d, 0x1c, 0x3b, 0x9e, 0x5f]),
  finalizeRequest: Buffer.from([0xa2, 0x5e, 0x7c, 0x4f, 0x3d, 0x8a, 0x1b, 0x6e]),
  cancelRequest: Buffer.from([0xc4, 0x3f, 0x2e, 0x1d, 0x5a, 0x7b, 0x9c, 0x8f]),

  // Content Registry
  registerContent: Buffer.from([0x12, 0x3a, 0x5b, 0x7c, 0x9d, 0xe1, 0xf2, 0x34]),

  // Worker Registry
  registerWorker: Buffer.from([0x23, 0x4b, 0x6c, 0x8d, 0xae, 0xf2, 0x13, 0x45]),
  updateWorker: Buffer.from([0x34, 0x5c, 0x7d, 0x9e, 0xbf, 0x13, 0x24, 0x56]),
  requestUnstake: Buffer.from([0x45, 0x6d, 0x8e, 0xaf, 0xc1, 0x24, 0x35, 0x67]),
  withdrawStake: Buffer.from([0x56, 0x7e, 0x9f, 0xb1, 0xd2, 0x35, 0x46, 0x78]),

  // Model Registry
  registerModel: Buffer.from([0x67, 0x8f, 0xa1, 0xc2, 0xe3, 0x46, 0x57, 0x89]),
  updateModel: Buffer.from([0x78, 0xa1, 0xb2, 0xd3, 0xf4, 0x57, 0x68, 0x9a]),

  // Rewards
  claimRewards: Buffer.from([0x89, 0xb2, 0xc3, 0xe4, 0x15, 0x68, 0x79, 0xab]),
};

/**
 * Create analysis request instruction
 */
export function createRequestInstruction(params: {
  requester: PublicKey;
  request: PublicKey;
  feeSource: PublicKey;
  feeVault: PublicKey;
  contentHash: Uint8Array;
  storageUri: string;
  modalities: Modality[];
  minWorkers: number;
  feeAmount: bigint;
  deadline: number;
  nonce: bigint;
}): TransactionInstruction {
  const {
    requester,
    request,
    feeSource,
    feeVault,
    contentHash,
    storageUri,
    modalities,
    minWorkers,
    feeAmount,
    deadline,
    nonce,
  } = params;

  // Encode instruction data
  const storageUriBuffer = Buffer.from(storageUri);
  const modalityBits = modalitiesToBits(modalities);

  const data = Buffer.concat([
    DISCRIMINATORS.createRequest,
    Buffer.from(contentHash),
    encodeString(storageUri),
    Buffer.from([modalityBits]),
    Buffer.from([minWorkers]),
    encodeU64(feeAmount),
    encodeI64(BigInt(deadline)),
    encodeU64(nonce),
  ]);

  return new TransactionInstruction({
    keys: [
      { pubkey: requester, isSigner: true, isWritable: true },
      { pubkey: request, isSigner: false, isWritable: true },
      { pubkey: feeSource, isSigner: false, isWritable: true },
      { pubkey: feeVault, isSigner: false, isWritable: true },
      { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    ],
    programId: PROGRAM_IDS.analysisMarketplace,
    data,
  });
}

/**
 * Commit result instruction
 */
export function commitResultInstruction(params: {
  worker: PublicKey;
  workerAccount: PublicKey;
  request: PublicKey;
  commit: PublicKey;
  commitment: Uint8Array;
}): TransactionInstruction {
  const { worker, workerAccount, request, commit, commitment } = params;

  const data = Buffer.concat([
    DISCRIMINATORS.commitResult,
    Buffer.from(commitment),
  ]);

  return new TransactionInstruction({
    keys: [
      { pubkey: worker, isSigner: true, isWritable: true },
      { pubkey: workerAccount, isSigner: false, isWritable: false },
      { pubkey: request, isSigner: false, isWritable: true },
      { pubkey: commit, isSigner: false, isWritable: true },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    ],
    programId: PROGRAM_IDS.analysisMarketplace,
    data,
  });
}

/**
 * Reveal result instruction
 */
export function revealResultInstruction(params: {
  worker: PublicKey;
  workerAccount: PublicKey;
  request: PublicKey;
  commit: PublicKey;
  reveal: PublicKey;
  model: PublicKey;
  verdict: number;
  confidence: number;
  detectionsHash: Uint8Array;
  salt: Uint8Array;
}): TransactionInstruction {
  const {
    worker,
    workerAccount,
    request,
    commit,
    reveal,
    model,
    verdict,
    confidence,
    detectionsHash,
    salt,
  } = params;

  const data = Buffer.concat([
    DISCRIMINATORS.revealResult,
    Buffer.from([verdict]),
    Buffer.from([confidence]),
    Buffer.from(detectionsHash),
    Buffer.from(salt),
  ]);

  return new TransactionInstruction({
    keys: [
      { pubkey: worker, isSigner: true, isWritable: true },
      { pubkey: workerAccount, isSigner: false, isWritable: true },
      { pubkey: request, isSigner: false, isWritable: true },
      { pubkey: commit, isSigner: false, isWritable: true },
      { pubkey: reveal, isSigner: false, isWritable: true },
      { pubkey: model, isSigner: false, isWritable: true },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    ],
    programId: PROGRAM_IDS.analysisMarketplace,
    data,
  });
}

/**
 * Finalize request instruction
 */
export function finalizeRequestInstruction(params: {
  authority: PublicKey;
  request: PublicKey;
  feeVault: PublicKey;
  treasury: PublicKey;
  reveals: PublicKey[];
}): TransactionInstruction {
  const { authority, request, feeVault, treasury, reveals } = params;

  const data = DISCRIMINATORS.finalizeRequest;

  const keys = [
    { pubkey: authority, isSigner: true, isWritable: true },
    { pubkey: request, isSigner: false, isWritable: true },
    { pubkey: feeVault, isSigner: false, isWritable: true },
    { pubkey: treasury, isSigner: false, isWritable: true },
    { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
    // Add reveal accounts as remaining accounts
    ...reveals.map((reveal) => ({
      pubkey: reveal,
      isSigner: false,
      isWritable: false,
    })),
  ];

  return new TransactionInstruction({
    keys,
    programId: PROGRAM_IDS.analysisMarketplace,
    data,
  });
}

/**
 * Cancel request instruction
 */
export function cancelRequestInstruction(params: {
  requester: PublicKey;
  request: PublicKey;
  feeVault: PublicKey;
  feeDestination: PublicKey;
}): TransactionInstruction {
  const { requester, request, feeVault, feeDestination } = params;

  const data = DISCRIMINATORS.cancelRequest;

  return new TransactionInstruction({
    keys: [
      { pubkey: requester, isSigner: true, isWritable: true },
      { pubkey: request, isSigner: false, isWritable: true },
      { pubkey: feeVault, isSigner: false, isWritable: true },
      { pubkey: feeDestination, isSigner: false, isWritable: true },
      { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
    ],
    programId: PROGRAM_IDS.analysisMarketplace,
    data,
  });
}

/**
 * Register content instruction
 */
export function registerContentInstruction(params: {
  creator: PublicKey;
  content: PublicKey;
  contentHash: Uint8Array;
  mediaType: MediaType;
  storageUri: string;
}): TransactionInstruction {
  const { creator, content, contentHash, mediaType, storageUri } = params;

  const mediaTypeValue = mediaType === MediaType.Image ? 0 : mediaType === MediaType.Video ? 1 : 2;

  const data = Buffer.concat([
    DISCRIMINATORS.registerContent,
    Buffer.from(contentHash),
    Buffer.from([mediaTypeValue]),
    encodeString(storageUri),
  ]);

  return new TransactionInstruction({
    keys: [
      { pubkey: creator, isSigner: true, isWritable: true },
      { pubkey: content, isSigner: false, isWritable: true },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    ],
    programId: PROGRAM_IDS.contentRegistry,
    data,
  });
}

/**
 * Register worker instruction
 */
export function registerWorkerInstruction(params: {
  operator: PublicKey;
  worker: PublicKey;
  stakeSource: PublicKey;
  stakeVault: PublicKey;
  modalities: Modality[];
  stakeAmount: bigint;
}): TransactionInstruction {
  const { operator, worker, stakeSource, stakeVault, modalities, stakeAmount } = params;

  const modalityBits = modalitiesToBits(modalities);

  const data = Buffer.concat([
    DISCRIMINATORS.registerWorker,
    Buffer.from([modalityBits]),
    encodeU64(stakeAmount),
  ]);

  return new TransactionInstruction({
    keys: [
      { pubkey: operator, isSigner: true, isWritable: true },
      { pubkey: worker, isSigner: false, isWritable: true },
      { pubkey: stakeSource, isSigner: false, isWritable: true },
      { pubkey: stakeVault, isSigner: false, isWritable: true },
      { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    ],
    programId: PROGRAM_IDS.workerRegistry,
    data,
  });
}

/**
 * Claim rewards instruction
 */
export function claimRewardsInstruction(params: {
  claimant: PublicKey;
  rewardAccount: PublicKey;
  tokenDestination: PublicKey;
  treasury: PublicKey;
}): TransactionInstruction {
  const { claimant, rewardAccount, tokenDestination, treasury } = params;

  const data = DISCRIMINATORS.claimRewards;

  return new TransactionInstruction({
    keys: [
      { pubkey: claimant, isSigner: true, isWritable: true },
      { pubkey: rewardAccount, isSigner: false, isWritable: true },
      { pubkey: tokenDestination, isSigner: false, isWritable: true },
      { pubkey: treasury, isSigner: false, isWritable: true },
      { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
    ],
    programId: PROGRAM_IDS.rewards,
    data,
  });
}

// ============================================================================
// Encoding utilities
// ============================================================================

function encodeU64(value: bigint): Buffer {
  const buffer = Buffer.alloc(8);
  buffer.writeBigUInt64LE(value);
  return buffer;
}

function encodeI64(value: bigint): Buffer {
  const buffer = Buffer.alloc(8);
  buffer.writeBigInt64LE(value);
  return buffer;
}

function encodeString(str: string): Buffer {
  const strBuffer = Buffer.from(str);
  const lenBuffer = Buffer.alloc(4);
  lenBuffer.writeUInt32LE(strBuffer.length);
  return Buffer.concat([lenBuffer, strBuffer]);
}
