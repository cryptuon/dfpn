import { defineStore } from 'pinia'
import { ref, watch } from 'vue'
import { useWalletStore } from './wallet'
import { fetchWorker, fetchRequests } from '../api/indexer'
import { fetchRewardAccount } from '../api/solana'
import type { IndexerWorker, IndexerRequest } from '../api/types'
import type { RewardAccount } from '../api/solana'

export const useDashboardStore = defineStore('dashboard', () => {
  const walletStore = useWalletStore()

  const worker = ref<IndexerWorker | null>(null)
  const rewards = ref<RewardAccount | null>(null)
  const myRequests = ref<IndexerRequest[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  async function load() {
    if (!walletStore.address) return

    loading.value = true
    error.value = null
    try {
      const results = await Promise.allSettled([
        fetchWorker(walletStore.address).catch(() => null),
        walletStore.publicKey
          ? fetchRewardAccount(walletStore.connection, walletStore.publicKey)
          : null,
        fetchRequests({ requester: walletStore.address, limit: 50 }),
      ])

      worker.value = results[0].status === 'fulfilled' ? results[0].value as IndexerWorker | null : null
      rewards.value = results[1].status === 'fulfilled' ? results[1].value as RewardAccount | null : null
      myRequests.value = results[2].status === 'fulfilled' ? results[2].value as IndexerRequest[] : []
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to load dashboard'
    } finally {
      loading.value = false
    }
  }

  watch(() => walletStore.address, (addr) => {
    if (addr) load()
    else {
      worker.value = null
      rewards.value = null
      myRequests.value = []
    }
  })

  return { worker, rewards, myRequests, loading, error, load }
})
