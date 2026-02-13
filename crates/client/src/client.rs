//! Main client implementation

use std::sync::Arc;
use tokio::sync::RwLock;

use invisible_crypto::IdentityKey;
use invisible_storage::Database;
use invisible_wallet::ShadowWallet;

use crate::Result;

/// Invisible client instance
pub struct InvisibleClient {
    /// User identity key
    identity: Arc<RwLock<Option<IdentityKey>>>,
    /// Storage database
    storage: Arc<Database>,
    /// Shadow wallet
    wallet: Arc<RwLock<Option<ShadowWallet>>>,
    /// Configuration
    config: ClientConfig,
}

/// Client configuration
#[derive(Debug, Clone)]
pub struct ClientConfig {
    /// Relay endpoints
    pub relay_endpoints: Vec<String>,
    /// Enable voice/video calls
    pub enable_calls: bool,
    /// Auto-download media
    pub auto_download_media: bool,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            relay_endpoints: vec!["https://relay.invisible.im".to_string()],
            enable_calls: true,
            auto_download_media: false,
        }
    }
}

impl InvisibleClient {
    /// Create a new client
    pub fn new(storage: Database, config: ClientConfig) -> Self {
        Self {
            identity: Arc::new(RwLock::new(None)),
            storage: Arc::new(storage),
            wallet: Arc::new(RwLock::new(None)),
            config,
        }
    }

    /// Initialize client with identity
    pub async fn init_identity(&self, identity: IdentityKey) -> Result<()> {
        let mut id = self.identity.write().await;
        *id = Some(identity);
        Ok(())
    }

    /// Initialize wallet
    pub async fn init_wallet(&self, wallet: ShadowWallet) -> Result<()> {
        let mut w = self.wallet.write().await;
        *w = Some(wallet);
        Ok(())
    }

    /// Get client identity
    pub async fn identity(&self) -> Option<IdentityKey> {
        self.identity.read().await.clone()
    }

    /// Check if authenticated
    pub async fn is_authenticated(&self) -> bool {
        self.identity.read().await.is_some()
    }
}
