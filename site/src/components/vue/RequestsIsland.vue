<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import { useRequestsStore } from '@dashboard/stores/requests'
import { formatDfpn, formatRelative, truncateAddress } from '@dashboard/composables/useFormatters'
import { usePagination } from '@dashboard/composables/usePagination'
import DataTable from '@dashboard/components/common/DataTable.vue'
import StatusBadge from '@dashboard/components/common/StatusBadge.vue'
import AddressDisplay from '@dashboard/components/common/AddressDisplay.vue'
import ModalityTags from '@dashboard/components/common/ModalityTags.vue'
import Pagination from '@dashboard/components/common/Pagination.vue'

const store = useRequestsStore()
const pagination = usePagination(20)

const statusFilter = ref('')
const searchQuery = ref('')
let searchTimeout: ReturnType<typeof setTimeout> | null = null

const columns = [
  { key: 'id', label: 'ID' },
  { key: 'requester', label: 'Requester' },
  { key: 'modalities', label: 'Modalities' },
  { key: 'status', label: 'Status' },
  { key: 'fee_amount', label: 'Fee', sortable: true },
  { key: 'commit_count', label: 'Commits' },
  { key: 'reveal_count', label: 'Reveals' },
  { key: 'created_at', label: 'Created', sortable: true },
]

async function loadData() {
  if (searchQuery.value.trim()) {
    await store.search(searchQuery.value.trim(), pagination.limit)
  } else {
    await store.load({
      status: statusFilter.value || undefined,
      limit: pagination.limit,
      offset: pagination.offset.value,
    })
  }
  pagination.totalItems.value = store.items.length >= pagination.limit ? (pagination.offset.value + pagination.limit + 1) : (pagination.offset.value + store.items.length)
}

function onSearchInput() {
  if (searchTimeout) clearTimeout(searchTimeout)
  searchTimeout = setTimeout(() => { pagination.reset(); loadData() }, 300)
}

onMounted(loadData)
watch([statusFilter], () => { pagination.reset(); loadData() })
watch([() => pagination.currentPage.value], loadData)
</script>

<template>
  <div class="space-y-6">
    <h2 class="text-2xl font-bold text-white">Analysis Requests</h2>

    <!-- Filters -->
    <div class="flex gap-4 flex-wrap">
      <input
        v-model="searchQuery"
        @input="onSearchInput"
        type="text"
        placeholder="Search by URI..."
        class="bg-dfpn-surface-light border border-dfpn-border rounded-lg px-3 py-2 text-sm text-gray-200 placeholder-gray-500 w-64 focus:outline-none focus:ring-1 focus:ring-dfpn-primary"
      />
      <select
        v-model="statusFilter"
        class="bg-dfpn-surface-light border border-dfpn-border rounded-lg px-3 py-2 text-sm text-gray-200"
      >
        <option value="">All Statuses</option>
        <option value="Open">Open</option>
        <option value="CommitClosed">Commit Closed</option>
        <option value="Finalized">Finalized</option>
        <option value="Expired">Expired</option>
        <option value="Cancelled">Cancelled</option>
        <option value="Disputed">Disputed</option>
      </select>
    </div>

    <!-- Table -->
    <div class="bg-dfpn-surface border border-dfpn-border rounded-xl overflow-hidden">
      <DataTable
        :columns="columns"
        :rows="store.items"
        :loading="store.loading"
        @row-click="(row) => window.location.href = `/requests/${row.id}`"
      >
        <template #cell-id="{ row }">
          <span class="font-mono text-xs text-dfpn-primary-light">{{ truncateAddress(row.id, 4) }}</span>
        </template>
        <template #cell-requester="{ row }">
          <AddressDisplay :address="row.requester" />
        </template>
        <template #cell-modalities="{ row }">
          <ModalityTags :modalities="row.modalities" />
        </template>
        <template #cell-status="{ row }">
          <StatusBadge :status="row.status" />
        </template>
        <template #cell-fee_amount="{ row }">
          <span class="font-mono text-sm">{{ formatDfpn(row.fee_amount) }}</span>
        </template>
        <template #cell-created_at="{ row }">
          <span class="text-xs text-gray-400">{{ formatRelative(row.created_at) }}</span>
        </template>
      </DataTable>
      <Pagination
        :current-page="pagination.currentPage.value"
        :total-pages="pagination.totalPages.value"
        :has-next="pagination.hasNext.value"
        :has-prev="pagination.hasPrev.value"
        @next="pagination.next()"
        @prev="pagination.prev()"
      />
    </div>
  </div>
</template>
