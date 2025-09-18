import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import tailwindcss from '@tailwindcss/vite'


export default defineConfig({
  plugins: [svelte(),     tailwindcss(),],
  optimizeDeps: {
    exclude: ['@tailwindcss/oxide', 'lightningcss'],
  },
  ssr: {
    external: ['@tailwindcss/oxide', 'lightningcss'],
  },
  build: {
    rollupOptions: {
      external: ['@tailwindcss/oxide', 'lightningcss'],
    },
  },
})
