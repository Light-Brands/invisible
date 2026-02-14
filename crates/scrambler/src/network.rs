//! Network Transmission Layer
//!
//! Handles actual transmission of Sphinx packets through the network:
//! - TCP/UDP transport to mix nodes
//! - Dead drop protocol implementation
//! - Connection pooling and management
//! - Retry logic and error handling
//!
//! ## Architecture
//!
//! - **PacketTransmitter:** Sends Sphinx packets to mix nodes
//! - **DeadDropProtocol:** Stores/retrieves messages from dead drop nodes
//! - **ConnectionPool:** Manages persistent connections to reduce latency
//! - **RetryPolicy:** Handles transient failures with exponential backoff

use std::net::SocketAddr;
use std::time::Duration;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::{sleep, timeout};
use serde::{Deserialize, Serialize};

use crate::error::{Result, ScramblerError};
use crate::sphinx::SphinxPacket;
use crate::dead_drop::{AccessToken, DeadDropClient, DeadDropConfig, StoredMessage};
use crate::shamir::{Share, reconstruct_secret, ShamirConfig};

/// Network protocol for mix node communication
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkProtocol {
    /// TCP transport (reliable)
    Tcp,
    /// UDP transport (faster, unreliable)
    Udp,
}

/// Network configuration
#[derive(Debug, Clone)]
pub struct NetworkConfig {
    /// Connection timeout (ms)
    pub connect_timeout_ms: u64,
    /// Read timeout (ms)
    pub read_timeout_ms: u64,
    /// Write timeout (ms)
    pub write_timeout_ms: u64,
    /// Maximum retry attempts
    pub max_retries: usize,
    /// Base retry delay (ms)
    pub retry_delay_ms: u64,
    /// Protocol to use
    pub protocol: NetworkProtocol,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            connect_timeout_ms: 5000,    // 5 seconds
            read_timeout_ms: 30000,      // 30 seconds
            write_timeout_ms: 5000,      // 5 seconds
            max_retries: 3,
            retry_delay_ms: 1000,        // 1 second
            protocol: NetworkProtocol::Tcp,
        }
    }
}

/// Mix node address information
#[derive(Debug, Clone)]
pub struct MixNodeAddr {
    /// Node address (host:port)
    pub address: String,
    /// Node public key
    pub public_key: Vec<u8>,
}

impl MixNodeAddr {
    /// Parse address string to SocketAddr
    pub fn socket_addr(&self) -> Result<SocketAddr> {
        self.address
            .parse()
            .map_err(|e| ScramblerError::NetworkError(format!("Invalid address: {}", e)))
    }
}

/// Wire protocol message types
#[derive(Debug, Clone, Serialize, Deserialize)]
enum WireMessage {
    /// Forward Sphinx packet to next hop
    ForwardPacket {
        packet: SphinxPacket,
    },
    /// Store message in dead drop
    StoreDeadDrop {
        drop_id: [u8; 32],
        access_token: AccessToken,
        payload: Vec<u8>,
    },
    /// Retrieve messages from dead drop
    RetrieveDeadDrop {
        access_token: AccessToken,
    },
    /// Response: stored successfully
    StoreSuccess {
        message_id: [u8; 16],
    },
    /// Response: retrieved messages
    RetrieveSuccess {
        messages: Vec<StoredMessage>,
    },
    /// Error response
    Error {
        message: String,
    },
}

/// Packet transmitter
///
/// Sends Sphinx packets through the network to mix nodes.
#[derive(Debug)]
pub struct PacketTransmitter {
    config: NetworkConfig,
}

impl PacketTransmitter {
    /// Create a new packet transmitter
    pub fn new(config: NetworkConfig) -> Self {
        Self { config }
    }

    /// Send a Sphinx packet to a mix node
    ///
    /// # Arguments
    /// * `packet` - The Sphinx packet to send
    /// * `node` - Destination mix node
    ///
    /// # Returns
    /// * Result indicating success or failure
    pub async fn send_packet(
        &self,
        packet: &SphinxPacket,
        node: &MixNodeAddr,
    ) -> Result<()> {
        let mut attempts = 0;
        let mut last_error = None;

        while attempts < self.config.max_retries {
            match self.try_send_packet(packet, node).await {
                Ok(_) => {
                    tracing::debug!(
                        node = %node.address,
                        attempt = attempts + 1,
                        "Packet sent successfully"
                    );
                    return Ok(());
                }
                Err(e) => {
                    tracing::warn!(
                        node = %node.address,
                        attempt = attempts + 1,
                        error = %e,
                        "Packet send failed, retrying"
                    );
                    last_error = Some(e);
                    attempts += 1;

                    if attempts < self.config.max_retries {
                        // Exponential backoff
                        let delay = self.config.retry_delay_ms * (1 << attempts);
                        sleep(Duration::from_millis(delay)).await;
                    }
                }
            }
        }

        Err(last_error.unwrap_or_else(|| {
            ScramblerError::NetworkError("Send failed after retries".to_string())
        }))
    }

    /// Single attempt to send a packet
    async fn try_send_packet(
        &self,
        packet: &SphinxPacket,
        node: &MixNodeAddr,
    ) -> Result<()> {
        let addr = node.socket_addr()?;

        // Connect with timeout
        let connect_future = TcpStream::connect(&addr);
        let mut stream = timeout(
            Duration::from_millis(self.config.connect_timeout_ms),
            connect_future,
        )
        .await
        .map_err(|_| ScramblerError::NetworkError("Connection timeout".to_string()))?
        .map_err(|e| ScramblerError::NetworkError(format!("Connection failed: {}", e)))?;

        // Serialize packet with wire protocol
        let message = WireMessage::ForwardPacket {
            packet: packet.clone(),
        };

        let serialized = bincode::serialize(&message)
            .map_err(|e| ScramblerError::NetworkError(format!("Serialization failed: {}", e)))?;

        // Send length prefix (4 bytes)
        let len = serialized.len() as u32;
        let len_bytes = len.to_be_bytes();

        // Write with timeout
        let write_future = async {
            stream.write_all(&len_bytes).await?;
            stream.write_all(&serialized).await?;
            stream.flush().await?;
            Ok::<(), std::io::Error>(())
        };

        timeout(
            Duration::from_millis(self.config.write_timeout_ms),
            write_future,
        )
        .await
        .map_err(|_| ScramblerError::NetworkError("Write timeout".to_string()))?
        .map_err(|e| ScramblerError::NetworkError(format!("Write failed: {}", e)))?;

        tracing::debug!(
            bytes_sent = serialized.len(),
            node = %node.address,
            "Packet transmitted"
        );

        Ok(())
    }
}

/// Dead drop protocol handler
///
/// Implements the dead drop protocol for storing and retrieving messages.
#[derive(Debug)]
pub struct DeadDropProtocol {
    config: NetworkConfig,
    client: DeadDropClient,
}

impl DeadDropProtocol {
    /// Create a new dead drop protocol handler
    pub fn new(config: NetworkConfig, drop_config: DeadDropConfig) -> Self {
        Self {
            config,
            client: DeadDropClient::new(drop_config),
        }
    }

    /// Store a message in a dead drop node
    ///
    /// # Arguments
    /// * `node` - Dead drop node address
    /// * `drop_id` - Drop identifier
    /// * `access_token` - Access token for retrieval
    /// * `payload` - Message payload
    ///
    /// # Returns
    /// * Message ID
    pub async fn store(
        &self,
        node: &MixNodeAddr,
        drop_id: [u8; 32],
        access_token: AccessToken,
        payload: Vec<u8>,
    ) -> Result<[u8; 16]> {
        let addr = node.socket_addr()?;

        // Connect
        let connect_future = TcpStream::connect(&addr);
        let mut stream = timeout(
            Duration::from_millis(self.config.connect_timeout_ms),
            connect_future,
        )
        .await
        .map_err(|_| ScramblerError::NetworkError("Connection timeout".to_string()))?
        .map_err(|e| ScramblerError::NetworkError(format!("Connection failed: {}", e)))?;

        // Send store request
        let message = WireMessage::StoreDeadDrop {
            drop_id,
            access_token,
            payload,
        };

        let serialized = bincode::serialize(&message)
            .map_err(|e| ScramblerError::NetworkError(format!("Serialization failed: {}", e)))?;

        // Write request
        let len = serialized.len() as u32;
        stream.write_all(&len.to_be_bytes()).await
            .map_err(|e| ScramblerError::NetworkError(format!("Write failed: {}", e)))?;
        stream.write_all(&serialized).await
            .map_err(|e| ScramblerError::NetworkError(format!("Write failed: {}", e)))?;
        stream.flush().await
            .map_err(|e| ScramblerError::NetworkError(format!("Flush failed: {}", e)))?;

        // Read response
        let mut len_bytes = [0u8; 4];
        let read_future = stream.read_exact(&mut len_bytes);
        timeout(
            Duration::from_millis(self.config.read_timeout_ms),
            read_future,
        )
        .await
        .map_err(|_| ScramblerError::NetworkError("Read timeout".to_string()))?
        .map_err(|e| ScramblerError::NetworkError(format!("Read failed: {}", e)))?;

        let response_len = u32::from_be_bytes(len_bytes) as usize;
        let mut response_bytes = vec![0u8; response_len];
        stream.read_exact(&mut response_bytes).await
            .map_err(|e| ScramblerError::NetworkError(format!("Read failed: {}", e)))?;

        let response: WireMessage = bincode::deserialize(&response_bytes)
            .map_err(|e| ScramblerError::NetworkError(format!("Deserialization failed: {}", e)))?;

        match response {
            WireMessage::StoreSuccess { message_id } => {
                tracing::debug!(
                    message_id = ?message_id,
                    node = %node.address,
                    "Message stored in dead drop"
                );
                Ok(message_id)
            }
            WireMessage::Error { message } => Err(ScramblerError::NetworkError(
                format!("Dead drop store failed: {}", message),
            )),
            _ => Err(ScramblerError::NetworkError(
                "Unexpected response from dead drop".to_string(),
            )),
        }
    }

    /// Retrieve messages from a dead drop node
    ///
    /// # Arguments
    /// * `node` - Dead drop node address
    /// * `access_token` - Access token
    ///
    /// # Returns
    /// * List of stored messages
    pub async fn retrieve(
        &self,
        node: &MixNodeAddr,
        access_token: &AccessToken,
    ) -> Result<Vec<StoredMessage>> {
        let addr = node.socket_addr()?;

        // Connect
        let connect_future = TcpStream::connect(&addr);
        let mut stream = timeout(
            Duration::from_millis(self.config.connect_timeout_ms),
            connect_future,
        )
        .await
        .map_err(|_| ScramblerError::NetworkError("Connection timeout".to_string()))?
        .map_err(|e| ScramblerError::NetworkError(format!("Connection failed: {}", e)))?;

        // Send retrieve request
        let message = WireMessage::RetrieveDeadDrop {
            access_token: *access_token,
        };

        let serialized = bincode::serialize(&message)
            .map_err(|e| ScramblerError::NetworkError(format!("Serialization failed: {}", e)))?;

        // Write request
        let len = serialized.len() as u32;
        stream.write_all(&len.to_be_bytes()).await
            .map_err(|e| ScramblerError::NetworkError(format!("Write failed: {}", e)))?;
        stream.write_all(&serialized).await
            .map_err(|e| ScramblerError::NetworkError(format!("Write failed: {}", e)))?;
        stream.flush().await
            .map_err(|e| ScramblerError::NetworkError(format!("Flush failed: {}", e)))?;

        // Read response
        let mut len_bytes = [0u8; 4];
        let read_future = stream.read_exact(&mut len_bytes);
        timeout(
            Duration::from_millis(self.config.read_timeout_ms),
            read_future,
        )
        .await
        .map_err(|_| ScramblerError::NetworkError("Read timeout".to_string()))?
        .map_err(|e| ScramblerError::NetworkError(format!("Read failed: {}", e)))?;

        let response_len = u32::from_be_bytes(len_bytes) as usize;
        let mut response_bytes = vec![0u8; response_len];
        stream.read_exact(&mut response_bytes).await
            .map_err(|e| ScramblerError::NetworkError(format!("Read failed: {}", e)))?;

        let response: WireMessage = bincode::deserialize(&response_bytes)
            .map_err(|e| ScramblerError::NetworkError(format!("Deserialization failed: {}", e)))?;

        match response {
            WireMessage::RetrieveSuccess { messages } => {
                tracing::debug!(
                    count = messages.len(),
                    node = %node.address,
                    "Messages retrieved from dead drop"
                );
                Ok(messages)
            }
            WireMessage::Error { message } => Err(ScramblerError::NetworkError(
                format!("Dead drop retrieve failed: {}", message),
            )),
            _ => Err(ScramblerError::NetworkError(
                "Unexpected response from dead drop".to_string(),
            )),
        }
    }

    /// Derive drop ID from recipient key
    pub fn derive_drop_id(&self, recipient_key: &[u8]) -> [u8; 32] {
        self.client.derive_drop_id(recipient_key)
    }

    /// Derive access token from shared secret
    pub fn derive_access_token(&self, shared_secret: &[u8]) -> AccessToken {
        self.client.derive_access_token(shared_secret)
    }
}

/// RPC response collector
///
/// Collects RPC response shares from dead drops and reconstructs the response.
#[derive(Debug)]
pub struct ResponseCollector {
    /// Dead drop protocol handler
    pub dead_drop: DeadDropProtocol,
    shamir_config: ShamirConfig,
}

impl ResponseCollector {
    /// Create a new response collector
    pub fn new(
        network_config: NetworkConfig,
        drop_config: DeadDropConfig,
        shamir_config: ShamirConfig,
    ) -> Self {
        Self {
            dead_drop: DeadDropProtocol::new(network_config, drop_config),
            shamir_config,
        }
    }

    /// Collect RPC response from dead drops
    ///
    /// # Arguments
    /// * `access_tokens` - Access tokens for dead drops (one per share)
    /// * `drop_nodes` - Dead drop node addresses
    /// * `max_wait` - Maximum wait time for responses
    ///
    /// # Returns
    /// * Reconstructed RPC response
    pub async fn collect_response(
        &self,
        access_tokens: &[AccessToken],
        drop_nodes: &[MixNodeAddr],
        max_wait: Duration,
    ) -> Result<Vec<u8>> {
        if access_tokens.len() != drop_nodes.len() {
            return Err(ScramblerError::NetworkError(
                "Access tokens and nodes count mismatch".to_string(),
            ));
        }

        // Poll dead drops for shares
        let poll_start = std::time::Instant::now();
        let mut shares: Vec<Share> = Vec::new();

        while shares.len() < self.shamir_config.threshold {
            if poll_start.elapsed() > max_wait {
                return Err(ScramblerError::NetworkError(format!(
                    "Timeout waiting for responses ({}/{})",
                    shares.len(),
                    self.shamir_config.threshold
                )));
            }

            // Try to retrieve from each dead drop
            for (i, (token, node)) in access_tokens.iter().zip(drop_nodes.iter()).enumerate() {
                // Skip if we already have this share
                if shares.iter().any(|s| s.index == (i + 1) as u8) {
                    continue;
                }

                // Try to retrieve messages
                match self.dead_drop.retrieve(node, token).await {
                    Ok(messages) => {
                        if !messages.is_empty() {
                            // Take first message as the share
                            let message = &messages[0];
                            shares.push(Share {
                                index: (i + 1) as u8,
                                data: message.payload.clone(),
                            });

                            tracing::debug!(
                                share_index = i + 1,
                                shares_collected = shares.len(),
                                threshold = self.shamir_config.threshold,
                                "Share retrieved"
                            );

                            // Check if we have enough shares
                            if shares.len() >= self.shamir_config.threshold {
                                break;
                            }
                        }
                    }
                    Err(e) => {
                        tracing::warn!(
                            node = %node.address,
                            error = %e,
                            "Failed to retrieve from dead drop"
                        );
                    }
                }
            }

            // If we don't have enough shares, wait before next poll
            if shares.len() < self.shamir_config.threshold {
                sleep(Duration::from_millis(500)).await;
            }
        }

        // Reconstruct response from shares
        let response = reconstruct_secret(&shares, &self.shamir_config)?;

        tracing::info!(
            shares_used = shares.len(),
            response_size = response.len(),
            "RPC response reconstructed"
        );

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_config_defaults() {
        let config = NetworkConfig::default();
        assert_eq!(config.connect_timeout_ms, 5000);
        assert_eq!(config.max_retries, 3);
        assert_eq!(config.protocol, NetworkProtocol::Tcp);
    }

    #[test]
    fn test_mix_node_addr_parsing() {
        let node = MixNodeAddr {
            address: "127.0.0.1:8080".to_string(),
            public_key: vec![0u8; 32],
        };

        let socket_addr = node.socket_addr().unwrap();
        assert_eq!(socket_addr.port(), 8080);
    }

    #[test]
    fn test_invalid_address() {
        let node = MixNodeAddr {
            address: "invalid_address".to_string(),
            public_key: vec![0u8; 32],
        };

        assert!(node.socket_addr().is_err());
    }

    #[test]
    fn test_dead_drop_token_derivation() {
        let config = NetworkConfig::default();
        let drop_config = DeadDropConfig::default();
        let protocol = DeadDropProtocol::new(config, drop_config);

        let recipient_key = b"test_recipient_key";
        let shared_secret = b"test_shared_secret";

        let drop_id = protocol.derive_drop_id(recipient_key);
        let token = protocol.derive_access_token(shared_secret);

        // Should be deterministic
        assert_eq!(drop_id, protocol.derive_drop_id(recipient_key));
        assert_eq!(token, protocol.derive_access_token(shared_secret));

        // Different inputs should produce different outputs
        let drop_id2 = protocol.derive_drop_id(b"different_key");
        assert_ne!(drop_id, drop_id2);
    }

    #[tokio::test]
    #[ignore] // Requires mix node running
    async fn test_packet_transmission() {
        use crate::sphinx::{build_packet, RouteSpec};
        use invisible_crypto::keys::KeyPair;

        let config = NetworkConfig::default();
        let transmitter = PacketTransmitter::new(config);

        // Create test packet
        let keypair = KeyPair::generate().unwrap();
        let route = RouteSpec {
            node_keys: vec![keypair.public_key().to_vec()],
            destination: vec![0u8; 32],
        };
        let packet = build_packet(&route, b"test message").unwrap();

        // Create test node (assumes mix node running on localhost:8080)
        let node = MixNodeAddr {
            address: "127.0.0.1:8080".to_string(),
            public_key: keypair.public_key().to_vec(),
        };

        // Try to send (will fail if no node running, but tests code paths)
        let result = transmitter.send_packet(&packet, &node).await;
        // We expect it to fail in CI, but the code should compile
        let _ = result;
    }
}
