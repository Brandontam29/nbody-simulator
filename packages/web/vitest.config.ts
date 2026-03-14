import { defineConfig, mergeConfig } from 'vitest/config'
import viteConfig from './vite.config'

export default mergeConfig(
  viteConfig,
  defineConfig({
    test: {
      environment: 'jsdom',
      setupFiles: ['./src/test/setup.js'],
      globals: true,
      browser: {
        enabled: false,
      },
      deps: {
        optimizer: {
          web: {
            include: ['vitest-canvas-mock']
          }
        }
      }
    }
  })
)
