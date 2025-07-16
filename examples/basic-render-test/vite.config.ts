import { defineConfig } from 'vite'

// https://vite.dev/config/
export default defineConfig({
  server: {
    fs: {
      // Allow serving files from one level up to the project root
      // This is necessary to load the .wasm file from the ../../pkg directory
      allow: ['../../'],
    },
  },
})
