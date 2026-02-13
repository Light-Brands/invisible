//! Invisible Relay Node Binary

use invisible_relay::{MixNode, NodeConfig, RelayServer};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("invisible_relay=debug,invisible_scrambler=info")
        .init();

    // Create node configuration
    let config = NodeConfig::default();

    tracing::info!(
        node_id = ?config.node_id,
        layer = config.layer,
        location = %config.location.country,
        "Starting Invisible relay node"
    );

    // Create mix node
    let node = MixNode::new(config.clone());

    // Create and start server
    let mut server = RelayServer::new(node);
    server.start(config.listen_addr).await?;

    // Run server
    tracing::info!("Relay node running");
    server.run().await?;

    Ok(())
}
