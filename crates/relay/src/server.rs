//! Relay Server
//!
//! Network server for processing Sphinx packets.

use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::UdpSocket;
use tokio::time;

use crate::error::Result;
use crate::node::{MixNode, NodeStats};

/// Relay server
#[derive(Debug)]
pub struct RelayServer {
    node: MixNode,
    socket: Option<UdpSocket>,
}

impl RelayServer {
    /// Create new relay server
    pub fn new(node: MixNode) -> Self {
        Self {
            node,
            socket: None,
        }
    }

    /// Start server
    pub async fn start(&mut self, bind_addr: SocketAddr) -> Result<()> {
        let socket = UdpSocket::bind(bind_addr).await?;
        tracing::info!(%bind_addr, "Relay server listening");

        self.socket = Some(socket);
        Ok(())
    }

    /// Run server main loop
    pub async fn run(&mut self) -> Result<()> {
        let socket = self.socket.as_ref()
            .ok_or_else(|| crate::error::RelayError::NetworkError(
                "Server not started".to_string()
            ))?;

        let mut buf = vec![0u8; 65536];
        let mut maintenance_interval = time::interval(Duration::from_secs(60));

        loop {
            tokio::select! {
                // Receive packets
                result = socket.recv_from(&mut buf) => {
                    let (len, _peer) = result?;
                    tracing::debug!(size = len, "Received packet");

                    // TODO: Deserialize and process Sphinx packet
                }

                // Periodic maintenance
                _ = maintenance_interval.tick() => {
                    self.node.maintain().await?;
                }
            }
        }
    }

    /// Get statistics
    pub fn stats(&self) -> NodeStats {
        self.node.stats()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node::NodeConfig;

    #[tokio::test]
    async fn test_server_creation() {
        let config = NodeConfig::default();
        let node = MixNode::new(config);
        let server = RelayServer::new(node);

        assert_eq!(server.stats().packets_received, 0);
    }

    #[tokio::test]
    async fn test_server_start() {
        use std::net::{IpAddr, Ipv4Addr};

        let config = NodeConfig::default();
        let node = MixNode::new(config);
        let mut server = RelayServer::new(node);

        let bind_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0);
        server.start(bind_addr).await.unwrap();
    }
}
