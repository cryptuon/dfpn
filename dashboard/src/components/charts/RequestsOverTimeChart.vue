<script setup lang="ts">
import { computed } from 'vue'
import { Line } from 'vue-chartjs'
import { Chart as ChartJS, CategoryScale, LinearScale, PointElement, LineElement, Filler, Tooltip } from 'chart.js'
import dayjs from 'dayjs'
import type { IndexerRequest } from '../../api/types'

ChartJS.register(CategoryScale, LinearScale, PointElement, LineElement, Filler, Tooltip)

const props = defineProps<{ requests: IndexerRequest[] }>()

const chartData = computed(() => {
  const buckets: Record<string, number> = {}
  const now = dayjs()

  // Initialize last 14 days
  for (let i = 13; i >= 0; i--) {
    buckets[now.subtract(i, 'day').format('MM/DD')] = 0
  }

  for (const req of props.requests) {
    const day = dayjs.unix(req.created_at).format('MM/DD')
    if (day in buckets) buckets[day]++
  }

  return {
    labels: Object.keys(buckets),
    datasets: [{
      label: 'Requests',
      data: Object.values(buckets),
      borderColor: '#8b5cf6',
      backgroundColor: 'rgba(139, 92, 246, 0.1)',
      fill: true,
      tension: 0.4,
      pointRadius: 3,
      pointBackgroundColor: '#8b5cf6',
    }],
  }
})

const options = {
  responsive: true,
  maintainAspectRatio: false,
  scales: {
    x: { grid: { color: 'rgba(55, 65, 81, 0.3)' }, ticks: { color: '#9ca3af' } },
    y: { grid: { color: 'rgba(55, 65, 81, 0.3)' }, ticks: { color: '#9ca3af' }, beginAtZero: true },
  },
  plugins: { legend: { display: false } },
}
</script>

<template>
  <div class="h-64">
    <Line :data="chartData" :options="options" />
  </div>
</template>
