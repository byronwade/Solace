use tauri::Window;
use url::Url;
use std::sync::Mutex;
use once_cell::sync::Lazy;

static PRIVATE_MODE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));

pub fn init_webview(window: &Window, initial_url: String) -> Result<(), String> {
    // Validate URL
    let url = ensure_valid_url(&initial_url)?;
    println!("Initializing webview with URL: {}", url);

    // Configure the window for webview
    window.eval(&format!(
        r#"
        console.log('Initializing webview...');
        if (!window.__SOLACE_INITIALIZED__) {{
            // Create webview container that fills the entire window
            const container = document.createElement('div');
            container.id = 'webview-container';
            container.style.cssText = 'position: fixed; top: 0; left: 0; width: 100vw; height: 100vh; z-index: 0; background: white;';
            document.body.appendChild(container);
            console.log('Created container');

            // Create webview iframe
            const webview = document.createElement('iframe');
            webview.id = 'main-webview';
            webview.src = '{}';
            webview.style.cssText = 'width: 100%; height: 100%; border: none; margin: 0; padding: 0; display: block; background: white;';
            webview.setAttribute('sandbox', 'allow-same-origin allow-scripts allow-forms allow-popups allow-downloads');
            webview.setAttribute('referrerpolicy', 'no-referrer');
            webview.setAttribute('loading', 'eager');
            container.appendChild(webview);
            console.log('Created webview with URL:', '{}');

            // Mark as initialized
            window.__SOLACE_INITIALIZED__ = true;

            // Handle dark mode
            const darkModeMediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
            const handleDarkMode = (e) => {{
                webview.style.background = e.matches ? '#1a1a1a' : 'white';
                container.style.background = e.matches ? '#1a1a1a' : 'white';
            }};
            darkModeMediaQuery.addListener(handleDarkMode);
            handleDarkMode(darkModeMediaQuery);

            // Handle navigation events
            webview.addEventListener('load', () => {{
                console.log('Webview loaded');
                try {{
                    const title = webview.contentDocument?.title || 'New Tab';
                    console.log('Page title:', title);
                }} catch (e) {{
                    console.error('Error getting title:', e);
                }}
            }});

            // Handle errors
            webview.addEventListener('error', (e) => {{
                console.error('Webview error:', e);
            }});

            // Ensure Svelte UI is always on top
            const style = document.createElement('style');
            style.textContent = `
                #webview-container {{ z-index: 0; }}
                #app {{ z-index: 1; pointer-events: auto; }}
                .clickthrough {{ pointer-events: none !important; }}
                .interactive {{ pointer-events: auto !important; }}
                iframe#main-webview {{ display: block !important; }}
            `;
            document.head.appendChild(style);

            // Make the app container interactive
            const app = document.getElementById('app');
            if (app) {{
                app.style.position = 'relative';
                app.style.zIndex = '1';
                app.classList.add('interactive');
            }}
        }}
        "#,
        url, url
    )).map_err(|e| e.to_string())?;

    Ok(())
}

pub fn navigate_to(window: &Window, url: String) -> Result<(), String> {
    let url = ensure_valid_url(&url)?;
    println!("Navigating to: {}", url);
    
    window.eval(&format!(
        r#"
        console.log('Navigating to:', '{}');
        const webview = document.getElementById('main-webview');
        if (webview) {{
            webview.src = '{}';
        }} else {{
            console.error('Webview element not found');
        }}
        "#,
        url, url
    )).map_err(|e| e.to_string())?;

    Ok(())
}

pub fn set_private_mode(window: &Window, enabled: bool) -> Result<(), String> {
    println!("Setting private mode: {}", enabled);
    *PRIVATE_MODE.lock().unwrap() = enabled;

    window.eval(&format!(
        r#"
        console.log('Setting private mode:', {});
        const webview = document.getElementById('main-webview');
        if (webview) {{
            if ({}) {{
                try {{
                    const frame = webview.contentWindow;
                    if (frame) {{
                        frame.localStorage.clear();
                        frame.sessionStorage.clear();
                        frame.document.cookie.split(";").forEach(function(c) {{ 
                            frame.document.cookie = c.replace(/^ +/, "").replace(/=.*/, "=;expires=" + new Date().toUTCString() + ";path=/"); 
                        }});
                        console.log('Cleared browsing data');
                    }}
                }} catch (e) {{
                    console.error('Error clearing data:', e);
                }}
            }}
        }} else {{
            console.error('Webview element not found');
        }}
        "#,
        enabled, enabled
    )).map_err(|e| e.to_string())?;

    Ok(())
}

fn ensure_valid_url(url: &str) -> Result<String, String> {
    let url = if !url.starts_with("http://") && !url.starts_with("https://") {
        format!("https://{}", url)
    } else {
        url.to_string()
    };

    // Parse and validate the URL
    let parsed_url = Url::parse(&url).map_err(|e| format!("Invalid URL: {}", e))?;

    // Ensure it's HTTP or HTTPS
    if parsed_url.scheme() != "http" && parsed_url.scheme() != "https" {
        return Err("Only HTTP and HTTPS URLs are supported".to_string());
    }

    // Return the normalized URL
    Ok(parsed_url.to_string())
} 