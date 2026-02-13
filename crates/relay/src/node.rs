//! Mix Node Implementation
//!
//! Relay nodes process Sphinx packets through the mixnet.

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::net::SocketAddr;

use invisible_scrambler::{
    dead_drop::{DeadDropNode, DeadDropConfig},
    mixnet::{GeoLocation, Jurisdiction, MixNodeState, MixStrategy},
    sphinx::{SphinxPacket, process_packet, ProcessedPacket},
};

use crate::error::{Result, RelayError};

/// Mix node configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConfig {
    /// Node identifier
    pub node_id: [u8; 32],
    /// Node layer (0-4)
    pub layer: u8,
    /// Private key
    pub private_key: Vec<u8>,
    /// Public key
    pub public_key: Vec<u8>,
    /// Listen address
    pub listen_addr: SocketAddr,
    /// Geographic location
    pub location: GeoLocation,
    /// Mix strategy
    pub mix_strategy: MixStrategy,
    /// Dead drop config
    pub dead_drop_config: DeadDropConfig,
}

impl Default for NodeConfig {
    fn default() -> Self {
        use std::net::{IpAddr, Ipv4Addr};

        Self {
            node_id: [0u8; 32],
            layer: 0,
            private_key: vec![0u8; 32],
            public_key: vec![0u8; 32],
            listen_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 8080),
            location: GeoLocation {
                country: "CH".to_string(),
                jurisdiction: Jurisdiction::PrivacyFriendly,
            },
            mix_strategy: MixStrategy::default(),
            dead_drop_config: DeadDropConfig::default(),
        }
    }
}

/// Node statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NodeStats {
    /// Packets received
    pub packets_received: u64,
    /// Packets forwarded
    pub packets_forwarded: u64,
    /// Packets delivered
    pub packets_delivered: u64,
    /// Current batch size
    pub current_batch_size: usize,
    /// Dead drop messages
    pub dead_drop_messages: usize,
}

/// Mix node
#[derive(Debug)]
pub struct MixNode {
    config: NodeConfig,
    mix_state: MixNodeState,
    dead_drop: DeadDropNode,
    stats: NodeStats,
    output_queue: VecDeque<(SphinxPacket, SocketAddr)>,
}

impl MixNode {
    /// Create new mix node
    pub fn new(config: NodeConfig) -> Self {
        let mix_node = invisible_scrambler::mixnet::MixNode {
            id: config.node_id,
            layer: config.layer,
            public_key: config.public_key.clone(),
            address: config.listen_addr.to_string(),
            location: config.location.clone(),
        };

        let mix_state = MixNodeState::new(mix_node, config.mix_strategy.clone());
        let dead_drop = DeadDropNode::new(config.dead_drop_config.clone());

        Self {
            config,
            mix_state,
            dead_drop,
            stats: NodeStats::default(),
            output_queue: VecDeque::new(),
        }
    }

    /// Process incoming packet
    pub async fn process_packet(&mut self, packet: SphinxPacket) -> Result<()> {
        self.stats.packets_received += 1;

        match process_packet(&packet, &self.config.private_key)? {
            ProcessedPacket::Forward { packet, next_hop } => {
                self.mix_state.add_packet(packet);

                tracing::debug!("Packet queued for forwarding");

                if self.mix_state.should_forward() {
                    self.forward_batch().await?;
                }
            }
            ProcessedPacket::Deliver { message } => {
                self.handle_final_payload(message).await?;
                self.stats.packets_delivered += 1;
            }
        }

        Ok(())
    }

    async fn forward_batch(&mut self) -> Result<()> {
        let packets = self.mix_state.mix_and_extract();

        for packet in packets {
            let next_hop = self.config.listen_addr; // TODO: Extract from packet
            self.output_queue.push_back((packet, next_hop));
            self.stats.packets_forwarded += 1;
        }

        Ok(())
    }

    async fn handle_final_payload(&mut self, payload: Vec<u8>) -> Result<()> {
        if payload.starts_with(b"DEADROP_STORE:") {
            self.handle_dead_drop_store(payload)?;
        } else {
            tracing::debug!(size = payload.len(), "Message delivered");
        }

        Ok(())
    }

    fn handle_dead_drop_store(&mut self, payload: Vec<u8>) -> Result<()> {
        if payload.len() < 78 {
            return Err(RelayError::InvalidPacket("Payload too short".to_string()));
        }

        let mut drop_id = [0u8; 32];
        drop_id.copy_from_slice(&payload[14..46]);

        let mut access_token = [0u8; 32];
        access_token.copy_from_slice(&payload[46..78]);

        let message = payload[78..].to_vec();

        self.dead_drop.store_message(drop_id, access_token, message)?;
        self.stats.dead_drop_messages = self.dead_drop.stats().total_messages;

        Ok(())
    }

    /// Get next output packet
    pub fn next_output(&mut self) -> Option<(SphinxPacket, SocketAddr)> {
        self.output_queue.pop_front()
    }

    /// Periodic maintenance
    pub async fn maintain(&mut self) -> Result<()> {
        self.dead_drop.cleanup_expired();

        if self.mix_state.should_forward() {
            self.forward_batch().await?;
        }

        self.stats.dead_drop_messages = self.dead_drop.stats().total_messages;
        self.stats.current_batch_size = self.output_queue.len();

        Ok(())
    }

    /// Get statistics
    pub fn stats(&self) -> NodeStats {
        self.stats.clone()
    }

    /// Get node ID
    pub fn node_id(&self) -> [u8; 32] {
        self.config.node_id
    }

    /// Get layer
    pub fn layer(&self) -> u8 {
        self.config.layer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_config(layer: u8) -> NodeConfig {
        let mut config = NodeConfig::default();
        config.layer = layer;
        config.node_id = [layer; 32];
        config
    }

    #[tokio::test]
    async fn test_node_creation() {
        let config = create_test_config(0);
        let node = MixNode::new(config);

        assert_eq!(node.layer(), 0);
        assert_eq!(node.stats().packets_received, 0);
    }

    #[tokio::test]
    async fn test_dead_drop_store() {
        let config = create_test_config(0);
        let mut node = MixNode::new(config);

        let mut payload = b"DEADROP_STORE:".to_vec();
        payload.extend_from_slice(&[1u8; 32]); // drop_id
        payload.extend_from_slice(&[2u8; 32]); // access_token
        payload.extend_from_slice(b"message");

        node.handle_dead_drop_store(payload).unwrap();
        assert_eq!(node.stats().dead_drop_messages, 1);
    }

    #[tokio::test]
    async fn test_stats() {
        let config = create_test_config(0);
        let node = MixNode::new(config);

        let stats = node.stats();
        assert_eq!(stats.packets_received, 0);
        assert_eq!(stats.packets_forwarded, 0);
    }
}
