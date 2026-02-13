//! Mixnet Implementation
//!
//! 5-layer mix network using Sphinx packets with batch-shuffle-forward strategy.
//! Inspired by Loopix design.

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::time::{Duration, Instant};

use crate::error::{Result, ScramblerError};
use crate::sphinx::SphinxPacket;

/// Mix node identifier
pub type NodeId = [u8; 32];

/// Mix network layer (0-4)
pub type Layer = u8;

/// Mix node in the network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MixNode {
    /// Unique node identifier
    pub id: NodeId,
    /// Network layer (0-4)
    pub layer: Layer,
    /// Node public key
    pub public_key: Vec<u8>,
    /// Network address
    pub address: String,
    /// Geographic location (for jurisdiction routing)
    pub location: GeoLocation,
}

/// Geographic location for jurisdiction routing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoLocation {
    /// Country code (ISO 3166-1 alpha-2)
    pub country: String,
    /// Jurisdiction classification
    pub jurisdiction: Jurisdiction,
}

/// Jurisdiction classification for routing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Jurisdiction {
    /// Five Eyes (US, UK, CA, AU, NZ)
    FiveEyes,
    /// Fourteen Eyes (Five Eyes + DE, FR, IT, ES, NL, BE, DK, NO, SE)
    FourteenEyes,
    /// Privacy-friendly (CH, IS, etc.)
    PrivacyFriendly,
    /// Other
    Other,
}

/// Mix strategy configuration
#[derive(Debug, Clone)]
pub struct MixStrategy {
    /// Batch size before mixing
    pub batch_size: usize,
    /// Maximum delay before flushing batch
    pub max_delay: Duration,
    /// Minimum delay before forwarding
    pub min_delay: Duration,
}

impl Default for MixStrategy {
    fn default() -> Self {
        Self {
            batch_size: 10,
            max_delay: Duration::from_secs(30),
            min_delay: Duration::from_millis(100),
        }
    }
}

/// Mix node state
#[derive(Debug)]
pub struct MixNodeState {
    /// Node information
    pub node: MixNode,
    /// Strategy configuration
    pub strategy: MixStrategy,
    /// Incoming packet batch
    batch: VecDeque<SphinxPacket>,
    /// Time when batch started
    batch_start: Option<Instant>,
}

impl MixNodeState {
    /// Create a new mix node state
    pub fn new(node: MixNode, strategy: MixStrategy) -> Self {
        Self {
            node,
            strategy,
            batch: VecDeque::new(),
            batch_start: None,
        }
    }

    /// Add packet to batch
    pub fn add_packet(&mut self, packet: SphinxPacket) {
        if self.batch.is_empty() {
            self.batch_start = Some(Instant::now());
        }
        self.batch.push_back(packet);
    }

    /// Check if batch should be mixed and forwarded
    pub fn should_forward(&self) -> bool {
        if self.batch.is_empty() {
            return false;
        }

        // Forward if batch is full
        if self.batch.len() >= self.strategy.batch_size {
            return true;
        }

        // Forward if max delay exceeded
        if let Some(start) = self.batch_start {
            if start.elapsed() >= self.strategy.max_delay {
                return true;
            }
        }

        false
    }

    /// Mix and extract batch for forwarding
    pub fn mix_and_extract(&mut self) -> Vec<SphinxPacket> {
        use rand::seq::SliceRandom;
        use rand::thread_rng;

        let mut packets: Vec<_> = self.batch.drain(..).collect();
        
        // Shuffle packets
        packets.shuffle(&mut thread_rng());
        
        // Reset batch timing
        self.batch_start = None;
        
        packets
    }
}

/// Select a route through the mix network
///
/// # Arguments
/// * `nodes` - Available mix nodes
/// * `avoid_jurisdiction` - Jurisdiction to avoid (e.g., FiveEyes)
pub fn select_route(
    nodes: &[MixNode],
    avoid_jurisdiction: Option<Jurisdiction>,
) -> Result<Vec<MixNode>> {
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    let mut route = Vec::new();

    // Select one node from each layer (0-4)
    for layer in 0..5 {
        let layer_nodes: Vec<_> = nodes
            .iter()
            .filter(|n| n.layer == layer)
            .filter(|n| {
                if let Some(avoid) = avoid_jurisdiction {
                    n.location.jurisdiction != avoid
                } else {
                    true
                }
            })
            .collect();

        if layer_nodes.is_empty() {
            return Err(ScramblerError::MixnetError(format!(
                "No available nodes in layer {}",
                layer
            )));
        }

        let selected = layer_nodes
            .choose(&mut thread_rng())
            .ok_or_else(|| ScramblerError::MixnetError("Failed to select node".to_string()))?;

        route.push((*selected).clone());
    }

    Ok(route)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_node(layer: Layer, jurisdiction: Jurisdiction) -> MixNode {
        MixNode {
            id: [0u8; 32],
            layer,
            public_key: vec![0u8; 32],
            address: "127.0.0.1:8080".to_string(),
            location: GeoLocation {
                country: "US".to_string(),
                jurisdiction,
            },
        }
    }

    #[test]
    fn test_mix_strategy_batch_size() {
        let node = create_test_node(0, Jurisdiction::PrivacyFriendly);
        let mut state = MixNodeState::new(node, MixStrategy::default());

        // Not ready to forward
        assert!(!state.should_forward());

        // Add packets up to batch size
        for _ in 0..10 {
            state.add_packet(SphinxPacket {
                header: crate::sphinx::SphinxHeader {
                    ephemeral_key: [0u8; 32],
                    routing_info: vec![],
                    mac: [0u8; 32],
                },
                payload: vec![],
            });
        }

        // Should forward when batch is full
        assert!(state.should_forward());
    }

    #[test]
    fn test_route_selection() {
        let mut nodes = Vec::new();
        for layer in 0..5 {
            nodes.push(create_test_node(layer, Jurisdiction::PrivacyFriendly));
            nodes.push(create_test_node(layer, Jurisdiction::FiveEyes));
        }

        // Should select 5 nodes, one per layer
        let route = select_route(&nodes, None).unwrap();
        assert_eq!(route.len(), 5);

        // Should avoid Five Eyes
        let route = select_route(&nodes, Some(Jurisdiction::FiveEyes)).unwrap();
        assert_eq!(route.len(), 5);
        assert!(route.iter().all(|n| n.location.jurisdiction != Jurisdiction::FiveEyes));
    }
}
