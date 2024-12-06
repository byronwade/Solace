pub mod blink;

use crate::ipc::{BrowserEngine, CertificateInfo, IPCEvent, SharedState};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::broadcast;

/// Trait that must be implemented by all browser engines
#[async_trait]
pub trait Engine: Send + Sync {
    /// Get the engine type
    fn engine_type(&self) -> BrowserEngine;
    
    /// Navigate to a URL
    async fn navigate(&self, url: String) -> Result<()>;
    
    /// Stop loading the current page
    async fn stop(&self) -> Result<()>;
    
    /// Reload the current page
    async fn reload(&self) -> Result<()>;
    
    /// Go back in history
    async fn go_back(&self) -> Result<()>;
    
    /// Go forward in history
    async fn go_forward(&self) -> Result<()>;
    
    /// Get the current URL
    fn current_url(&self) -> String;
    
    /// Get the page title
    fn title(&self) -> Option<String>;
    
    /// Get the favicon URL
    fn favicon(&self) -> Option<String>;
    
    /// Get certificate information for the current page
    fn certificate_info(&self) -> Option<CertificateInfo>;
    
    /// Check if the current connection is secure
    fn is_secure(&self) -> bool;
}

/// Manages multiple browser engines and handles switching between them
pub struct EngineManager {
    current_engine: Box<dyn Engine>,
    shared_state: SharedState,
    event_tx: broadcast::Sender<IPCEvent>,
}

impl EngineManager {
    pub fn new(
        initial_engine: Box<dyn Engine>,
        shared_state: SharedState,
        event_tx: broadcast::Sender<IPCEvent>,
    ) -> Self {
        Self {
            current_engine: initial_engine,
            shared_state,
            event_tx,
        }
    }
    
    pub async fn switch_engine(&mut self, engine_type: BrowserEngine) -> Result<()> {
        // Create new engine instance based on type
        let new_engine: Box<dyn Engine> = match engine_type {
            BrowserEngine::Blink => Box::new(blink::BlinkEngine::new().await?),
            BrowserEngine::Gecko => todo!("Implement Gecko engine"),
            BrowserEngine::Webkit => todo!("Implement WebKit engine"),
            BrowserEngine::Servo => todo!("Implement Servo engine"),
        };
        
        // Get current URL to restore in new engine
        let current_url = self.current_engine.current_url();
        
        // Switch engines
        self.current_engine = new_engine;
        
        // Restore current URL in new engine
        if !current_url.is_empty() {
            self.current_engine.navigate(current_url).await?;
        }
        
        // Update shared state
        {
            let mut state = self.shared_state.write();
            state.current_engine = engine_type;
        }
        
        // Notify UI of engine switch
        self.event_tx
            .send(IPCEvent::StateUpdate(self.shared_state.read().clone()))
            .map_err(|e| anyhow::anyhow!("Failed to send state update: {}", e))?;
            
        Ok(())
    }
    
    pub fn current_engine(&self) -> &dyn Engine {
        self.current_engine.as_ref()
    }
} 