<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import { useRouter } from 'vue-router'
import { useModelsStore } from '../stores/models'
import { formatNumber } from '../composables/useFormatters'
import { usePagination } from '../composables/usePagination'
import DataTable from '../components/common/DataTable.vue'
import StatusBadge from '../components/common/StatusBadge.vue'
import AddressDisplay from '../components/common/AddressDisplay.vue'
import ModalityTags from '../components/common/ModalityTags.vue'
import Pagination from '../components/common/Pagination.vue'

const router = useRouter()
const store = useModelsStore()
const pagination = usePagination(20)
const statusFilter = ref('')

const columns = [
  { key: 'name', label: 'Name' },
  { key: 'version', label: 'Version' },
  { key: 'developer', label: 'Developer' },
  { key: 'modalities', label: 'Modalities' },
  { key: 'status', label: 'Status' },
  { key: 'score', label: 'Score', sortable: true },
  { key: 'total_uses', label: 'Total Uses', sortable: true },
]

async function loadData() {
  await store.load({
    status: statusFilter.value || undefined,
    limit: pagination.limit,
    offset: pagination.offset.value,
  })
  pagination.totalItems.value = store.items.length >= pagination.limit ? (pagination.offset.value + pagination.limit + 1) : (pagination.offset.value + store.items.length)
}

onMounted(loadData)
watch([statusFilter], () => { pagination.reset(); loadData() })
watch([() => pagination.currentPage.value], loadData)
</script>

<template>
  <div class="space-y-6">
    <h2 class="text-2xl font-bold text-white">Models</h2>

    <!-- Filters -->
    <div class="flex gap-4">
      <select
        v-model="statusFilter"
        class="bg-dfpn-surface-light border border-dfpn-border rounded-lg px-3 py-2 text-sm text-gray-200"
      >
        <option value="">All Statuses</option>
        <option value="Active">Active</option>
        <option value="Pending">Pending</option>
        <option value="Retired">Retired</option>
        <option value="Suspended">Suspended</option>
      </select>
    </div>

    <!-- Table -->
    <div class="bg-dfpn-surface border border-dfpn-border rounded-xl overflow-hidden">
      <DataTable
        :columns="columns"
        :rows="store.items"
        :loading="store.loading"
        @row-click="(row) => router.push(`/models/${row.id}`)"
      >
        <template #cell-name="{ row }">
          <span class="font-medium text-white">{{ row.name }}</span>
        </template>
        <template #cell-version="{ row }">
          <span class="font-mono text-xs text-gray-400">{{ row.version }}</span>
        </template>
        <template #cell-developer="{ row }">
          <AddressDisplay :address="row.developer" />
        </template>
        <template #cell-modalities="{ row }">
          <ModalityTags :modalities="row.modalities" />
        </template>
        <template #cell-status="{ row }">
          <StatusBadge :status="row.status" />
        </template>
        <template #cell-score="{ row }">
          <span class="text-sm">{{ (row.score / 100).toFixed(1) }}%</span>
        </template>
        <template #cell-total_uses="{ row }">
          <span class="text-sm">{{ formatNumber(row.total_uses) }}</span>
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
