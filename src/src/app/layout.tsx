import type { Metadata } from "next";
import { Inter } from "next/font/google";
import "./globals.css";

const inter = Inter({ subsets: ["latin"] });

export const metadata: Metadata = {
	title: "Solace Browser",
	description: "A privacy-first, high-performance web browser",
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
	return (
		<html lang="en" className="h-full">
			<body className={`${inter.className} h-full bg-gray-50 dark:bg-gray-900`}>{children}</body>
		</html>
	);
}
