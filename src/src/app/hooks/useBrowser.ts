import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { listen, UnlistenFn, Event } from "@tauri-apps/api/event";

interface BrowserSettings {
	dark_mode: boolean;
	hardware_acceleration: boolean;
	privacy_mode: boolean;
	cache_enabled: boolean;
}

interface SettingsEvent extends Event<BrowserSettings> {
	payload: BrowserSettings;
}

interface HardwareAccelerationEvent extends Event<boolean> {
	payload: boolean;
}

export function useBrowser() {
	const [settings, setSettings] = useState<BrowserSettings | null>(null);
	const [isLoading, setIsLoading] = useState(true);
	const [error, setError] = useState<string | null>(null);

	useEffect(() => {
		// Load initial settings
		invoke<BrowserSettings>("get_settings")
			.then(setSettings)
			.catch((err: Error) => setError(err.toString()))
			.finally(() => setIsLoading(false));

		// Listen for settings updates
		const unlistenSettings = listen<BrowserSettings>("settings-updated", (event: SettingsEvent) => {
			setSettings(event.payload);
		});

		// Listen for hardware acceleration changes
		const unlistenHardware = listen<boolean>("hardware-acceleration-changed", (event: HardwareAccelerationEvent) => {
			setSettings((prev) => (prev ? { ...prev, hardware_acceleration: event.payload } : null));
		});

		return () => {
			Promise.all([unlistenSettings.then((unlisten: UnlistenFn) => unlisten()), unlistenHardware.then((unlisten: UnlistenFn) => unlisten())]).catch(console.error);
		};
	}, []);

	const updateSettings = async (newSettings: BrowserSettings) => {
		try {
			await invoke("update_settings", { settings: newSettings });
		} catch (err) {
			const error = err as Error;
			setError(error.toString());
			throw error;
		}
	};

	const toggleHardwareAcceleration = async () => {
		try {
			const newState = await invoke<boolean>("toggle_hardware_acceleration");
			return newState;
		} catch (err) {
			const error = err as Error;
			setError(error.toString());
			throw error;
		}
	};

	const navigate = async (url: string) => {
		try {
			await invoke("navigate", { url });
		} catch (err) {
			const error = err as Error;
			setError(error.toString());
			throw error;
		}
	};

	return {
		settings,
		isLoading,
		error,
		updateSettings,
		toggleHardwareAcceleration,
		navigate,
	};
}
