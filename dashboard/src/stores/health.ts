import { defineStore } from 'pinia'
import { ref } from 'vue'
import { fetchHealth } from '../api/indexer'
import type { HealthResponse } from '../api/types'

export const useHealthStore = defineStore('health', () => {
  const data = ref<HealthResponse | null>(null)
  const loading = ref(false)
  const error = ref<string | null>(null)

  async function load() {
    loading.value = true
    error.value = null
    try {
      data.value = await fetchHealth()
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to fetch health'
    } finally {
      loading.value = false
    }
  }

  return { data, loading, error, load }
})
