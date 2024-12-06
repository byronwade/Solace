import type { UnlistenFn } from "@tauri-apps/api/event";

export interface WebviewConfig {
	url: string;
	engine_type: string;
	is_private: boolean;
	sandbox: SandboxConfig;
	security: SecurityConfig;
}

export interface SandboxConfig {
	allow_scripts: boolean;
	allow_forms: boolean;
	allow_popups: boolean;
	allow_downloads: boolean;
	allow_modals: boolean;
	allow_orientation_lock: boolean;
	allow_pointer_lock: boolean;
	allow_presentation: boolean;
	allow_same_origin: boolean;
	allow_top_navigation: boolean;
	allow_forms_submit: boolean;
	allow_scripts_eval: boolean;
	allow_storage: "memory" | "persistent";
}

export interface SecurityConfig {
	strict_mixed_content_checking: boolean;
	strict_transport_security: boolean;
	block_third_party_cookies: boolean;
	disable_cache: boolean;
	clear_data_on_close: boolean;
}

export interface TabInfo {
	id: string;
	url: string;
	title: string;
	isActive: boolean;
	isPrivate: boolean;
	engineType: string;
}

export interface WebviewEvent {
	payload: [string, string] | string;
}

export interface WebviewInstance {
	config: WebviewConfig;
	id: string;
}
