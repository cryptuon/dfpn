<script setup lang="ts">
import { ref } from 'vue'

defineProps<{
  columns: { key: string; label: string; sortable?: boolean; class?: string }[]
  rows: Record<string, any>[] // eslint-disable-line @typescript-eslint/no-explicit-any
  loading?: boolean
}>()

defineEmits<{
  rowClick: [row: Record<string, any>] // eslint-disable-line @typescript-eslint/no-explicit-any
}>()

const sortKey = ref<string | null>(null)
const sortAsc = ref(true)

function toggleSort(key: string) {
  if (sortKey.value === key) {
    sortAsc.value = !sortAsc.value
  } else {
    sortKey.value = key
    sortAsc.value = true
  }
}
</script>

<template>
  <div class="overflow-x-auto">
    <table class="w-full text-sm text-left">
      <thead class="text-xs text-gray-400 uppercase bg-dfpn-surface-light border-b border-dfpn-border">
        <tr>
          <th
            v-for="col in columns"
            :key="col.key"
            class="px-4 py-3 whitespace-nowrap"
            :class="[col.class, col.sortable ? 'cursor-pointer select-none hover:text-gray-200' : '']"
            @click="col.sortable && toggleSort(col.key)"
          >
            <span class="flex items-center gap-1">
              {{ col.label }}
              <template v-if="col.sortable && sortKey === col.key">
                <svg v-if="sortAsc" class="w-3 h-3" fill="currentColor" viewBox="0 0 20 20"><path d="M5.293 9.707a1 1 0 010-1.414l4-4a1 1 0 011.414 0l4 4a1 1 0 01-1.414 1.414L10 6.414l-3.293 3.293a1 1 0 01-1.414 0z"/></svg>
                <svg v-else class="w-3 h-3" fill="currentColor" viewBox="0 0 20 20"><path d="M14.707 10.293a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 111.414-1.414L10 13.586l3.293-3.293a1 1 0 011.414 0z"/></svg>
              </template>
            </span>
          </th>
        </tr>
      </thead>
      <tbody v-if="loading">
        <tr>
          <td :colspan="columns.length" class="px-4 py-12 text-center text-gray-500">
            <div class="flex items-center justify-center gap-2">
              <svg class="animate-spin h-5 w-5" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"></path>
              </svg>
              Loading...
            </div>
          </td>
        </tr>
      </tbody>
      <tbody v-else-if="rows.length === 0">
        <tr>
          <td :colspan="columns.length" class="px-4 py-12 text-center text-gray-500">
            No data found
          </td>
        </tr>
      </tbody>
      <tbody v-else>
        <tr
          v-for="(row, i) in rows"
          :key="i"
          class="border-b border-dfpn-border hover:bg-dfpn-surface-light/50 cursor-pointer transition-colors"
          @click="$emit('rowClick', row)"
        >
          <td v-for="col in columns" :key="col.key" class="px-4 py-3" :class="col.class">
            <slot :name="`cell-${col.key}`" :row="row" :value="row[col.key]">
              {{ row[col.key] }}
            </slot>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>
