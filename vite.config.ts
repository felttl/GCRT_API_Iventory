import { defineConfig } from "vite";
export default defineConfig({
  root: "target/aggregate",
  server: { 
    proxy: { 
            "/api": "http://localhost:3000" 
        }
    },
});
