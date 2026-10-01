import { defineConfig } from "vite";

export default defineConfig({
  server: {
    fs: {
      // Allow serving the wasm-pack output outside www/.
      allow: [".."],
    },
  },
});
