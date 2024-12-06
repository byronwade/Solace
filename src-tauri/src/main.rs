#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};
use window_shadows::set_shadow;

mod engine;
use engine::{EngineConfig, init_engine, switch_engine, navigate, set_private_mode as set_engine_private_mode};

#[tauri::command]
async fn navigate_to(window: tauri::Window, url: String) -> Result<(), String> {
    println!("Navigating to: {}", url);
    navigate(&window, &url)
}

#[tauri::command]
async fn switch_browser_engine(window: tauri::Window, engine_type: String) -> Result<(), String> {
    println!("Switching engine to: {}", engine_type);
    let config = EngineConfig {
        engine_type,
        is_private: false, // Maintain current private mode state
    };
    switch_engine(&window, config)
}

#[tauri::command]
async fn set_private_mode(window: tauri::Window, enabled: bool) -> Result<(), String> {
    println!("Setting private mode: {}", enabled);
    set_engine_private_mode(&window, enabled)
}

#[tauri::command]
async fn minimize_window(window: tauri::Window) {
    window.minimize().unwrap();
}

#[tauri::command]
async fn maximize_window(window: tauri::Window) {
    if window.is_maximized().unwrap() {
        window.unmaximize().unwrap();
    } else {
        window.maximize().unwrap();
    }
}

#[tauri::command]
async fn close_window(window: tauri::Window) {
    window.close().unwrap();
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();

            // Apply vibrancy effect on macOS
            #[cfg(target_os = "macos")]
            apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None)
                .expect("Failed to apply vibrancy");

            // Apply window shadow
            #[cfg(target_os = "macos")]
            set_shadow(&window, true).expect("Failed to set window shadow");

            // Initialize engine with default configuration
            let config = EngineConfig {
                engine_type: String::from("webkit"),
                is_private: false,
            };
            
            init_engine(&window, config).expect("Failed to initialize engine");

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            navigate_to,
            switch_browser_engine,
            set_private_mode,
            minimize_window,
            maximize_window,
            close_window
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
