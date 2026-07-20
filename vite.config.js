import { defineConfig } from "vite";

export default defineConfig({
  base: "/bag-of-words/",
  root: "web",
  build: {
    outDir: "dist",
    emptyOutDir: true,
  },
});
