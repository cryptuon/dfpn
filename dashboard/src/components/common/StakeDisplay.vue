<script setup lang="ts">
import { computed } from 'vue'
import { formatDfpn } from '../../composables/useFormatters'

const props = defineProps<{ amount: number | bigint }>()
const formatted = computed(() => formatDfpn(props.amount))
</script>

<template>
  <span class="font-mono text-sm">{{ formatted }} <span class="text-gray-500">DFPN</span></span>
</template>
