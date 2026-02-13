# Cryptography — Invisible Encryption Stack

## Overview

Invisible uses a layered encryption architecture providing forward secrecy, post-compromise security, and post-quantum resistance at every layer. No single algorithm failure can expose user messages. Each layer is independently auditable and replaceable without disrupting the others.

The encryption stack is tightly coupled with the [Identity System](identity-system.md) for key ownership verification and the [Data Models](data-models.md) for encrypted-at-rest storage schemas.

---

## Key Exchange

### X3DH — Initial Key Agreement

Extended Triple Diffie-Hellman (X3DH) establishes the initial shared secret between two parties who may be offline. The protocol uses identity keys, signed prekeys, and one-time prekeys to produce a shared secret with mutual authentication.

- Initiator computes three (or four, if one-time prekey is available) DH operations
- Output fed into HKDF-SHA-256 to derive the initial root key
- One-time prekeys are consumed on use and never reused

### PQXDH — Post-Quantum Extension

PQXDH extends X3DH with a post-quantum KEM encapsulation running in parallel:

- **Classical component:** X25519 Diffie-Hellman (unchanged from X3DH)
- **Post-quantum component:** ML-KEM (Kyber-1024) encapsulation against a PQ prekey
- Every key exchange produces both a classical and post-quantum shared secret, combined via HKDF
- The combined secret is secure if **either** the classical or post-quantum component remains unbroken
- PQ prekeys are bundled alongside signed prekeys and rotated on the same weekly schedule

See [Identity System](identity-system.md) for prekey bundle structure and upload lifecycle.

---

## Message Encryption — Double Ratchet

The Signal Protocol Double Ratchet provides per-message key derivation with both forward secrecy and post-compromise security:

- Each message is encrypted with a unique key derived from the current ratchet state
- **Sending chain** and **receiving chain** advance independently via symmetric-key ratchet steps (HMAC-based)
- A **Diffie-Hellman ratchet step** occurs whenever the conversation direction changes, introducing new entropy
- Message keys are destroyed immediately after decryption — a compromised device cannot decrypt messages already read and wiped
- Out-of-order messages are handled by storing skipped message keys (capped at 256 per chain, then force-rotated)

The ratchet state machine is defined in [Data Models](data-models.md) under the `ConversationSession` schema.

---

## Daily Master Key Rotation

Defense-in-depth: on top of the per-message ratchet, a daily master key rotation provides an absolute upper bound on key lifetime.

- New Diffie-Hellman exchange every 24 hours (configurable: 12h / 24h / 48h)
- Forces a full ratchet reset — the entire chain key hierarchy is re-derived from fresh keying material
- Limits the window of any key compromise to at most one rotation period, even if no messages are exchanged
- **Trigger rules:**
  - Rotation is triggered by the sender on the first message after the rotation period expires
  - If no messages are sent, rotation occurs on the next app open (foreground event)
  - Both parties must complete the rotation handshake before old keys are purged
- Rotation metadata (timestamps, epoch counters) is tracked in the session state; see [Data Models](data-models.md)

---

## Symmetric Encryption

All symmetric encryption uses **AES-256-GCM** with the following parameters:

| Parameter | Value |
|-----------|-------|
| Key size | 256 bits |
| Nonce size | 96 bits (random) |
| Tag size | 128 bits |

- 96-bit random nonces, never reused (probabilistic uniqueness guaranteed by random generation with a 96-bit space)
- Authentication tag is verified **before** any plaintext is released — decryption is atomic (authenticate-then-decrypt)
- Applied uniformly to: message payloads, local database fields, media file encryption, attachment encryption

---

## Digital Signatures

- **Ed25519** for all identity signatures and message authentication
- Each device has a long-term Ed25519 identity key generated on-device during registration (see [Identity System](identity-system.md))
- Signed prekeys for X3DH bundles are signed by the device identity key and rotated weekly
- Message-level authentication: each encrypted message includes an Ed25519 signature over the ciphertext and associated metadata (sender identity, timestamp, message index)
- Signature verification is mandatory — messages with invalid signatures are dropped silently

---

## Post-Quantum Resistance

Invisible implements a hybrid post-quantum strategy designed to survive the arrival of cryptographically-relevant quantum computers:

- **ML-KEM (CRYSTALS-Kyber, FIPS 203)** at the key exchange layer, security level 5 (Kyber-1024)
- **Hybrid construction:** classical X25519 + ML-KEM-1024 — both shared secrets are combined via HKDF before use
- If either algorithm is secure, the combined key is secure — no single quantum or classical break compromises the exchange
- **Future migration path:** prepared for post-quantum signature migration to ML-DSA (CRYSTALS-Dilithium, FIPS 204) when Ed25519 replacement becomes necessary
- PQ algorithm versions are tracked per-session to support protocol upgrades without breaking existing conversations

---

## Key Hierarchy

```
Device Identity Key (Ed25519, long-term)
|-- Signed Prekey (X25519, rotated weekly)
|-- One-Time Prekeys (X25519, single use)
|-- PQ Prekey (ML-KEM-1024, rotated weekly)
+-- Per-Conversation Root Key
    |-- Sending Chain Key -> Message Key 1, 2, 3...
    |-- Receiving Chain Key -> Message Key 1, 2, 3...
    +-- Daily Master Key (rotated every 24h)
        +-- New Root Key derivation
```

**Key lifecycle rules:**

- The Device Identity Key is the root of trust. It is generated once and never exported. See [Identity System](identity-system.md) for identity verification and safety number protocols.
- Signed Prekeys rotate weekly. The previous signed prekey is retained for 48 hours after rotation to handle in-flight messages, then securely deleted.
- One-Time Prekeys are generated in batches of 100, uploaded to the server, and consumed exactly once.
- The Per-Conversation Root Key is derived during X3DH/PQXDH and evolves with each DH ratchet step.
- Daily Master Key rotation re-derives the root key from fresh DH output, resetting both chain keys.

---

## Key Storage

- All keys are encrypted at rest via **SQLCipher** (AES-256 in CBC mode with HMAC-SHA-256 authentication)
- Database encryption key derived from: `Argon2id(device_key + passphrase + 2FA_secret)`
  - Argon2id parameters: 3 iterations, 256 MB memory, 4 lanes (tuned per-device on first setup)
  - The `device_key` is hardware-bound where available (Secure Enclave on iOS, StrongBox on Android)
- Identity keys **never leave the device** — there is no cloud key backup, no key escrow, no key recovery service
- If the device is lost, the identity is lost — this is by design, not a limitation
- Prekey bundles uploaded to the server contain only public keys; private components remain on-device

Storage schemas are defined in [Data Models](data-models.md) under the `KeyStore` and `SessionStore` models.

---

## Key Deletion Policy

Aggressive key deletion minimizes the window of exposure from device compromise:

| Key Type | Deletion Trigger |
|----------|-----------------|
| One-time prekeys | Deleted immediately after use in X3DH |
| Message keys | Deleted immediately after successful decryption |
| Skipped message keys | Deleted after 48 hours or when cap (256) is reached |
| Expired daily master keys | Securely wiped after both parties confirm rotation |
| Old ratchet states | Purged after message delivery is confirmed by recipient |
| Signed prekeys (previous) | Deleted 48 hours after rotation |
| PQ prekeys (previous) | Deleted 48 hours after rotation |

**Secure deletion protocol:**
1. Overwrite key material with cryptographically random bytes
2. Call `fsync` to flush to storage
3. Overwrite again with zeros
4. `fsync` again
5. Unlink the data

On platforms with flash storage where overwrite guarantees are weak, SQLCipher's encrypted database provides the defense layer — deleting the database key renders all stored keys unrecoverable.

---

## Forward Secrecy + Post-Compromise Security

These two properties are the foundation of Invisible's security guarantees:

**Forward secrecy:** Compromising current keys cannot decrypt past messages. Achieved through:
- Per-message key derivation via the symmetric ratchet
- Immediate deletion of message keys after use
- DH ratchet steps introduce new ephemeral keys that cannot be derived from prior state

**Post-compromise security:** After a key compromise, security is restored within one DH ratchet step. Achieved through:
- The DH ratchet introduces fresh entropy from a new ephemeral key pair on every direction change
- An attacker who compromises the current ratchet state loses access as soon as the next DH ratchet step completes
- Daily master key rotation provides a hard upper bound — even without any message exchange, keys refresh every 24 hours

The [Scrambler](scrambler.md) system builds on these guarantees to provide message deniability and metadata scrambling on top of the cryptographic layer.

---

## Cryptographic Primitives Summary

| Primitive | Algorithm | Purpose |
|-----------|-----------|---------|
| Key exchange | X25519 + ML-KEM-1024 | Hybrid post-quantum key agreement |
| Symmetric encryption | AES-256-GCM | Message and storage encryption |
| Signatures | Ed25519 | Identity and message authentication |
| KDF | HKDF-SHA-256 | Key derivation in ratchet |
| Password KDF | Argon2id | Database key derivation from credentials |
| Hash | SHA-256, BLAKE2b | Integrity, commitments |

Algorithm selection criteria: NIST-approved or widely peer-reviewed, constant-time implementations available, battle-tested in production systems (Signal, WireGuard, Noise framework).

---

## Implementation Notes

- **Language:** Crypto core implemented in Rust for memory safety, no garbage collector pauses, and deterministic resource cleanup
- **Libraries:**
  - `libsignal-protocol` — Double Ratchet, X3DH, session management
  - `ring` — AES-256-GCM, Ed25519, X25519, HKDF, SHA-256
  - `ml-kem` crate — ML-KEM-1024 (FIPS 203) key encapsulation
  - `argon2` crate — Argon2id key derivation
  - `blake2` crate — BLAKE2b hashing for commitments
- **Memory safety:**
  - All crypto operations run in memory-locked regions (`mlock` / `mprotect`) to prevent swapping sensitive material to disk
  - Zeroization of all sensitive material on drop via the `zeroize` crate (implements `Drop` with guaranteed zeroing)
  - No heap allocations for key material where avoidable; stack-allocated fixed-size buffers preferred
- **Timing safety:** Constant-time comparisons for all secret-dependent operations (`ring::constant_time::verify_slices_are_equal`)
- **Build configuration:** Rust crypto core compiled with `panic = "abort"` to prevent stack unwinding from leaking key material
- **Audit surface:** The crypto core is designed as a standalone crate (`invisible-crypto`) with a minimal public API, enabling focused third-party security audits independent of the application layer

---

## Cross-References

- **[Identity System](identity-system.md)** — Device identity key generation, safety number verification, prekey bundle management
- **[Data Models](data-models.md)** — Encrypted storage schemas (`KeyStore`, `SessionStore`, `ConversationSession`), ratchet state serialization
- **[Scrambler](scrambler.md)** — Metadata scrambling, message deniability, and traffic analysis resistance built on top of the encryption stack
