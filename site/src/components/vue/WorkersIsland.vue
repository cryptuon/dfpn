<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import { useWorkersStore } from '@dashboard/stores/workers'
import { formatReputation, successRate } from '@dashboard/composables/useFormatters'
import { usePagination } from '@dashboard/composables/usePagination'
import DataTable from '@dashboard/components/common/DataTable.vue'
import StatusBadge from '@dashboard/components/common/StatusBadge.vue'
import AddressDisplay from '@dashboard/components/common/AddressDisplay.vue'
import ModalityTags from '@dashboard/components/common/ModalityTags.vue'
import StakeDisplay from '@dashboard/components/common/StakeDisplay.vue'
import Pagination from '@dashboard/components/common/Pagination.vue'

const store = useWorkersStore()
const pagination = usePagination(20)

const statusFilter = ref('')
const columns = [
  { key: 'operator', label: 'Operator' },
  { key: 'stake', label: 'Stake', sortable: true },
  { key: 'reputation', label: 'Reputation', sortable: true },
  { key: 'modalities', label: 'Modalities' },
  { key: 'status', label: 'Status' },
  { key: 'tasks_completed', label: 'Completed', sortable: true },
  { key: 'tasks_failed', label: 'Failed', sortable: true },
  { key: 'success_rate', label: 'Success Rate' },
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
    <div class="flex items-center justify-between">
      <h2 class="text-2xl font-bold text-white">Workers</h2>
    </div>

    <!-- Filters -->
    <div class="flex gap-4">
      <select
        v-model="statusFilter"
        class="bg-dfpn-surface-light border border-dfpn-border rounded-lg px-3 py-2 text-sm text-gray-200"
      >
        <option value="">All Statuses</option>
        <option value="Active">Active</option>
        <option value="Inactive">Inactive</option>
        <option value="Slashed">Slashed</option>
        <option value="Unbonding">Unbonding</option>
      </select>
    </div>

    <!-- Table -->
    <div class="bg-dfpn-surface border border-dfpn-border rounded-xl overflow-hidden">
      <DataTable
        :columns="columns"
        :rows="store.items"
        :loading="store.loading"
        @row-click="(row) => window.location.href = `/workers/${row.operator}`"
      >
        <template #cell-operator="{ row }">
          <AddressDisplay :address="row.operator" />
        </template>
        <template #cell-stake="{ row }">
          <StakeDisplay :amount="row.stake" />
        </template>
        <template #cell-reputation="{ row }">
          <div class="flex items-center gap-2">
            <span class="text-sm">{{ formatReputation(row.reputation) }}</span>
            <div class="w-16 bg-gray-800 rounded-full h-1.5">
              <div class="bg-dfpn-primary rounded-full h-1.5" :style="{ width: `${row.reputation / 100}%` }"></div>
            </div>
          </div>
        </template>
        <template #cell-modalities="{ row }">
          <ModalityTags :modalities="row.modalities" />
        </template>
        <template #cell-status="{ row }">
          <StatusBadge :status="row.status" />
        </template>
        <template #cell-success_rate="{ row }">
          <span class="text-sm">{{ successRate(row.tasks_completed, row.tasks_failed) }}</span>
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
