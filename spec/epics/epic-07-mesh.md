# Epic 07: Mesh — Bluetooth/WiFi Direct Offline Messaging

## Overview
Enable peer-to-peer messaging via Bluetooth and WiFi Direct when internet is unavailable, with store-and-forward for reaching offline contacts through mesh relay.

## User Stories

### US-07.1: Bluetooth Direct Messaging
As a user, I want to send encrypted messages via Bluetooth when I have no internet.

**Acceptance Criteria:**
- BLE (Bluetooth Low Energy) transport for nearby contacts
- Same E2EE as internet messaging (Double Ratchet keys)
- Automatic discovery of nearby Invisible users (opt-in)
- Range: ~30-100 meters typical BLE range
- No internet connection required

### US-07.2: WiFi Direct Messaging
As a user, I want to send messages via WiFi Direct for higher bandwidth offline transfer.

**Acceptance Criteria:**
- WiFi Direct connection for nearby contacts
- Higher bandwidth than BLE (suitable for media transfer)
- Encrypted with same conversation keys
- Automatic fallback: internet → WiFi Direct → BLE

### US-07.3: Store-and-Forward Mesh
As a user, I want my messages to reach offline contacts through intermediate mesh nodes.

**Acceptance Criteria:**
- Encrypted messages stored on intermediate devices
- Forwarded when intermediate device encounters the recipient
- Messages remain E2EE (intermediate devices can't read them)
- TTL: messages expire after configurable period
- Hop limit: max 5 hops to prevent infinite relay

### US-07.4: Mesh Network Discovery
As a user, I want to discover nearby Invisible users without revealing my identity.

**Acceptance Criteria:**
- Anonymous beacon broadcast (no identity info)
- Connection requires mutual opt-in
- Discovery uses random ephemeral identifiers
- No persistent mesh identity

## Technical Requirements
- BLE: Flutter Blue Plus plugin
- WiFi Direct: platform-specific APIs (Android WiFi P2P, iOS Multipeer Connectivity)
- Store-and-forward: encrypted blob with recipient's public key
- Mesh routing: gossip protocol with TTL

## Dependencies
- Epic 00 (Foundation), Epic 01 (Messaging), Epic 02 (Contacts)

## Architecture References
- [cryptography.md](../architecture/cryptography.md), [identity-system.md](../architecture/identity-system.md)
