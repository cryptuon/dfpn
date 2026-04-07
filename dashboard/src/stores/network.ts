import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

export type NetworkName = 'devnet' | 'testnet' | 'mainnet'

interface NetworkConfig {
  name: NetworkName
  label: string
  rpcUrl: string
  programIds: {
    contentRegistry: string
    analysisMarketplace: string
    modelRegistry: string
    workerRegistry: string
    rewards: string
  }
}

const NETWORKS: Record<NetworkName, NetworkConfig> = {
  devnet: {
    name: 'devnet',
    label: 'Devnet',
    rpcUrl: 'https://api.devnet.solana.com',
    programIds: {
      contentRegistry: 'GokivDYuQXPZCWRkwMhdH2h91KpDQXBEmpgBgs55bnpH',
      analysisMarketplace: '9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin',
      modelRegistry: 'Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS',
      workerRegistry: 'HmbTLCmaGvZhKnn1Zfa1JVnp7vkMV4DYVxPLWBVoN65L',
      rewards: '4uQeVj5tqViQh7yWWGStvkEG1Zmhx6uasJtWCJziofM',
    },
  },
  testnet: {
    name: 'testnet',
    label: 'Testnet',
    rpcUrl: 'https://api.testnet.solana.com',
    programIds: {
      contentRegistry: 'GokivDYuQXPZCWRkwMhdH2h91KpDQXBEmpgBgs55bnpH',
      analysisMarketplace: '9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin',
      modelRegistry: 'Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS',
      workerRegistry: 'HmbTLCmaGvZhKnn1Zfa1JVnp7vkMV4DYVxPLWBVoN65L',
      rewards: '4uQeVj5tqViQh7yWWGStvkEG1Zmhx6uasJtWCJziofM',
    },
  },
  mainnet: {
    name: 'mainnet',
    label: 'Mainnet',
    rpcUrl: 'https://api.mainnet-beta.solana.com',
    programIds: {
      contentRegistry: 'GokivDYuQXPZCWRkwMhdH2h91KpDQXBEmpgBgs55bnpH',
      analysisMarketplace: '9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin',
      modelRegistry: 'Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS',
      workerRegistry: 'HmbTLCmaGvZhKnn1Zfa1JVnp7vkMV4DYVxPLWBVoN65L',
      rewards: '4uQeVj5tqViQh7yWWGStvkEG1Zmhx6uasJtWCJziofM',
    },
  },
}

export const useNetworkStore = defineStore('network', () => {
  const saved = localStorage.getItem('dfpn-network') as NetworkName | null
  const networkName = ref<NetworkName>(saved && saved in NETWORKS ? saved : 'devnet')

  const config = computed(() => NETWORKS[networkName.value])
  const rpcUrl = computed(() => config.value.rpcUrl)

  function switchNetwork(name: NetworkName) {
    networkName.value = name
    localStorage.setItem('dfpn-network', name)
  }

  return { networkName, config, rpcUrl, switchNetwork, networks: NETWORKS }
})
