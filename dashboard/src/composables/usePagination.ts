import { ref, computed } from 'vue'

export function usePagination(pageSize = 20) {
  const currentPage = ref(1)
  const totalItems = ref(0)

  const offset = computed(() => (currentPage.value - 1) * pageSize)
  const totalPages = computed(() => Math.max(1, Math.ceil(totalItems.value / pageSize)))
  const hasNext = computed(() => currentPage.value < totalPages.value)
  const hasPrev = computed(() => currentPage.value > 1)

  function next() {
    if (hasNext.value) currentPage.value++
  }

  function prev() {
    if (hasPrev.value) currentPage.value--
  }

  function reset() {
    currentPage.value = 1
  }

  return { currentPage, totalItems, offset, totalPages, hasNext, hasPrev, next, prev, reset, limit: pageSize }
}
