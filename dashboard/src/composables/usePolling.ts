import { onMounted, onUnmounted, ref } from 'vue'

export function usePolling(fn: () => Promise<void>, intervalMs = 30000) {
  const isPolling = ref(false)
  let timer: ReturnType<typeof setInterval> | null = null

  async function execute() {
    isPolling.value = true
    try {
      await fn()
    } finally {
      isPolling.value = false
    }
  }

  function start() {
    execute()
    timer = setInterval(execute, intervalMs)
  }

  function stop() {
    if (timer) {
      clearInterval(timer)
      timer = null
    }
  }

  onMounted(start)
  onUnmounted(stop)

  return { isPolling, execute, stop }
}
