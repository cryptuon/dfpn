<script setup lang="ts">
defineProps<{
  label: string
  value?: string | number
  icon?: string
  color?: string
}>()
</script>

<template>
  <div class="bg-dfpn-surface border border-dfpn-border rounded-xl p-6">
    <div class="flex items-center justify-between">
      <p class="text-sm font-medium text-gray-400">{{ label }}</p>
      <div v-if="icon" class="w-10 h-10 rounded-lg flex items-center justify-center" :class="color ?? 'bg-dfpn-primary/20'">
        <slot name="icon" />
      </div>
    </div>
    <p v-if="value !== undefined" class="mt-2 text-3xl font-bold text-white">{{ value }}</p>
    <slot />
    <slot name="footer" />
  </div>
</template>
