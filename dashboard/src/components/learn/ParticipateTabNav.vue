<script setup lang="ts">
import { useRoute } from 'vue-router'

const route = useRoute()

const tabs = [
  { path: '/learn/participate/workers', label: 'Workers' },
  { path: '/learn/participate/clients', label: 'Clients' },
  { path: '/learn/participate/model-developers', label: 'Model Developers' },
]
</script>

<template>
  <div class="border-b border-dfpn-border mb-6">
    <nav class="flex gap-0">
      <router-link
        v-for="tab in tabs"
        :key="tab.path"
        :to="tab.path"
        class="px-5 py-3 text-sm font-medium border-b-2 transition-colors -mb-px"
        :class="route.path === tab.path
          ? 'border-dfpn-primary text-white'
          : 'border-transparent text-gray-400 hover:text-gray-200 hover:border-gray-600'"
      >
        {{ tab.label }}
      </router-link>
    </nav>
  </div>
</template>
