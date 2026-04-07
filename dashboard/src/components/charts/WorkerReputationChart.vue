<script setup lang="ts">
import { computed } from 'vue'
import { Bar } from 'vue-chartjs'
import { Chart as ChartJS, CategoryScale, LinearScale, BarElement, Tooltip } from 'chart.js'
import type { IndexerWorker } from '../../api/types'

ChartJS.register(CategoryScale, LinearScale, BarElement, Tooltip)

const props = defineProps<{ workers: IndexerWorker[] }>()

const chartData = computed(() => {
  const ranges = ['0-20%', '20-40%', '40-60%', '60-80%', '80-100%']
  const counts = [0, 0, 0, 0, 0]

  for (const w of props.workers) {
    const pct = w.reputation / 100
    const idx = Math.min(Math.floor(pct / 20), 4)
    counts[idx]++
  }

  return {
    labels: ranges,
    datasets: [{
      label: 'Workers',
      data: counts,
      backgroundColor: '#8b5cf6',
      borderRadius: 4,
    }],
  }
})

const options = {
  responsive: true,
  maintainAspectRatio: false,
  scales: {
    x: { grid: { display: false }, ticks: { color: '#9ca3af' } },
    y: { grid: { color: 'rgba(55, 65, 81, 0.3)' }, ticks: { color: '#9ca3af' }, beginAtZero: true },
  },
  plugins: { legend: { display: false } },
}
</script>

<template>
  <div class="h-64">
    <Bar :data="chartData" :options="options" />
  </div>
</template>
