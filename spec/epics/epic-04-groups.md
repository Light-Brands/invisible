# Epic 04: Groups — MLS-Based Group Messaging

## Overview
Implement secure group messaging using the Message Layer Security (MLS) protocol (RFC 9420) with encrypted member lists and forward secrecy.

## User Stories

### US-04.1: Create Group
As a user, I want to create an encrypted group conversation.

**Acceptance Criteria:**
- Generate MLS group with TreeKEM ratchet tree
- Founder as initial leaf node
- Group identifier is random (unlinkable to members)
- Encrypted member list stored locally only
- Server never sees group composition

### US-04.2: Add Members
As a group member, I want to invite others via QR code or one-time link.

**Acceptance Criteria:**
- MLS Welcome message + tree Update
- New member receives current group state
- Group key rotated on member add
- All existing members notified (encrypted)

### US-04.3: Remove Members
As a group admin, I want to remove a member and revoke their access.

**Acceptance Criteria:**
- Member removal triggers tree update
- Full key rotation (removed member loses future access)
- Forward secrecy maintained
- All remaining members update their tree state

### US-04.4: Send/Receive Group Messages
As a group member, I want to send and receive encrypted messages in the group.

**Acceptance Criteria:**
- Message encrypted with current group key (AES-256-GCM)
- Routed through Scrambler (same as 1-on-1)
- All members retrieve from their dead drop queues
- Out-of-order handling for group messages

### US-04.5: Group Key Updates
As a user, I want group keys to periodically refresh for post-compromise security.

**Acceptance Criteria:**
- Members periodically update their leaf keys
- Logarithmic update cost (TreeKEM)
- Post-compromise security: one update restores security after compromise

## Technical Requirements
- MLS: OpenMLS crate (Rust)
- Max group size: 1000 members (recommended ≤50)
- Group messages fragmented via Scrambler like 1-on-1

## Dependencies
- Epic 00 (Foundation)
- Epic 01 (Messaging)
- Epic 02 (Contacts)
- Epic 03 (Scrambler)

## Architecture References
- [group-messaging.md](../architecture/group-messaging.md)
- [cryptography.md](../architecture/cryptography.md)
