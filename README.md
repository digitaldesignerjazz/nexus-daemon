# nexus-daemon

> The high-performance async Rust core of the **Nexus** decentralized ecosystem.

[![CI](https://github.com/digitaldesignerjazz/nexus-daemon/actions/workflows/ci.yml/badge.svg)](https://github.com/digitaldesignerjazz/nexus-daemon/actions/workflows/ci.yml)
[![Rust](https://img.shields.io/badge/rust-1.80+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](LICENSE)

**nexus-daemon** is the central coordination daemon that brings together three powerful layers:

- **Mesh Networking** — Yggdrasil, xMesh, NovaNet, QNET with privacy (Tor/I2P) and intelligent routing
- **Blockchain** — XCoin / QCoin with support for QNET runes, on-chain anchoring, and governance
- **AI Agent Swarms** — Self-improving, emotional AI coordination (inspired by Ara) with agent-to-agent communication over the mesh

It acts as the "nervous system" for a new generation of decentralized, self-evolving infrastructure.

## Vision

Nexus aims to create a unified, privacy-first, globally scalable platform where:

- Mesh networks become intelligent and self-optimizing
- Blockchain provides verifiable anchoring and economic coordination
- AI agents collaborate autonomously while remaining auditable and aligned

The daemon is designed to run on edge hardware (routers, custom boards, Soilnova/Vista Nova prototypes) as well as servers, enabling true decentralized physical infrastructure (DePIN) combined with advanced AI.

## Current Status

**Scaffold Phase** — Solid modular foundation is complete and ready for rapid feature development.

### Implemented
- Clean modular architecture (`mesh/`, `blockchain/`, `ai/`)
- Tokio-based async runtime with structured tracing
- TOML configuration system with dedicated sections per layer
- Graceful shutdown handling
- In-memory stubs for all three layers with demo wiring
- CI pipeline (check, clippy, test, fmt) via GitHub Actions

### Next Milestones
- Real Yggdrasil / libp2p mesh backend
- XCoin/QCoin JSON-RPC client + QNET rune execution
- Full AI swarm task distribution and emotional state model
- Prometheus metrics + optional egui dashboard (Grok Launcher style)
- Docker Compose stack + hardware deployment examples

## Architecture Overview

```
                    +-------------------+
                    |   nexus-daemon    |
                    +---------+---------+
                              |
        +---------------------+---------------------+
        |                     |                     |
   +----+----+          +-----+-----+          +----+----+
   |  Mesh   |          | Blockchain|          |   AI    |
   | Manager |          |  Client   |          |Coordinator|
   +---------+          +-----------+          +---------+
        |                     |                     |
   Yggdrasil /          XCoin/QCoin node      Agent Swarms
   libp2p / QNET                                 (over mesh)
```

The three layers communicate via channels and can anchor important decisions (AI proposals, network optimizations) directly on-chain.

## Getting Started

### Prerequisites
- Rust 1.80+ (stable)
- Cargo

### Build & Run

```bash
git clone https://github.com/digitaldesignerjazz/nexus-daemon.git
cd nexus-daemon

cargo check
cargo run -- --dry-run          # Validate config only
RUST_LOG=debug cargo run        # Run with detailed logs
```

### Configuration

The daemon looks for a TOML config (default path can be overridden with `--config`).

Example sections include:
- `[mesh]` — Peers, listen address, Tor settings
- `[blockchain]` — XCoin RPC endpoint, QNET features
- `[ai]` — Max agents, emotional state toggle, communication channel
- `[monitoring]` — Prometheus settings

A full example config will be added soon.

## Roadmap

- [ ] Production-ready mesh backend (Yggdrasil + libp2p hybrid)
- [ ] Full XCoin/QCoin integration + on-chain event anchoring
- [ ] Advanced AI swarm orchestration with self-improvement loops
- [ ] Hardware deployment guides (Tenda Nova, custom boards, Soilnova)
- [ ] Monitoring dashboard + metrics
- [ ] Optional GUI (egui) inspired by Grok Launcher
- [ ] Docker + Kubernetes deployment examples

## Related Projects

This daemon is part of the larger **Nexus** initiative, which also includes:

- Mesh networking infrastructure (xMesh / NovaNet / QNET)
- XCoin / QCoin blockchain and rune system
- AI agent swarms and emotional/self-improving models
- Hardware prototypes (Soilnova, Vista Nova, Grok Launcher)

## Contributing

Contributions are very welcome! Whether you're interested in:

- Rust systems programming
- Mesh networking & privacy
- Blockchain integration
- AI agent architectures
- Hardware / embedded deployment

Feel free to open issues or pull requests. Please run `cargo fmt` and `cargo clippy` before submitting.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Acknowledgments

Built as part of ongoing work on decentralized, self-improving infrastructure combining mesh networks, blockchain, and emotional AI.

---

**Status**: Active development (Scaffold → Core Implementation)

If this project resonates with you — decentralized systems, autonomous AI, privacy-first infrastructure — star it, watch it, or reach out. We're just getting started.
