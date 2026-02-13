//! HTTP API for relay node

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::node::RelayNode;
use crate::error::Result;
use invisible_scrambler::sphinx::SphinxPacket;

/// API state
#[derive(Clone)]
pub struct ApiState {
    /// Relay node
    pub node: Arc<RelayNode>,
}

/// Create the API router
pub fn create_router(node: Arc<RelayNode>) -> Router {
    let state = ApiState { node };

    Router::new()
        .route("/health", get(health_check))
        .route("/packet", post(submit_packet))
        .route("/metrics", get(metrics))
        .with_state(state)
}

/// Health check endpoint
async fn health_check(State(state): State<ApiState>) -> impl IntoResponse {
    let response = HealthResponse {
        status: "ok".to_string(),
        node_id: hex::encode(state.node.node_id()),
        layer: state.node.layer(),
    };

    Json(response)
}

/// Submit packet endpoint
async fn submit_packet(
    State(state): State<ApiState>,
    Json(packet): Json<SphinxPacket>,
) -> impl IntoResponse {
    match state.node.process_packet(packet).await {
        Ok(_) => (StatusCode::ACCEPTED, "Packet accepted").into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            format!("Packet rejected: {}", e),
        )
            .into_response(),
    }
}

/// Metrics endpoint (Prometheus format)
async fn metrics() -> impl IntoResponse {
    // TODO: Export Prometheus metrics
    "# HELP invisible_relay_packets_total Total packets processed\n\
     # TYPE invisible_relay_packets_total counter\n\
     invisible_relay_packets_total 0\n"
}

/// Health check response
#[derive(Debug, Serialize, Deserialize)]
struct HealthResponse {
    status: String,
    node_id: String,
    layer: u8,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node::RelayConfig;
    use invisible_scrambler::mixnet::{GeoLocation, Jurisdiction, MixNode, MixStrategy};
    use invisible_scrambler::cover_traffic::CoverTrafficConfig;
    use invisible_scrambler::temporal::TemporalConfig;

    fn create_test_node() -> Arc<RelayNode> {
        let config = RelayConfig {
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
        };

        Arc::new(RelayNode::new(config))
    }

    #[test]
    fn test_router_creation() {
        let node = create_test_node();
        let _router = create_router(node);
    }
}
