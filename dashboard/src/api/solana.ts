import { Buffer } from 'buffer'
import { Connection, PublicKey } from '@solana/web3.js'

const REWARDS_PROGRAM_ID = new PublicKey('4uQeVj5tqViQh7yWWGStvkEG1Zmhx6uasJtWCJziofM')
const REWARD_SEED = Buffer.from('reward')

export interface RewardAccount {
  claimant: string
  pendingAmount: number
  totalClaimed: number
  lastClaimAt: number | null
}

function deriveRewardPDA(claimant: PublicKey): PublicKey {
  const [pda] = PublicKey.findProgramAddressSync(
    [REWARD_SEED, claimant.toBuffer()],
    REWARDS_PROGRAM_ID,
  )
  return pda
}

export async function fetchRewardAccount(
  connection: Connection,
  walletPubkey: PublicKey,
): Promise<RewardAccount | null> {
  const pda = deriveRewardPDA(walletPubkey)
  const accountInfo = await connection.getAccountInfo(pda)
  if (!accountInfo || !accountInfo.data) return null

  const data = accountInfo.data
  // Anchor discriminator: 8 bytes
  // claimant: 32 bytes
  // pending_amount: 8 bytes (u64 LE)
  // total_claimed: 8 bytes (u64 LE)
  // last_claim_at: 1 byte option + 8 bytes i64 LE
  const offset = 8
  const claimant = new PublicKey(data.subarray(offset, offset + 32)).toBase58()
  const pendingAmount = Number(data.readBigUInt64LE(offset + 32))
  const totalClaimed = Number(data.readBigUInt64LE(offset + 40))
  const hasLastClaim = data[offset + 48] === 1
  const lastClaimAt = hasLastClaim ? Number(data.readBigInt64LE(offset + 49)) : null

  return { claimant, pendingAmount, totalClaimed, lastClaimAt }
}
