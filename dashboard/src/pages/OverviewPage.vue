<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'
import { useHealthStore } from '../stores/health'
import { fetchRequests, fetchWorkers } from '../api/indexer'
import type { IndexerRequest, IndexerWorker } from '../api/types'
import { formatNumber, formatReputation, formatRelative } from '../composables/useFormatters'
import StatCard from '../components/common/StatCard.vue'
import StatusBadge from '../components/common/StatusBadge.vue'
import AddressDisplay from '../components/common/AddressDisplay.vue'
import ModalityTags from '../components/common/ModalityTags.vue'
import LoadingSpinner from '../components/common/LoadingSpinner.vue'
import RequestsOverTimeChart from '../components/charts/RequestsOverTimeChart.vue'
import ModalityDistributionChart from '../components/charts/ModalityDistributionChart.vue'
import WorkerReputationChart from '../components/charts/WorkerReputationChart.vue'

const router = useRouter()
const healthStore = useHealthStore()
const recentRequests = ref<IndexerRequest[]>([])
const topWorkers = ref<IndexerWorker[]>([])
const allRequests = ref<IndexerRequest[]>([])
const allWorkers = ref<IndexerWorker[]>([])
const loading = ref(true)

onMounted(async () => {
  try {
    await Promise.all([
      healthStore.load(),
      fetchRequests({ limit: 10 }).then(r => recentRequests.value = r),
      fetchWorkers({ status: 'Active', limit: 5 }).then(w => topWorkers.value = w),
      fetchRequests({ limit: 500 }).then(r => allRequests.value = r),
      fetchWorkers({ limit: 500 }).then(w => allWorkers.value = w),
    ])
  } finally {
    loading.value = false
  }
})
</script>

<template>
  <div class="space-y-6">
    <h2 class="text-2xl font-bold text-white">Network Overview</h2>

    <LoadingSpinner v-if="loading" />

    <template v-else>
      <!-- Stat cards -->
      <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
        <StatCard label="Total Requests" :value="formatNumber(healthStore.data?.request_count ?? 0)" icon="true">
          <template #icon>
            <svg class="w-5 h-5 text-dfpn-primary-light" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
            </svg>
          </template>
        </StatCard>
        <StatCard label="Active Workers" :value="formatNumber(healthStore.data?.worker_count ?? 0)" icon="true" color="bg-emerald-500/20">
          <template #icon>
            <svg class="w-5 h-5 text-emerald-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0z" />
            </svg>
          </template>
        </StatCard>
        <StatCard label="Registered Models" :value="formatNumber(healthStore.data?.model_count ?? 0)" icon="true" color="bg-blue-500/20">
          <template #icon>
            <svg class="w-5 h-5 text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
            </svg>
          </template>
        </StatCard>
        <StatCard label="Network Status" :value="healthStore.data?.status === 'ok' ? 'Healthy' : 'Degraded'" icon="true" color="bg-yellow-500/20">
          <template #icon>
            <svg class="w-5 h-5 text-yellow-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M13 10V3L4 14h7v7l9-11h-7z" />
            </svg>
          </template>
        </StatCard>
      </div>

      <!-- Charts -->
      <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <div class="bg-dfpn-surface border border-dfpn-border rounded-xl p-6">
          <h3 class="text-sm font-medium text-gray-400 mb-4">Requests Over Time</h3>
          <RequestsOverTimeChart :requests="allRequests" />
        </div>
        <div class="bg-dfpn-surface border border-dfpn-border rounded-xl p-6">
          <h3 class="text-sm font-medium text-gray-400 mb-4">Modality Distribution</h3>
          <ModalityDistributionChart :requests="allRequests" />
        </div>
      </div>

      <!-- Recent requests + Top workers -->
      <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <!-- Recent Requests -->
        <div class="bg-dfpn-surface border border-dfpn-border rounded-xl overflow-hidden">
          <div class="px-6 py-4 border-b border-dfpn-border flex items-center justify-between">
            <h3 class="text-sm font-medium text-gray-400">Recent Requests</h3>
            <router-link to="/requests" class="text-xs text-dfpn-primary-light hover:underline">View all</router-link>
          </div>
          <div class="divide-y divide-dfpn-border">
            <div
              v-for="req in recentRequests"
              :key="req.id"
              class="px-6 py-3 hover:bg-dfpn-surface-light/50 cursor-pointer transition-colors"
              @click="router.push(`/requests/${req.id}`)"
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
            <router-link to="/workers" class="text-xs text-dfpn-primary-light hover:underline">View all</router-link>
          </div>
          <div class="divide-y divide-dfpn-border">
            <div
              v-for="w in topWorkers"
              :key="w.id"
              class="px-6 py-3 hover:bg-dfpn-surface-light/50 cursor-pointer transition-colors"
              @click="router.push(`/workers/${w.operator}`)"
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

      <!-- Worker Reputation Histogram -->
      <div class="bg-dfpn-surface border border-dfpn-border rounded-xl p-6">
        <h3 class="text-sm font-medium text-gray-400 mb-4">Worker Reputation Distribution</h3>
        <WorkerReputationChart :workers="allWorkers" />
      </div>
    </template>
  </div>
</template>
