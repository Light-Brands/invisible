//! Scrambler Orchestrator
//!
//! Integrates all 7 layers of the Scrambler network obfuscation system:
//! - Layer 0: Ghost VPN (mandatory WireGuard tunnel)
//! - Layer 1: Shamir Fragmentation (K-of-N secret sharing)
//! - Layer 2: 5-Layer Mixnet (Sphinx packets)
//! - Layer 3: Cover Traffic (constant-rate stream)
//! - Layer 7: Temporal Scrambling (Poisson delays)
//!
//! Provides unified API for sending messages through all privacy layers.

use std::time::Duration;

use crate::cover_traffic::{CoverTrafficConfig, CoverTrafficGenerator};
use crate::dead_drop::{DeadDropConfig};
use crate::error::{Result, ScramblerError};
use crate::mixnet::{select_route, Jurisdiction, MixNode};
use crate::network::{MixNodeAddr, NetworkConfig, PacketTransmitter, ResponseCollector};
use crate::shamir::{split_secret, reconstruct_secret, ShamirConfig};
use crate::sphinx::{build_packet, RouteSpec};
use crate::temporal::{TemporalConfig, TemporalDelayGenerator};
use crate::vpn::{VpnConfig, VpnManager};

/// Scrambler orchestrator configuration
#[derive(Debug, Clone)]
pub struct ScramblerConfig {
    /// VPN configuration (Layer 0)
    pub vpn: VpnConfig,
    /// Shamir secret sharing (Layer 1)
    pub shamir: ShamirConfig,
    /// Cover traffic generation (Layer 3)
    pub cover_traffic: CoverTrafficConfig,
    /// Temporal delay (Layer 7)
    pub temporal: TemporalConfig,
    /// Network transmission
    pub network: NetworkConfig,
    /// Dead drop configuration
    pub dead_drop: DeadDropConfig,
    /// Avoid specific jurisdictions
    pub avoid_jurisdiction: Option<Jurisdiction>,
}

impl Default for ScramblerConfig {
    fn default() -> Self {
        use std::net::{IpAddr, Ipv4Addr, SocketAddr};
        use crate::vpn::{VpnEndpoint};

        Self {
            vpn: VpnConfig {
                private_key: vec![0u8; 32], // Placeholder - should be generated
                local_address: "10.0.0.2/24".to_string(),
                endpoints: vec![
                    VpnEndpoint {
                        public_key: vec![0u8; 32],
                        address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4)), 51820),
                        location: "US-East".to_string(),
                        latency_ms: None,
                    },
                ],
                keepalive_interval: 25,
                max_session_time: 3600, // 1 hour
            },
            shamir: ShamirConfig::default(),
            cover_traffic: CoverTrafficConfig::default(),
            temporal: TemporalConfig::default(),
            network: NetworkConfig::default(),
            dead_drop: DeadDropConfig::default(),
            avoid_jurisdiction: Some(Jurisdiction::FiveEyes),
        }
    }
}

/// Scrambler orchestrator
///
/// Manages all layers of network obfuscation and provides a unified API.
#[derive(Debug)]
pub struct Scrambler {
    /// Configuration
    config: ScramblerConfig,
    /// VPN manager (Layer 0)
    vpn: VpnManager,
    /// Cover traffic generator (Layer 3)
    cover_traffic: CoverTrafficGenerator,
    /// Temporal delay generator (Layer 7)
    temporal: TemporalDelayGenerator,
    /// Available mix nodes (Layer 2)
    mix_nodes: Vec<MixNode>,
    /// Network packet transmitter
    packet_transmitter: PacketTransmitter,
    /// RPC response collector
    response_collector: ResponseCollector,
}

impl Scrambler {
    /// Create a new Scrambler orchestrator
    pub fn new(config: ScramblerConfig, mix_nodes: Vec<MixNode>) -> Self {
        let vpn = VpnManager::new(config.vpn.clone());
        let cover_traffic = CoverTrafficGenerator::new(config.cover_traffic.clone());
        let temporal = TemporalDelayGenerator::new(config.temporal.clone());
        let packet_transmitter = PacketTransmitter::new(config.network.clone());
        let response_collector = ResponseCollector::new(
            config.network.clone(),
            config.dead_drop.clone(),
            config.shamir.clone(),
        );

        Self {
            config,
            vpn,
            cover_traffic,
            temporal,
            mix_nodes,
            packet_transmitter,
            response_collector,
        }
    }

    /// Initialize the scrambler
    ///
    /// Connects to VPN and starts background tasks.
    pub async fn initialize(&mut self) -> Result<()> {
        // Connect to VPN (Layer 0)
        self.vpn.connect().await?;

        tracing::info!("VPN connected, scrambler initialized");

        // Note: Background tasks like cover traffic and maintenance should be
        // spawned by the caller using the maintain() method in a loop.
        // This keeps the API simple and gives caller control over task lifecycle.
        //
        // Example usage:
        //   scrambler.initialize().await?;
        //   tokio::spawn(async move {
        //       loop {
        //           if let Err(e) = scrambler.maintain().await {
        //               tracing::error!("Maintenance error: {}", e);
        //           }
        //           tokio::time::sleep(Duration::from_secs(30)).await;
        //       }
        //   });

        Ok(())
    }

    /// Send a message through all privacy layers
    ///
    /// # Arguments
    /// * `message` - The plaintext message to send
    /// * `destination` - Destination public key
    ///
    /// # Returns
    /// * `MessageHandle` - Handle to track message delivery
    pub async fn send_message(
        &mut self,
        message: &[u8],
        destination: &[u8],
    ) -> Result<MessageHandle> {
        // Ensure VPN is connected (Layer 0)
        if !self.vpn.is_connected() {
            return Err(ScramblerError::VpnError(
                "VPN not connected".to_string(),
            ));
        }

        // Layer 1: Fragment message using Shamir secret sharing
        let shares = split_secret(message, &self.config.shamir)?;

        tracing::debug!(
            shares = shares.len(),
            threshold = self.config.shamir.threshold,
            "Message fragmented"
        );

        // Layer 2: Create Sphinx packets through mixnet
        let mut packet_handles = Vec::new();

        for share in shares.iter() {
            // Select route through mixnet
            let route = select_route(&self.mix_nodes, self.config.avoid_jurisdiction)?;

            // Create route specification
            let route_spec = RouteSpec {
                node_keys: route.iter().map(|node| node.public_key.clone()).collect(),
                destination: destination.to_vec(),
            };

            // Create Sphinx packet
            let packet = build_packet(&route_spec, &share.data)?;

            // Layer 7: Apply temporal delay
            let delay = self.temporal.generate_delay();

            packet_handles.push(PacketHandle {
                packet,
                route: route.iter().map(|n| n.id).collect(),
                delay,
            });
        }

        tracing::info!(
            packets = packet_handles.len(),
            "Message prepared for transmission"
        );

        Ok(MessageHandle {
            id: generate_message_id(),
            packet_handles,
        })
    }

    /// Receive and reconstruct a message from shares
    ///
    /// # Arguments
    /// * `shares` - Received message shares
    ///
    /// # Returns
    /// * `Vec<u8>` - Reconstructed plaintext message
    pub fn receive_message(&self, shares: &[Vec<u8>]) -> Result<Vec<u8>> {
        use crate::shamir::Share;

        // Convert to Shamir shares
        let shamir_shares: Vec<Share> = shares
            .iter()
            .enumerate()
            .map(|(i, data)| Share {
                index: (i + 1) as u8,
                data: data.clone(),
            })
            .collect();

        // Reconstruct message from shares
        let message = reconstruct_secret(&shamir_shares, &self.config.shamir)?;

        tracing::info!(
            shares = shamir_shares.len(),
            message_size = message.len(),
            "Message reconstructed"
        );

        Ok(message)
    }

    /// Route RPC call through Scrambler (Privacy Parity for Wallet Operations)
    ///
    /// Routes blockchain RPC requests through the full 8-layer Scrambler stack,
    /// ensuring wallet queries have the same privacy as messages.
    ///
    /// # Arguments
    /// * `rpc_request` - Serialized RPC request (JSON-RPC usually)
    /// * `destination` - Destination node public key (blockchain node)
    ///
    /// # Returns
    /// * `Vec<u8>` - RPC response from destination
    pub async fn route_rpc_call(
        &mut self,
        rpc_request: &[u8],
        destination: &[u8],
    ) -> Result<Vec<u8>> {
        // Ensure VPN is connected
        if !self.vpn.is_connected() {
            return Err(ScramblerError::VpnError(
                "VPN not connected - cannot route RPC".to_string(),
            ));
        }

        tracing::info!(
            request_size = rpc_request.len(),
            "Routing RPC call through Scrambler"
        );

        // Layer 1: Fragment RPC request using Shamir secret sharing
        // This prevents any single node from seeing the full request
        let shares = split_secret(rpc_request, &self.config.shamir)?;

        // Layer 2: Route each share through different mixnet paths
        // This provides path diversity and timing obfuscation
        let mut packet_handles = Vec::new();
        let mut access_tokens = Vec::new();
        let mut drop_nodes = Vec::new();

        for share in shares.iter() {
            // Select independent route for this share
            let route = select_route(&self.mix_nodes, self.config.avoid_jurisdiction)?;

            // Create route specification
            let route_spec = RouteSpec {
                node_keys: route.iter().map(|node| node.public_key.clone()).collect(),
                destination: destination.to_vec(),
            };

            // Wrap in Sphinx packet
            let packet = build_packet(&route_spec, &share.data)?;

            // Layer 7: Apply temporal delay
            let delay = self.temporal.generate_delay();

            // Prepare dead drop for response
            // Derive access token from share data
            let access_token = self.response_collector.dead_drop.derive_access_token(
                &share.data
            );

            access_tokens.push(access_token);

            // Use last node in route as dead drop node
            if let Some(last_node) = route.last() {
                drop_nodes.push(MixNodeAddr {
                    address: last_node.address.clone(),
                    public_key: last_node.public_key.clone(),
                });
            } else {
                return Err(ScramblerError::NetworkError(
                    "Empty route selected".to_string()
                ));
            }

            packet_handles.push((packet, route, delay));

            tracing::debug!(
                share_index = share.index,
                delay_ms = delay.as_millis(),
                "RPC share routed"
            );
        }

        // Step 1: Send packets with temporal delays
        for (packet, route, delay) in packet_handles {
            // Apply temporal delay
            tokio::time::sleep(delay).await;

            // Send packet through first hop
            if let Some(first_node) = route.first() {
                let node_addr = MixNodeAddr {
                    address: first_node.address.clone(),
                    public_key: first_node.public_key.clone(),
                };

                self.packet_transmitter.send_packet(&packet, &node_addr).await?;

                tracing::debug!(
                    first_hop = %first_node.address,
                    "RPC share packet transmitted"
                );
            } else {
                return Err(ScramblerError::NetworkError(
                    "Empty route - cannot send packet".to_string()
                ));
            }
        }

        // Step 2: Collect responses from dead drops
        // Maximum wait time: 30 seconds for all shares to arrive
        let max_wait = Duration::from_secs(30);

        let response = self.response_collector.collect_response(
            &access_tokens,
            &drop_nodes,
            max_wait,
        ).await?;

        tracing::info!(
            response_size = response.len(),
            shares = access_tokens.len(),
            "RPC response collected and reconstructed"
        );

        Ok(response)
    }

    /// Generate cover traffic
    ///
    /// Should be called periodically to maintain constant-rate traffic.
    pub async fn generate_cover_traffic(&self) -> Result<()> {
        use rand::seq::SliceRandom;

        let packet = self.cover_traffic.generate_cover_packet()?;
        let delay = self.cover_traffic.next_delay();

        // Select a random mix node to send cover traffic through
        if let Some(node) = self.mix_nodes.choose(&mut rand::thread_rng()) {
            let node_addr = MixNodeAddr {
                address: node.address.clone(),
                public_key: node.public_key.clone(),
            };

            // Send cover packet (errors are logged but don't stop cover traffic)
            match self.packet_transmitter.send_packet(&packet, &node_addr).await {
                Ok(_) => {
                    tracing::debug!(
                        node = %hex::encode(&node.id[0..8]),
                        delay_ms = delay.as_millis(),
                        "Cover traffic packet sent"
                    );
                }
                Err(e) => {
                    tracing::warn!(
                        error = %e,
                        "Failed to send cover traffic packet (non-critical)"
                    );
                }
            }
        }

        tokio::time::sleep(delay).await;

        Ok(())
    }

    /// Maintain scrambler state
    ///
    /// Should be called periodically to:
    /// - Check VPN connection health
    /// - Generate cover traffic
    /// - Clean up expired state
    pub async fn maintain(&mut self) -> Result<()> {
        // Maintain VPN connection
        self.vpn.maintain().await?;

        // Generate cover traffic
        self.generate_cover_traffic().await?;

        Ok(())
    }

    /// Get VPN status
    pub fn vpn_connected(&self) -> bool {
        self.vpn.is_connected()
    }

    /// Get VPN uptime
    pub fn vpn_uptime(&self) -> Option<Duration> {
        self.vpn.uptime()
    }
}

/// Handle for tracking message delivery
#[derive(Debug)]
pub struct MessageHandle {
    /// Unique message ID
    pub id: [u8; 16],
    /// Packet handles for each share
    packet_handles: Vec<PacketHandle>,
}

/// Handle for a single packet
#[derive(Debug)]
struct PacketHandle {
    /// The Sphinx packet
    packet: crate::sphinx::SphinxPacket,
    /// Route through mixnet (node IDs)
    route: Vec<[u8; 32]>,
    /// Delay before sending
    delay: Duration,
}

/// Generate a unique message ID
fn generate_message_id() -> [u8; 16] {
    use rand::RngCore;
    let mut id = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut id);
    id
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mixnet::{GeoLocation, Jurisdiction, MixNode};

    fn create_test_nodes() -> Vec<MixNode> {
        let mut nodes = Vec::new();
        for layer in 0..5 {
            nodes.push(MixNode {
                id: [layer; 32],
                layer,
                public_key: vec![layer; 32],
                address: format!("127.0.0.{}:8080", layer),
                location: GeoLocation {
                    country: "CH".to_string(),
                    jurisdiction: Jurisdiction::PrivacyFriendly,
                },
            });
        }
        nodes
    }

    #[tokio::test]
    #[ignore] // Requires WireGuard installed and sudo privileges
    async fn test_scrambler_initialization() {
        let config = ScramblerConfig::default();
        let nodes = create_test_nodes();
        let mut scrambler = Scrambler::new(config, nodes);

        // Should initialize successfully
        scrambler.initialize().await.unwrap();
        assert!(scrambler.vpn_connected());
    }

    #[tokio::test]
    #[ignore] // Requires WireGuard installed and sudo privileges
    async fn test_message_fragmentation() {
        let config = ScramblerConfig::default();
        let nodes = create_test_nodes();
        let mut scrambler = Scrambler::new(config, nodes);
        scrambler.initialize().await.unwrap();

        let message = b"Test message for fragmentation";
        let destination = vec![0u8; 32];

        // Should fragment and create packets
        let handle = scrambler
            .send_message(message, &destination)
            .await
            .unwrap();

        // Should create N shares (default 5)
        assert_eq!(handle.packet_handles.len(), 5);
    }

    #[test]
    fn test_message_reconstruction() {
        let config = ScramblerConfig::default();
        let nodes = create_test_nodes();
        let scrambler = Scrambler::new(config, nodes);

        let message = b"Test message";

        // Split message
        let shares = split_secret(message, &scrambler.config.shamir).unwrap();

        // Reconstruct from shares
        let share_data: Vec<Vec<u8>> = shares.iter().map(|s| s.data.clone()).collect();
        let reconstructed = scrambler.receive_message(&share_data).unwrap();

        assert_eq!(reconstructed, message);
    }

    #[tokio::test]
    async fn test_vpn_not_connected() {
        let config = ScramblerConfig::default();
        let nodes = create_test_nodes();
        let mut scrambler = Scrambler::new(config, nodes);

        // Should fail without VPN connection
        let message = b"Test message";
        let destination = vec![0u8; 32];
        let result = scrambler.send_message(message, &destination).await;

        assert!(result.is_err());
    }
}
