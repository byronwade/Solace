mod engine;
mod ipc;
mod network;
mod security;
mod ui;

use anyhow::Result;
use engine::{blink::BlinkEngine, EngineManager};
use ipc::{BrowserState, IPCCommand, IPCEvent, IPCHandler, IPCResponse, NetworkRoute, PrivacyMode, SharedState};
use network::{DefaultNetworkManager, NetworkManager};
use parking_lot::RwLock;
use security::{DefaultSecurityManager, SecurityManager};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::broadcast;
use tracing::{info, warn};

struct App {
    engine_manager: Arc<RwLock<EngineManager>>,
    network_manager: Arc<DefaultNetworkManager>,
    security_manager: Arc<DefaultSecurityManager>,
    shared_state: SharedState,
    event_tx: broadcast::Sender<IPCEvent>,
}

impl App {
    async fn new() -> Result<Self> {
        // Set up event channel
        let (event_tx, _) = broadcast::channel(100);
        
        // Create initial state
        let shared_state = Arc::new(RwLock::new(BrowserState::default()));
        
        // Create managers
        let network_manager = Arc::new(DefaultNetworkManager::new());
        let security_manager = Arc::new(DefaultSecurityManager::new());
        
        // Create initial engine (Blink)
        let initial_engine = Box::new(BlinkEngine::new().await?);
        
        // Create engine manager
        let engine_manager = Arc::new(RwLock::new(EngineManager::new(
            initial_engine,
            shared_state.clone(),
            event_tx.clone(),
        )));
        
        Ok(Self {
            engine_manager,
            network_manager,
            security_manager,
            shared_state,
            event_tx,
        })
    }
}

#[async_trait::async_trait]
impl IPCHandler for App {
    async fn handle_command(&self, command: IPCCommand) -> Result<IPCResponse> {
        match command {
            IPCCommand::Navigate { url } => {
                self.engine_manager.read().current_engine().navigate(url).await?;
                Ok(IPCResponse { success: true, data: None, error: None })
            }
            IPCCommand::SwitchEngine { engine } => {
                self.engine_manager.write().switch_engine(engine).await?;
                Ok(IPCResponse { success: true, data: None, error: None })
            }
            IPCCommand::SetPrivacyMode { mode } => {
                self.security_manager.set_privacy_mode(&mode).await?;
                let mut state = self.shared_state.write();
                state.privacy_mode = mode.clone();
                self.event_tx.send(IPCEvent::StateUpdate(state.clone()))?;
                Ok(IPCResponse { success: true, data: None, error: None })
            }
            IPCCommand::SetNetworkRoute { route } => {
                self.network_manager.set_route(&route).await?;
                let mut state = self.shared_state.write();
                state.network_route = route.clone();
                self.event_tx.send(IPCEvent::StateUpdate(state.clone()))?;
                Ok(IPCResponse { success: true, data: None, error: None })
            }
            IPCCommand::Reload => {
                self.engine_manager.read().current_engine().reload().await?;
                Ok(IPCResponse { success: true, data: None, error: None })
            }
            IPCCommand::Stop => {
                self.engine_manager.read().current_engine().stop().await?;
                Ok(IPCResponse { success: true, data: None, error: None })
            }
            IPCCommand::GoBack => {
                self.engine_manager.read().current_engine().go_back().await?;
                Ok(IPCResponse { success: true, data: None, error: None })
            }
            IPCCommand::GoForward => {
                self.engine_manager.read().current_engine().go_forward().await?;
                Ok(IPCResponse { success: true, data: None, error: None })
            }
        }
    }
    
    async fn subscribe(&self) -> broadcast::Receiver<IPCEvent> {
        self.event_tx.subscribe()
    }
    
    async fn send_event(&self, event: IPCEvent) -> Result<()> {
        self.event_tx.send(event)?;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    info!("Starting Solace Browser");
    
    // Create application
    let app = App::new().await?;
    
    // Create shared app state for use with web server
    let app_state = Arc::new(app);
    
    // Start the web server for UI communication
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    let app = ui::router(app_state);
    
    info!("UI server listening on {}", addr);
    
    // Run the server
    axum::serve(tokio::net::TcpListener::bind(addr).await?, app)
        .await
        .map_err(|e| anyhow::anyhow!("Server error: {}", e))?;
    
    Ok(())
}
