use tauri::Window;

pub fn minimize_window(window: &Window) -> Result<(), String> {
    match window.minimize() {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to minimize window: {}", e))
    }
}

pub fn maximize_window(window: &Window) -> Result<(), String> {
    match window.is_maximized() {
        Ok(is_maximized) => {
            if is_maximized {
                match window.unmaximize() {
                    Ok(_) => Ok(()),
                    Err(e) => Err(format!("Failed to unmaximize window: {}", e))
                }
            } else {
                match window.maximize() {
                    Ok(_) => Ok(()),
                    Err(e) => Err(format!("Failed to maximize window: {}", e))
                }
            }
        },
        Err(e) => Err(format!("Failed to check window state: {}", e))
    }
}

pub fn close_window(window: &Window) -> Result<(), String> {
    match window.close() {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to close window: {}", e))
    }
} 