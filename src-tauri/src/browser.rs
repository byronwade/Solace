use serde::{Deserialize, Serialize};
use tauri::{command, State, Window};
use std::sync::Mutex;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BrowserSettings {
    pub dark_mode: bool,
    pub hardware_acceleration: bool,
    pub privacy_mode: bool,
    pub cache_enabled: bool,
}

#[derive(Default)]
pub struct BrowserState {
    settings: Mutex<BrowserSettings>,
}

impl Default for BrowserSettings {
    fn default() -> Self {
        Self {
            dark_mode: false,
            hardware_acceleration: true,
            privacy_mode: true,
            cache_enabled: true,
        }
    }
}

#[command]
pub async fn get_settings(state: State<'_, BrowserState>) -> Result<BrowserSettings, String> {
    let settings = state.settings.lock().map_err(|e| e.to_string())?;
    Ok(settings.clone())
}

#[command]
pub async fn update_settings(
    state: State<'_, BrowserState>,
    window: Window,
    settings: BrowserSettings,
) -> Result<(), String> {
    {
        let mut current_settings = state.settings.lock().map_err(|e| e.to_string())?;
        *current_settings = settings.clone();
    }
    
    // Emit an event to notify the frontend of settings changes
    window
        .emit("settings-updated", settings)
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

#[command]
pub async fn navigate(window: Window, url: String) -> Result<(), String> {
    // Validate URL and handle navigation
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err("Invalid URL scheme".to_string());
    }
    
    window
        .emit("navigation-requested", url)
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

#[command]
pub async fn toggle_hardware_acceleration(
    state: State<'_, BrowserState>,
    window: Window,
) -> Result<bool, String> {
    let mut settings = state.settings.lock().map_err(|e| e.to_string())?;
    settings.hardware_acceleration = !settings.hardware_acceleration;
    
    // Emit the change
    window
        .emit("hardware-acceleration-changed", settings.hardware_acceleration)
        .map_err(|e| e.to_string())?;
    
    Ok(settings.hardware_acceleration)
} 