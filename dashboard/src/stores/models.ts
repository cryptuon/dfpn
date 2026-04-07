import { defineStore } from 'pinia'
import { ref } from 'vue'
import { fetchModels, fetchModel } from '../api/indexer'
import type { IndexerModel, ModelQuery } from '../api/types'

export const useModelsStore = defineStore('models', () => {
  const items = ref<IndexerModel[]>([])
  const current = ref<IndexerModel | null>(null)
  const loading = ref(false)
  const error = ref<string | null>(null)

  async function load(params?: ModelQuery) {
    loading.value = true
    error.value = null
    try {
      items.value = await fetchModels(params)
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to fetch models'
    } finally {
      loading.value = false
    }
  }

  async function loadOne(id: string) {
    loading.value = true
    error.value = null
    try {
      current.value = await fetchModel(id)
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to fetch model'
      current.value = null
    } finally {
      loading.value = false
    }
  }

  return { items, current, loading, error, load, loadOne }
})
