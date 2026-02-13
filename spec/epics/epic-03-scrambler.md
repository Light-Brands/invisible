# Epic 03: The Scrambler — 7-Layer Network Obfuscation

## Overview
Implement Invisible's core differentiator: the 7-layer network obfuscation system that makes traffic analysis effectively impossible, even for a global passive adversary.

## User Stories

### US-03.1: Sphinx Packet Format
As a developer, I need a Sphinx packet implementation so all network packets are cryptographically uniform and unlinkable.

**Acceptance Criteria:**
- All packets exactly 2KB
- Nested encryption layers (one per mix node)
- Packets indistinguishable from each other (no metadata leakage)
- Per-hop routing info encrypted for each node
- Replay protection via tag checking

### US-03.2: Message Fragmentation (Layer 1)
As a user, I want my messages split across multiple paths so intercepting any single path reveals nothing.

**Acceptance Criteria:**
- Shamir's Secret Sharing: configurable K-of-N (default 3-of-5)
- Each share padded to Sphinx packet size
- Shares routed through different geographic paths
- Reassembly requires K shares
- K-1 shares reveal zero information (information-theoretic)

### US-03.3: Mixnet Routing (Layer 2)
As a user, I want my messages routed through a 5-layer mixnet for traffic analysis resistance.

**Acceptance Criteria:**
- 5-layer path: Entry Gateway → Mix 1 → Mix 2 → Mix 3 → Exit Gateway
- Each node: batch collect → decrypt layer → random delay (50ms-5s) → shuffle → forward
- Loopix-inspired architecture
- Geographically distributed nodes per layer
- Path selection is client-side

### US-03.4: Cover Traffic (Layer 3)
As a user, I want constant background traffic so an observer can't tell when I'm actually messaging.

**Acceptance Criteria:**
- Constant-rate Sphinx packets (default: 1/second)
- Real messages replace cover packets (no traffic pattern change)
- Cover packets are loop traffic (return to sender)
- Inter-node cover traffic between mix nodes
- Configurable rate: 0.5-5 packets/second

### US-03.5: Jurisdiction Routing (Layer 4)
As a user, I want my message path to cross multiple legal jurisdictions so no single government can surveil my full path.

**Acceptance Criteria:**
- No two consecutive nodes in same country or intelligence alliance
- Avoids Five Eyes clustering
- Minimum 5 countries per path
- Node directory includes jurisdiction metadata
- Prefers strong privacy law jurisdictions

### US-03.6: Protocol Camouflage (Layer 5)
As a user, I want my Invisible traffic to look like normal web traffic so DPI can't identify it.

**Acceptance Criteria:**
- Default transport: obfs5 (random bytes to DPI)
- Fallback: uTLS mimicking Chrome/Firefox TLS fingerprints
- Emergency: domain fronting via CDN
- Packet size/timing shaping matches chosen profile
- DPI cannot distinguish from normal browsing

### US-03.7: Dead Drop Architecture (Layer 6)
As a user, I want my messages deposited at anonymous relay points so sender and recipient never connect to the same node.

**Acceptance Criteria:**
- Sender deposits at anonymous mailbox
- Recipient retrieves from different relay
- Dead drop cannot correlate sender and recipient
- IDs rotate per session
- Multiple dead drops per conversation

### US-03.8: Temporal Scrambling (Layer 7)
As a user, I want random delays added to my messages so timing analysis is impossible.

**Acceptance Criteria:**
- Local random delay before mixnet entry (Poisson distribution)
- Each mix node adds independent random delay
- Recipient polls on own schedule
- No temporal correlation between send and receive
- Configurable: "instant" (low delay) to "maximum" (up to 60s)

## Technical Requirements
- Sphinx: custom Rust implementation
- Shamir: sss crate
- Mix nodes: Rust + tokio async
- Pluggable transports: obfs4/obfs5, uTLS (Go via FFI or Rust port)
- All layers independently testable

## Dependencies
- Epic 00 (Foundation)
- Epic 01 (Messaging) — for integration
- Epic 14 (Relay Nodes) — for infrastructure

## Architecture References
- [scrambler.md](../architecture/scrambler.md)
- [ghost-vpn.md](../architecture/ghost-vpn.md)
- [zero-log-doctrine.md](../architecture/zero-log-doctrine.md)
