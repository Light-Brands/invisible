//! Service Health Dashboard
//!
//! Monitors and reports status of all Invisible platform services.

use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

/// Overall platform health status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HealthStatus {
    /// All systems operational
    Healthy,
    /// Some non-critical services degraded
    Degraded,
    /// Critical services offline
    Unhealthy,
}

/// Individual service status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceStatus {
    /// Service name
    pub name: String,
    /// Is service running?
    pub running: bool,
    /// Is service healthy?
    pub healthy: bool,
    /// Last health check time
    pub last_check: SystemTime,
    /// Additional status message
    pub message: Option<String>,
    /// Response time in milliseconds
    pub latency_ms: Option<u64>,
}

/// Dashboard showing all service statuses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceDashboard {
    /// Overall platform health
    pub overall_status: HealthStatus,
    /// VPN connection status (Layer 0)
    pub vpn: ServiceStatus,
    /// Mix network status (Layer 2)
    pub mixnet: ServiceStatus,
    /// Cover traffic generation (Layer 3)
    pub cover_traffic: ServiceStatus,
    /// Dead drop relay status (Layer 6)
    pub dead_drops: ServiceStatus,
    /// Shadow Wallet RPC connections
    pub wallet_rpcs: WalletRpcStatus,
    /// Number of active connections
    pub active_connections: u32,
    /// Messages sent in last hour
    pub messages_sent_1h: u32,
    /// Messages received in last hour
    pub messages_received_1h: u32,
}

/// Wallet RPC connection statuses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletRpcStatus {
    /// Bitcoin RPC status
    pub bitcoin: Option<ServiceStatus>,
    /// Monero RPC status
    pub monero: Option<ServiceStatus>,
    /// Zcash RPC status
    pub zcash: Option<ServiceStatus>,
    /// Ethereum RPC status
    pub ethereum: Option<ServiceStatus>,
}

impl ServiceDashboard {
    /// Create a new dashboard
    pub fn new() -> Self {
        let now = SystemTime::now();
        Self {
            overall_status: HealthStatus::Unhealthy,
            vpn: ServiceStatus {
                name: "VPN (Layer 0)".to_string(),
                running: false,
                healthy: false,
                last_check: now,
                message: Some("Not initialized".to_string()),
                latency_ms: None,
            },
            mixnet: ServiceStatus {
                name: "Mix Network (Layer 2)".to_string(),
                running: false,
                healthy: false,
                last_check: now,
                message: Some("Not initialized".to_string()),
                latency_ms: None,
            },
            cover_traffic: ServiceStatus {
                name: "Cover Traffic (Layer 3)".to_string(),
                running: false,
                healthy: false,
                last_check: now,
                message: Some("Not initialized".to_string()),
                latency_ms: None,
            },
            dead_drops: ServiceStatus {
                name: "Dead Drop Relays (Layer 6)".to_string(),
                running: false,
                healthy: false,
                last_check: now,
                message: Some("Not initialized".to_string()),
                latency_ms: None,
            },
            wallet_rpcs: WalletRpcStatus {
                bitcoin: None,
                monero: None,
                zcash: None,
                ethereum: None,
            },
            active_connections: 0,
            messages_sent_1h: 0,
            messages_received_1h: 0,
        }
    }

    /// Update overall health based on service statuses
    pub fn update_overall_health(&mut self) {
        let critical_services = [&self.vpn, &self.mixnet];
        let critical_healthy = critical_services.iter().all(|s| s.healthy);

        let optional_services = [&self.cover_traffic, &self.dead_drops];
        let optional_healthy = optional_services.iter().filter(|s| s.healthy).count();

        if critical_healthy && optional_healthy == optional_services.len() {
            self.overall_status = HealthStatus::Healthy;
        } else if critical_healthy {
            self.overall_status = HealthStatus::Degraded;
        } else {
            self.overall_status = HealthStatus::Unhealthy;
        }
    }

    /// Generate status report for display
    pub fn generate_report(&self) -> String {
        let status_icon = match self.overall_status {
            HealthStatus::Healthy => "✅",
            HealthStatus::Degraded => "⚠️",
            HealthStatus::Unhealthy => "❌",
        };

        let mut report = format!(
            "{} INVISIBLE PLATFORM STATUS\n",
            status_icon
        );
        report.push_str(&format!("Overall: {:?}\n\n", self.overall_status));
        report.push_str("CORE SERVICES:\n");
        report.push_str(&self.format_service(&self.vpn));
        report.push_str(&self.format_service(&self.mixnet));
        report.push_str(&self.format_service(&self.cover_traffic));
        report.push_str(&self.format_service(&self.dead_drops));

        report.push_str("\nWALLET RPCs:\n");
        if let Some(ref btc) = self.wallet_rpcs.bitcoin {
            report.push_str(&self.format_service(btc));
        }
        if let Some(ref xmr) = self.wallet_rpcs.monero {
            report.push_str(&self.format_service(xmr));
        }
        if let Some(ref zec) = self.wallet_rpcs.zcash {
            report.push_str(&self.format_service(zec));
        }

        report.push_str(&format!("\nMETRICS:\n"));
        report.push_str(&format!("  Active Connections: {}\n", self.active_connections));
        report.push_str(&format!("  Messages Sent (1h): {}\n", self.messages_sent_1h));
        report.push_str(&format!("  Messages Received (1h): {}\n", self.messages_received_1h));

        report
    }

    fn format_service(&self, service: &ServiceStatus) -> String {
        let icon = if service.healthy { "✅" } else { "❌" };
        let latency = service.latency_ms
            .map(|ms| format!(" ({}ms)", ms))
            .unwrap_or_default();
        let msg = service.message
            .as_ref()
            .map(|m| format!(" - {}", m))
            .unwrap_or_default();

        format!("  {} {}{}{}\n", icon, service.name, latency, msg)
    }
}

impl Default for ServiceDashboard {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dashboard_creation() {
        let dashboard = ServiceDashboard::new();
        assert_eq!(dashboard.overall_status, HealthStatus::Unhealthy);
        assert!(!dashboard.vpn.healthy);
    }

    #[test]
    fn test_health_update() {
        let mut dashboard = ServiceDashboard::new();

        // Mark critical services as healthy
        dashboard.vpn.healthy = true;
        dashboard.mixnet.healthy = true;
        dashboard.update_overall_health();

        // Should be degraded (optional services not healthy)
        assert_eq!(dashboard.overall_status, HealthStatus::Degraded);

        // Mark all services healthy
        dashboard.cover_traffic.healthy = true;
        dashboard.dead_drops.healthy = true;
        dashboard.update_overall_health();

        assert_eq!(dashboard.overall_status, HealthStatus::Healthy);
    }
}
