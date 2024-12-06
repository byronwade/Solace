import { unstable_cache } from "next/cache";
import { BrowserState, IPCCommand, IPCEvent, IPCResponse } from "@/types/ipc";

const API_BASE = "http://localhost:3001/api";

export async function sendCommand<T = void>(command: IPCCommand): Promise<IPCResponse<T>> {
	const response = await fetch(`${API_BASE}/command`, {
		method: "POST",
		headers: {
			"Content-Type": "application/json",
		},
		body: JSON.stringify(command),
	});

	return response.json();
}

export const getBrowserState = unstable_cache(
	async (): Promise<BrowserState> => {
		const response = await sendCommand<BrowserState>({
			type: "GET_STATE",
		});

		if (!response.success || !response.data) {
			throw new Error("Failed to get browser state");
		}

		return response.data;
	},
	["browserState"],
	{ revalidate: 1 }
);

export function subscribeToEvents(onEvent: (event: IPCEvent) => void): () => void {
	const eventSource = new EventSource(`${API_BASE}/subscribe`);

	eventSource.onmessage = (event) => {
		const ipcEvent = JSON.parse(event.data) as IPCEvent;
		onEvent(ipcEvent);
	};

	return () => eventSource.close();
}
