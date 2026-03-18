import { createHash } from 'crypto';
import { PublicKey } from '@solana/web3.js';
import { Modality } from './types';
import { MODALITY_FLAGS } from './constants';

/**
 * Compute SHA-256 hash of content
 */
export function computeContentHash(data: Buffer | Uint8Array): Uint8Array {
  const hash = createHash('sha256');
  hash.update(data);
  return new Uint8Array(hash.digest());
}

/**
 * Compute commitment for commit-reveal protocol
 *
 * commitment = SHA256(verdict || confidence || detections_hash || salt || worker || request)
 */
export function computeCommitment(
  verdict: number,
  confidence: number,
  detectionsHash: Uint8Array,
  salt: Uint8Array,
  worker: PublicKey,
  request: PublicKey
): Uint8Array {
  const hash = createHash('sha256');

  // Result bytes: verdict (1) + confidence (1) + detections_hash (32)
  hash.update(Buffer.from([verdict, confidence]));
  hash.update(detectionsHash);
  hash.update(salt);
  hash.update(worker.toBuffer());
  hash.update(request.toBuffer());

  return new Uint8Array(hash.digest());
}

/**
 * Generate random salt for commitment
 */
export function generateSalt(): Uint8Array {
  const salt = new Uint8Array(16);
  crypto.getRandomValues(salt);
  return salt;
}

/**
 * Parse modalities from bit field
 */
export function parseModalities(bits: number): Modality[] {
  const modalities: Modality[] = [];

  if (bits & MODALITY_FLAGS.ImageAuthenticity) {
    modalities.push(Modality.ImageAuthenticity);
  }
  if (bits & MODALITY_FLAGS.VideoAuthenticity) {
    modalities.push(Modality.VideoAuthenticity);
  }
  if (bits & MODALITY_FLAGS.AudioAuthenticity) {
    modalities.push(Modality.AudioAuthenticity);
  }
  if (bits & MODALITY_FLAGS.FaceManipulation) {
    modalities.push(Modality.FaceManipulation);
  }
  if (bits & MODALITY_FLAGS.VoiceCloning) {
    modalities.push(Modality.VoiceCloning);
  }
  if (bits & MODALITY_FLAGS.GeneratedContent) {
    modalities.push(Modality.GeneratedContent);
  }

  return modalities;
}

/**
 * Convert modalities to bit field
 */
export function modalitiesToBits(modalities: Modality[]): number {
  let bits = 0;

  for (const modality of modalities) {
    bits |= MODALITY_FLAGS[modality];
  }

  return bits;
}

/**
 * Convert modality enum to display string
 */
export function modalityToString(modality: Modality): string {
  switch (modality) {
    case Modality.ImageAuthenticity:
      return 'Image Authenticity';
    case Modality.VideoAuthenticity:
      return 'Video Authenticity';
    case Modality.AudioAuthenticity:
      return 'Audio Authenticity';
    case Modality.FaceManipulation:
      return 'Face Manipulation';
    case Modality.VoiceCloning:
      return 'Voice Cloning';
    case Modality.GeneratedContent:
      return 'AI Generated Content';
    default:
      return 'Unknown';
  }
}

/**
 * Format DFPN amount from base units to display
 */
export function formatDfpn(amount: bigint, decimals: number = 9): string {
  const divisor = BigInt(10 ** decimals);
  const whole = amount / divisor;
  const fraction = amount % divisor;

  if (fraction === BigInt(0)) {
    return whole.toString();
  }

  const fractionStr = fraction.toString().padStart(decimals, '0').replace(/0+$/, '');
  return `${whole}.${fractionStr}`;
}

/**
 * Parse DFPN amount from display to base units
 */
export function parseDfpn(amount: string, decimals: number = 9): bigint {
  const parts = amount.split('.');
  const whole = BigInt(parts[0] || '0');

  if (parts.length === 1) {
    return whole * BigInt(10 ** decimals);
  }

  const fractionStr = parts[1].padEnd(decimals, '0').slice(0, decimals);
  const fraction = BigInt(fractionStr);

  return whole * BigInt(10 ** decimals) + fraction;
}

/**
 * Derive PDA for content account
 */
export function deriveContentPDA(
  contentHash: Uint8Array,
  programId: PublicKey
): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [Buffer.from('content'), Buffer.from(contentHash)],
    programId
  );
}

/**
 * Derive PDA for request account
 */
export function deriveRequestPDA(
  contentHash: Uint8Array,
  nonce: bigint,
  programId: PublicKey
): [PublicKey, number] {
  const nonceBuffer = Buffer.alloc(8);
  nonceBuffer.writeBigUInt64LE(nonce);

  return PublicKey.findProgramAddressSync(
    [Buffer.from('request'), Buffer.from(contentHash), nonceBuffer],
    programId
  );
}

/**
 * Derive PDA for worker account
 */
export function deriveWorkerPDA(
  operator: PublicKey,
  programId: PublicKey
): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [Buffer.from('worker'), operator.toBuffer()],
    programId
  );
}

/**
 * Derive PDA for model account
 */
export function deriveModelPDA(
  developer: PublicKey,
  modelId: Uint8Array,
  programId: PublicKey
): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [Buffer.from('model'), developer.toBuffer(), Buffer.from(modelId)],
    programId
  );
}

/**
 * Derive PDA for reward account
 */
export function deriveRewardPDA(
  claimant: PublicKey,
  programId: PublicKey
): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [Buffer.from('reward'), claimant.toBuffer()],
    programId
  );
}
