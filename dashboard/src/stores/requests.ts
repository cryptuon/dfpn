import { defineStore } from 'pinia'
import { ref } from 'vue'
import { fetchRequests, fetchRequest, searchRequests } from '../api/indexer'
import type { IndexerRequest, RequestQuery } from '../api/types'

export const useRequestsStore = defineStore('requests', () => {
  const items = ref<IndexerRequest[]>([])
  const current = ref<IndexerRequest | null>(null)
  const loading = ref(false)
  const error = ref<string | null>(null)

  async function load(params?: RequestQuery) {
    loading.value = true
    error.value = null
    try {
      items.value = await fetchRequests(params)
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to fetch requests'
    } finally {
      loading.value = false
    }
  }

  async function loadOne(id: string) {
    loading.value = true
    error.value = null
    try {
      current.value = await fetchRequest(id)
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to fetch request'
      current.value = null
    } finally {
      loading.value = false
    }
  }

  async function search(q: string, limit?: number) {
    loading.value = true
    error.value = null
    try {
      items.value = await searchRequests(q, limit)
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Search failed'
    } finally {
      loading.value = false
    }
  }

  return { items, current, loading, error, load, loadOne, search }
})
