<script setup lang="ts">
defineProps<{
  value: string
  label: string
  color?: string
}>()
</script>

<template>
  <div class="text-center">
    <p class="text-4xl font-bold" :class="color ?? 'text-dfpn-primary-light'">{{ value }}</p>
    <p class="text-sm text-gray-400 mt-1">{{ label }}</p>
  </div>
</template>
