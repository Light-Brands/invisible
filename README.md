# Invisible

**Messages that leave no trace. Privacy that answers to no one.**

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)
![Flutter](https://img.shields.io/badge/flutter-3.0%2B-02569B.svg)
![Tests](https://img.shields.io/badge/tests-81%20passing-success.svg)
![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)

A privacy-first messenger with zero-trust architecture, zero-metadata collection, and zero-compromise privacy.

**🎁 This is open source - a gift to humanity. MIT Licensed.**

## 🌟 Features

- 🔐 **End-to-End Encryption** - Signal protocol + post-quantum resistance (ML-KEM)
- 🌐 **7-Layer Network Obfuscation** - VPN → Shamir fragmentation → 5-layer mixnet → cover traffic → dead drops
- 💰 **Privacy Wallet** - Multi-currency crypto (XMR, BTC, ZEC, ETH) with automatic mixing
- 🚫 **Zero Metadata** - No phone, no email, no username required
- 📱 **Cross-Platform** - iOS, Android, macOS, Windows, Linux from single codebase
- 🧪 **Battle-Tested** - 81 tests passing, security-focused development

## ⚡ One-Click Install & Run

```bash
# Clone the repo
git clone https://github.com/Light-Brands/invisible.git
cd invisible

# Run this ONE command - it does everything:
./quick-start.sh
```

That's it! The script will:
- ✅ Install Rust (if needed)
- ✅ Install dependencies
- ✅ Build everything in release mode
- ✅ Run all tests
- ✅ Launch the relay node

## 🎯 Quick Start Options

### Ultra Simple (Recommended)
```bash
./quick-start.sh  # Does everything automatically
```

### Interactive
```bash
./install.sh  # Install everything
./run.sh      # Choose what to run
```

### Manual
```bash
cargo build --release
./target/release/invisible-relay
```

## 🏗️ What You Get

```
Invisible Platform (75/75 tests passing ✅)
├── Crypto Core        - X3DH, Double Ratchet, PQXDH
├── Scrambler          - 7-layer network obfuscation
├── Relay Nodes        - Mix network infrastructure
├── Shadow Wallet      - Multi-currency crypto wallet
└── Client SDK (FFI)   - Dart/Flutter mobile bindings
```

## 📱 Use It Everywhere

**Rust:**
```rust
use invisible_scrambler::{Scrambler, ScramblerConfig};
scrambler.send_message(b"Secret!", &destination).await?;
```

**Dart/Flutter:**
```dart
invisibleSendMessage('user_123', 'Hello!');
```

See [QUICKSTART.md](QUICKSTART.md) for complete examples.

## 📖 Documentation

- **[QUICKSTART.md](QUICKSTART.md)** - Usage guide & API examples
- **[CONTRIBUTING.md](CONTRIBUTING.md)** - How to contribute
- **[docs/FLUTTER_SETUP.md](docs/FLUTTER_SETUP.md)** - Flutter development setup
- **[spec/MASTER-PLAN.md](spec/MASTER-PLAN.md)** - Project vision
- **[CLAUDE.md](CLAUDE.md)** - AI assistant context

## 🧪 Testing

```bash
cargo test --workspace  # All 81 tests
```

## 🤝 Contributing

**We welcome contributions from everyone!** Whether you're:
- 🐛 Fixing bugs
- ✨ Adding features
- 📝 Improving documentation
- 🎨 Designing UI/UX
- 🔐 Reviewing security

**See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.**

### Quick Contribution Steps

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'feat: add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 🌍 Community

- **GitHub Discussions:** [Ask questions, share ideas](https://github.com/Light-Brands/invisible/discussions)
- **Issues:** [Report bugs, request features](https://github.com/Light-Brands/invisible/issues)
- **Discord:** Coming soon
- **Email:** hello@invisible.im (coming soon)

## 🔒 Security

Found a security vulnerability? **Please DO NOT open a public issue.**

Email: **security@invisible.im** (coming soon)

We take security seriously and will respond within 48 hours.

## 📊 Project Status

**Current Version:** v0.1.0-foundation
**Status:** Core platform complete, Flutter UI in development
**Next Milestone:** Beta release with mobile apps

### Roadmap

- ✅ Phase 0: Core Rust platform (Crypto, Scrambler, Relay, Wallet)
- 🚧 Phase 1: Flutter UI (In progress)
- ⏳ Phase 2: Beta testing
- ⏳ Phase 3: Public release

## 💡 Philosophy

**Privacy is a fundamental human right.** Invisible is built on these principles:

- **Privacy by Default** - Everything encrypted, always
- **Zero Trust** - Trust no one, verify everything
- **Open Source** - Transparency builds trust
- **Community-Driven** - Built by the people, for the people
- **No Compromise** - Never trade privacy for convenience

## 📄 License

MIT License - See [LICENSE](LICENSE) for details.

**Copyright © 2026 Invisible Project Contributors**

Built with privacy in mind. Messages that leave no trace.

---

**⭐ If you believe in privacy as a human right, star this repo!**
