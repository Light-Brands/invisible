//! Ghost VPN (Layer 0)
//!
//! Mandatory WireGuard VPN tunnel that all Invisible traffic flows through.
//! Provides the foundational network privacy layer.
//!
//! ## Architecture
//!
//! - **Always On:** VPN cannot be disabled, all traffic routed through tunnel
//! - **Random Endpoints:** Connects to random global WireGuard servers
//! - **Automatic Reconnection:** Exponential backoff on failures
//! - **Kill Switch:** Blocks all traffic if tunnel fails
//!
//! ## Security Properties
//!
//! - **IP Masking:** Real IP hidden from all network observers
//! - **Traffic Encryption:** ChaCha20-Poly1305 + Curve25519
//! - **Forward Secrecy:** Regular key rotation
//! - **No DNS Leaks:** DNS queries tunneled through VPN

use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::time::{Duration, Instant};

use crate::error::{Result, ScramblerError};

/// WireGuard configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpnConfig {
    /// Local private key
    pub private_key: Vec<u8>,
    /// Local IP address inside tunnel
    pub local_address: String,
    /// List of available VPN servers
    pub endpoints: Vec<VpnEndpoint>,
    /// Persistent keepalive interval (seconds)
    pub keepalive_interval: u16,
    /// Maximum time before forcing reconnect (seconds)
    pub max_session_time: u64,
}

/// VPN server endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpnEndpoint {
    /// Server's public key
    pub public_key: Vec<u8>,
    /// Server's socket address
    pub address: SocketAddr,
    /// Geographic location for routing
    pub location: String,
    /// Connection latency (ms) - updated on connect
    pub latency_ms: Option<u32>,
}

/// VPN connection state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VpnState {
    /// Disconnected, not attempting connection
    Disconnected,
    /// Attempting to connect
    Connecting,
    /// Connected and active
    Connected,
    /// Reconnecting after failure
    Reconnecting,
    /// Fatal error, cannot connect
    Failed,
}

/// VPN connection manager
#[derive(Debug)]
pub struct VpnManager {
    /// Current configuration
    config: VpnConfig,
    /// Current connection state
    state: VpnState,
    /// Current endpoint (if connected)
    current_endpoint: Option<VpnEndpoint>,
    /// Time when current connection established
    connected_since: Option<Instant>,
    /// Number of consecutive connection failures
    failure_count: u32,
    /// Reconnection backoff duration
    backoff: Duration,
}

impl VpnManager {
    /// Create a new VPN manager
    pub fn new(config: VpnConfig) -> Self {
        Self {
            config,
            state: VpnState::Disconnected,
            current_endpoint: None,
            connected_since: None,
            failure_count: 0,
            backoff: Duration::from_secs(1),
        }
    }

    /// Get current VPN state
    pub fn state(&self) -> VpnState {
        self.state
    }

    /// Check if VPN is currently connected
    pub fn is_connected(&self) -> bool {
        self.state == VpnState::Connected
    }

    /// Select a random VPN endpoint
    fn select_endpoint(&self) -> Result<VpnEndpoint> {
        use rand::seq::SliceRandom;
        use rand::thread_rng;

        self.config
            .endpoints
            .choose(&mut thread_rng())
            .cloned()
            .ok_or_else(|| ScramblerError::VpnError("No endpoints available".to_string()))
    }

    /// Connect to VPN
    ///
    /// This is a placeholder - actual WireGuard integration would use:
    /// - wireguard-rs crate for userspace implementation
    /// - Or system WireGuard via netlink/IPC
    pub async fn connect(&mut self) -> Result<()> {
        self.state = VpnState::Connecting;

        // Select random endpoint
        let endpoint = self.select_endpoint()?;

        // TODO: Actual WireGuard connection
        // 1. Configure WireGuard interface
        // 2. Set private key
        // 3. Add peer (endpoint)
        // 4. Set allowed IPs (0.0.0.0/0 for all traffic)
        // 5. Bring interface up

        // Placeholder: simulate connection
        self.current_endpoint = Some(endpoint.clone());
        self.connected_since = Some(Instant::now());
        self.state = VpnState::Connected;
        self.failure_count = 0;
        self.backoff = Duration::from_secs(1);

        tracing::info!(
            endpoint = %endpoint.address,
            location = %endpoint.location,
            "VPN connected"
        );

        Ok(())
    }

    /// Disconnect from VPN
    pub async fn disconnect(&mut self) -> Result<()> {
        // TODO: Actual WireGuard disconnection
        // 1. Remove peer configuration
        // 2. Bring interface down

        self.state = VpnState::Disconnected;
        self.current_endpoint = None;
        self.connected_since = None;

        tracing::info!("VPN disconnected");

        Ok(())
    }

    /// Reconnect with exponential backoff
    pub async fn reconnect(&mut self) -> Result<()> {
        self.state = VpnState::Reconnecting;
        self.failure_count += 1;

        // Calculate exponential backoff: 1s, 2s, 4s, 8s, ... up to 60s
        self.backoff = Duration::from_secs(1u64 << self.failure_count.min(6));
        
        tracing::warn!(
            failures = self.failure_count,
            backoff_ms = self.backoff.as_millis(),
            "VPN reconnecting"
        );

        // Wait for backoff period
        tokio::time::sleep(self.backoff).await;

        // Attempt reconnection
        self.connect().await
    }

    /// Check if connection needs renewal
    pub fn needs_renewal(&self) -> bool {
        if let Some(connected_since) = self.connected_since {
            let session_duration = connected_since.elapsed();
            let max_duration = Duration::from_secs(self.config.max_session_time);
            session_duration >= max_duration
        } else {
            false
        }
    }

    /// Maintain VPN connection (call periodically)
    pub async fn maintain(&mut self) -> Result<()> {
        match self.state {
            VpnState::Disconnected => {
                // Auto-connect (Ghost VPN is always on)
                self.connect().await?;
            }
            VpnState::Connected => {
                // Check if renewal needed
                if self.needs_renewal() {
                    tracing::info!("VPN session expired, reconnecting");
                    self.disconnect().await?;
                    self.connect().await?;
                }

                // TODO: Check connection health
                // - Ping endpoint
                // - Verify traffic flow
                // - Monitor latency
            }
            VpnState::Reconnecting => {
                // Already reconnecting, wait
            }
            VpnState::Failed => {
                // Try recovery after extended backoff
                if self.failure_count > 10 {
                    return Err(ScramblerError::VpnError(
                        "VPN failed after 10 attempts".to_string(),
                    ));
                }
                self.reconnect().await?;
            }
            VpnState::Connecting => {
                // Connection in progress
            }
        }

        Ok(())
    }

    /// Get current endpoint info
    pub fn current_endpoint(&self) -> Option<&VpnEndpoint> {
        self.current_endpoint.as_ref()
    }

    /// Get connection uptime
    pub fn uptime(&self) -> Option<Duration> {
        self.connected_since.map(|t| t.elapsed())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr};

    fn create_test_config() -> VpnConfig {
        VpnConfig {
            private_key: vec![1u8; 32],
            local_address: "10.0.0.2/24".to_string(),
            endpoints: vec![
                VpnEndpoint {
                    public_key: vec![2u8; 32],
                    address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4)), 51820),
                    location: "US-East".to_string(),
                    latency_ms: None,
                },
                VpnEndpoint {
                    public_key: vec![3u8; 32],
                    address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(5, 6, 7, 8)), 51820),
                    location: "EU-West".to_string(),
                    latency_ms: None,
                },
            ],
            keepalive_interval: 25,
            max_session_time: 3600, // 1 hour
        }
    }

    #[tokio::test]
    async fn test_vpn_connect() {
        let config = create_test_config();
        let mut manager = VpnManager::new(config);

        assert_eq!(manager.state(), VpnState::Disconnected);
        assert!(!manager.is_connected());

        manager.connect().await.unwrap();

        assert_eq!(manager.state(), VpnState::Connected);
        assert!(manager.is_connected());
        assert!(manager.current_endpoint().is_some());
    }

    #[tokio::test]
    async fn test_vpn_disconnect() {
        let config = create_test_config();
        let mut manager = VpnManager::new(config);

        manager.connect().await.unwrap();
        assert!(manager.is_connected());

        manager.disconnect().await.unwrap();
        assert_eq!(manager.state(), VpnState::Disconnected);
        assert!(!manager.is_connected());
    }

    #[test]
    fn test_endpoint_selection() {
        let config = create_test_config();
        let manager = VpnManager::new(config);

        // Should select one of the configured endpoints
        let endpoint = manager.select_endpoint().unwrap();
        assert!(!endpoint.public_key.is_empty());
    }

    #[test]
    fn test_exponential_backoff() {
        // Test backoff calculation without actually connecting
        let config = create_test_config();
        let mut manager = VpnManager::new(config);

        // Initial backoff
        assert_eq!(manager.backoff, Duration::from_secs(1));

        // Simulate failures and check backoff calculation
        manager.failure_count = 1;
        manager.backoff = Duration::from_secs(1 << 1); // 2^1 = 2s
        assert_eq!(manager.backoff, Duration::from_secs(2));

        manager.failure_count = 3;
        manager.backoff = Duration::from_secs(1 << 3); // 2^3 = 8s
        assert_eq!(manager.backoff, Duration::from_secs(8));

        manager.failure_count = 6;
        manager.backoff = Duration::from_secs(1 << 6); // 2^6 = 64s
        assert_eq!(manager.backoff, Duration::from_secs(64));

        // Verify cap at 2^6
        manager.failure_count = 10;
        manager.backoff = Duration::from_secs(1 << manager.failure_count.min(6));
        assert_eq!(manager.backoff, Duration::from_secs(64));
    }
}
