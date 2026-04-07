<script setup lang="ts">
defineProps<{
  icon: string
  title: string
  description: string
  color?: string
}>()
</script>

<template>
  <div class="bg-dfpn-surface border border-dfpn-border rounded-xl p-6 hover:border-dfpn-primary/30 transition-colors">
    <div
      class="w-12 h-12 rounded-lg flex items-center justify-center mb-4"
      :class="color ?? 'bg-dfpn-primary/20'"
    >
      <svg class="w-6 h-6" :class="color ? 'text-white' : 'text-dfpn-primary-light'" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" :d="icon" />
      </svg>
    </div>
    <h3 class="text-lg font-semibold text-white mb-2">{{ title }}</h3>
    <p class="text-sm text-gray-400 leading-relaxed">{{ description }}</p>
    <slot />
  </div>
</template>
