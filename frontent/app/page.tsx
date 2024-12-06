import { Suspense } from "react";
import dynamic from "next/dynamic";
import { unstable_cache } from "next/cache";
import { getBrowserState } from "@/lib/ipc";

// Import BrowserControls with no SSR since it uses browser APIs
const BrowserControls = dynamic(() => import("@/components/BrowserControls"), {
	ssr: false,
});

// Cache the initial browser state
const getInitialState = unstable_cache(
	async () => {
		try {
			return await getBrowserState();
		} catch (e) {
			console.error("Failed to get initial browser state:", e);
			return null;
		}
	},
	["initialBrowserState"],
	{ revalidate: 1 }
);

export default async function Home() {
	const initialState = await getInitialState();

	return (
		<main className="flex flex-col h-screen bg-white dark:bg-black">
			<Suspense fallback={<div>Loading controls...</div>}>
				<BrowserControls />
			</Suspense>

			<div className="flex-1 relative">
				{/* This div will be used by the Rust backend to render the browser viewport */}
				<div id="browser-viewport" className="absolute inset-0" />
			</div>

			{/* Display security info */}
			{initialState?.isSecure && (
				<div className="absolute top-4 right-4 flex items-center space-x-2 text-green-500">
					<svg xmlns="http://www.w3.org/2000/svg" className="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
						<path fillRule="evenodd" d="M10 1.944a1 1 0 01.485.12l6 3A1 1 0 0117 6v4c0 4.244-2.862 7.532-7 8.674-4.138-1.142-7-4.43-7-8.674V6a1 1 0 01.515-.936l6-3A1 1 0 0110 1.944zM5 6.678V10c0 2.83 1.308 5.23 3.613 6.616C9.4 15.866 10.6 15 12 15c1.4 0 2.6.866 3.387 1.616C17.692 15.23 19 12.83 19 10V6.678l-4.293-2.146a1 1 0 00-.914 0L10 6.377l-3.793-1.845a1 1 0 00-.914 0L5 6.678z" clipRule="evenodd" />
					</svg>
					<span>Secure Connection</span>
				</div>
			)}
		</main>
	);
}
