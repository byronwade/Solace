use crate::ipc::NetworkRoute;
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;

#[async_trait]
pub trait NetworkManager: Send + Sync {
    async fn set_route(&self, route: &NetworkRoute) -> Result<()>;
    async fn get_current_route(&self) -> NetworkRoute;
}

pub struct DefaultNetworkManager {
    current_route: Arc<RwLock<NetworkRoute>>,
}

impl DefaultNetworkManager {
    pub fn new() -> Self {
        Self {
            current_route: Arc::new(RwLock::new(NetworkRoute::Direct)),
        }
    }
}

#[async_trait]
impl NetworkManager for DefaultNetworkManager {
    async fn set_route(&self, route: &NetworkRoute) -> Result<()> {
        let mut current = self.current_route.write().await;
        *current = route.clone();
        Ok(())
    }

    async fn get_current_route(&self) -> NetworkRoute {
        self.current_route.read().await.clone()
    }
} 