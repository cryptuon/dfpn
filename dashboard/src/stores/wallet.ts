import { defineStore } from 'pinia'
import { ref, computed, watch } from 'vue'
import { Connection, PublicKey } from '@solana/web3.js'
import { useNetworkStore } from './network'

export const useWalletStore = defineStore('wallet', () => {
  const networkStore = useNetworkStore()
  const publicKey = ref<PublicKey | null>(null)
  const connected = ref(false)
  const connecting = ref(false)
  const walletName = ref<string | null>(null)

  const address = computed(() => publicKey.value?.toBase58() ?? null)
  const connection = computed(() => new Connection(networkStore.rpcUrl, 'confirmed'))

  let provider: any = null // eslint-disable-line @typescript-eslint/no-explicit-any

  function getPhantomProvider() {
    if ('phantom' in window) {
      const phantom = (window as any).phantom // eslint-disable-line @typescript-eslint/no-explicit-any
      if (phantom?.solana?.isPhantom) return phantom.solana
    }
    if ('solana' in window) {
      const solana = (window as any).solana // eslint-disable-line @typescript-eslint/no-explicit-any
      if (solana?.isPhantom) return solana
    }
    return null
  }

  async function connect() {
    const phantom = getPhantomProvider()
    if (!phantom) {
      window.open('https://phantom.app/', '_blank')
      return
    }

    connecting.value = true
    try {
      const resp = await phantom.connect()
      provider = phantom
      publicKey.value = new PublicKey(resp.publicKey.toString())
      connected.value = true
      walletName.value = 'Phantom'

      phantom.on('disconnect', () => {
        publicKey.value = null
        connected.value = false
        walletName.value = null
      })

      phantom.on('accountChanged', (newPk: PublicKey | null) => {
        if (newPk) {
          publicKey.value = new PublicKey(newPk.toString())
        } else {
          disconnect()
        }
      })
    } finally {
      connecting.value = false
    }
  }

  async function disconnect() {
    if (provider) {
      await provider.disconnect()
    }
    publicKey.value = null
    connected.value = false
    walletName.value = null
    provider = null
  }

  // Auto-reconnect if previously connected
  async function autoConnect() {
    const phantom = getPhantomProvider()
    if (phantom) {
      try {
        const resp = await phantom.connect({ onlyIfTrusted: true })
        provider = phantom
        publicKey.value = new PublicKey(resp.publicKey.toString())
        connected.value = true
        walletName.value = 'Phantom'
      } catch {
        // User hasn't previously connected, ignore
      }
    }
  }

  // Reconnect when network changes
  watch(() => networkStore.rpcUrl, () => {
    // Connection is computed, so it auto-updates
  })

  return { publicKey, connected, connecting, walletName, address, connection, connect, disconnect, autoConnect }
})
