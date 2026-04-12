<script setup lang="ts">
import { onMounted } from 'vue'
import { useWalletStore } from '@dashboard/stores/wallet'
import { useDashboardStore } from '@dashboard/stores/dashboard'
import { formatDfpn, formatReputation, formatRelative, successRate, truncateAddress } from '@dashboard/composables/useFormatters'
import StatCard from '@dashboard/components/common/StatCard.vue'
import StatusBadge from '@dashboard/components/common/StatusBadge.vue'
import ModalityTags from '@dashboard/components/common/ModalityTags.vue'
import StakeDisplay from '@dashboard/components/common/StakeDisplay.vue'
import LoadingSpinner from '@dashboard/components/common/LoadingSpinner.vue'
import EmptyState from '@dashboard/components/common/EmptyState.vue'

const walletStore = useWalletStore()
const dashboardStore = useDashboardStore()

onMounted(() => {
  if (walletStore.connected) {
    dashboardStore.load()
  }
})
</script>

<template>
  <div class="space-y-6">
    <h2 class="text-2xl font-bold text-white">My Dashboard</h2>

    <!-- Not connected -->
    <div v-if="!walletStore.connected" class="bg-dfpn-surface border border-dfpn-border rounded-xl p-12 text-center">
      <svg class="w-16 h-16 mx-auto text-gray-600 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" />
      </svg>
      <h3 class="text-lg font-medium text-gray-300 mb-2">Wallet Not Connected</h3>
      <p class="text-gray-500 mb-6">Connect your Solana wallet to view your personal stats, rewards, and submitted requests.</p>
      <button
        @click="walletStore.connect()"
        class="px-6 py-2.5 bg-dfpn-primary hover:bg-dfpn-primary/80 text-white rounded-lg font-medium transition-colors"
      >
        Connect Wallet
      </button>
    </div>

    <!-- Connected -->
    <template v-else>
      <LoadingSpinner v-if="dashboardStore.loading" />

      <template v-else>
        <!-- Worker Section -->
        <div class="bg-dfpn-surface border border-dfpn-border rounded-xl p-6">
          <h3 class="text-sm font-medium text-gray-400 mb-4">My Worker</h3>

          <template v-if="dashboardStore.worker">
            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
              <StatCard label="Status">
                <template #default>
                  <div class="mt-2"><StatusBadge :status="dashboardStore.worker.status" /></div>
                </template>
              </StatCard>
              <StatCard label="Stake">
                <template #default>
                  <p class="mt-2 text-2xl font-bold text-white"><StakeDisplay :amount="dashboardStore.worker.stake" /></p>
                </template>
              </StatCard>
              <StatCard label="Reputation" :value="formatReputation(dashboardStore.worker.reputation)">
                <template #footer>
                  <div class="mt-2 w-full bg-gray-800 rounded-full h-2">
                    <div class="bg-dfpn-primary rounded-full h-2" :style="{ width: `${dashboardStore.worker.reputation / 100}%` }"></div>
                  </div>
                </template>
              </StatCard>
              <StatCard label="Success Rate" :value="successRate(dashboardStore.worker.tasks_completed, dashboardStore.worker.tasks_failed)" />
            </div>
            <div class="mt-4">
              <p class="text-xs text-gray-500 mb-1">Supported Modalities</p>
              <ModalityTags :modalities="dashboardStore.worker.modalities" />
            </div>
          </template>

          <EmptyState v-else message="You are not a registered worker on this network" />
        </div>

        <!-- Rewards Section -->
        <div class="bg-dfpn-surface border border-dfpn-border rounded-xl p-6">
          <h3 class="text-sm font-medium text-gray-400 mb-4">My Rewards</h3>

          <template v-if="dashboardStore.rewards">
            <div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
              <StatCard label="Pending Rewards" :value="`${formatDfpn(dashboardStore.rewards.pendingAmount)} DFPN`" />
              <StatCard label="Total Claimed" :value="`${formatDfpn(dashboardStore.rewards.totalClaimed)} DFPN`" />
              <StatCard label="Last Claim" :value="dashboardStore.rewards.lastClaimAt ? formatRelative(dashboardStore.rewards.lastClaimAt) : 'Never'" />
            </div>
          </template>

          <EmptyState v-else message="No reward account found" />
        </div>

        <!-- My Requests Section -->
        <div class="bg-dfpn-surface border border-dfpn-border rounded-xl overflow-hidden">
          <div class="px-6 py-4 border-b border-dfpn-border">
            <h3 class="text-sm font-medium text-gray-400">My Submitted Requests</h3>
          </div>

          <template v-if="dashboardStore.myRequests.length > 0">
            <div class="divide-y divide-dfpn-border">
              <div
                v-for="req in dashboardStore.myRequests"
                :key="req.id"
                class="px-6 py-3 hover:bg-dfpn-surface-light/50 cursor-pointer transition-colors flex items-center justify-between"
                @click="window.location.href = `/requests/${req.id}`"
              >
                <div class="flex items-center gap-3">
                  <span class="font-mono text-xs text-dfpn-primary-light">{{ truncateAddress(req.id, 4) }}</span>
                  <ModalityTags :modalities="req.modalities" />
                </div>
                <div class="flex items-center gap-3">
                  <StatusBadge :status="req.status" />
                  <span class="text-xs text-gray-500">{{ formatRelative(req.created_at) }}</span>
                </div>
              </div>
            </div>
          </template>

          <EmptyState v-else message="You haven't submitted any analysis requests" />
        </div>
      </template>
    </template>
  </div>
</template>
