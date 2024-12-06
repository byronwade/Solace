import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";

// https://vitejs.dev/config/
export default defineConfig(({ mode }) => ({
	plugins: [svelte()],
	server: {
		port: 1420,
		strictPort: true,
	},
	// Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
	clearScreen: false,
	// tauri expects a fixed port, fail if that port is not available
	envPrefix: ["VITE_", "TAURI_"],
	build: {
		// Tauri supports es2021
		target: ["es2021", "chrome100", "safari13"],
		// don't minify for debug builds
		minify: mode === "development" ? false : "esbuild",
		// produce sourcemaps for debug builds
		sourcemap: mode === "development",
	},
}));
