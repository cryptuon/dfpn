<script setup lang="ts">
import { onMounted } from 'vue'
import { useWorkersStore } from '../stores/workers'
import { formatReputation, formatNumber, successRate } from '../composables/useFormatters'
import StatusBadge from '../components/common/StatusBadge.vue'
import AddressDisplay from '../components/common/AddressDisplay.vue'
import ModalityTags from '../components/common/ModalityTags.vue'
import StakeDisplay from '../components/common/StakeDisplay.vue'
import StatCard from '../components/common/StatCard.vue'
import LoadingSpinner from '../components/common/LoadingSpinner.vue'
import EmptyState from '../components/common/EmptyState.vue'

const props = defineProps<{ operator: string }>()
const store = useWorkersStore()

onMounted(() => store.loadOne(props.operator))
</script>

<template>
  <div class="space-y-6">
    <router-link to="/workers" class="text-sm text-dfpn-primary-light hover:underline">&larr; Back to Workers</router-link>

    <LoadingSpinner v-if="store.loading" />
    <EmptyState v-else-if="!store.current" message="Worker not found" />

    <template v-else>
      <!-- Header -->
      <div class="bg-dfpn-surface border border-dfpn-border rounded-xl p-6">
        <div class="flex items-center justify-between flex-wrap gap-4">
          <div>
            <h2 class="text-xl font-bold text-white mb-2">Worker Details</h2>
            <AddressDisplay :address="store.current.operator" :chars="8" />
          </div>
          <StatusBadge :status="store.current.status" />
        </div>
      </div>

      <!-- Stats -->
      <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
        <StatCard label="Stake">
          <template #default>
            <p class="mt-2 text-2xl font-bold text-white"><StakeDisplay :amount="store.current.stake" /></p>
          </template>
        </StatCard>
        <StatCard label="Reputation" :value="formatReputation(store.current.reputation)">
          <template #footer>
            <div class="mt-2 w-full bg-gray-800 rounded-full h-2">
              <div class="bg-dfpn-primary rounded-full h-2" :style="{ width: `${store.current.reputation / 100}%` }"></div>
            </div>
          </template>
        </StatCard>
        <StatCard label="Tasks Completed" :value="formatNumber(store.current.tasks_completed)" />
        <StatCard label="Success Rate" :value="successRate(store.current.tasks_completed, store.current.tasks_failed)" />
      </div>

      <!-- Details -->
      <div class="bg-dfpn-surface border border-dfpn-border rounded-xl p-6 space-y-4">
        <h3 class="text-sm font-medium text-gray-400">Details</h3>
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div>
            <p class="text-xs text-gray-500 mb-1">Tasks Failed</p>
            <p class="text-sm text-white">{{ formatNumber(store.current.tasks_failed) }}</p>
          </div>
          <div>
            <p class="text-xs text-gray-500 mb-1">Last Active Slot</p>
            <p class="text-sm text-white font-mono">{{ formatNumber(store.current.last_active_slot) }}</p>
          </div>
          <div class="md:col-span-2">
            <p class="text-xs text-gray-500 mb-1">Supported Modalities</p>
            <ModalityTags :modalities="store.current.modalities" />
          </div>
        </div>
      </div>
    </template>
  </div>
</template>
