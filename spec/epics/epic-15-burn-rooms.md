# Epic 15: Burn Rooms — Self-Destructing Conversations

## Overview
Implement self-destructing chat rooms where every message, key, and trace is permanently destroyed when the timer expires.

## User Stories

### US-15.1: Create Burn Room
As a user, I want to create a self-destructing conversation with a configurable timer.

**Acceptance Criteria:**
- Timer presets: 15min, 60min (default), 4hr, 24hr, custom (min 5min)
- Messages auto-delete after timer from SEND time
- Room deletes after timer from last message
- Ephemeral room key generated (separate from conversation key)

### US-15.2: 1-on-1 Burn Chat
As a user, I want a timed burn chat with one other person.

**Acceptance Criteria:**
- Both parties must agree to timer setting
- Either party can trigger immediate burn
- Same E2EE as regular chat + ephemeral room key layer
- No screenshots, no copy/paste, no forwarding

### US-15.3: Group Burn Room
As a user, I want a group burn room for sensitive multi-party conversations.

**Acceptance Criteria:**
- Up to 50 participants
- Creator sets timer
- One-time invite links (expire after single use)
- Anonymous participation option (messages shown without sender names)
- Creator can trigger immediate burn for everyone
- Max participants configurable

### US-15.4: Burn Room Restrictions
As a user, I expect burn rooms to prevent any data persistence.

**Acceptance Criteria:**
- No screenshots (FLAG_SECURE / capture detection)
- No copy/paste of messages
- No forwarding messages
- No media saving
- Typing notifications disabled
- Read receipts disabled
- No visible participant list (see messages, not who's in room)

### US-15.5: Dead Man's Switch
As a user, I want burn rooms to auto-destruct if abandoned.

**Acceptance Criteria:**
- Configurable inactivity trigger: 30min / 1hr / 4hr
- If no messages sent within inactivity period, room auto-burns
- Prevents abandoned rooms from persisting
- Independent of main burn timer

### US-15.6: Coordinated Burn
As a user, I want burn to happen simultaneously on all participants' devices.

**Acceptance Criteria:**
- Burn signal: signed with room creator's key
- Sent through Scrambler to all participants
- Each device: secure wipe of room messages + destroy room key
- Dead drop queues for room abandoned (auto-purge on relay)
- No record room ever existed after burn

### US-15.7: Anti-Recovery
As a security guarantee, I want burned messages to be cryptographically unrecoverable.

**Acceptance Criteria:**
- Ephemeral room key destroyed on burn
- Even pre-burn database backup + forensic extraction cannot recover messages
- Secure overwrite (random bytes) of message data in local DB
- Room key never stored persistently — held in RAM during room lifetime

## Technical Requirements
- Ephemeral key: AES-256-GCM room key, RAM-only
- Burn signal: Ed25519-signed, routed through Scrambler
- Timer: high-precision local timer + tolerance for network delay
- Secure wipe: overwrite + unlink

## Dependencies
- Epic 00 (Foundation), Epic 01 (Messaging), Epic 04 (Groups), Epic 03 (Scrambler)

## Architecture References
- [burn-rooms.md](../architecture/burn-rooms.md), [group-messaging.md](../architecture/group-messaging.md), [zero-log-doctrine.md](../architecture/zero-log-doctrine.md)
