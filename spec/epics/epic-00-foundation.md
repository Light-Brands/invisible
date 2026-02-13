# Epic 00: Foundation — Core Crypto Library & Local Storage

## Overview
Build the cryptographic foundation that everything else depends on: the Rust crypto core, encrypted local storage, and device identity generation.

## User Stories

### US-00.1: Crypto Core Library
As a developer, I need a Rust crypto library implementing Signal Protocol Double Ratchet, X3DH, and Ed25519 so all higher-level features have a secure cryptographic foundation.

**Acceptance Criteria:**
- X3DH key agreement implemented and tested
- Double Ratchet with sending/receiving chains
- Ed25519 key generation and signing
- All operations in memory-locked regions (mlock)
- Zeroization of sensitive data on drop
- Constant-time comparisons for secret-dependent operations
- Full test suite with known-answer tests

### US-00.2: Post-Quantum Key Exchange
As a user, I want my key exchanges to be resistant to quantum computing attacks so my messages remain secure even against future quantum adversaries.

**Acceptance Criteria:**
- PQXDH implementation: ML-KEM-1024 (Kyber) + X25519 hybrid
- Combined shared secret via HKDF
- If either algorithm is secure, combined key is secure
- Integration tests with Double Ratchet

### US-00.3: Encrypted Local Database
As a user, I want all my data encrypted at rest so that physical device compromise doesn't expose my messages.

**Acceptance Criteria:**
- SQLCipher database with AES-256 encryption
- Key derived via Argon2id (device_key + passphrase + 2FA_secret)
- Argon2id params: 256MB memory, 4 iterations, 4 parallelism
- Schema for messages, conversations, contacts, key_store tables
- Secure deletion with random byte overwrite

### US-00.4: Device Identity Generation
As a user, I want my device to generate a unique cryptographic identity on first launch so I can communicate without registering an account.

**Acceptance Criteria:**
- Ed25519 identity key pair generated from CSPRNG
- Identity key stored in encrypted key_store
- Signed prekey generation (X25519, rotated weekly)
- One-time prekey batch generation
- PQ prekey generation (ML-KEM-1024)

### US-00.5: Daily Master Key Rotation
As a user, I want my encryption keys to rotate daily for defense-in-depth beyond per-message ratcheting.

**Acceptance Criteria:**
- Daily DH exchange on top of Double Ratchet
- Configurable rotation period (12h / 24h / 48h)
- Rotation triggered on first message after period
- If no messages, rotation occurs on next app open
- Old master keys securely wiped after rotation

## Technical Requirements
- Language: Rust
- Libraries: libsignal-protocol, ring, ML-KEM crate, sss, zeroize
- All crypto operations tested with NIST/RFC test vectors
- Memory safety: no unsafe blocks in crypto paths
- FFI bindings for Flutter/Dart client

## Dependencies
- None (foundation epic)

## Architecture References
- [cryptography.md](../architecture/cryptography.md)
- [data-models.md](../architecture/data-models.md)
- [identity-system.md](../architecture/identity-system.md)
