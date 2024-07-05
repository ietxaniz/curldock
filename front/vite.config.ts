import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  server: {
    port: 2081,
    hmr: {
      port: 2081,
      protocol: 'ws',
      host: 'localhost',
    },
    watch: {
      usePolling: true,
    }
  }
})
