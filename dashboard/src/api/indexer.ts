import type { HealthResponse, IndexerRequest, IndexerWorker, IndexerModel, RequestQuery, WorkerQuery, ModelQuery } from './types'

const API_BASE = '/api'

function buildUrl(path: string, params?: Record<string, string | number | undefined>): string {
  const url = new URL(`${API_BASE}${path}`, window.location.origin)
  if (params) {
    for (const [key, value] of Object.entries(params)) {
      if (value !== undefined && value !== '') {
        url.searchParams.set(key, String(value))
      }
    }
  }
  return url.toString()
}

async function fetchJson<T>(url: string): Promise<T> {
  const res = await fetch(url)
  if (!res.ok) {
    throw new Error(`API error: ${res.status} ${res.statusText}`)
  }
  return res.json()
}

export async function fetchHealth(): Promise<HealthResponse> {
  return fetchJson<HealthResponse>(buildUrl('/health'))
}

export async function fetchRequests(params?: RequestQuery): Promise<IndexerRequest[]> {
  return fetchJson<IndexerRequest[]>(buildUrl('/requests', params as Record<string, string | number | undefined>))
}

export async function fetchRequest(id: string): Promise<IndexerRequest> {
  return fetchJson<IndexerRequest>(buildUrl(`/requests/${id}`))
}

export async function searchRequests(q: string, limit?: number): Promise<IndexerRequest[]> {
  return fetchJson<IndexerRequest[]>(buildUrl('/requests/search', { q, limit }))
}

export async function fetchWorkers(params?: WorkerQuery): Promise<IndexerWorker[]> {
  return fetchJson<IndexerWorker[]>(buildUrl('/workers', params as Record<string, string | number | undefined>))
}

export async function fetchWorker(operator: string): Promise<IndexerWorker> {
  return fetchJson<IndexerWorker>(buildUrl(`/workers/${operator}`))
}

export async function fetchModels(params?: ModelQuery): Promise<IndexerModel[]> {
  return fetchJson<IndexerModel[]>(buildUrl('/models', params as Record<string, string | number | undefined>))
}

export async function fetchModel(id: string): Promise<IndexerModel> {
  return fetchJson<IndexerModel>(buildUrl(`/models/${id}`))
}

export async function searchModels(q: string, limit?: number): Promise<IndexerModel[]> {
  return fetchJson<IndexerModel[]>(buildUrl('/models/search', { q, limit }))
}
