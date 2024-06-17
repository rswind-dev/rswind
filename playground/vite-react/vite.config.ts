import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react-swc'
import rswind from '@rswind/vite'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    rswind(),
    react(),
  ],
})
