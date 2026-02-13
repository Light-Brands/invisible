# Invisible - Project Context for AI Assistants

**Messages that leave no trace. Privacy that answers to no one.**

Invisible is a maximally secure, privacy-first messenger with zero-trust architecture, zero-metadata collection, and zero-compromise privacy. Built for internal team use with no feature gating.

## Project Identity

**Type:** Privacy-focused secure messaging platform with integrated crypto wallet
**Stage:** Early development - specification complete, implementation starting
**Purpose:** Internal team communications with maximum security and privacy

## Always Apply Rules

Core rules that apply to all tasks:

@rules/git-interaction.mdc
@rules/security-first.mdc

## Tech Stack

### Core Components

- **Crypto Core:** Rust (libsignal-protocol, ring, ML-KEM, Shamir Secret Sharing)
- **Mobile Clients:** Flutter/Dart
- **Relay/Mix Nodes:** Rust (Sphinx packet format, Loopix-inspired)
- **VPN:** WireGuard (ChaCha20 + Poly1305 + Curve25519)
- **Local Storage:** SQLCipher + Argon2id KDF
- **Networking:** libp2p, Tor fallback
- **Transports:** obfs4/obfs5, uTLS fingerprint mimicry
- **Voice/Video:** WebRTC + custom SRTP key exchange
- **Wallet:** monero-rs, zcash_client_backend, rust-bitcoin, ethers-rs
- **Swaps:** COMIT HTLC library, XMR-BTC atomic swap protocol
- **DeFi:** WalletConnect v2, custom RPC proxy
- **2FA:** TOTP (RFC 6238) + FIDO2/WebAuthn

### Development Tools

- **Rust:** 1.70+ (stable), rustfmt, clippy
- **Testing:** cargo test, criterion (benchmarks), proptest (property testing)
- **CI/CD:** GitHub Actions
- **Pre-commit:** rustfmt, clippy, security audit checks

## Architecture Overview

### Core Privacy Principles

**Privacy Parity:** Every privacy layer protecting messages also protects payments. No exceptions.

**The Scrambler:** 7-layer network obfuscation system
1. **Ghost VPN (Layer 0):** Mandatory WireGuard tunnel, random global endpoint
2. **Shamir Fragmentation (Layer 1):** K-of-N shares across separate paths
3. **5-Layer Mixnet (Layer 2):** Sphinx packets, batch-shuffle-forward
4. **Cover Traffic (Layer 3):** Constant-rate stream, real ops replace dummies
5. **Jurisdiction Routing (Layer 4):** Multi-country paths, no Five Eyes clustering
6. **Protocol Camouflage (Layer 5):** obfs5/uTLS/domain fronting
7. **Dead Drops (Layer 6):** Anonymous relay mailboxes
8. **Temporal Scrambling (Layer 7):** Poisson-distributed random delays

### Security Features

- **Zero Identifiers:** No phone, no email, no username - ever
- **E2EE:** X3DH + PQXDH + Double Ratchet with post-quantum resistance
- **Zero-Log Doctrine:** RAM-only relay nodes, no disk writes
- **Mandatory 2FA:** Architecturally required, part of key derivation
- **Burn Rooms:** Self-destructing conversations with ephemeral keys
- **Panic Wipe:** Duress PIN/gesture destroys all data including wallet
- **Shadow Wallet:** Non-custodial privacy-first crypto (XMR, ZEC, BTC, ETH)

## Project Structure

```
invisible/
├── Cargo.toml                 # Workspace manifest
├── AGENTS.md                  # This file
├── README.md                  # Project overview
├── .ai-coding-config/         # Submodule: AI tooling configuration
├── spec/                      # Complete specifications
│   ├── MASTER-PLAN.md         # Strategic vision
│   ├── architecture/          # Technical deep-dives
│   ├── epics/                 # Feature development specs
│   └── brand/                 # Visual identity and voice
└── crates/                    # Rust workspace (to be created)
    ├── crypto/                # Cryptography primitives
    ├── scrambler/             # Network obfuscation
    ├── relay/                 # Relay node implementation
    ├── wallet/                # Shadow Wallet
    └── client/                # Client libraries
```

## Development Workflow

### Security-First Principles

1. **Constant-Time Operations:** All crypto operations must be constant-time
2. **Memory Safety:** Use zeroize for sensitive data, no unsafe unless audited
3. **Defense in Depth:** Multiple independent security layers
4. **Fail Secure:** Errors default to secure state (e.g., connection drop vs. plaintext)
5. **Minimal Attack Surface:** Remove unused dependencies, minimize complexity

### Code Quality Standards

- **Tests Required:** All crypto code needs unit tests + property tests
- **Fuzzing:** Critical parsers and crypto code must be fuzzed
- **Security Audits:** External audits before production use
- **Performance:** Benchmark crypto operations, optimize hot paths
- **Documentation:** Explain security assumptions and threat model

### Git Workflow

**Commit Format:** `{emoji} {imperative verb} {concise description}`

Security-relevant emojis:
- 🔒 Security improvements
- 🔐 Cryptography changes
- 🛡️ Privacy enhancements
- 🔑 Key management
- ⚡ Performance optimizations
- ✅ Tests added/updated
- 📝 Documentation

**Branch Strategy:**
- `main` - stable, deployable code
- `feature/*` - new features
- `security/*` - security fixes (fast-track review)
- `epic/*` - epic-level work

### Testing Philosophy

1. **Unit Tests:** Every public function, especially crypto
2. **Property Tests:** Use proptest for crypto invariants
3. **Integration Tests:** Test component interactions
4. **Fuzzing:** AFL/cargo-fuzz for parsers and crypto
5. **Benchmarks:** Criterion benchmarks for performance tracking

## Key Commands for This Project

### Development
- `/autotask` - Autonomous feature implementation
- `/systematic-debugging` - Root cause analysis for complex bugs
- `/verify-fix` - Confirm security fixes actually work

### Security & Quality
- `/security-reviewer` - OWASP audits, crypto review, vulnerability scanning
- `/architecture-auditor` - Design pattern validation, dependency audits
- `/test-engineer` - Write comprehensive tests for crypto code
- `/error-handling-reviewer` - Ensure errors fail securely

### Code Review
- `/multi-review` - Run multiple specialized reviewers in parallel
- `/robustness-reviewer` - Production readiness, resilience checking
- `/performance-reviewer` - Find bottlenecks, optimize hot paths

### Workflow
- `/load-rules` - Smart context loading for current task
- `/wrap-up` - Merge PR, sync local, clean up branch

## Important Constraints

### Never Do This
- ❌ Use unsafe Rust without thorough review and justification
- ❌ Store sensitive data without zeroizing on drop
- ❌ Use variable-time operations for crypto
- ❌ Trust user input without validation
- ❌ Skip tests for security-critical code
- ❌ Commit secrets or test keys to git
- ❌ Use deprecated crypto primitives

### Always Do This
- ✅ Use constant-time comparisons for secrets
- ✅ Zeroize sensitive data (keys, passwords, plaintexts)
- ✅ Add property tests for crypto invariants
- ✅ Document threat model and security assumptions
- ✅ Benchmark performance-critical paths
- ✅ Fuzz parsers and protocol handlers
- ✅ Review dependencies for security issues

## Dependency Philosophy

**Minimize Attack Surface:**
- Use well-audited cryptography libraries (ring, libsignal)
- Avoid unnecessary dependencies
- Audit all dependencies with cargo-audit
- Pin versions, review updates carefully
- Prefer pure Rust over FFI when possible

**Critical Dependencies (audited):**
- `ring` - Cryptographic primitives
- `libsignal-protocol` - Signal protocol implementation
- `argon2` - Password hashing
- `zeroize` - Secure memory clearing
- `subtle` - Constant-time operations

## Links

- [Master Plan](spec/MASTER-PLAN.md) - Strategic vision and positioning
- [Architecture Specs](spec/architecture/) - Technical deep-dives
- [Epics](spec/epics/) - Feature development roadmap
- [Crypto Spec](spec/architecture/cryptography.md) - Encryption stack
- [Scrambler Spec](spec/architecture/scrambler.md) - Network obfuscation
- [Shadow Wallet Spec](spec/architecture/shadow-wallet.md) - Crypto wallet

## Context for AI Assistants

When working on Invisible:
1. **Security is paramount** - every decision should prioritize user privacy
2. **Assume sophisticated adversaries** - nation-state level capabilities
3. **Defense in depth** - multiple independent security layers
4. **Privacy parity** - messages and money get equal protection
5. **Zero trust** - verify everything, trust nothing
6. **Fail secure** - errors should never leak data or metadata

This is not just another messenger. This is a tool for people whose safety depends on unbreakable privacy.
