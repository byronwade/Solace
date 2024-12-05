/** @type {import('next').NextConfig} */
const nextConfig = {
	output: "export", // Required for Tauri static bundling
	images: {
		unoptimized: true, // Required for static export
	},
	// Enable latest Next.js features
	experimental: {
		serverActions: false, // Disabled for Tauri
		optimizePackageImports: true,
		turbo: {
			loaders: {
				// Enable Turbopack loaders for faster builds
				".js": ["swc"],
				".jsx": ["swc"],
				".ts": ["swc"],
				".tsx": ["swc"],
			},
		},
		// Enable React optimizations
		runtime: "edge",
		optimizeCss: true,
		scrollRestoration: true,
		typedRoutes: true,
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
		// Add performance optimizations
		config.optimization = {
			...config.optimization,
			runtimeChunk: "single",
			splitChunks: {
				chunks: "all",
				maxInitialRequests: 25,
				minSize: 20000,
			},
		};
		return config;
	},
	// Enable strict mode for better performance
	reactStrictMode: true,
	// Minimize JavaScript bundles
	swcMinify: true,
	// Enable compiler optimizations
	compiler: {
		removeConsole: process.env.NODE_ENV === "production",
		styledComponents: true,
	},
	// Cache optimization
	onDemandEntries: {
		maxInactiveAge: 25 * 1000,
		pagesBufferLength: 2,
	},
};

module.exports = nextConfig;
