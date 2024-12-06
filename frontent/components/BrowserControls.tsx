"use client";

import { useEffect, useState } from "react";
import { useQuery, useQueryClient } from "@tanstack/react-query";
import { BrowserState, IPCEvent } from "@/types/ipc";
import { getBrowserState, sendCommand, subscribeToEvents } from "@/lib/ipc";

export default function BrowserControls() {
	const queryClient = useQueryClient();
	const [url, setUrl] = useState("");

	const { data: browserState } = useQuery({
		queryKey: ["browserState"],
		queryFn: getBrowserState,
		staleTime: 1000,
	});

	useEffect(() => {
		const unsubscribe = subscribeToEvents((event: IPCEvent) => {
			if (event.type === "STATE_UPDATE") {
				queryClient.setQueryData(["browserState"], event.payload);
			}
		});

		return unsubscribe;
	}, [queryClient]);

	const handleNavigate = async (e: React.FormEvent) => {
		e.preventDefault();
		await sendCommand({
			type: "NAVIGATE",
			payload: { url },
		});
	};

	const handleEngineChange = async (engine: string) => {
		await sendCommand({
			type: "SWITCH_ENGINE",
			payload: { engine: engine as BrowserState["currentEngine"] },
		});
	};

	const handlePrivacyModeChange = async (mode: string) => {
		await sendCommand({
			type: "SET_PRIVACY_MODE",
			payload: { mode: mode as BrowserState["privacyMode"] },
		});
	};

	return (
		<div className="flex items-center space-x-4 p-4 bg-gray-100 dark:bg-gray-900">
			<div className="flex-1">
				<form onSubmit={handleNavigate}>
					<input type="url" value={url} onChange={(e) => setUrl(e.target.value)} className="w-full px-4 py-2 rounded border dark:bg-gray-800 dark:text-white" placeholder="Enter URL" />
				</form>
			</div>

			<select value={browserState?.currentEngine} onChange={(e) => handleEngineChange(e.target.value)} className="px-4 py-2 rounded border dark:bg-gray-800 dark:text-white">
				<option value="blink">Blink</option>
				<option value="gecko">Gecko</option>
				<option value="webkit">WebKit</option>
				<option value="servo">Servo</option>
			</select>

			<select value={browserState?.privacyMode} onChange={(e) => handlePrivacyModeChange(e.target.value)} className="px-4 py-2 rounded border dark:bg-gray-800 dark:text-white">
				<option value="normal">Normal</option>
				<option value="private">Private</option>
				<option value="tor">Tor</option>
				<option value="vpn">VPN</option>
			</select>

			<button onClick={() => sendCommand({ type: "GO_BACK" })} className="px-4 py-2 rounded bg-gray-200 dark:bg-gray-700 dark:text-white">
				Back
			</button>

			<button onClick={() => sendCommand({ type: "GO_FORWARD" })} className="px-4 py-2 rounded bg-gray-200 dark:bg-gray-700 dark:text-white">
				Forward
			</button>

			<button onClick={() => sendCommand({ type: "RELOAD" })} className="px-4 py-2 rounded bg-gray-200 dark:bg-gray-700 dark:text-white">
				Reload
			</button>

			{browserState?.isLoading && <div className="animate-spin text-2xl">⌛</div>}
		</div>
	);
}
