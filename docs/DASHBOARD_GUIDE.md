# Invisible Platform - Service Dashboard Guide

**Real-time monitoring and health status for all platform services.**

## Overview

The Invisible platform now includes a comprehensive service monitoring dashboard that displays real-time status of all 7 security layers and optional services. This dashboard provides immediate visibility into system health and helps diagnose issues quickly.

## What You Can Monitor

### Critical Services (Must Be Green ✅)

1. **VPN (Layer 0)** - WireGuard tunnel status
   - Connection state
   - Handshake freshness
   - Latency to VPN gateway
   - Endpoint location

2. **Mix Network (Layer 2)** - Sphinx packet routing
   - Number of reachable mix nodes (need min 3, optimal 5)
   - Average latency across hops
   - Packet success rate

### Important Services (Should Be Green ✅)

3. **Cover Traffic (Layer 3)** - Privacy obfuscation
   - Packets generated per hour
   - Size distribution statistics
   - Timing jitter

4. **Dead Drop Relays (Layer 6)** - Anonymous message retrieval
   - Number of available nodes
   - Store/retrieve success rate
   - Average retrieval latency

### Optional Services (Nice To Have ⚠️)

5. **Wallet RPCs** - Blockchain integration
   - Bitcoin RPC status
   - Monero RPC status
   - Zcash RPC status
   - Ethereum RPC status (future)

## Using the Dashboard

### In Code (Rust API)

```rust
use invisible_client::{InvisibleClient, ServiceDashboard};
use std::time::SystemTime;

// Create client and dashboard
let client = InvisibleClient::new(scrambler, wallet);
let mut dashboard = ServiceDashboard::new();

// Update VPN status
dashboard.vpn.running = true;
dashboard.vpn.healthy = client.is_vpn_connected();
dashboard.vpn.latency_ms = Some(client.vpn_latency_ms());
dashboard.vpn.last_check = SystemTime::now();
dashboard.vpn.message = Some("Connected to NL-Amsterdam".to_string());

// Update mix network status
let reachable_nodes = client.count_reachable_mix_nodes().await;
dashboard.mixnet.running = reachable_nodes > 0;
dashboard.mixnet.healthy = reachable_nodes >= 3;  // Minimum viable
dashboard.mixnet.latency_ms = client.average_mix_latency_ms().await;
dashboard.mixnet.message = Some(format!("{}/5 layers reachable", reachable_nodes));

// Update cover traffic status
dashboard.cover_traffic.running = true;
dashboard.cover_traffic.healthy = true;
dashboard.cover_traffic.message = Some(
    format!("{} packets/hour", client.cover_traffic_count_1h())
);

// Update overall health
dashboard.update_overall_health();

// Display status report
println!("{}", dashboard.generate_report());
```

### Example Output

```
✅ INVISIBLE PLATFORM STATUS
Overall: Healthy

CORE SERVICES:
  ✅ VPN (Layer 0) (12ms) - Connected to NL-Amsterdam
  ✅ Mix Network (Layer 2) (156ms) - 5/5 layers reachable
  ✅ Cover Traffic (Layer 3) - 612 packets/hour
  ✅ Dead Drop Relays (Layer 6) (89ms) - 3/3 nodes available

WALLET RPCs:
  ✅ Bitcoin RPC (45ms) - Testnet, Block 2,450,123
  ⚠️  Monero RPC - Not configured (optional)
  ⚠️  Zcash RPC - Not configured (optional)

METRICS:
  Active Connections: 0
  Messages Sent (1h): 0
  Messages Received (1h): 0
```

## Health Status Levels

| Status | Icon | Meaning | Action Required |
|--------|------|---------|----------------|
| **Healthy** | ✅ | All critical services operational, optional services may be offline | None - system fully operational |
| **Degraded** | ⚠️ | Critical services working but some optional features unavailable | Review warnings, fix when convenient |
| **Unhealthy** | ❌ | Critical services offline, platform will not work correctly | IMMEDIATE - fix critical services |

### Status Determination Logic

```rust
// Critical services: VPN + Mix Network
let critical_healthy = vpn.healthy && mixnet.healthy;

// Optional services: Cover traffic, Dead drops, Wallet RPCs
let optional_count = count_healthy_optional_services();

if critical_healthy && all_optional_healthy {
    status = Healthy;  // ✅ Everything working
} else if critical_healthy {
    status = Degraded;  // ⚠️ Core works, some features limited
} else {
    status = Unhealthy;  // ❌ Platform broken
}
```

## Building a Web Dashboard

### Simple HTTP Server

```rust
use invisible_client::ServiceDashboard;
use axum::{Router, routing::get, Json};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/metrics", get(metrics_handler));

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn health_handler() -> Json<ServiceDashboard> {
    let dashboard = get_current_dashboard().await;
    Json(dashboard)
}
```

### Frontend (React Example)

```typescript
import React, { useEffect, useState } from 'react';

interface ServiceStatus {
  name: string;
  running: boolean;
  healthy: boolean;
  latency_ms?: number;
  message?: string;
}

function Dashboard() {
  const [status, setStatus] = useState<ServiceDashboard | null>(null);

  useEffect(() => {
    // Poll every 5 seconds
    const interval = setInterval(async () => {
      const response = await fetch('http://localhost:8080/health');
      const data = await response.json();
      setStatus(data);
    }, 5000);

    return () => clearInterval(interval);
  }, []);

  if (!status) return <div>Loading...</div>;

  return (
    <div className="dashboard">
      <h1>
        {status.overall_status === 'Healthy' ? '✅' :
         status.overall_status === 'Degraded' ? '⚠️' : '❌'}
        {' '}Invisible Platform
      </h1>

      <div className="services">
        <ServiceCard service={status.vpn} critical={true} />
        <ServiceCard service={status.mixnet} critical={true} />
        <ServiceCard service={status.cover_traffic} critical={false} />
        <ServiceCard service={status.dead_drops} critical={false} />
      </div>

      <div className="metrics">
        <Metric label="Active Connections" value={status.active_connections} />
        <Metric label="Messages Sent (1h)" value={status.messages_sent_1h} />
        <Metric label="Messages Received (1h)" value={status.messages_received_1h} />
      </div>
    </div>
  );
}

function ServiceCard({ service, critical }: { service: ServiceStatus, critical: boolean }) {
  const icon = service.healthy ? '✅' : '❌';
  const badge = critical ? <span className="critical">CRITICAL</span> : null;

  return (
    <div className={`service-card ${service.healthy ? 'healthy' : 'unhealthy'}`}>
      <h3>{icon} {service.name} {badge}</h3>
      <p>{service.message || 'No status available'}</p>
      {service.latency_ms && <span className="latency">{service.latency_ms}ms</span>}
    </div>
  );
}
```

## Flutter Mobile Dashboard

```dart
import 'package:flutter/material.dart';
import 'package:invisible_app/ffi/client_ffi.dart';

class DashboardScreen extends StatefulWidget {
  @override
  _DashboardScreenState createState() => _DashboardScreenState();
}

class _DashboardScreenState extends State<DashboardScreen> {
  ServiceDashboard? _dashboard;
  Timer? _timer;

  @override
  void initState() {
    super.initState();
    _updateStatus();
    _timer = Timer.periodic(Duration(seconds: 5), (_) => _updateStatus());
  }

  Future<void> _updateStatus() async {
    final dashboard = await InvisibleClient.getDashboard();
    setState(() {
      _dashboard = dashboard;
    });
  }

  @override
  void dispose() {
    _timer?.cancel();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    if (_dashboard == null) {
      return Center(child: CircularProgressIndicator());
    }

    return Scaffold(
      appBar: AppBar(
        title: Text('Invisible Status'),
        backgroundColor: _getStatusColor(_dashboard!.overallStatus),
      ),
      body: RefreshIndicator(
        onRefresh: _updateStatus,
        child: ListView(
          children: [
            _buildOverallStatus(),
            Divider(),
            _buildServiceSection('Critical Services', [
              _dashboard!.vpn,
              _dashboard!.mixnet,
            ], critical: true),
            Divider(),
            _buildServiceSection('Optional Services', [
              _dashboard!.coverTraffic,
              _dashboard!.deadDrops,
            ], critical: false),
            Divider(),
            _buildMetrics(),
          ],
        ),
      ),
    );
  }

  Widget _buildServiceCard(ServiceStatus service, bool critical) {
    return Card(
      margin: EdgeInsets.all(8),
      color: service.healthy ? Colors.green[50] : Colors.red[50],
      child: ListTile(
        leading: Icon(
          service.healthy ? Icons.check_circle : Icons.error,
          color: service.healthy ? Colors.green : Colors.red,
          size: 32,
        ),
        title: Row(
          children: [
            Text(service.name),
            if (critical)
              Chip(
                label: Text('CRITICAL'),
                backgroundColor: Colors.red[100],
              ),
          ],
        ),
        subtitle: Text(service.message ?? 'No status'),
        trailing: service.latencyMs != null
            ? Text('${service.latencyMs}ms')
            : null,
      ),
    );
  }
}
```

## Monitoring & Alerts

### Setting Up Alerts

```rust
use invisible_client::{ServiceDashboard, HealthStatus};

async fn monitor_with_alerts(client: &InvisibleClient) {
    loop {
        let dashboard = get_current_dashboard(client).await;

        // Check for critical failures
        if !dashboard.vpn.healthy {
            send_alert("CRITICAL: VPN disconnected!").await;
        }

        if !dashboard.mixnet.healthy {
            send_alert("CRITICAL: Mix network unavailable!").await;
        }

        // Check for degraded performance
        if let Some(latency) = dashboard.mixnet.latency_ms {
            if latency > 2000 {
                send_warning("High latency on mix network").await;
            }
        }

        // Check overall status
        match dashboard.overall_status {
            HealthStatus::Unhealthy => {
                send_critical_alert(&dashboard).await;
            }
            HealthStatus::Degraded => {
                log_warning(&dashboard).await;
            }
            HealthStatus::Healthy => {
                // All good
            }
        }

        tokio::time::sleep(Duration::from_secs(30)).await;
    }
}
```

### Prometheus Metrics Export

```rust
use prometheus::{Registry, Counter, Gauge};

// Define metrics
lazy_static! {
    static ref VPN_CONNECTED: Gauge = Gauge::new("vpn_connected", "VPN connection status").unwrap();
    static ref MIXNET_REACHABLE: Gauge = Gauge::new("mixnet_nodes_reachable", "Number of reachable mix nodes").unwrap();
    static ref MESSAGES_SENT: Counter = Counter::new("messages_sent_total", "Total messages sent").unwrap();
}

// Update from dashboard
fn update_prometheus(dashboard: &ServiceDashboard) {
    VPN_CONNECTED.set(if dashboard.vpn.healthy { 1.0 } else { 0.0 });
    MIXNET_REACHABLE.set(parse_reachable_nodes(&dashboard.mixnet.message).into());
    MESSAGES_SENT.inc_by(dashboard.messages_sent_1h as f64);
}
```

## Quick Reference

### Dashboard API Endpoints (if using HTTP server)

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/health` | GET | Full dashboard JSON |
| `/health/vpn` | GET | VPN status only |
| `/health/mixnet` | GET | Mix network status |
| `/health/cover` | GET | Cover traffic stats |
| `/health/wallet` | GET | Wallet RPC statuses |
| `/metrics` | GET | Prometheus format |

### Status Codes

- **200 OK** - Healthy or Degraded (platform functional)
- **503 Service Unavailable** - Unhealthy (critical services down)

### Health Check Command

```bash
# Run automated health check
./scripts/health-check.sh

# Check specific service
./scripts/health-check.sh --service vpn
```

---

**Dashboard Status:** ✅ **FULLY IMPLEMENTED**

The service monitoring dashboard is production-ready with comprehensive status tracking, health determination logic, and easy integration options for web, mobile, and monitoring systems!
