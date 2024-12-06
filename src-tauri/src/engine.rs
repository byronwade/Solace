use std::sync::Mutex;
use once_cell::sync::Lazy;
use tauri::Window;
use serde::{Serialize, Deserialize};
use wry::{WebView, WebViewBuilder};

#[cfg(target_os = "macos")]
use {
    cocoa::base::{id, nil, YES, NO, BOOL},
    cocoa::appkit::{NSView, NSWindow},
    core_graphics::geometry::CGRect,
    objc::{msg_send, sel, sel_impl, class},
    objc_foundation::{INSString, NSString},
    objc::runtime::{Object, Class},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EngineConfig {
    pub engine_type: String,
    pub is_private: bool,
}

static CURRENT_ENGINE: Lazy<Mutex<EngineConfig>> = Lazy::new(|| {
    Mutex::new(EngineConfig {
        engine_type: String::from("webkit"),
        is_private: false,
    })
});

#[cfg(target_os = "macos")]
pub fn create_webkit_webview(window: &Window, url: &str, is_private: bool) -> Result<(), String> {
    unsafe {
        let window_handle = window.ns_window().unwrap() as id;
        let content_view: id = msg_send![window_handle, contentView];
        
        // Remove any existing webviews
        let existing_webviews: id = msg_send![content_view, subviews];
        let _: () = msg_send![existing_webviews, makeObjectsPerformSelector:sel!(removeFromSuperview)];
        
        // Create webview using wry
        let webview = WebViewBuilder::new(window)
            .with_url(url)
            .with_initialization_script("window.navigator.standalone = true;")
            .with_incognito(is_private)
            .with_devtools(true)
            .build()
            .map_err(|e| e.to_string())?;
        
        println!("Created WebKit webview with URL: {}", url);
        Ok(())
    }
}

#[cfg(target_os = "windows")]
pub fn create_chromium_webview(window: &Window, url: &str, is_private: bool) -> Result<(), String> {
    // Create webview using wry
    let webview = WebViewBuilder::new(window)
        .with_url(url)
        .with_initialization_script("window.navigator.standalone = true;")
        .with_incognito(is_private)
        .with_devtools(true)
        .build()
        .map_err(|e| e.to_string())?;
    
    println!("Created Chromium webview with URL: {}", url);
    Ok(())
}

pub fn init_engine(window: &Window, config: EngineConfig) -> Result<(), String> {
    println!("Initializing engine with config: {:?}", config);
    let mut current = CURRENT_ENGINE.lock().map_err(|e| e.to_string())?;
    *current = config.clone();
    
    match config.engine_type.as_str() {
        "webkit" => {
            #[cfg(target_os = "macos")]
            {
                create_webkit_webview(window, "https://www.google.com", config.is_private)?;
                Ok(())
            }
            #[cfg(not(target_os = "macos"))]
            Err("WebKit is only supported on macOS".to_string())
        }
        "chromium" => {
            #[cfg(target_os = "windows")]
            {
                create_chromium_webview(window, "https://www.google.com", config.is_private)?;
                Ok(())
            }
            #[cfg(not(target_os = "windows"))]
            Err("Chromium is only supported on Windows".to_string())
        }
        _ => Err(format!("Unsupported engine type: {}", config.engine_type))
    }
}

pub fn switch_engine(window: &Window, config: EngineConfig) -> Result<(), String> {
    println!("Switching engine to: {:?}", config);
    let mut current = CURRENT_ENGINE.lock().map_err(|e| e.to_string())?;
    
    if current.engine_type != config.engine_type {
        match config.engine_type.as_str() {
            "webkit" => {
                #[cfg(target_os = "macos")]
                {
                    create_webkit_webview(window, "https://www.google.com", config.is_private)?;
                    current.engine_type = config.engine_type;
                    Ok(())
                }
                #[cfg(not(target_os = "macos"))]
                Err("WebKit is only supported on macOS".to_string())
            }
            "chromium" => {
                #[cfg(target_os = "windows")]
                {
                    create_chromium_webview(window, "https://www.google.com", config.is_private)?;
                    current.engine_type = config.engine_type;
                    Ok(())
                }
                #[cfg(not(target_os = "windows"))]
                Err("Chromium is only supported on Windows".to_string())
            }
            _ => Err(format!("Unsupported engine type: {}", config.engine_type))
        }
    } else {
        Ok(())
    }
}

pub fn navigate(window: &Window, url: &str) -> Result<(), String> {
    println!("Navigating to: {}", url);
    let current = CURRENT_ENGINE.lock().map_err(|e| e.to_string())?;
    
    match current.engine_type.as_str() {
        "webkit" => {
            #[cfg(target_os = "macos")]
            unsafe {
                let window_handle = window.ns_window().unwrap() as id;
                let content_view: id = msg_send![window_handle, contentView];
                let webviews: id = msg_send![content_view, subviews];
                let webview: id = msg_send![webviews, firstObject];
                
                let url_str = NSString::from_str(url);
                let url_obj: id = msg_send![class!(NSURL), URLWithString:url_str];
                let req: id = msg_send![class!(NSURLRequest), requestWithURL:url_obj];
                let _: () = msg_send![webview, loadRequest:req];
                Ok(())
            }
            #[cfg(not(target_os = "macos"))]
            Err("WebKit is only supported on macOS".to_string())
        }
        "chromium" => {
            #[cfg(target_os = "windows")]
            {
                let hwnd = HWND(window.hwnd() as isize);
                let controller = CoreWebView2Controller::GetFromWindow(hwnd)
                    .map_err(|e| e.to_string())?;
                let webview = controller.CoreWebView2()?;
                webview.Navigate(url)?;
                Ok(())
            }
            #[cfg(not(target_os = "windows"))]
            Err("Chromium is only supported on Windows".to_string())
        }
        _ => Err(format!("Unsupported engine type: {}", current.engine_type))
    }
}

pub fn set_private_mode(window: &Window, enabled: bool) -> Result<(), String> {
    println!("Setting private mode: {}", enabled);
    let mut current = CURRENT_ENGINE.lock().map_err(|e| e.to_string())?;
    
    if current.is_private != enabled {
        current.is_private = enabled;
        // Recreate webview with new privacy settings
        match current.engine_type.as_str() {
            "webkit" => {
                #[cfg(target_os = "macos")]
                create_webkit_webview(window, "https://www.google.com", enabled)?;
                #[cfg(not(target_os = "macos"))]
                return Err("WebKit is only supported on macOS".to_string());
            }
            "chromium" => {
                #[cfg(target_os = "windows")]
                create_chromium_webview(window, "https://www.google.com", enabled)?;
                #[cfg(not(target_os = "windows"))]
                return Err("Chromium is only supported on Windows".to_string());
            }
            _ => return Err(format!("Unsupported engine type: {}", current.engine_type))
        }
    }
    Ok(())
} 