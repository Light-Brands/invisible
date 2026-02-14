//! Cover Traffic Generation
//!
//! Generates constant-rate cover traffic to hide real message patterns.

use std::time::Duration;
use crate::error::Result;
use crate::sphinx::SphinxPacket;

/// Cover traffic configuration
#[derive(Debug, Clone)]
pub struct CoverTrafficConfig {
    /// Rate of cover packets per second
    pub rate: f64,
    /// Jitter in packet timing (percentage)
    pub jitter: f64,
}

impl Default for CoverTrafficConfig {
    fn default() -> Self {
        Self {
            rate: 10.0,  // 10 packets/sec
            jitter: 0.1, // 10% jitter
        }
    }
}

/// Cover traffic generator
#[derive(Debug)]
pub struct CoverTrafficGenerator {
    config: CoverTrafficConfig,
}

impl CoverTrafficGenerator {
    /// Create a new cover traffic generator
    pub fn new(config: CoverTrafficConfig) -> Self {
        Self { config }
    }

    /// Generate a cover packet
    ///
    /// Creates a realistic-looking Sphinx packet that is indistinguishable from real traffic.
    /// Uses random routing, realistic payload sizes, and proper Sphinx construction.
    pub fn generate_cover_packet(&self) -> Result<SphinxPacket> {
        use rand::Rng;
        use crate::sphinx::{RouteSpec, build_packet, MAX_HOPS};

        // Generate random route through mix nodes (3-5 hops)
        let num_hops = rand::thread_rng().gen_range(3..=MAX_HOPS);
        let mut node_keys = Vec::new();

        for _ in 0..num_hops {
            let mut key = vec![0u8; 32];
            rand::thread_rng().fill(&mut key[..]);
            node_keys.push(key);
        }

        // Random destination address (32 bytes)
        let mut destination = vec![0u8; 32];
        rand::thread_rng().fill(&mut destination[..]);

        let route = RouteSpec {
            node_keys,
            destination,
        };

        // Generate realistic payload size distribution
        // Most messages are small (100-500 bytes), some larger (up to 2KB)
        let payload_size = if rand::thread_rng().gen_bool(0.8) {
            // 80% small messages
            rand::thread_rng().gen_range(100..500)
        } else {
            // 20% larger messages
            rand::thread_rng().gen_range(500..2048)
        };

        // Generate random payload data
        let mut payload = vec![0u8; payload_size];
        rand::thread_rng().fill(&mut payload[..]);

        // Build proper Sphinx packet
        let packet = build_packet(&route, &payload)?;

        tracing::debug!(
            hops = num_hops,
            payload_size = payload_size,
            "Generated cover traffic packet"
        );

        Ok(packet)
    }

    /// Calculate delay until next cover packet
    pub fn next_delay(&self) -> Duration {
        use rand::Rng;
        
        let base_delay = 1.0 / self.config.rate;
        let jitter = rand::thread_rng().gen_range(-self.config.jitter..self.config.jitter);
        let delay_secs = base_delay * (1.0 + jitter);
        
        Duration::from_secs_f64(delay_secs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cover_traffic_generation() {
        let gen = CoverTrafficGenerator::new(CoverTrafficConfig::default());
        let packet = gen.generate_cover_packet().unwrap();

        // Verify packet has non-empty payload (realistic cover traffic)
        assert!(!packet.payload.is_empty(), "Cover packet should have payload");

        // Verify payload is within realistic size range
        assert!(packet.payload.len() >= 100, "Payload too small");
        assert!(packet.payload.len() <= 2048, "Payload too large");

        // Verify header structure is valid
        assert_eq!(packet.header.ephemeral_key.len(), 32, "Invalid ephemeral key size");
        assert!(!packet.header.routing_info.is_empty(), "Routing info should not be empty");
        assert_eq!(packet.header.mac.len(), 32, "Invalid MAC size");
    }

    #[test]
    fn test_cover_traffic_timing() {
        let config = CoverTrafficConfig {
            rate: 10.0,
            jitter: 0.1,
        };
        let gen = CoverTrafficGenerator::new(config);

        // Test that delays are within expected range
        for _ in 0..100 {
            let delay = gen.next_delay();
            let delay_secs = delay.as_secs_f64();

            // Base delay is 1/rate = 0.1s
            // With 10% jitter, should be between 0.09 and 0.11s
            assert!(delay_secs >= 0.09 && delay_secs <= 0.11,
                "Delay {} outside expected range", delay_secs);
        }
    }

    #[test]
    fn test_cover_traffic_indistinguishability() {
        let gen = CoverTrafficGenerator::new(CoverTrafficConfig::default());

        // Generate multiple packets and verify they're different
        let packet1 = gen.generate_cover_packet().unwrap();
        let packet2 = gen.generate_cover_packet().unwrap();

        // Different packets should have different ephemeral keys
        assert_ne!(packet1.header.ephemeral_key, packet2.header.ephemeral_key,
            "Cover packets should have different ephemeral keys");

        // Different packets should have different payloads
        assert_ne!(packet1.payload, packet2.payload,
            "Cover packets should have different payloads");
    }
}
