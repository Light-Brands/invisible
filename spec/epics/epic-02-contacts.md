# Epic 02: Contacts — Zero-ID Identity & Key Exchange

## Overview
Implement the zero-identifier contact system: no phone numbers, no emails, no usernames. Contact exchange via QR codes and one-time links only.

## User Stories

### US-02.1: QR Code Contact Exchange
As a user, I want to add a contact by scanning their QR code so we can start communicating without sharing any personal information.

**Acceptance Criteria:**
- QR code encodes: identity key + signed prekey + one-time prekey + PQ prekey
- Camera-based QR scanning
- X3DH key agreement initiated on scan
- Pairwise identifier derived for this conversation
- Contact added to encrypted local contact store

### US-02.2: One-Time Link Exchange
As a user, I want to share a one-time link to add a contact remotely.

**Acceptance Criteria:**
- Generate HTTPS link containing encrypted key bundle
- Link expires after single use
- Link expires after configurable time (default: 24h)
- Key exchange happens through relay network
- Link cannot be reused or intercepted for replay

### US-02.3: Pairwise Anonymous Identifiers
As a user, I want each contact to know me by a different identifier so no one can correlate my conversations.

**Acceptance Criteria:**
- Unique pairwise ID per conversation: HKDF(key_A, key_B, "pairwise")
- Contact A sees different ID than Contact B for same user
- Relay nodes see only queue identifiers, never identity keys
- No global identifier exists

### US-02.4: Key Verification (Safety Numbers)
As a user, I want to verify my contact's identity to prevent man-in-the-middle attacks.

**Acceptance Criteria:**
- Safety number: SHA-256(identity_key_A || identity_key_B) displayed as 60-digit number
- QR code verification option
- Key change alerts with visual warning
- Verification status stored locally

### US-02.5: Contact Management
As a user, I want to manage my contacts (rename, delete, block).

**Acceptance Criteria:**
- User-chosen display names (encrypted locally)
- Delete contact: removes all conversation data + keys
- Block: silently drops messages from blocked contact
- No contact discovery from address book

## Technical Requirements
- QR code: qr_code crate (Rust) + Flutter camera plugin
- One-time links: Ed25519-signed, AES-256-GCM encrypted bundles
- Pairwise IDs: HKDF-SHA-256

## Dependencies
- Epic 00 (Foundation)

## Architecture References
- [identity-system.md](../architecture/identity-system.md)
- [cryptography.md](../architecture/cryptography.md)
