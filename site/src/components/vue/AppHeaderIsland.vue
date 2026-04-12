<script setup lang="ts">
import { onMounted } from 'vue'
import { useNetworkStore, type NetworkName } from '@dashboard/stores/network'
import { useWalletStore } from '@dashboard/stores/wallet'
import { truncateAddress } from '@dashboard/composables/useFormatters'

const networkStore = useNetworkStore()
const walletStore = useWalletStore()

const networkColors: Record<NetworkName, string> = {
  devnet: 'bg-yellow-500',
  testnet: 'bg-blue-500',
  mainnet: 'bg-emerald-500',
}

onMounted(() => {
  walletStore.autoConnect()
})
</script>

<template>
  <!-- Network selector -->
  <div class="relative">
    <select
      :value="networkStore.networkName"
      @change="networkStore.switchNetwork(($event.target as HTMLSelectElement).value as NetworkName)"
      class="appearance-none bg-dfpn-surface-light border border-dfpn-border rounded-lg px-3 py-1.5 pr-8 text-sm text-gray-200 cursor-pointer hover:border-gray-500 focus:outline-none focus:ring-1 focus:ring-dfpn-primary"
    >
      <option v-for="(net, key) in networkStore.networks" :key="key" :value="key">
        {{ net.label }}
      </option>
    </select>
    <div class="absolute right-2 top-1/2 -translate-y-1/2 pointer-events-none flex items-center">
      <span class="w-2 h-2 rounded-full mr-1" :class="networkColors[networkStore.networkName]"></span>
    </div>
  </div>

  <!-- Wallet button -->
  <button
    @click="walletStore.connected ? walletStore.disconnect() : walletStore.connect()"
    class="flex items-center gap-2 px-4 py-1.5 rounded-lg text-sm font-medium transition-colors"
    :class="walletStore.connected
      ? 'bg-dfpn-surface-light border border-dfpn-border text-gray-200 hover:border-gray-500'
      : 'bg-dfpn-primary hover:bg-dfpn-primary/80 text-white'"
    :disabled="walletStore.connecting"
  >
    <template v-if="walletStore.connecting">
      <svg class="animate-spin h-4 w-4" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
        <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"></path>
      </svg>
      Connecting...
    </template>
    <template v-else-if="walletStore.connected && walletStore.address">
      <span class="w-2 h-2 rounded-full bg-emerald-400"></span>
      {{ truncateAddress(walletStore.address) }}
    </template>
    <template v-else>
      Connect Wallet
    </template>
  </button>
</template>
