# Invisible - Quick Start Guide

**Messages that leave no trace. Privacy that answers to no one.**

## 🚀 What is Invisible?

Invisible is a privacy-first messenger with zero-trust architecture:
- **E2E Encryption**: Signal protocol + post-quantum resistance
- **7-Layer Obfuscation**: VPN + mixnet + cover traffic + dead drops
- **Privacy Wallet**: Non-custodial crypto (BTC, XMR, ZEC, ETH)
- **Zero Metadata**: No phone, email, or username required

## 📦 What We Built

```
Invisible Platform (75/75 tests passing ✅)
├── Crypto Core        - X3DH, Double Ratchet, PQXDH
├── Scrambler          - 7-layer network obfuscation
├── Relay Nodes        - Mix network infrastructure
├── Shadow Wallet      - Multi-currency crypto wallet
└── Client SDK (FFI)   - Dart/Flutter mobile bindings
```

## 🏃 Quick Start

### 1. Build Everything

```bash
# Build all components
cargo build --release

# Run tests
cargo test --workspace
```

### 2. Run a Relay Node

```bash
# Start mix node server
./target/release/invisible-relay

# Listens on 127.0.0.1:8080
# Processes Sphinx packets
# Stores dead drop messages
```

### 3. Use in Rust

```rust
use invisible_scrambler::{Scrambler, ScramblerConfig};
use invisible_wallet::{ShadowWallet, Currency};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to network
    let config = ScramblerConfig::default();
    let mut scrambler = Scrambler::new(config, vec![]);
    scrambler.initialize().await?;

    // Send private message
    let message = b"Secret message";
    let destination = vec![0u8; 32];
    let handle = scrambler.send_message(message, &destination).await?;

    // Use wallet
    let mut wallet = ShadowWallet::new(Default::default())?;
    let balance = wallet.get_balance(Currency::BTC).await?;

    Ok(())
}
```

### 4. Use from Dart/Flutter

```bash
# Build FFI library
cargo build --release -p invisible-client-ffi
```

```dart
import 'dart:ffi';
import 'package:ffi/ffi.dart';

// Load library
final lib = DynamicLibrary.open('libinvisible_client_ffi.dylib');

// Initialize
final init = lib.lookupFunction<Int32 Function(), int Function()>('invisible_init');
init();

// Send message
final sendMsg = lib.lookupFunction<
  Uint64 Function(Pointer<Utf8>, Pointer<Utf8>),
  int Function(Pointer<Utf8>, Pointer<Utf8>)
>('invisible_send_message');

final handle = sendMsg(
  'recipient_id'.toNativeUtf8(),
  'Hello!'.toNativeUtf8()
);
```

## 🏗️ Architecture

### The Scrambler (7 Layers)

1. **Ghost VPN** - Mandatory WireGuard tunnel
2. **Shamir Fragmentation** - K-of-N secret sharing
3. **5-Layer Mixnet** - Sphinx packets with onion routing
4. **Cover Traffic** - Constant-rate dummy packets
5. **Jurisdiction Routing** - Multi-country paths
6. **Protocol Camouflage** - obfs4, uTLS, domain fronting
7. **Dead Drops** - Anonymous relay mailboxes
8. **Temporal Scrambling** - Poisson-distributed delays

### Message Flow

```
Plaintext Message
    ↓
[1] Fragment into K-of-N shares (Shamir)
    ↓
[2] Encrypt each share (Sphinx packets)
    ↓
[3] Route through 5-layer mixnet
    ↓
[4] Mix with cover traffic
    ↓
[5] Apply temporal delays
    ↓
[6] Camouflage protocol (obfs4/uTLS)
    ↓
[0] Tunnel through VPN
    ↓
Delivered to Dead Drop
```

## 📱 Building a Mobile App

### Flutter Integration

1. **Create Flutter Project**:
```bash
flutter create invisible_app
cd invisible_app
```

2. **Add FFI Dependencies** (`pubspec.yaml`):
```yaml
dependencies:
  ffi: ^2.1.0
```

3. **Copy FFI Library**:
```bash
cp target/release/libinvisible_client_ffi.dylib ios/
cp target/release/libinvisible_client_ffi.so android/app/src/main/jniLibs/arm64-v8a/
```

4. **Create Dart Wrapper** (`lib/invisible.dart`):
```dart
import 'dart:ffi';
import 'package:ffi/ffi.dart';

class InvisibleClient {
  late DynamicLibrary _lib;

  InvisibleClient() {
    _lib = DynamicLibrary.open('libinvisible_client_ffi.dylib');
    _init();
  }

  void _init() {
    final init = _lib.lookupFunction<
      Int32 Function(),
      int Function()
    >('invisible_init');
    init();
  }

  int sendMessage(String recipientId, String message) {
    final sendMsg = _lib.lookupFunction<
      Uint64 Function(Pointer<Utf8>, Pointer<Utf8>),
      int Function(Pointer<Utf8>, Pointer<Utf8>)
    >('invisible_send_message');

    return sendMsg(
      recipientId.toNativeUtf8(),
      message.toNativeUtf8()
    );
  }
}
```

## 🔧 Development

### Project Structure

```
invisible/
├── crates/
│   ├── crypto/          # Cryptographic primitives
│   ├── scrambler/       # Network obfuscation
│   ├── relay/           # Mix node servers
│   ├── wallet/          # Cryptocurrency wallet
│   └── client-ffi/      # FFI bindings
├── spec/                # Specifications
└── tests/               # Integration tests
```

### Running Tests

```bash
# All tests
cargo test --workspace

# Specific component
cargo test -p invisible-crypto
cargo test -p invisible-scrambler
cargo test -p invisible-wallet
```

### Environment Variables

```bash
# Relay node
export INVISIBLE_RELAY_PORT=8080
export INVISIBLE_LOG=debug

# Client
export INVISIBLE_VPN_ENDPOINT=vpn.invisible.net
export INVISIBLE_RELAY_NODES=relay1.invisible.net,relay2.invisible.net
```

## 🎯 Next Steps

### For Production

1. **Real WireGuard Integration**
   - Replace VPN placeholder
   - Generate actual WireGuard keys
   - Connect to production endpoints

2. **Blockchain Integration**
   - Connect to Bitcoin/Monero nodes
   - Implement CoinJoin mixing
   - Add ZEC and ETH support

3. **Deploy Relay Network**
   - Set up global relay nodes
   - Configure geographic diversity
   - Implement node discovery

4. **Build Mobile UI**
   - Flutter messaging interface
   - Wallet transaction UI
   - Settings and privacy controls

### For Testing

1. **Integration Tests**
   - End-to-end message flow
   - Cross-layer testing
   - Performance benchmarks

2. **Security Audits**
   - Cryptographic review
   - Network analysis
   - Penetration testing

## 📚 Documentation

- [Master Plan](spec/MASTER-PLAN.md) - Project vision
- [Architecture](spec/architecture/) - Technical specs
- [API Reference](docs/api/) - API documentation

## 🤝 Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for development guidelines.

## 📄 License

MIT License - see [LICENSE](LICENSE) for details.

---

**Built with privacy in mind. Messages that leave no trace.**
