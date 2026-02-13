//! Relay node binary

use std::net::SocketAddr;
use std::sync::Arc;

use invisible_relay::{RelayNode, api};
use invisible_relay::node::RelayConfig;
use invisible_scrambler::mixnet::{GeoLocation, Jurisdiction, MixNode, MixStrategy};
use invisible_scrambler::cover_traffic::CoverTrafficConfig;
use invisible_scrambler::temporal::TemporalConfig;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "invisible_relay=info,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting Invisible Relay Node");

    // Load configuration (TODO: from file/env)
    let config = RelayConfig {
        node: MixNode {
            id: [0u8; 32],  // TODO: Generate or load
            layer: std::env::var("MIXNET_LAYER")
                .ok()
                .and_then(|l| l.parse().ok())
                .unwrap_or(0),
            public_key: vec![0u8; 32],  // TODO: Load
            address: std::env::var("RELAY_ADDRESS")
                .unwrap_or_else(|_| "0.0.0.0:8080".to_string()),
            location: GeoLocation {
                country: "US".to_string(),  // TODO: Detect or configure
                jurisdiction: Jurisdiction::PrivacyFriendly,
            },
        },
        mix_strategy: MixStrategy::default(),
        cover_traffic: CoverTrafficConfig::default(),
        temporal: TemporalConfig::default(),
        private_key: vec![0u8; 32],  // TODO: Load securely
    };

    // Create relay node
    let node = Arc::new(RelayNode::new(config.clone()));

    // Start background tasks
    node.start().await?;

    // Create HTTP API
    let app = api::create_router(node);

    // Parse bind address
    let addr: SocketAddr = config.node.address.parse()?;
    tracing::info!("Listening on {}", addr);

    // Start server
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
