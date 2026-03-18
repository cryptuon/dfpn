import {
  Connection,
  PublicKey,
  Transaction,
  TransactionSignature,
  Commitment,
} from '@solana/web3.js';
import { getAssociatedTokenAddress, TOKEN_PROGRAM_ID } from '@solana/spl-token';

import {
  AnalysisRequest,
  AnalysisResult,
  ContentAccount,
  CreateRequestParams,
  ModelAccount,
  ModelFilters,
  Modality,
  RegisterContentParams,
  RequestFilters,
  RequestStatus,
  RewardAccount,
  Verdict,
  WorkerAccount,
  WorkerFilters,
  WorkerResult,
  AuditTrail,
} from './types';
import { PROGRAM_IDS, SEEDS } from './constants';
import { deriveRequestPDA, deriveContentPDA, deriveWorkerPDA, deriveRewardPDA } from './utils';
import {
  createRequestInstruction,
  commitResultInstruction,
  revealResultInstruction,
  cancelRequestInstruction,
  registerContentInstruction,
  claimRewardsInstruction,
} from './instructions';
import {
  fetchAnalysisRequest,
  fetchWorker,
  fetchModel,
  fetchRevealsForRequest,
  fetchOpenRequests,
  fetchWorkers,
  deserializeAnalysisRequest,
  deserializeContent,
  deserializeWorker,
  deserializeModel,
  deserializeReward,
} from './accounts';

/**
 * Client options
 */
export interface ClientOptions {
  /** Commitment level for queries */
  commitment?: Commitment;
  /** Custom program IDs */
  programIds?: Partial<typeof PROGRAM_IDS>;
  /** Indexer URL for fast queries */
  indexerUrl?: string;
}

/**
 * Wallet interface for signing
 */
export interface Wallet {
  publicKey: PublicKey;
  signTransaction<T extends Transaction>(tx: T): Promise<T>;
  signAllTransactions<T extends Transaction>(txs: T[]): Promise<T[]>;
}

/**
 * Options for waiting on results
 */
export interface WaitOptions {
  /** Timeout in milliseconds */
  timeout?: number;
  /** Polling interval in milliseconds */
  pollInterval?: number;
  /** Callback on status update */
  onStatusUpdate?: (status: RequestStatus) => void;
}

/**
 * DFPN Client for interacting with the network
 */
export class DFPNClient {
  private connection: Connection;
  private wallet: Wallet;
  private options: ClientOptions;
  private programIds: typeof PROGRAM_IDS;

  constructor(
    connection: Connection,
    wallet: Wallet,
    options: ClientOptions = {}
  ) {
    this.connection = connection;
    this.wallet = wallet;
    this.options = options;
    this.programIds = {
      ...PROGRAM_IDS,
      ...options.programIds,
    };
  }

  // =========================================================================
  // Request Operations
  // =========================================================================

  /**
   * Create an analysis request
   */
  async createRequest(params: CreateRequestParams): Promise<{
    requestId: PublicKey;
    signature: TransactionSignature;
  }> {
    const nonce = BigInt(Date.now());
    const [requestPDA] = deriveRequestPDA(
      params.contentHash,
      nonce,
      this.programIds.analysisMarketplace
    );

    // Calculate deadline
    const deadline = typeof params.deadline === 'number'
      ? params.deadline
      : Math.floor(params.deadline.getTime() / 1000);

    // Get fee source token account
    const dfpnMint = await this.getDfpnMint();
    const feeSource = await getAssociatedTokenAddress(
      dfpnMint,
      this.wallet.publicKey
    );

    // Derive fee vault PDA
    const [feeVault] = PublicKey.findProgramAddressSync(
      [SEEDS.feeVault, requestPDA.toBuffer()],
      this.programIds.analysisMarketplace
    );

    // Build transaction
    const tx = new Transaction();

    tx.add(
      createRequestInstruction({
        requester: this.wallet.publicKey,
        request: requestPDA,
        feeSource,
        feeVault,
        contentHash: params.contentHash,
        storageUri: params.storageUri,
        modalities: params.modalities,
        minWorkers: params.minWorkers,
        feeAmount: params.feeAmount,
        deadline,
        nonce,
      })
    );

    // Sign and send
    const signature = await this.sendTransaction(tx);

    return {
      requestId: requestPDA,
      signature,
    };
  }

  /**
   * Get request by ID
   */
  async getRequest(requestId: PublicKey): Promise<AnalysisRequest | null> {
    return fetchAnalysisRequest(this.connection, requestId);
  }

  /**
   * Get request status
   */
  async getRequestStatus(requestId: PublicKey): Promise<RequestStatus> {
    const request = await this.getRequest(requestId);
    if (!request) {
      throw new Error('Request not found');
    }
    return request.status;
  }

  /**
   * Wait for request result
   */
  async waitForResult(
    requestId: PublicKey,
    options: WaitOptions = {}
  ): Promise<AnalysisResult> {
    const {
      timeout = 600000, // 10 minutes
      pollInterval = 5000, // 5 seconds
      onStatusUpdate,
    } = options;

    const startTime = Date.now();

    while (Date.now() - startTime < timeout) {
      const status = await this.getRequestStatus(requestId);

      if (onStatusUpdate) {
        onStatusUpdate(status);
      }

      if (status === RequestStatus.Finalized) {
        return this.getResult(requestId);
      }

      if (status === RequestStatus.Expired || status === RequestStatus.Cancelled) {
        throw new Error(`Request ${status.toLowerCase()}`);
      }

      await this.sleep(pollInterval);
    }

    throw new Error('Timeout waiting for result');
  }

  /**
   * Get aggregated result for a finalized request
   */
  async getResult(requestId: PublicKey): Promise<AnalysisResult> {
    // Fetch the request to verify it's finalized
    const request = await this.getRequest(requestId);
    if (!request) {
      throw new Error('Request not found');
    }

    if (request.status !== RequestStatus.Finalized) {
      throw new Error(`Request is not finalized: ${request.status}`);
    }

    // Fetch all reveals
    const workerResults = await fetchRevealsForRequest(this.connection, requestId);

    if (workerResults.length === 0) {
      throw new Error('No reveals found for request');
    }

    // Calculate consensus
    const verdictCounts = new Map<Verdict, number>();
    let totalConfidence = 0;

    for (const result of workerResults) {
      verdictCounts.set(
        result.verdict,
        (verdictCounts.get(result.verdict) || 0) + 1
      );
      totalConfidence += result.confidence;
    }

    // Determine final verdict (majority wins)
    let finalVerdict = Verdict.Inconclusive;
    let maxCount = 0;

    for (const [verdict, count] of verdictCounts) {
      if (count > maxCount) {
        maxCount = count;
        finalVerdict = verdict;
      }
    }

    // Determine consensus type
    const consensusType: 'Unanimous' | 'Majority' | 'Split' =
      workerResults.length === 1
        ? 'Unanimous'
        : maxCount === workerResults.length
        ? 'Unanimous'
        : maxCount > workerResults.length / 2
        ? 'Majority'
        : 'Split';

    // Build audit trail
    const audit: AuditTrail = {
      requestTx: '', // Would need to fetch from transaction history
      finalizeTx: '',
      commits: [],
      reveals: workerResults.map((r) => ({
        worker: r.worker,
        slot: r.revealSlot,
      })),
    };

    return {
      requestId,
      verdict: finalVerdict,
      confidence: Math.round(totalConfidence / workerResults.length),
      consensusType,
      workerResults,
      finalizedAt: new Date(),
      audit,
    };
  }

  /**
   * Cancel a request (before any commits)
   */
  async cancelRequest(requestId: PublicKey): Promise<TransactionSignature> {
    const dfpnMint = await this.getDfpnMint();
    const feeDestination = await getAssociatedTokenAddress(
      dfpnMint,
      this.wallet.publicKey
    );

    const [feeVault] = PublicKey.findProgramAddressSync(
      [SEEDS.feeVault, requestId.toBuffer()],
      this.programIds.analysisMarketplace
    );

    const tx = new Transaction();
    tx.add(
      cancelRequestInstruction({
        requester: this.wallet.publicKey,
        request: requestId,
        feeVault,
        feeDestination,
      })
    );

    return this.sendTransaction(tx);
  }

  /**
   * List open requests matching filters
   */
  async listOpenRequests(filters?: RequestFilters): Promise<AnalysisRequest[]> {
    // Use indexer if available
    if (this.options.indexerUrl) {
      return this.queryIndexer('/requests', { status: 'Open', ...filters });
    }

    // Fall back to on-chain query
    let requests = await fetchOpenRequests(this.connection);

    // Apply filters
    if (filters) {
      if (filters.modalities && filters.modalities.length > 0) {
        requests = requests.filter((r) =>
          filters.modalities!.some((m) => r.requiredModalities.includes(m))
        );
      }

      if (filters.minFee !== undefined) {
        requests = requests.filter((r) => r.feeAmount >= filters.minFee!);
      }

      if (filters.requester) {
        requests = requests.filter((r) =>
          r.requester.equals(filters.requester!)
        );
      }
    }

    return requests;
  }

  // =========================================================================
  // Content Operations
  // =========================================================================

  /**
   * Register content
   */
  async registerContent(params: RegisterContentParams): Promise<{
    contentId: PublicKey;
    signature: TransactionSignature;
  }> {
    const [contentPDA] = deriveContentPDA(
      params.contentHash,
      this.programIds.contentRegistry
    );

    const tx = new Transaction();
    tx.add(
      registerContentInstruction({
        creator: this.wallet.publicKey,
        content: contentPDA,
        contentHash: params.contentHash,
        mediaType: params.mediaType,
        storageUri: params.storageUri,
      })
    );

    const signature = await this.sendTransaction(tx);

    return {
      contentId: contentPDA,
      signature,
    };
  }

  /**
   * Get content by hash
   */
  async getContent(contentHash: Uint8Array): Promise<ContentAccount | null> {
    const [contentPDA] = deriveContentPDA(
      contentHash,
      this.programIds.contentRegistry
    );

    const accountInfo = await this.connection.getAccountInfo(contentPDA);
    if (!accountInfo) {
      return null;
    }

    return deserializeContent(contentPDA, Buffer.from(accountInfo.data));
  }

  // =========================================================================
  // Worker Operations
  // =========================================================================

  /**
   * List workers
   */
  async listWorkers(filters?: WorkerFilters): Promise<WorkerAccount[]> {
    // Use indexer if available
    if (this.options.indexerUrl) {
      return this.queryIndexer('/workers', filters as unknown as Record<string, unknown>);
    }

    // Fall back to on-chain query
    let workers = await fetchWorkers(this.connection);

    // Apply filters
    if (filters) {
      if (filters.modalities && filters.modalities.length > 0) {
        workers = workers.filter((w) =>
          filters.modalities!.some((m) => w.supportedModalities.includes(m))
        );
      }

      if (filters.minReputation !== undefined) {
        workers = workers.filter(
          (w) => w.reputationScore >= filters.minReputation!
        );
      }

      if (filters.status) {
        workers = workers.filter((w) => w.status === filters.status);
      }
    }

    return workers;
  }

  /**
   * Get worker by operator
   */
  async getWorker(operator: PublicKey): Promise<WorkerAccount | null> {
    return fetchWorker(this.connection, operator);
  }

  // =========================================================================
  // Model Operations
  // =========================================================================

  /**
   * List models
   */
  async listModels(filters?: ModelFilters): Promise<ModelAccount[]> {
    // Use indexer if available
    if (this.options.indexerUrl) {
      return this.queryIndexer('/models', filters as unknown as Record<string, unknown>);
    }

    // For on-chain query, we'd need to implement getProgramAccounts
    // This is a placeholder
    return [];
  }

  /**
   * Get model by ID
   */
  async getModel(modelId: PublicKey): Promise<ModelAccount | null> {
    return fetchModel(this.connection, modelId);
  }

  // =========================================================================
  // Reward Operations
  // =========================================================================

  /**
   * Get reward account for current wallet
   */
  async getRewardAccount(): Promise<RewardAccount | null> {
    const [rewardPDA] = deriveRewardPDA(
      this.wallet.publicKey,
      this.programIds.rewards
    );

    const accountInfo = await this.connection.getAccountInfo(rewardPDA);
    if (!accountInfo) {
      return null;
    }

    return deserializeReward(rewardPDA, Buffer.from(accountInfo.data));
  }

  /**
   * Claim accumulated rewards
   */
  async claimRewards(): Promise<{
    amount: bigint;
    signature: TransactionSignature;
  }> {
    // Get reward account to check pending amount
    const rewardAccount = await this.getRewardAccount();
    if (!rewardAccount) {
      throw new Error('No reward account found');
    }

    if (rewardAccount.pendingAmount === BigInt(0)) {
      throw new Error('No pending rewards to claim');
    }

    const [rewardPDA] = deriveRewardPDA(
      this.wallet.publicKey,
      this.programIds.rewards
    );

    const dfpnMint = await this.getDfpnMint();
    const tokenDestination = await getAssociatedTokenAddress(
      dfpnMint,
      this.wallet.publicKey
    );

    const [treasury] = PublicKey.findProgramAddressSync(
      [SEEDS.treasury],
      this.programIds.rewards
    );

    const tx = new Transaction();
    tx.add(
      claimRewardsInstruction({
        claimant: this.wallet.publicKey,
        rewardAccount: rewardPDA,
        tokenDestination,
        treasury,
      })
    );

    const signature = await this.sendTransaction(tx);

    return {
      amount: rewardAccount.pendingAmount,
      signature,
    };
  }

  // =========================================================================
  // Utility Methods
  // =========================================================================

  /**
   * Get DFPN token mint
   */
  private async getDfpnMint(): Promise<PublicKey> {
    // In production, this would be read from a config or the treasury account
    // For now, return a placeholder
    const [treasury] = PublicKey.findProgramAddressSync(
      [SEEDS.treasury],
      this.programIds.rewards
    );

    // TODO: Fetch treasury account and read mint field
    return new PublicKey('DFPNmint11111111111111111111111111111111111');
  }

  /**
   * Query the indexer API
   */
  private async queryIndexer<T>(
    endpoint: string,
    params?: Record<string, unknown>
  ): Promise<T[]> {
    if (!this.options.indexerUrl) {
      throw new Error('Indexer URL not configured');
    }

    const url = new URL(endpoint, this.options.indexerUrl);

    if (params) {
      for (const [key, value] of Object.entries(params)) {
        if (value !== undefined) {
          url.searchParams.set(key, String(value));
        }
      }
    }

    const response = await fetch(url.toString());

    if (!response.ok) {
      throw new Error(`Indexer request failed: ${response.statusText}`);
    }

    return response.json() as Promise<T[]>;
  }

  /**
   * Send and confirm transaction
   */
  private async sendTransaction(tx: Transaction): Promise<TransactionSignature> {
    tx.recentBlockhash = (
      await this.connection.getLatestBlockhash()
    ).blockhash;
    tx.feePayer = this.wallet.publicKey;

    const signed = await this.wallet.signTransaction(tx);
    const signature = await this.connection.sendRawTransaction(signed.serialize());

    await this.connection.confirmTransaction(signature, this.options.commitment);

    return signature;
  }

  /**
   * Sleep utility
   */
  private sleep(ms: number): Promise<void> {
    return new Promise((resolve) => setTimeout(resolve, ms));
  }

  /**
   * Get connection
   */
  getConnection(): Connection {
    return this.connection;
  }

  /**
   * Get wallet public key
   */
  getWalletPublicKey(): PublicKey {
    return this.wallet.publicKey;
  }
}
