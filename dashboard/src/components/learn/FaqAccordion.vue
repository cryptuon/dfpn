<script setup lang="ts">
import { ref } from 'vue'

defineProps<{
  items: { question: string; answer: string }[]
}>()

const openIndex = ref<number | null>(null)

function toggle(i: number) {
  openIndex.value = openIndex.value === i ? null : i
}
</script>

<template>
  <div class="space-y-3">
    <div
      v-for="(item, i) in items"
      :key="i"
      class="bg-dfpn-surface border border-dfpn-border rounded-xl overflow-hidden"
    >
      <button
        @click="toggle(i)"
        class="w-full flex items-center justify-between px-6 py-4 text-left hover:bg-dfpn-surface-light/50 transition-colors"
      >
        <span class="text-sm font-medium text-white pr-4">{{ item.question }}</span>
        <svg
          class="w-5 h-5 text-gray-400 shrink-0 transition-transform duration-200"
          :class="openIndex === i ? 'rotate-180' : ''"
          fill="none" stroke="currentColor" viewBox="0 0 24 24"
        >
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M19 9l-7 7-7-7" />
        </svg>
      </button>
      <div
        v-if="openIndex === i"
        class="px-6 pb-4 text-sm text-gray-400 leading-relaxed border-t border-dfpn-border pt-4"
      >
        {{ item.answer }}
      </div>
    </div>
  </div>
</template>
