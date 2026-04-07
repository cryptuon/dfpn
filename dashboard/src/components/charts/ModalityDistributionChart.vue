<script setup lang="ts">
import { computed } from 'vue'
import { Doughnut } from 'vue-chartjs'
import { Chart as ChartJS, ArcElement, Tooltip, Legend } from 'chart.js'
import { MODALITY_MAP } from '../../composables/useModalities'
import type { IndexerRequest } from '../../api/types'

ChartJS.register(ArcElement, Tooltip, Legend)

const props = defineProps<{ requests: IndexerRequest[] }>()

const chartData = computed(() => {
  const counts: Record<number, number> = {}
  for (const bit of Object.keys(MODALITY_MAP).map(Number)) counts[bit] = 0

  for (const req of props.requests) {
    for (const bit of Object.keys(MODALITY_MAP).map(Number)) {
      if (req.modalities & bit) counts[bit]++
    }
  }

  const labels = Object.keys(MODALITY_MAP).map(Number).map(b => MODALITY_MAP[b])
  const data = Object.keys(MODALITY_MAP).map(Number).map(b => counts[b])

  return {
    labels,
    datasets: [{
      data,
      backgroundColor: [
        '#3b82f6', '#8b5cf6', '#10b981',
        '#f97316', '#ec4899', '#06b6d4',
      ],
      borderWidth: 0,
    }],
  }
})

const options = {
  responsive: true,
  maintainAspectRatio: false,
  plugins: {
    legend: {
      position: 'right' as const,
      labels: { color: '#9ca3af', padding: 12, usePointStyle: true, pointStyleWidth: 8 },
    },
  },
}
</script>

<template>
  <div class="h-64">
    <Doughnut :data="chartData" :options="options" />
  </div>
</template>
