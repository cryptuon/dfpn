<script setup lang="ts">
defineProps<{
  phases: {
    label: string
    title: string
    duration: string
    items: string[]
    status: 'completed' | 'current' | 'upcoming'
  }[]
}>()

const statusStyles = {
  completed: 'bg-emerald-500 border-emerald-500',
  current: 'bg-dfpn-primary border-dfpn-primary animate-pulse',
  upcoming: 'bg-gray-600 border-gray-600',
}

const statusLabels = {
  completed: 'Completed',
  current: 'In Progress',
  upcoming: 'Upcoming',
}
</script>

<template>
  <div class="relative space-y-0">
    <div
      v-for="(phase, i) in phases"
      :key="phase.label"
      class="relative flex gap-6 pb-10 last:pb-0"
    >
      <!-- Timeline line + dot -->
      <div class="flex flex-col items-center">
        <div
          class="w-4 h-4 rounded-full border-2 shrink-0"
          :class="statusStyles[phase.status]"
        ></div>
        <div v-if="i < phases.length - 1" class="w-0.5 flex-1 bg-dfpn-border mt-2"></div>
      </div>

      <!-- Content -->
      <div class="flex-1 -mt-1">
        <div class="flex items-center gap-3 mb-2">
          <span class="text-xs font-bold uppercase tracking-wider text-gray-500">{{ phase.label }}</span>
          <span
            class="text-[10px] px-2 py-0.5 rounded-full font-medium"
            :class="{
              'bg-emerald-500/20 text-emerald-400': phase.status === 'completed',
              'bg-dfpn-primary/20 text-dfpn-primary-light': phase.status === 'current',
              'bg-gray-500/20 text-gray-400': phase.status === 'upcoming',
            }"
          >
            {{ statusLabels[phase.status] }}
          </span>
        </div>
        <h3 class="text-lg font-semibold text-white">{{ phase.title }}</h3>
        <p class="text-xs text-gray-500 mt-0.5">{{ phase.duration }}</p>
        <ul class="mt-3 space-y-1.5">
          <li v-for="item in phase.items" :key="item" class="flex items-start gap-2 text-sm text-gray-400">
            <svg class="w-4 h-4 mt-0.5 shrink-0" :class="phase.status === 'completed' ? 'text-emerald-400' : 'text-gray-600'" fill="currentColor" viewBox="0 0 20 20">
              <path v-if="phase.status === 'completed'" fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
              <path v-else fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm1-11a1 1 0 10-2 0v3.586L7.707 11.293a1 1 0 101.414 1.414l2-2A1 1 0 0011 10V7z" clip-rule="evenodd" />
            </svg>
            {{ item }}
          </li>
        </ul>
      </div>
    </div>
  </div>
</template>
