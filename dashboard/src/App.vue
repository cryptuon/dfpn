<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useWalletStore } from './stores/wallet'
import AppSidebar from './components/layout/AppSidebar.vue'
import AppHeader from './components/layout/AppHeader.vue'

const walletStore = useWalletStore()
const sidebarCollapsed = ref(false)

onMounted(() => {
  walletStore.autoConnect()
})
</script>

<template>
  <div class="min-h-screen bg-gray-950 text-gray-100">
    <AppSidebar :collapsed="sidebarCollapsed" @toggle="sidebarCollapsed = !sidebarCollapsed" />

    <div class="transition-all duration-300" :class="sidebarCollapsed ? 'ml-16' : 'ml-60'">
      <AppHeader />
      <main class="p-6 max-w-7xl mx-auto">
        <router-view />
      </main>
    </div>
  </div>
</template>
