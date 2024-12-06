use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BrowserEngine {
    Blink,
    Gecko,
    Webkit,
    Servo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrivacyMode {
    Normal,
    Private,
    Tor,
    Vpn,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkRoute {
    Direct,
    Tor,
    Vpn,
    Proxy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateInfo {
    pub issuer: String,
    pub valid_from: String,
    pub valid_to: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserState {
    pub current_url: String,
    pub is_loading: bool,
    pub current_engine: BrowserEngine,
    pub privacy_mode: PrivacyMode,
    pub network_route: NetworkRoute,
    pub title: Option<String>,
    pub favicon: Option<String>,
    pub is_secure: bool,
    pub certificate_info: Option<CertificateInfo>,
}

impl Default for BrowserState {
    fn default() -> Self {
        Self {
            current_url: String::from("about:blank"),
            is_loading: false,
            current_engine: BrowserEngine::Blink,
            privacy_mode: PrivacyMode::Normal,
            network_route: NetworkRoute::Direct,
            title: None,
            favicon: None,
            is_secure: false,
            certificate_info: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum IPCCommand {
    Navigate { url: String },
    SwitchEngine { engine: BrowserEngine },
    SetPrivacyMode { mode: PrivacyMode },
    SetNetworkRoute { route: NetworkRoute },
    Reload,
    Stop,
    GoBack,
    GoForward,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum IPCEvent {
    StateUpdate(BrowserState),
    Error { code: String, message: String },
    DownloadProgress { id: String, progress: f64 },
    CertificateError { url: String, error: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPCResponse<T = ()> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<ErrorInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorInfo {
    pub code: String,
    pub message: String,
}

pub const CACHE_KEYS: &[&str] = &[
    "browserState",
    "browserHistory",
    "downloads",
    "certificates",
]; 