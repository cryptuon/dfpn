<script setup lang="ts">
defineProps<{
  currentPage: number
  totalPages: number
  hasNext: boolean
  hasPrev: boolean
}>()

defineEmits<{
  next: []
  prev: []
}>()
</script>

<template>
  <div class="flex items-center justify-between px-4 py-3 border-t border-dfpn-border">
    <p class="text-sm text-gray-400">
      Page {{ currentPage }} of {{ totalPages }}
    </p>
    <div class="flex gap-2">
      <button
        :disabled="!hasPrev"
        @click="$emit('prev')"
        class="px-3 py-1.5 text-sm rounded-lg border border-dfpn-border bg-dfpn-surface hover:bg-dfpn-surface-light disabled:opacity-40 disabled:cursor-not-allowed transition-colors"
      >
        Previous
      </button>
      <button
        :disabled="!hasNext"
        @click="$emit('next')"
        class="px-3 py-1.5 text-sm rounded-lg border border-dfpn-border bg-dfpn-surface hover:bg-dfpn-surface-light disabled:opacity-40 disabled:cursor-not-allowed transition-colors"
      >
        Next
      </button>
    </div>
  </div>
</template>
