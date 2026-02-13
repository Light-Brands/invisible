//! Relay Node Implementation

use std::sync::Arc;
use tokio::sync::RwLock;

use invisible_scrambler::mixnet::{MixNode, MixNodeState, MixStrategy};
use invisible_scrambler::cover_traffic::{CoverTrafficConfig, CoverTrafficGenerator};
use invisible_scrambler::sphinx::{SphinxPacket, process_packet, ProcessedPacket};
use invisible_scrambler::temporal::{TemporalConfig, TemporalDelayGenerator};

use crate::error::{Result, RelayError};

/// Relay node configuration
#[derive(Debug, Clone)]
pub struct RelayConfig {
    /// Node information
    pub node: MixNode,
    /// Mixing strategy
    pub mix_strategy: MixStrategy,
    /// Cover traffic configuration
    pub cover_traffic: CoverTrafficConfig,
    /// Temporal delay configuration
    pub temporal: TemporalConfig,
    /// Node private key
    pub private_key: Vec<u8>,
}

/// Relay node instance
#[derive(Debug)]
pub struct RelayNode {
    /// Node configuration
    config: RelayConfig,
    /// Mix node state
    state: Arc<RwLock<MixNodeState>>,
    /// Cover traffic generator
    cover_traffic: CoverTrafficGenerator,
    /// Temporal delay generator
    temporal: TemporalDelayGenerator,
}

impl RelayNode {
    /// Create a new relay node
    pub fn new(config: RelayConfig) -> Self {
        let mix_state = MixNodeState::new(
            config.node.clone(),
            config.mix_strategy.clone(),
        );

        Self {
            cover_traffic: CoverTrafficGenerator::new(config.cover_traffic.clone()),
            temporal: TemporalDelayGenerator::new(config.temporal.clone()),
            config,
            state: Arc::new(RwLock::new(mix_state)),
        }
    }

    /// Start the relay node
    pub async fn start(&self) -> Result<()> {
        tracing::info!("Starting relay node {}", hex::encode(&self.config.node.id));

        // Spawn background tasks
        self.spawn_cover_traffic_task();
        self.spawn_mixing_task();

        Ok(())
    }

    /// Process an incoming packet
    pub async fn process_packet(&self, packet: SphinxPacket) -> Result<()> {
        tracing::debug!("Processing incoming packet");

        // Process Sphinx packet
        let processed = process_packet(&packet, &self.config.private_key)?;

        match processed {
            ProcessedPacket::Forward { next_hop, packet } => {
                // Add to mix queue
                let mut state = self.state.write().await;
                state.add_packet(packet);
                
                tracing::debug!("Packet queued for mixing, next hop: {}", hex::encode(&next_hop));
            }
            ProcessedPacket::Deliver { message } => {
                // Final destination - deliver message
                tracing::info!("Packet delivered: {} bytes", message.len());
                // TODO: Forward to recipient's mailbox
            }
        }

        Ok(())
    }

    /// Spawn cover traffic generation task
    fn spawn_cover_traffic_task(&self) {
        let generator = self.cover_traffic.clone();
        let node = self.clone();

        tokio::spawn(async move {
            loop {
                // Wait for next cover packet timing
                let delay = generator.next_delay();
                tokio::time::sleep(delay).await;

                // Generate and send cover packet
                match generator.generate_cover_packet() {
                    Ok(packet) => {
                        tracing::trace!("Generated cover packet");
                        if let Err(e) = node.process_packet(packet).await {
                            tracing::error!("Failed to process cover packet: {}", e);
                        }
                    }
                    Err(e) => {
                        tracing::error!("Failed to generate cover packet: {}", e);
                    }
                }
            }
        });
    }

    /// Spawn packet mixing task
    fn spawn_mixing_task(&self) {
        let state = Arc::clone(&self.state);
        let temporal = self.temporal.clone();

        tokio::spawn(async move {
            loop {
                // Check if batch is ready
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

                let should_forward = {
                    let state = state.read().await;
                    state.should_forward()
                };

                if should_forward {
                    let mut state_write = state.write().await;
                    let packets = state_write.mix_and_extract();
                    
                    tracing::info!("Mixing and forwarding {} packets", packets.len());

                    // Apply temporal delays
                    for packet in packets {
                        let delay = temporal.generate_delay();
                        
                        // Spawn task to forward after delay
                        tokio::spawn(async move {
                            tokio::time::sleep(delay).await;
                            // TODO: Forward packet to next hop
                            tracing::trace!("Forwarding packet after {:?} delay", delay);
                        });
                    }
                }
            }
        });
    }

    /// Get node ID
    pub fn node_id(&self) -> &[u8; 32] {
        &self.config.node.id
    }

    /// Get node layer
    pub fn layer(&self) -> u8 {
        self.config.node.layer
    }
}

impl Clone for RelayNode {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            state: Arc::clone(&self.state),
            cover_traffic: CoverTrafficGenerator::new(self.config.cover_traffic.clone()),
            temporal: TemporalDelayGenerator::new(self.config.temporal.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use invisible_scrambler::mixnet::{GeoLocation, Jurisdiction};

    fn create_test_config() -> RelayConfig {
        RelayConfig {
            node: MixNode {
                id: [0u8; 32],
                layer: 0,
                public_key: vec![0u8; 32],
                address: "127.0.0.1:8080".to_string(),
                location: GeoLocation {
                    country: "US".to_string(),
                    jurisdiction: Jurisdiction::PrivacyFriendly,
                },
            },
            mix_strategy: MixStrategy::default(),
            cover_traffic: CoverTrafficConfig::default(),
            temporal: TemporalConfig::default(),
            private_key: vec![0u8; 32],
        }
    }

    #[test]
    fn test_relay_node_creation() {
        let config = create_test_config();
        let node = RelayNode::new(config);
        
        assert_eq!(node.node_id(), &[0u8; 32]);
        assert_eq!(node.layer(), 0);
    }
}
