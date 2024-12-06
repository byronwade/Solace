mod types;
pub use types::*;

use anyhow::Result;
use parking_lot::RwLock;
use std::sync::Arc;

/// Trait for handling IPC commands from the UI
#[async_trait::async_trait]
pub trait IPCHandler: Send + Sync {
    /// Handle an IPC command and return a response
    async fn handle_command(&self, command: IPCCommand) -> Result<IPCResponse>;
    
    /// Subscribe to IPC events
    async fn subscribe(&self) -> tokio::sync::broadcast::Receiver<IPCEvent>;
    
    /// Send an IPC event to all subscribers
    async fn send_event(&self, event: IPCEvent) -> Result<()>;
}

/// The shared browser state that can be accessed from multiple threads
pub type SharedState = Arc<RwLock<BrowserState>>; 