<script setup lang="ts">
defineProps<{
  steps: { number: number; title: string; description: string }[]
  direction?: 'vertical' | 'horizontal'
}>()
</script>

<template>
  <!-- Vertical -->
  <div v-if="direction !== 'horizontal'" class="relative space-y-0">
    <div
      v-for="(step, i) in steps"
      :key="step.number"
      class="relative flex gap-4 pb-8 last:pb-0"
    >
      <!-- Line -->
      <div class="flex flex-col items-center">
        <div class="w-10 h-10 rounded-full bg-dfpn-primary/20 border-2 border-dfpn-primary flex items-center justify-center text-dfpn-primary-light font-bold text-sm shrink-0">
          {{ step.number }}
        </div>
        <div v-if="i < steps.length - 1" class="w-0.5 flex-1 bg-dfpn-primary/20 mt-2"></div>
      </div>
      <!-- Content -->
      <div class="pt-1.5">
        <h4 class="text-sm font-semibold text-white">{{ step.title }}</h4>
        <p class="text-sm text-gray-400 mt-1">{{ step.description }}</p>
      </div>
    </div>
  </div>

  <!-- Horizontal -->
  <div v-else class="flex items-start gap-0 overflow-x-auto">
    <div
      v-for="(step, i) in steps"
      :key="step.number"
      class="flex items-start min-w-0 flex-1"
    >
      <div class="flex flex-col items-center text-center px-2">
        <div class="w-10 h-10 rounded-full bg-dfpn-primary/20 border-2 border-dfpn-primary flex items-center justify-center text-dfpn-primary-light font-bold text-sm shrink-0">
          {{ step.number }}
        </div>
        <h4 class="text-xs font-semibold text-white mt-2">{{ step.title }}</h4>
        <p class="text-xs text-gray-400 mt-1">{{ step.description }}</p>
      </div>
      <div v-if="i < steps.length - 1" class="h-0.5 bg-dfpn-primary/20 self-center mt-0 min-w-4 flex-shrink-0" style="margin-top: 20px;"></div>
    </div>
  </div>
</template>
