<script setup lang="ts">
import { ref } from 'vue'
import { truncateAddress } from '../../composables/useFormatters'
import { useNetworkStore } from '../../stores/network'

const props = defineProps<{ address: string; chars?: number }>()
const networkStore = useNetworkStore()
const copied = ref(false)

function explorerUrl(): string {
  const cluster = networkStore.networkName === 'mainnet' ? '' : `?cluster=${networkStore.networkName}`
  return `https://solscan.io/account/${props.address}${cluster}`
}

async function copy() {
  await navigator.clipboard.writeText(props.address)
  copied.value = true
  setTimeout(() => { copied.value = false }, 1500)
}
</script>

<template>
  <span class="inline-flex items-center gap-1.5 font-mono text-sm">
    <a
      :href="explorerUrl()"
      target="_blank"
      rel="noopener"
      class="text-dfpn-primary-light hover:text-dfpn-primary underline-offset-2 hover:underline"
      :title="address"
    >
      {{ truncateAddress(address, chars ?? 4) }}
    </a>
    <button
      @click="copy"
      class="text-gray-500 hover:text-gray-300 transition-colors"
      :title="copied ? 'Copied!' : 'Copy address'"
    >
      <svg v-if="!copied" xmlns="http://www.w3.org/2000/svg" class="h-3.5 w-3.5" viewBox="0 0 20 20" fill="currentColor">
        <path d="M8 3a1 1 0 011-1h2a1 1 0 110 2H9a1 1 0 01-1-1z" />
        <path d="M6 3a2 2 0 00-2 2v11a2 2 0 002 2h8a2 2 0 002-2V5a2 2 0 00-2-2 3 3 0 01-3 3H9a3 3 0 01-3-3z" />
      </svg>
      <svg v-else xmlns="http://www.w3.org/2000/svg" class="h-3.5 w-3.5 text-emerald-400" viewBox="0 0 20 20" fill="currentColor">
        <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
      </svg>
    </button>
  </span>
</template>
