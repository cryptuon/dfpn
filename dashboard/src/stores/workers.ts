import { defineStore } from 'pinia'
import { ref } from 'vue'
import { fetchWorkers, fetchWorker } from '../api/indexer'
import type { IndexerWorker, WorkerQuery } from '../api/types'

export const useWorkersStore = defineStore('workers', () => {
  const items = ref<IndexerWorker[]>([])
  const current = ref<IndexerWorker | null>(null)
  const loading = ref(false)
  const error = ref<string | null>(null)

  async function load(params?: WorkerQuery) {
    loading.value = true
    error.value = null
    try {
      items.value = await fetchWorkers(params)
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to fetch workers'
    } finally {
      loading.value = false
    }
  }

  async function loadOne(operator: string) {
    loading.value = true
    error.value = null
    try {
      current.value = await fetchWorker(operator)
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to fetch worker'
      current.value = null
    } finally {
      loading.value = false
    }
  }

  return { items, current, loading, error, load, loadOne }
})
