import { Suspense } from "react";
import { unstable_cache } from "next/cache";
import { headers } from "next/headers";

const fetchPageContent = unstable_cache(
	async (url: string) => {
		const response = await fetch(url, {
			next: { revalidate: 60 }, // Cache for 1 minute
		});
		if (!response.ok) {
			throw new Error(`Failed to fetch: ${response.statusText}`);
		}
		return response;
	},
	["page-content"]
);

function LoadingSpinner() {
	return (
		<div className="flex items-center justify-center h-full">
			<div className="animate-spin h-8 w-8 border-4 border-blue-500 rounded-full border-t-transparent" />
		</div>
	);
}

async function BrowserFrame({ url }: { url: string }) {
	try {
		await fetchPageContent(url);

		return <iframe src={url} className="w-full h-full border-none" sandbox="allow-same-origin allow-scripts allow-popups allow-forms" />;
	} catch (error) {
		return (
			<div className="flex items-center justify-center h-full">
				<p className="text-red-500">Failed to load page: {(error as Error).message}</p>
			</div>
		);
	}
}

export default async function ViewPage({ searchParams }: { searchParams: { url: string } }) {
	const headersList = headers();
	const url = searchParams.url;

	if (!url) {
		return (
			<div className="flex items-center justify-center h-full">
				<p className="text-gray-500">Enter a URL to start browsing</p>
			</div>
		);
	}

	return (
		<div className="h-full w-full">
			<Suspense fallback={<LoadingSpinner />}>
				<BrowserFrame url={url} />
			</Suspense>
		</div>
	);
}
