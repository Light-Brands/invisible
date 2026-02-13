//! Prometheus metrics for relay node

use prometheus::{Counter, Histogram, IntGauge, Registry};
use std::sync::Arc;

/// Relay node metrics
#[derive(Clone)]
pub struct RelayMetrics {
    /// Total packets received
    pub packets_received: Counter,
    /// Total packets forwarded
    pub packets_forwarded: Counter,
    /// Total packets delivered
    pub packets_delivered: Counter,
    /// Total cover packets generated
    pub cover_packets_generated: Counter,
    /// Current batch size
    pub batch_size: IntGauge,
    /// Packet processing latency
    pub processing_latency: Histogram,
    /// Mix delay histogram
    pub mix_delay: Histogram,
}

impl RelayMetrics {
    /// Create new metrics instance
    pub fn new(registry: &Registry) -> Result<Self, prometheus::Error> {
        let packets_received = Counter::new(
            "invisible_relay_packets_received_total",
            "Total packets received",
        )?;
        registry.register(Box::new(packets_received.clone()))?;

        let packets_forwarded = Counter::new(
            "invisible_relay_packets_forwarded_total",
            "Total packets forwarded",
        )?;
        registry.register(Box::new(packets_forwarded.clone()))?;

        let packets_delivered = Counter::new(
            "invisible_relay_packets_delivered_total",
            "Total packets delivered to recipients",
        )?;
        registry.register(Box::new(packets_delivered.clone()))?;

        let cover_packets_generated = Counter::new(
            "invisible_relay_cover_packets_total",
            "Total cover packets generated",
        )?;
        registry.register(Box::new(cover_packets_generated.clone()))?;

        let batch_size = IntGauge::new(
            "invisible_relay_batch_size",
            "Current number of packets in mix batch",
        )?;
        registry.register(Box::new(batch_size.clone()))?;

        let processing_latency = Histogram::with_opts(
            prometheus::HistogramOpts::new(
                "invisible_relay_processing_latency_seconds",
                "Packet processing latency in seconds",
            )
            .buckets(vec![0.001, 0.005, 0.01, 0.05, 0.1, 0.5, 1.0]),
        )?;
        registry.register(Box::new(processing_latency.clone()))?;

        let mix_delay = Histogram::with_opts(
            prometheus::HistogramOpts::new(
                "invisible_relay_mix_delay_seconds",
                "Mix delay before forwarding in seconds",
            )
            .buckets(vec![1.0, 5.0, 10.0, 30.0, 60.0, 300.0]),
        )?;
        registry.register(Box::new(mix_delay.clone()))?;

        Ok(Self {
            packets_received,
            packets_forwarded,
            packets_delivered,
            cover_packets_generated,
            batch_size,
            processing_latency,
            mix_delay,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_creation() {
        let registry = Registry::new();
        let metrics = RelayMetrics::new(&registry).unwrap();
        
        // Test metric operations
        metrics.packets_received.inc();
        assert_eq!(metrics.packets_received.get(), 1.0);
    }
}
