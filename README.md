# Invisible

**Messages that leave no trace. Privacy that answers to no one.**

Privacy-first secure messenger with zero-trust architecture, zero-metadata collection, and zero-compromise privacy. Built with maximum security for internal team communications.

## 🎯 Vision

The world's most private messenger combining Signal Protocol security with 7-layer network obfuscation and integrated privacy-first crypto wallet.

## ⚡ Quick Start

### Prerequisites

- **Rust:** 1.70+ (stable)
- **Flutter:** 3.0.0+ 
- **Docker:** For development environment
- **Node.js:** For tooling (optional)

### Development Setup

```bash
# Clone repository
git clone https://github.com/Light-Brands/invisible.git
cd invisible

# Start development environment (3-node mixnet)
docker-compose up -d dev

# Run tests
cargo test --all

# Build mobile app
cd mobile
flutter pub get
flutter run
```

### Docker Quick Start

```bash
# Start full stack (relay network + monitoring)
docker-compose up -d

# View logs
docker-compose logs -f relay-1

# Access Grafana dashboards
open http://localhost:3000
```

## 🏗️ Architecture

```
┌─────────────────────────────────────────┐
│         Flutter Mobile App              │ ← Material Design 3
└──────────────┬──────────────────────────┘
               │ FFI (C ABI)
┌──────────────▼──────────────────────────┐
│         Client Library                  │ ← High-level API
├─────────────────────────────────────────┤
│         Messaging Engine                │ ← Conversations, sessions
├─────────────────────────────────────────┤
│  Crypto (X3DH + Double Ratchet)        │ ← Signal Protocol
├─────────────────────────────────────────┤
│  Scrambler (7-Layer Obfuscation)       │ ← Sphinx + Mixnet
├─────────────────────────────────────────┤
│  Storage (SQLCipher + Argon2id)        │ ← AES-256 encryption
├─────────────────────────────────────────┤
│  Shadow Wallet (Multi-currency)         │ ← XMR, ZEC, BTC, ETH
└─────────────────────────────────────────┘
```

## 📦 Project Structure

```
invisible/
├── crates/                    # Rust workspace
│   ├── crypto/                # Cryptographic primitives
│   ├── scrambler/             # 7-layer network obfuscation
│   ├── relay/                 # Mix network relay node
│   ├── wallet/                # Shadow Wallet (multi-currency)
│   ├── storage/               # Encrypted SQLCipher database
│   ├── messaging/             # Core messaging engine
│   ├── client/                # High-level client library
│   └── ffi/                   # Flutter FFI bridge
├── mobile/                    # Flutter mobile app
│   ├── lib/screens/           # UI screens
│   ├── lib/widgets/           # Reusable widgets
│   └── lib/services/          # Service layer
├── tests/                     # Integration tests
├── docker/                    # Docker configuration
├── monitoring/                # Prometheus + Grafana
└── spec/                      # Technical specifications
```

## 🔐 Security Features

### Cryptography
- **X3DH** - Extended Triple Diffie-Hellman key agreement
- **Double Ratchet** - Forward secrecy and post-compromise security
- **Ed25519** - Identity signatures
- **X25519** - Key exchange
- **AES-256-GCM** - Message encryption
- **Argon2id** - Key derivation (65536 memory, 3 iterations)

### Network Privacy
- **Ghost VPN** - Mandatory WireGuard tunnel
- **Sphinx Packets** - Cryptographic packet format
- **5-Layer Mixnet** - Batch-shuffle-forward mixing
- **Cover Traffic** - Constant-rate dummy packets
- **Temporal Delays** - Poisson-distributed timing obfuscation
- **Jurisdiction Routing** - Multi-country path selection

### Data Protection
- **SQLCipher** - Encrypted database (AES-256)
- **Zeroization** - Memory cleared on drop
- **2FA Integration** - TOTP part of encryption key
- **No Plaintext** - Everything encrypted at rest
- **Panic Wipe** - Duress PIN destroys all data

## 💳 Shadow Wallet

Privacy-first multi-currency wallet integrated with messaging:

- **Monero (XMR)** - Privacy by default
- **Zcash (ZEC)** - Shielded transactions
- **Bitcoin (BTC)** - CoinJoin support
- **Ethereum (ETH)** - Privacy RPC proxy

Features:
- HD Wallet (BIP39/BIP44)
- Atomic swaps (HTLC)
- Non-custodial
- Privacy parity with messaging

## 🧪 Testing

```bash
# Unit tests
cargo test

# Integration tests
cargo test --test integration

# Property tests
cargo test --all-features -- --include-ignored proptest

# Benchmarks
cargo bench

# Coverage
cargo tarpaulin --all-features
```

## 📊 Monitoring

Development environment includes:

- **Prometheus** - Metrics collection (http://localhost:9090)
- **Grafana** - Visualization dashboards (http://localhost:3000)
- **Relay Metrics** - Packets, latency, batch sizes

## 🚀 Deployment

See deployment guides:
- **Docker:** `docker/README.md`
- **Kubernetes:** `k8s/README.md` (TODO)
- **Terraform:** `terraform/README.md` (TODO)

## 📱 Mobile Development

```bash
cd mobile

# Install dependencies
flutter pub get

# Run on device
flutter run

# Build APK
flutter build apk --release

# Build iOS
flutter build ios --release
```

## 🔨 Development Workflow

### Pre-commit Checks

```bash
# Format
cargo fmt --all

# Lint
cargo clippy --all-targets --all-features

# Security audit
cargo audit
```

### CI/CD

GitHub Actions runs:
- Multi-platform tests (Linux, macOS, Windows)
- Clippy linting
- Security audits
- Code coverage
- Benchmarks

## 📚 Documentation

- [Master Plan](spec/MASTER-PLAN.md) - Strategic vision
- [Architecture](spec/architecture/) - Technical specs
- [Crypto Spec](spec/architecture/cryptography.md) - Encryption details
- [Scrambler Spec](spec/architecture/scrambler.md) - Network obfuscation
- [CLAUDE.md](CLAUDE.md) - AI assistant context

## 🤝 Contributing

Internal project. For security issues, contact security@invisible.im

## 📄 License

Proprietary - Internal use only

## 🎯 Roadmap

- [x] Phase 0: Foundation (Weeks 1-12)
  - [x] Core crypto (X3DH, Double Ratchet)
  - [x] Network obfuscation (Scrambler)
  - [x] Encrypted storage (SQLCipher)
  - [x] Shadow Wallet (multi-currency)
  - [x] CI/CD pipeline
  - [x] Docker dev environment
- [ ] Phase 1: Core Messaging (Weeks 13-24)
  - [x] Messaging engine foundation
  - [x] Client library
  - [x] Flutter UI screens
  - [ ] Double Ratchet integration
  - [ ] Message sync protocol
  - [ ] Group messaging
- [ ] Phase 2: Voice/Video (Weeks 25-36)
- [ ] Phase 3: Mobile Polish (Weeks 37-48)
- [ ] Phase 4: Advanced Features (Weeks 49-60)
- [ ] Phase 5: Scale & Optimize (Weeks 61-72)
- [ ] Phase 6: Security Audit (Weeks 73-84)
- [ ] Phase 7: Production Launch (Weeks 85-88)

---

**Built with 🔒 for maximum privacy**
