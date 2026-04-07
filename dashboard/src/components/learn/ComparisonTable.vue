<script setup lang="ts">
defineProps<{
  columns: { key: string; label: string }[]
  rows: Record<string, string>[]
}>()
</script>

<template>
  <div class="bg-dfpn-surface border border-dfpn-border rounded-xl overflow-hidden">
    <div class="overflow-x-auto">
      <table class="w-full text-sm text-left">
        <thead class="text-xs text-gray-400 uppercase bg-dfpn-surface-light border-b border-dfpn-border">
          <tr>
            <th v-for="col in columns" :key="col.key" class="px-4 py-3 whitespace-nowrap">
              {{ col.label }}
            </th>
          </tr>
        </thead>
        <tbody>
          <tr
            v-for="(row, i) in rows"
            :key="i"
            class="border-b border-dfpn-border last:border-0"
          >
            <td
              v-for="(col, j) in columns"
              :key="col.key"
              class="px-4 py-3"
              :class="j === 0 ? 'font-medium text-white' : 'text-gray-400'"
            >
              {{ row[col.key] }}
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>
