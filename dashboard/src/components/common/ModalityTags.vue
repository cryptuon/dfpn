<script setup lang="ts">
import { computed } from 'vue'
import { parseModalities } from '../../composables/useModalities'

const props = defineProps<{ modalities: number }>()
const tags = computed(() => parseModalities(props.modalities))
</script>

<template>
  <div class="flex flex-wrap gap-1">
    <span
      v-for="tag in tags"
      :key="tag.bit"
      class="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium"
      :class="tag.color"
    >
      {{ tag.label }}
    </span>
  </div>
</template>
