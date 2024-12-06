use crate::ipc::{BrowserEngine, BrowserState, CertificateInfo, IPCEvent, SharedState};
use anyhow::Result;
use async_trait::async_trait;
use chromiumoxide::{Browser, BrowserConfig, Page};
use chromiumoxide_cdp::cdp::browser_protocol::page::EventPageLifecycle;
use futures::StreamExt;
use parking_lot::RwLock;
use std::sync::Arc;
use tokio::sync::broadcast;
use tracing::{error, info};

pub struct BlinkEngine {
    browser: Arc<Browser>,
    page: Arc<RwLock<Option<Page>>>,
    current_url: Arc<RwLock<String>>,
    title: Arc<RwLock<Option<String>>>,
    favicon: Arc<RwLock<Option<String>>>,
    certificate_info: Arc<RwLock<Option<CertificateInfo>>>,
    is_secure: Arc<RwLock<bool>>,
    shared_state: SharedState,
    event_tx: broadcast::Sender<IPCEvent>,
}

// Implement Send for BlinkEngine
unsafe impl Send for BlinkEngine {}
unsafe impl Sync for BlinkEngine {}

impl BlinkEngine {
    pub async fn new() -> Result<Self> {
        let current_url = Arc::new(RwLock::new(String::from("about:blank")));
        let page = Arc::new(RwLock::new(None));
        let title = Arc::new(RwLock::new(None));
        let favicon = Arc::new(RwLock::new(None));
        let certificate_info = Arc::new(RwLock::new(None));
        let is_secure = Arc::new(RwLock::new(false));
        
        // Create browser config
        let config = BrowserConfig::builder()
            .window_size(1280, 800)
            .with_head()
            .build()
            .map_err(|e| anyhow::anyhow!("Failed to build browser config: {}", e))?;
            
        // Launch browser
        let (browser, mut handler) = Browser::launch(config)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to launch browser: {}", e))?;
        let browser = Arc::new(browser);
        
        // Create event channel
        let (event_tx, _) = broadcast::channel(100);
        let event_tx_clone = event_tx.clone();
        
        // Create shared state
        let shared_state = Arc::new(RwLock::new(BrowserState::default()));
        let shared_state_clone = shared_state.clone();
        
        // Handle browser events
        tokio::spawn(async move {
            while let Some(event) = handler.next().await {
                match event {
                    Ok(event) => {
                        let mut state = shared_state_clone.write();
                        
                        // Update state based on event type
                        match event {
                            EventPageLifecycle::NavigationStarted(nav) => {
                                state.current_url = nav.url.to_string();
                                state.is_loading = true;
                            }
                            EventPageLifecycle::NavigationCompleted(nav) => {
                                state.current_url = nav.url.to_string();
                                state.is_loading = false;
                            }
                            EventPageLifecycle::TitleChanged(title) => {
                                state.title = Some(title.title);
                            }
                            EventPageLifecycle::SecurityStateChanged(security) => {
                                state.is_secure = security.secure;
                                if let Some(cert) = security.certificate {
                                    state.certificate_info = Some(CertificateInfo {
                                        issuer: cert.issuer,
                                        valid_from: cert.valid_from,
                                        valid_to: cert.valid_to,
                                    });
                                }
                            }
                            _ => {}
                        }
                        
                        // Send state update
                        if let Err(e) = event_tx_clone.send(IPCEvent::StateUpdate(state.clone())) {
                            error!("Failed to send state update: {}", e);
                        }
                    }
                    Err(e) => error!("Browser event error: {}", e),
                }
            }
        });
        
        Ok(Self {
            browser,
            page,
            current_url,
            title,
            favicon,
            certificate_info,
            is_secure,
            shared_state,
            event_tx,
        })
    }
}

#[async_trait]
impl super::Engine for BlinkEngine {
    fn engine_type(&self) -> BrowserEngine {
        BrowserEngine::Blink
    }
    
    async fn navigate(&self, url: String) -> Result<()> {
        let page = if let Some(page) = &*self.page.read() {
            page.clone()
        } else {
            let page = self.browser.new_page(&url)
                .await
                .map_err(|e| anyhow::anyhow!("Failed to create new page: {}", e))?;
            *self.page.write() = Some(page.clone());
            page
        };
        
        page.goto(&url)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to navigate to {}: {}", url, e))?;
        *self.current_url.write() = url;
        Ok(())
    }
    
    async fn reload(&self) -> Result<()> {
        if let Some(page) = &*self.page.read() {
            page.reload()
                .await
                .map_err(|e| anyhow::anyhow!("Failed to reload page: {}", e))?;
        }
        Ok(())
    }
    
    async fn stop(&self) -> Result<()> {
        if let Some(page) = &*self.page.read() {
            page.evaluate("window.stop()")
                .await
                .map_err(|e| anyhow::anyhow!("Failed to stop page: {}", e))?;
        }
        Ok(())
    }
    
    async fn go_back(&self) -> Result<()> {
        if let Some(page) = &*self.page.read() {
            page.evaluate("window.history.back()")
                .await
                .map_err(|e| anyhow::anyhow!("Failed to go back: {}", e))?;
        }
        Ok(())
    }
    
    async fn go_forward(&self) -> Result<()> {
        if let Some(page) = &*self.page.read() {
            page.evaluate("window.history.forward()")
                .await
                .map_err(|e| anyhow::anyhow!("Failed to go forward: {}", e))?;
        }
        Ok(())
    }
    
    fn current_url(&self) -> String {
        self.current_url.read().clone()
    }
    
    fn title(&self) -> Option<String> {
        self.title.read().clone()
    }
    
    fn favicon(&self) -> Option<String> {
        self.favicon.read().clone()
    }
    
    fn certificate_info(&self) -> Option<CertificateInfo> {
        self.certificate_info.read().clone()
    }
    
    fn is_secure(&self) -> bool {
        *self.is_secure.read()
    }
}

impl Drop for BlinkEngine {
    fn drop(&mut self) {
        if let Some(page) = &*self.page.read() {
            let _ = futures::executor::block_on(page.evaluate("window.close()"));
        }
    }
} 