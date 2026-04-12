import { defineConfig } from 'astro/config'
import vue from '@astrojs/vue'
import node from '@astrojs/node'
import sitemap from '@astrojs/sitemap'
import mdx from '@astrojs/mdx'
import tailwindcss from '@tailwindcss/vite'
import { nodePolyfills } from 'vite-plugin-node-polyfills'
import llmsFull from './src/integrations/llms-full'

export default defineConfig({
  site: 'https://dfpn.cryptuon.com',
  adapter: node({ mode: 'standalone' }),

  integrations: [
    vue({
      appEntrypoint: '/src/vue-app',
    }),
    sitemap({
      filter: (page) => !page.includes('/my-dashboard'),
    }),
    mdx(),
    llmsFull(),
  ],

  vite: {
    plugins: [
      tailwindcss(),
      nodePolyfills({ include: ['buffer'] }),
    ],
    resolve: {
      alias: {
        '@dashboard': new URL('../dashboard/src', import.meta.url).pathname,
      },
    },
    server: {
      proxy: {
        '/api': {
          target: 'http://127.0.0.1:3030',
          changeOrigin: true,
          rewrite: (path: string) => path.replace(/^\/api/, ''),
        },
      },
    },
  },
})
