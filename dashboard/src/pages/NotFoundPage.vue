<script setup lang="ts">
</script>

<template>
  <div class="flex flex-col items-center justify-center min-h-[60vh] text-center">
    <h1 class="text-6xl font-bold text-gray-700 mb-4">404</h1>
    <p class="text-lg text-gray-500 mb-8">Page not found</p>
    <router-link
      to="/"
      class="px-6 py-2.5 bg-dfpn-primary hover:bg-dfpn-primary/80 text-white rounded-lg font-medium transition-colors"
    >
      Back to Overview
    </router-link>
  </div>
</template>
