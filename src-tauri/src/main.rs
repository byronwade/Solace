#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod browser;
use browser::{BrowserState, get_settings, update_settings, navigate, toggle_hardware_acceleration};

fn main() {
    let context = tauri::generate_context!();
    
    tauri::Builder::default()
        .manage(BrowserState::default())
        .invoke_handler(tauri::generate_handler![
            get_settings,
            update_settings,
            navigate,
            toggle_hardware_acceleration
        ])
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            
            // Platform-specific window setup
            #[cfg(target_os = "windows")]
            {
                use window_shadows::set_shadow;
                set_shadow(&window, true).expect("Failed to add window shadows");
                
                // Enable hardware acceleration and set optimal WebView2 settings
                use webview2_com::Microsoft::Web::WebView2::Win32::CoreWebView2Environment;
                let env = CoreWebView2Environment::CreateWithOptions(
                    None,
                    None,
                    None,
                ).expect("Failed to create WebView2 environment");
                
                if let Ok(settings) = env.CreateCoreWebView2Controller(window.hwnd() as _) {
                    if let Ok(webview) = settings.CoreWebView2() {
                        webview.SetIsWebMessageEnabled(true).unwrap();
                        webview.SetAreDefaultContextMenusEnabled(true).unwrap();
                        webview.SetIsStatusBarEnabled(false).unwrap();
                    }
                }
            }
            
            #[cfg(target_os = "macos")]
            {
                use window_shadows::set_shadow;
                set_shadow(&window, true).expect("Failed to add window shadows");
                
                // macOS-specific optimizations
                unsafe {
                    use cocoa::appkit::{NSWindow, NSWindowStyleMask};
                    let ns_window = window.ns_window().unwrap() as cocoa::base::id;
                    NSWindow::setAllowsAutomaticWindowTabbing_(ns_window, false);
                    let mask = NSWindow::styleMask(ns_window);
                    NSWindow::setStyleMask_(ns_window, mask | NSWindowStyleMask::NSFullSizeContentViewWindowMask);
                    NSWindow::setTitlebarAppearsTransparent_(ns_window, true);
                }
            }
            
            Ok(())
        })
        .run(context)
        .expect("Error while running Solace Browser");
} 