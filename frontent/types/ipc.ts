// Browser Engine Types
export type BrowserEngine = "blink" | "gecko" | "webkit" | "servo";

// Privacy Mode Types
export type PrivacyMode = "normal" | "private" | "tor" | "vpn";

// Network Route Types
export type NetworkRoute = "direct" | "tor" | "vpn" | "proxy";

// Browser State Interface
export interface BrowserState {
	currentUrl: string;
	isLoading: boolean;
	currentEngine: BrowserEngine;
	privacyMode: PrivacyMode;
	networkRoute: NetworkRoute;
	title?: string;
	favicon?: string;
	isSecure: boolean;
	certificateInfo?: {
		issuer: string;
		validFrom: string;
		validTo: string;
	};
}

// IPC Commands
export type IPCCommand = { type: "NAVIGATE"; payload: { url: string } } | { type: "SWITCH_ENGINE"; payload: { engine: BrowserEngine } } | { type: "SET_PRIVACY_MODE"; payload: { mode: PrivacyMode } } | { type: "SET_NETWORK_ROUTE"; payload: { route: NetworkRoute } } | { type: "RELOAD" } | { type: "STOP" } | { type: "GO_BACK" } | { type: "GO_FORWARD" } | { type: "GET_STATE" };

// IPC Events
export type IPCEvent = { type: "STATE_UPDATE"; payload: BrowserState } | { type: "ERROR"; payload: { code: string; message: string } } | { type: "DOWNLOAD_PROGRESS"; payload: { id: string; progress: number } } | { type: "CERTIFICATE_ERROR"; payload: { url: string; error: string } };

// Response Types
export interface IPCResponse<T = void> {
	success: boolean;
	data?: T;
	error?: {
		code: string;
		message: string;
	};
}

// Cache Keys
export const CACHE_KEYS = {
	BROWSER_STATE: "browserState",
	HISTORY: "browserHistory",
	DOWNLOADS: "downloads",
	CERTIFICATES: "certificates",
} as const;
