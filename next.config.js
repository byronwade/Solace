/** @type {import('next').NextConfig} */
const nextConfig = {
	output: "export", // Required for Tauri static bundling
	images: {
		unoptimized: true, // Required for static export
	},
	// Disable server components for Tauri
	experimental: {
		serverActions: false,
	},
	// Optimize for Tauri's development server
	webpack: (config, { dev }) => {
		if (dev) {
			config.watchOptions = {
				...config.watchOptions,
				poll: 1000,
				aggregateTimeout: 300,
			};
		}
		return config;
	},
};

module.exports = nextConfig;
