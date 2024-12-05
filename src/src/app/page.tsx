"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";
import { useBrowser } from "./hooks/useBrowser";

export default function BrowserInterface() {
	const [url, setUrl] = useState("");
	const [isLoading, setIsLoading] = useState(false);
	const router = useRouter();
	const { settings, error, navigate, toggleHardwareAcceleration } = useBrowser();

	const handleNavigation = async (e: React.FormEvent) => {
		e.preventDefault();
		setIsLoading(true);

		try {
			const processedUrl = url.startsWith("http") ? url : `https://${url}`;
			await navigate(processedUrl);
			router.push(`/view?url=${encodeURIComponent(processedUrl)}`);
		} catch (err) {
			console.error("Navigation failed:", err);
		} finally {
			setIsLoading(false);
		}
	};

	return (
		<main className="h-full flex flex-col">
			{error && <div className="bg-red-500 text-white px-4 py-2">{error}</div>}

			{/* Browser Chrome */}
			<div className="h-16 bg-white dark:bg-gray-800 shadow-sm flex items-center px-4 space-x-4">
				<div className="flex space-x-2">
					<button onClick={() => router.back()} className="p-2 rounded-full hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors">
						←
					</button>
					<button onClick={() => router.forward()} className="p-2 rounded-full hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors">
						→
					</button>
					<button onClick={() => router.refresh()} className="p-2 rounded-full hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors">
						↻
					</button>
				</div>

				{/* URL Bar */}
				<form onSubmit={handleNavigation} className="flex-1 relative">
					<input type="text" value={url} onChange={(e) => setUrl(e.target.value)} className="w-full h-10 px-4 rounded-lg bg-gray-100 dark:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-blue-500" placeholder="Enter URL or search..." />
					{isLoading && (
						<div className="absolute right-4 top-1/2 -translate-y-1/2">
							<div className="animate-spin h-5 w-5 border-2 border-blue-500 rounded-full border-t-transparent" />
						</div>
					)}
				</form>

				{/* Browser Actions */}
				<div className="flex space-x-2">
					<button onClick={toggleHardwareAcceleration} className={`p-2 rounded-full hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors ${settings?.hardware_acceleration ? "text-blue-500" : ""}`}>
						⚡
					</button>
					<button className="p-2 rounded-full hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors">⋯</button>
				</div>
			</div>

			{/* Browser Content */}
			<div className="flex-1 relative bg-white dark:bg-gray-800">{/* Content will be rendered in the /view route */}</div>
		</main>
	);
}
