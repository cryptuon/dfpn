<script setup lang="ts">
import { useWalletStore } from '../../stores/wallet'
import { truncateAddress } from '../../composables/useFormatters'

const walletStore = useWalletStore()
</script>

<template>
  <button
    @click="walletStore.connected ? walletStore.disconnect() : walletStore.connect()"
    class="flex items-center gap-2 px-4 py-2 rounded-lg text-sm font-medium transition-colors"
    :class="walletStore.connected
      ? 'bg-dfpn-surface-light border border-dfpn-border text-gray-200 hover:border-red-500 hover:text-red-400'
      : 'bg-dfpn-primary hover:bg-dfpn-primary/80 text-white'"
    :disabled="walletStore.connecting"
  >
    <template v-if="walletStore.connected && walletStore.address">
      {{ truncateAddress(walletStore.address, 6) }}
    </template>
    <template v-else>
      Connect Wallet
    </template>
  </button>
</template>
