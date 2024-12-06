import type { Metadata } from "next";
import localFont from "next/font/local";
import { unstable_cache } from "next/cache";
import { HydrationBoundary, QueryClient, dehydrate } from "@tanstack/react-query";
import Providers from "./providers";
import "./globals.css";

const geistSans = localFont({
  src: "./fonts/GeistVF.woff",
  variable: "--font-geist-sans",
  weight: "100 900",
});
const geistMono = localFont({
  src: "./fonts/GeistMonoVF.woff",
  variable: "--font-geist-mono",
  weight: "100 900",
});

export const metadata: Metadata = {
	title: "Solace Browser",
	description: "A secure, multi-engine browser built with Rust and Next.js",
};

// Create a client
const getQueryClient = unstable_cache(
  () => new QueryClient({
    defaultOptions: {
      queries: {
        staleTime: 5 * 1000, // 5 seconds
        gcTime: 10 * 60 * 1000, // 10 minutes
        refetchOnWindowFocus: false,
        refetchOnReconnect: false,
      },
    },
  }),
  ['queryClient'],
  { revalidate: 3600 } // Revalidate cache every hour
);

export default async function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  const queryClient = await getQueryClient();
  const dehydratedState = dehydrate(queryClient);

  return (
		<html lang="en">
			<body className={`${geistSans.variable} ${geistMono.variable} antialiased`}>
				<Providers>
					<HydrationBoundary state={dehydratedState}>{children}</HydrationBoundary>
				</Providers>
			</body>
		</html>
  );
}
