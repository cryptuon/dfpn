<script setup lang="ts">
defineProps<{ status: string }>()

const colorMap: Record<string, string> = {
  Active: 'bg-emerald-500/20 text-emerald-400 border-emerald-500/30',
  Open: 'bg-blue-500/20 text-blue-400 border-blue-500/30',
  Finalized: 'bg-emerald-500/20 text-emerald-400 border-emerald-500/30',
  CommitClosed: 'bg-yellow-500/20 text-yellow-400 border-yellow-500/30',
  Pending: 'bg-yellow-500/20 text-yellow-400 border-yellow-500/30',
  Inactive: 'bg-gray-500/20 text-gray-400 border-gray-500/30',
  Expired: 'bg-gray-500/20 text-gray-400 border-gray-500/30',
  Cancelled: 'bg-gray-500/20 text-gray-400 border-gray-500/30',
  Retired: 'bg-gray-500/20 text-gray-400 border-gray-500/30',
  Slashed: 'bg-red-500/20 text-red-400 border-red-500/30',
  Disputed: 'bg-red-500/20 text-red-400 border-red-500/30',
  Suspended: 'bg-red-500/20 text-red-400 border-red-500/30',
  Unbonding: 'bg-orange-500/20 text-orange-400 border-orange-500/30',
}

function getColor(status: string): string {
  return colorMap[status] ?? 'bg-gray-500/20 text-gray-400 border-gray-500/30'
}
</script>

<template>
  <span
    class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium border"
    :class="getColor(status)"
  >
    {{ status }}
  </span>
</template>
