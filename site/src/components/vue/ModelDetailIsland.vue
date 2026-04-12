<script setup lang="ts">
import { onMounted } from 'vue'
import { useModelsStore } from '@dashboard/stores/models'
import { formatNumber, formatTimestamp } from '@dashboard/composables/useFormatters'
import StatusBadge from '@dashboard/components/common/StatusBadge.vue'
import AddressDisplay from '@dashboard/components/common/AddressDisplay.vue'
import ModalityTags from '@dashboard/components/common/ModalityTags.vue'
import StatCard from '@dashboard/components/common/StatCard.vue'
import LoadingSpinner from '@dashboard/components/common/LoadingSpinner.vue'
import EmptyState from '@dashboard/components/common/EmptyState.vue'

const props = defineProps<{ id: string }>()
const store = useModelsStore()

onMounted(() => store.loadOne(props.id))
</script>

<template>
  <div class="space-y-6">
    <a href="/models" class="text-sm text-dfpn-primary-light hover:underline">&larr; Back to Models</a>

    <LoadingSpinner v-if="store.loading" />
    <EmptyState v-else-if="!store.current" message="Model not found" />

    <template v-else>
      <div class="bg-dfpn-surface border border-dfpn-border rounded-xl p-6">
        <div class="flex items-center justify-between flex-wrap gap-4">
          <div>
            <h2 class="text-xl font-bold text-white">{{ store.current.name }}</h2>
            <p class="text-sm text-gray-400 mt-1">
              Version <span class="font-mono">{{ store.current.version }}</span>
              &middot; by <AddressDisplay :address="store.current.developer" :chars="4" />
            </p>
          </div>
          <StatusBadge :status="store.current.status" />
        </div>
      </div>

      <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
        <StatCard label="Score" :value="`${(store.current.score / 100).toFixed(1)}%`" />
        <StatCard label="Total Uses" :value="formatNumber(store.current.total_uses)" />
        <StatCard label="Created" :value="formatTimestamp(store.current.created_at)" />
      </div>

      <div class="bg-dfpn-surface border border-dfpn-border rounded-xl p-6 space-y-4">
        <h3 class="text-sm font-medium text-gray-400">Details</h3>
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div>
            <p class="text-xs text-gray-500 mb-1">Model URI</p>
            <p class="text-sm text-white font-mono break-all">{{ store.current.model_uri }}</p>
          </div>
          <div>
            <p class="text-xs text-gray-500 mb-1">Account ID</p>
            <p class="text-sm"><AddressDisplay :address="store.current.id" :chars="6" /></p>
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
