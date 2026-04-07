<script setup lang="ts">
import { onMounted } from 'vue'
import { useRequestsStore } from '../stores/requests'
import { formatDfpn, formatTimestamp, formatRelative } from '../composables/useFormatters'
import StatusBadge from '../components/common/StatusBadge.vue'
import AddressDisplay from '../components/common/AddressDisplay.vue'
import ModalityTags from '../components/common/ModalityTags.vue'
import StatCard from '../components/common/StatCard.vue'
import LoadingSpinner from '../components/common/LoadingSpinner.vue'
import EmptyState from '../components/common/EmptyState.vue'

const props = defineProps<{ id: string }>()
const store = useRequestsStore()

onMounted(() => store.loadOne(props.id))
</script>

<template>
  <div class="space-y-6">
    <router-link to="/requests" class="text-sm text-dfpn-primary-light hover:underline">&larr; Back to Requests</router-link>

    <LoadingSpinner v-if="store.loading" />
    <EmptyState v-else-if="!store.current" message="Request not found" />

    <template v-else>
      <div class="bg-dfpn-surface border border-dfpn-border rounded-xl p-6">
        <div class="flex items-center justify-between flex-wrap gap-4">
          <div>
            <h2 class="text-xl font-bold text-white mb-2">Analysis Request</h2>
            <p class="text-sm font-mono text-gray-400">{{ store.current.id }}</p>
          </div>
          <StatusBadge :status="store.current.status" />
        </div>
      </div>

      <!-- Progress -->
      <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
        <StatCard label="Fee" :value="`${formatDfpn(store.current.fee_amount)} DFPN`" />
        <StatCard label="Commits" :value="store.current.commit_count" />
        <StatCard label="Reveals" :value="store.current.reveal_count" />
        <StatCard label="Created" :value="formatRelative(store.current.created_at)" />
      </div>

      <!-- Details -->
      <div class="bg-dfpn-surface border border-dfpn-border rounded-xl p-6 space-y-4">
        <h3 class="text-sm font-medium text-gray-400">Details</h3>
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div>
            <p class="text-xs text-gray-500 mb-1">Requester</p>
            <AddressDisplay :address="store.current.requester" :chars="8" />
          </div>
          <div>
            <p class="text-xs text-gray-500 mb-1">Content Hash</p>
            <p class="text-sm font-mono text-white break-all">{{ store.current.content_hash }}</p>
          </div>
          <div>
            <p class="text-xs text-gray-500 mb-1">Storage URI</p>
            <p class="text-sm font-mono text-white break-all">{{ store.current.storage_uri }}</p>
          </div>
          <div>
            <p class="text-xs text-gray-500 mb-1">Deadline</p>
            <p class="text-sm text-white">{{ formatTimestamp(store.current.deadline) }}</p>
          </div>
          <div>
            <p class="text-xs text-gray-500 mb-1">Commit Deadline</p>
            <p class="text-sm text-white">{{ formatTimestamp(store.current.commit_deadline) }}</p>
          </div>
          <div class="md:col-span-2">
            <p class="text-xs text-gray-500 mb-1">Required Modalities</p>
            <ModalityTags :modalities="store.current.modalities" />
          </div>
        </div>
      </div>
    </template>
  </div>
</template>
