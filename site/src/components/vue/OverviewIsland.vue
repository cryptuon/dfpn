<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useHealthStore } from '@dashboard/stores/health'
import { fetchRequests, fetchWorkers } from '@dashboard/api/indexer'
import type { IndexerRequest, IndexerWorker } from '@dashboard/api/types'
import { formatNumber, formatReputation, formatRelative } from '@dashboard/composables/useFormatters'
import StatusBadge from '@dashboard/components/common/StatusBadge.vue'
import AddressDisplay from '@dashboard/components/common/AddressDisplay.vue'
import ModalityTags from '@dashboard/components/common/ModalityTags.vue'
import LoadingSpinner from '@dashboard/components/common/LoadingSpinner.vue'
import StatHighlight from '@dashboard/components/learn/StatHighlight.vue'

const healthStore = useHealthStore()
const recentRequests = ref<IndexerRequest[]>([])
const topWorkers = ref<IndexerWorker[]>([])
const loading = ref(true)

onMounted(async () => {
  try {
    await Promise.all([
      healthStore.load(),
      fetchRequests({ limit: 10 }).then(r => recentRequests.value = r),
      fetchWorkers({ status: 'Active', limit: 5 }).then(w => topWorkers.value = w),
    ])
  } finally {
    loading.value = false
  }
})

function navigateTo(path: string) {
  window.location.href = path
}
</script>

<template>
  <!-- Live Stats -->
  <div class="grid grid-cols-1 md:grid-cols-3 gap-6 max-w-xl mx-auto pt-2">
    <StatHighlight
      :value="loading ? '...' : formatNumber(healthStore.data?.request_count ?? 0)"
      label="Total Requests"
    />
    <StatHighlight
      :value="loading ? '...' : formatNumber(healthStore.data?.worker_count ?? 0)"
      label="Active Workers"
    />
    <StatHighlight
      :value="loading ? '...' : formatNumber(healthStore.data?.model_count ?? 0)"
      label="Active Models"
    />
  </div>

  <!-- Live Network Activity -->
  <div class="mt-8">
    <h2 class="text-2xl font-bold text-white mb-4">Live Network Activity</h2>
    <LoadingSpinner v-if="loading" />
    <div v-else class="grid grid-cols-1 lg:grid-cols-2 gap-6">
      <!-- Recent Requests -->
      <div class="bg-dfpn-surface border border-dfpn-border rounded-xl overflow-hidden">
        <div class="px-6 py-4 border-b border-dfpn-border flex items-center justify-between">
          <h3 class="text-sm font-medium text-gray-400">Recent Requests</h3>
          <a href="/requests" class="text-xs text-dfpn-primary-light hover:underline">View all</a>
        </div>
        <div class="divide-y divide-dfpn-border">
          <div
            v-for="req in recentRequests"
            :key="req.id"
            class="px-6 py-3 hover:bg-dfpn-surface-light/50 cursor-pointer transition-colors"
            @click="navigateTo(`/requests/${req.id}`)"
          >
            <div class="flex items-center justify-between">
              <div class="flex items-center gap-3">
                <AddressDisplay :address="req.requester" :chars="3" />
                <ModalityTags :modalities="req.modalities" />
              </div>
              <div class="flex items-center gap-3">
                <StatusBadge :status="req.status" />
                <span class="text-xs text-gray-500">{{ formatRelative(req.created_at) }}</span>
              </div>
            </div>
          </div>
          <div v-if="recentRequests.length === 0" class="px-6 py-8 text-center text-gray-500 text-sm">
            No requests yet
          </div>
        </div>
      </div>

      <!-- Top Workers -->
      <div class="bg-dfpn-surface border border-dfpn-border rounded-xl overflow-hidden">
        <div class="px-6 py-4 border-b border-dfpn-border flex items-center justify-between">
          <h3 class="text-sm font-medium text-gray-400">Top Workers</h3>
          <a href="/workers" class="text-xs text-dfpn-primary-light hover:underline">View all</a>
        </div>
        <div class="divide-y divide-dfpn-border">
          <div
            v-for="w in topWorkers"
            :key="w.id"
            class="px-6 py-3 hover:bg-dfpn-surface-light/50 cursor-pointer transition-colors"
            @click="navigateTo(`/workers/${w.operator}`)"
          >
            <div class="flex items-center justify-between">
              <AddressDisplay :address="w.operator" :chars="4" />
              <div class="flex items-center gap-4">
                <div class="text-right">
                  <div class="text-sm font-medium text-white">{{ formatReputation(w.reputation) }}</div>
                  <div class="text-xs text-gray-500">reputation</div>
                </div>
                <div class="w-24 bg-gray-800 rounded-full h-2">
                  <div
                    class="bg-dfpn-primary rounded-full h-2 transition-all"
                    :style="{ width: `${w.reputation / 100}%` }"
                  ></div>
                </div>
              </div>
            </div>
          </div>
          <div v-if="topWorkers.length === 0" class="px-6 py-8 text-center text-gray-500 text-sm">
            No active workers
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
