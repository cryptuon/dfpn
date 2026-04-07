export interface HealthResponse {
  status: string
  request_count: number
  worker_count: number
  model_count: number
}

export interface IndexerRequest {
  id: string
  requester: string
  content_hash: string
  storage_uri: string
  modalities: number
  status: string
  fee_amount: number
  deadline: number
  commit_deadline: number
  created_at: number
  commit_count: number
  reveal_count: number
}

export interface IndexerWorker {
  id: string
  operator: string
  stake: number
  reputation: number
  modalities: number
  status: string
  tasks_completed: number
  tasks_failed: number
  last_active_slot: number
}

export interface IndexerModel {
  id: string
  developer: string
  name: string
  version: string
  modalities: number
  model_uri: string
  status: string
  score: number
  total_uses: number
  created_at: number
}

export interface RequestQuery {
  status?: string
  requester?: string
  modalities?: number
  limit?: number
  offset?: number
}

export interface WorkerQuery {
  status?: string
  operator?: string
  min_reputation?: number
  modalities?: number
  limit?: number
  offset?: number
}

export interface ModelQuery {
  status?: string
  developer?: string
  modalities?: number
  limit?: number
  offset?: number
}
