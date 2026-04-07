<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'
import { useHealthStore } from '../stores/health'
import { fetchRequests, fetchWorkers } from '../api/indexer'
import type { IndexerRequest, IndexerWorker } from '../api/types'
import { formatNumber, formatReputation, formatRelative } from '../composables/useFormatters'
import StatusBadge from '../components/common/StatusBadge.vue'
import AddressDisplay from '../components/common/AddressDisplay.vue'
import ModalityTags from '../components/common/ModalityTags.vue'
import LoadingSpinner from '../components/common/LoadingSpinner.vue'
import InfoCard from '../components/learn/InfoCard.vue'
import StatHighlight from '../components/learn/StatHighlight.vue'

const router = useRouter()
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
</script>

<template>
  <div class="space-y-8">
    <!-- Hero -->
    <div class="bg-dfpn-surface border border-dfpn-border rounded-xl p-6 text-center space-y-6">
      <h1 class="text-4xl font-bold text-white">Deepfake Proof Network</h1>
      <p class="text-lg text-gray-400 max-w-2xl mx-auto">
        Decentralized deepfake detection powered by economic incentives on Solana.
      </p>
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
    </div>

    <!-- What is DFPN -->
    <div>
      <h2 class="text-2xl font-bold text-white mb-4">What is DFPN?</h2>
      <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
        <InfoCard
          icon="M21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H3m9 9a9 9 0 01-9-9m9 9c1.657 0 3-4.03 3-9s-1.343-9-3-9m0 18c-1.657 0-3-4.03-3-9s1.343-9 3-9m-9 9a9 9 0 019-9"
          title="Decentralized Detection"
          description="Multiple independent workers analyze each request for reliable results."
        />
        <InfoCard
          icon="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
          title="Earn Rewards"
          description="Workers and model developers earn DFPN tokens for accurate contributions."
        />
        <InfoCard
          icon="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"
          title="On-Chain Transparency"
          description="Every request, result, and reputation score is recorded on Solana."
        />
      </div>
    </div>

    <!-- Get Involved -->
    <div>
      <h2 class="text-2xl font-bold text-white mb-4">Get Involved</h2>
      <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
        <router-link
          to="/learn/participate/workers"
          class="bg-dfpn-surface border border-dfpn-border rounded-xl p-6 hover:border-dfpn-primary/50 transition-colors group block"
        >
          <div class="w-12 h-12 rounded-lg bg-dfpn-primary/20 flex items-center justify-center mb-4">
            <svg class="w-6 h-6 text-dfpn-primary-light" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M5 12h14M5 12a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v4a2 2 0 01-2 2M5 12a2 2 0 00-2 2v4a2 2 0 002 2h14a2 2 0 002-2v-4a2 2 0 00-2-2" />
            </svg>
          </div>
          <h3 class="text-lg font-semibold text-white mb-2">Workers</h3>
          <p class="text-sm text-gray-400">Run detection nodes and earn rewards for analyzing media.</p>
          <span class="inline-block mt-3 text-sm text-dfpn-primary-light group-hover:underline">Learn more &rarr;</span>
        </router-link>

        <router-link
          to="/learn/participate/clients"
          class="bg-dfpn-surface border border-dfpn-border rounded-xl p-6 hover:border-dfpn-primary/50 transition-colors group block"
        >
          <div class="w-12 h-12 rounded-lg bg-emerald-500/20 flex items-center justify-center mb-4">
            <svg class="w-6 h-6 text-emerald-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
            </svg>
          </div>
          <h3 class="text-lg font-semibold text-white mb-2">Clients</h3>
          <p class="text-sm text-gray-400">Submit media for deepfake analysis with consensus results.</p>
          <span class="inline-block mt-3 text-sm text-dfpn-primary-light group-hover:underline">Learn more &rarr;</span>
        </router-link>

        <router-link
          to="/learn/participate/model-developers"
          class="bg-dfpn-surface border border-dfpn-border rounded-xl p-6 hover:border-dfpn-primary/50 transition-colors group block"
        >
          <div class="w-12 h-12 rounded-lg bg-blue-500/20 flex items-center justify-center mb-4">
            <svg class="w-6 h-6 text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4" />
            </svg>
          </div>
          <h3 class="text-lg font-semibold text-white mb-2">Model Developers</h3>
          <p class="text-sm text-gray-400">Register detection models and earn from every request that uses them.</p>
          <span class="inline-block mt-3 text-sm text-dfpn-primary-light group-hover:underline">Learn more &rarr;</span>
        </router-link>
      </div>
    </div>

    <!-- Live Network Activity -->
    <div>
      <h2 class="text-2xl font-bold text-white mb-4">Live Network Activity</h2>
      <LoadingSpinner v-if="loading" />
      <div v-else class="grid grid-cols-1 lg:grid-cols-2 gap-6">
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
    </div>

    <!-- Get Started Banner -->
    <div class="bg-dfpn-surface border border-dfpn-border rounded-xl p-6">
      <h3 class="text-lg font-semibold text-white mb-4">New to DFPN?</h3>
      <div class="flex flex-wrap gap-4">
        <router-link
          to="/learn"
          class="px-5 py-2.5 bg-dfpn-primary text-white text-sm font-medium rounded-lg hover:bg-dfpn-primary/80 transition-colors"
        >
          Learn what DFPN is
        </router-link>
        <router-link
          to="/learn/how-it-works"
          class="px-5 py-2.5 bg-dfpn-surface border border-dfpn-border text-gray-400 text-sm font-medium rounded-lg hover:border-dfpn-primary/30 transition-colors"
        >
          See how it works
        </router-link>
        <router-link
          to="/learn/faq"
          class="px-5 py-2.5 bg-dfpn-surface border border-dfpn-border text-gray-400 text-sm font-medium rounded-lg hover:border-dfpn-primary/30 transition-colors"
        >
          Read the FAQ
        </router-link>
      </div>
    </div>
  </div>
</template>
