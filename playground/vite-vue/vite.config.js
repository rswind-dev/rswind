import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import arrow from '@rswind/vite'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [vue(), arrow()],
})
