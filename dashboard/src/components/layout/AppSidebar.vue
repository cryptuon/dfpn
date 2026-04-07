<script setup lang="ts">
import { useRoute } from 'vue-router'

const route = useRoute()

defineProps<{ collapsed: boolean }>()
defineEmits<{ toggle: [] }>()

interface NavItem {
  path: string
  label: string
  icon: string
  matchPrefix?: string
}

interface NavSection {
  label: string
  items: NavItem[]
}

const navSections: NavSection[] = [
  {
    label: 'LEARN',
    items: [
      { path: '/learn', label: 'What is DFPN', icon: 'M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z' },
      { path: '/learn/how-it-works', label: 'How It Works', icon: 'M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.066 2.573c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.573 1.066c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.066-2.573c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z M15 12a3 3 0 11-6 0 3 3 0 016 0z' },
      { path: '/learn/participate/workers', label: 'Participate', icon: 'M18 9v3m0 0v3m0-3h3m-3 0h-3m-2-5a4 4 0 11-8 0 4 4 0 018 0zM3 20a6 6 0 0112 0v1H3v-1z', matchPrefix: '/learn/participate' },
      { path: '/learn/tokenomics', label: 'Tokenomics', icon: 'M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z' },
      { path: '/learn/detection-models', label: 'Detection Models', icon: 'M9 3v2m6-2v2M9 19v2m6-2v2M5 9H3m2 6H3m18-6h-2m2 6h-2M7 19h10a2 2 0 002-2V7a2 2 0 00-2-2H7a2 2 0 00-2 2v10a2 2 0 002 2zM9 9h6v6H9V9z' },
      { path: '/learn/roadmap', label: 'Roadmap', icon: 'M9 20l-5.447-2.724A1 1 0 013 16.382V5.618a1 1 0 011.447-.894L9 7m0 13l6-3m-6 3V7m6 10l4.553 2.276A1 1 0 0021 18.382V7.618a1 1 0 00-.553-.894L15 4m0 13V4m0 0L9 7' },
      { path: '/learn/faq', label: 'FAQ', icon: 'M8.228 9c.549-1.165 2.03-2 3.772-2 2.21 0 4 1.343 4 3 0 1.4-1.278 2.575-3.006 2.907-.542.104-.994.54-.994 1.093m0 3h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z' },
    ],
  },
  {
    label: 'NETWORK',
    items: [
      { path: '/', label: 'Overview', icon: 'M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6' },
      { path: '/workers', label: 'Workers', icon: 'M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z' },
      { path: '/models', label: 'Models', icon: 'M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z' },
      { path: '/requests', label: 'Requests', icon: 'M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-3 7h3m-3 4h3m-6-4h.01M9 16h.01' },
      { path: '/my-dashboard', label: 'My Dashboard', icon: 'M5.121 17.804A13.937 13.937 0 0112 16c2.5 0 4.847.655 6.879 1.804M15 10a3 3 0 11-6 0 3 3 0 016 0zm6 2a9 9 0 11-18 0 9 9 0 0118 0z' },
    ],
  },
]

function isActive(item: NavItem): boolean {
  if (item.matchPrefix) return route.path.startsWith(item.matchPrefix)
  if (item.path === '/') return route.path === '/'
  if (item.path === '/learn') return route.path === '/learn'
  return route.path.startsWith(item.path)
}
</script>

<template>
  <aside
    class="fixed left-0 top-0 h-full bg-dfpn-surface border-r border-dfpn-border z-30 transition-all duration-300 flex flex-col"
    :class="collapsed ? 'w-16' : 'w-60'"
  >
    <!-- Logo -->
    <div class="flex items-center gap-3 px-4 h-16 border-b border-dfpn-border shrink-0">
      <router-link to="/" class="flex items-center gap-3">
        <div class="w-8 h-8 rounded-lg bg-dfpn-primary flex items-center justify-center text-white font-bold text-sm shrink-0">
          DF
        </div>
        <span v-if="!collapsed" class="font-bold text-lg text-white whitespace-nowrap">DFPN</span>
      </router-link>
    </div>

    <!-- Navigation -->
    <nav class="flex-1 py-2 px-2 overflow-y-auto">
      <div v-for="section in navSections" :key="section.label" class="mb-2">
        <p v-if="!collapsed" class="text-[10px] font-semibold tracking-wider text-gray-500 uppercase px-3 pt-3 pb-1">
          {{ section.label }}
        </p>
        <div v-else class="h-px bg-dfpn-border mx-2 my-2"></div>

        <router-link
          v-for="item in section.items"
          :key="item.path"
          :to="item.path"
          class="flex items-center gap-3 px-3 py-2 rounded-lg text-sm font-medium transition-colors"
          :class="isActive(item)
            ? 'bg-dfpn-primary/20 text-dfpn-primary-light'
            : 'text-gray-400 hover:text-gray-200 hover:bg-dfpn-surface-light'"
        >
          <svg class="w-5 h-5 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" :d="item.icon" />
          </svg>
          <span v-if="!collapsed" class="whitespace-nowrap">{{ item.label }}</span>
        </router-link>
      </div>
    </nav>

    <!-- Collapse toggle -->
    <button
      @click="$emit('toggle')"
      class="flex items-center justify-center h-12 border-t border-dfpn-border text-gray-500 hover:text-gray-300 transition-colors"
    >
      <svg class="w-5 h-5 transition-transform" :class="collapsed ? 'rotate-180' : ''" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M11 19l-7-7 7-7m8 14l-7-7 7-7" />
      </svg>
    </button>
  </aside>
</template>
