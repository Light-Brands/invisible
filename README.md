# Invisible

**Messages that leave no trace. Privacy that answers to no one.**

A privacy-first messenger with zero-trust architecture, zero-metadata collection, and zero-compromise privacy.

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
- **[spec/MASTER-PLAN.md](spec/MASTER-PLAN.md)** - Project vision
- **[CLAUDE.md](CLAUDE.md)** - AI assistant context

## 🧪 Testing

```bash
cargo test --workspace  # All 75 tests
```

## 📄 License

MIT License - Built with privacy in mind.
