//! Temporal Scrambling
//!
//! Poisson-distributed random delays to prevent timing correlation attacks.

use std::time::Duration;
use crate::error::Result;

/// Temporal delay configuration
#[derive(Debug, Clone)]
pub struct TemporalConfig {
    /// Mean delay in seconds (λ parameter for Poisson)
    pub mean_delay: f64,
    /// Minimum delay to prevent zero-delay attacks
    pub min_delay: Duration,
    /// Maximum delay to prevent indefinite queueing
    pub max_delay: Duration,
}

impl Default for TemporalConfig {
    fn default() -> Self {
        Self {
            mean_delay: 5.0,                        // 5 second mean
            min_delay: Duration::from_millis(100),  // 100ms minimum
            max_delay: Duration::from_secs(60),     // 60 second maximum
        }
    }
}

/// Temporal delay generator
#[derive(Debug)]
pub struct TemporalDelayGenerator {
    config: TemporalConfig,
}

impl TemporalDelayGenerator {
    /// Create a new temporal delay generator
    pub fn new(config: TemporalConfig) -> Self {
        Self { config }
    }

    /// Generate a Poisson-distributed delay
    pub fn generate_delay(&self) -> Duration {
        use rand::Rng;
        
        // Generate exponentially-distributed delay (Poisson inter-arrival time)
        let u: f64 = rand::thread_rng().gen();
        let delay_secs = -self.config.mean_delay * u.ln();
        
        // Clamp to min/max bounds
        let delay = Duration::from_secs_f64(delay_secs);
        
        if delay < self.config.min_delay {
            self.config.min_delay
        } else if delay > self.config.max_delay {
            self.config.max_delay
        } else {
            delay
        }
    }

    /// Generate multiple delays for a batch
    pub fn generate_batch_delays(&self, count: usize) -> Vec<Duration> {
        (0..count).map(|_| self.generate_delay()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delay_generation() {
        let gen = TemporalDelayGenerator::new(TemporalConfig::default());
        
        // Generate delays and check bounds
        for _ in 0..100 {
            let delay = gen.generate_delay();
            assert!(delay >= gen.config.min_delay);
            assert!(delay <= gen.config.max_delay);
        }
    }

    #[test]
    fn test_batch_delays() {
        let gen = TemporalDelayGenerator::new(TemporalConfig::default());
        let delays = gen.generate_batch_delays(10);
        
        assert_eq!(delays.len(), 10);
        assert!(delays.iter().all(|d| *d >= gen.config.min_delay));
        assert!(delays.iter().all(|d| *d <= gen.config.max_delay));
    }
}
