# Epic 01: Messaging — 1-on-1 E2EE Communication

## Overview
Implement 1-on-1 end-to-end encrypted messaging using the Double Ratchet protocol with disappearing messages.

## User Stories

### US-01.1: Send Encrypted Message
As a user, I want to send an end-to-end encrypted message to a contact so only they can read it.

**Acceptance Criteria:**
- Message encrypted with Double Ratchet derived key (AES-256-GCM)
- New message key per message (forward secrecy)
- Message key destroyed after encryption
- Message routed through relay network
- Delivery confirmation (encrypted)

### US-01.2: Receive and Decrypt Message
As a user, I want to receive and decrypt messages from contacts seamlessly.

**Acceptance Criteria:**
- Retrieve encrypted messages from dead drop queue
- Decrypt with correct ratchet state
- Handle out-of-order messages (message key caching with limit)
- Message key destroyed after decryption
- Store decrypted message in encrypted local DB

### US-01.3: Disappearing Messages
As a user, I want messages to auto-delete after a configurable time period so nothing persists beyond my chosen retention.

**Acceptance Criteria:**
- Retention options: 24h (default), 7d, 30d, 90d
- No "forever" option
- Timer starts from message send time
- Secure deletion (random byte overwrite)
- Purge runs on background timer + every app launch
- Per-conversation retention settings

### US-01.4: Message Status
As a user, I want to know if my message was delivered.

**Acceptance Criteria:**
- Delivery confirmation via encrypted acknowledgment
- Status: pending → delivered
- No read receipts by default (optional, per-conversation)
- Status metadata stored locally only

### US-01.5: Offline Message Queue
As a user, I want messages sent to me while offline to be available when I reconnect.

**Acceptance Criteria:**
- Dead drop queues hold messages up to 72 hours
- Messages retrieved on reconnect
- Multiple dead drops per conversation for redundancy
- Queue IDs rotate per session

## Technical Requirements
- Double Ratchet from libsignal-protocol (Rust)
- AES-256-GCM via ring crate
- Message serialization: Protocol Buffers (compact, typed)
- Max message size: 64KB (larger = chunked)

## Dependencies
- Epic 00 (Foundation)

## Architecture References
- [cryptography.md](../architecture/cryptography.md)
- [data-models.md](../architecture/data-models.md)
- [scrambler.md](../architecture/scrambler.md)
