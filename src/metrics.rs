//! Prometheus metrics for Nexus Daemon.
//!
//! Exposes key operational metrics for mesh, blockchain, and AI layers.
//! Served on a separate HTTP port (default 9091) at `/metrics`.

use prometheus::{
    register_gauge, register_int_counter, register_int_gauge, Gauge, IntCounter, IntGauge, TextEncoder,
};
use std::sync::LazyLock;
use tracing::info;

/// Global metrics registry and collectors
pub static NEXUS_METRICS: LazyLock<NexusMetrics> = LazyLock::new(NexusMetrics::new);

pub struct NexusMetrics {
    /// Uptime of the daemon in seconds
    pub uptime: Gauge,

    // === Mesh Layer ===
    /// Number of currently connected mesh peers
    pub mesh_connected_peers: IntGauge,
    /// Total mesh messages processed
    pub mesh_messages_total: IntCounter,
    /// Average mesh peer latency (placeholder for future)
    pub mesh_peer_latency: Gauge,

    // === AI Swarm ===
    /// Number of currently active AI agents
    pub ai_active_agents: IntGauge,
    /// Total AI messages routed
    pub ai_messages_routed_total: IntCounter,

    // === Blockchain ===
    /// Current blockchain height (if connected)
    pub blockchain_height: IntGauge,
    /// Total blockchain events anchored
    pub blockchain_events_anchored_total: IntCounter,
}

impl NexusMetrics {
    fn new() -> Self {
        Self {
            uptime: register_gauge!("nexus_uptime_seconds", "Uptime of nexus-daemon in seconds").unwrap(),

            // Mesh
            mesh_connected_peers: register_int_gauge!(
                "nexus_mesh_connected_peers",
                "Number of currently connected mesh peers"
            )
            .unwrap(),
            mesh_messages_total: register_int_counter!(
                "nexus_mesh_messages_total",
                "Total number of mesh messages processed"
            )
            .unwrap(),
            mesh_peer_latency: register_gauge!(
                "nexus_mesh_peer_latency_seconds",
                "Average latency to mesh peers in seconds"
            )
            .unwrap(),

            // AI
            ai_active_agents: register_int_gauge!(
                "nexus_ai_active_agents",
                "Number of currently active AI agents in the swarm"
            )
            .unwrap(),
            ai_messages_routed_total: register_int_counter!(
                "nexus_ai_messages_routed_total",
                "Total AI messages routed through the coordinator"
            )
            .unwrap(),

            // Blockchain
            blockchain_height: register_int_gauge!(
                "nexus_blockchain_height",
                "Current height of the connected blockchain"
            )
            .unwrap(),
            blockchain_events_anchored_total: register_int_counter!(
                "nexus_blockchain_events_anchored_total",
                "Total number of events/decisions anchored on-chain"
            )
            .unwrap(),
        }
    }
}

/// Start a Prometheus metrics HTTP server on the given port.
pub async fn start_metrics_server(port: u16) {
    use std::net::SocketAddr;
    use tokio::net::TcpListener;

    let addr = SocketAddr::from(([0, 0, 0], port));
    let listener = TcpListener::bind(addr).await.expect("Failed to bind metrics port");

    info!("📊 Prometheus metrics server listening on http://0.0.0.0:{}", port);

    loop {
        let (mut stream, _) = listener.accept().await.unwrap();

        // Very simple HTTP response for /metrics
        let encoder = TextEncoder::new();
        let metric_families = prometheus::gather();
        let mut buffer = Vec::new();
        encoder.encode(&metric_families, &mut buffer).unwrap();

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain; version=0.0.4\r\nContent-Length: {}\r\n\r\n",
            buffer.len()
        );

        let _ = tokio::io::AsyncWriteExt::write_all(&mut stream, response.as_bytes()).await;
        let _ = tokio::io::AsyncWriteExt::write_all(&mut stream, &buffer).await;
    }
}

/// Helper to increment mesh message counter
pub fn record_mesh_message() {
    NEXUS_METRICS.mesh_messages_total.inc();
}

/// Helper to update AI agent count
pub fn set_active_agents(count: i64) {
    NEXUS_METRICS.ai_active_agents.set(count);
}

/// Helper to record anchored blockchain event
pub fn record_blockchain_event() {
    NEXUS_METRICS.blockchain_events_anchored_total.inc();
}
