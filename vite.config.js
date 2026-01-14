import { defineConfig } from "vite";
import { sveltekit } from "@sveltejs/kit/vite";

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST;

// https://vite.dev/config/
export default defineConfig(async () => ({
  plugins: [sveltekit()],

  clearScreen: false,
  
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      ignored: ["**/src-tauri/**"],
    },
  },
  
  // Don't externalize Tauri packages - let them be bundled
  ssr: {
    noExternal: ['@tauri-apps/api', '@tauri-apps/plugin-notification'],
  },
  
  build: {
    // Remove the external configuration that's breaking production
    // rollupOptions: {
    //   external: [/^@tauri-apps\//],
    // },
    target: 'esnext', // Support top-level await
  },
}));