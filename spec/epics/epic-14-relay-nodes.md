# Epic 14: Relay Nodes — Community Infrastructure

## Overview
Build the relay, mix, and VPN node software that anyone can run. Nodes are stateless, RAM-only, zero-log by design, with warrant canary attestations.

## User Stories

### US-14.1: Relay Node Software
As a community member, I want to run an Invisible relay node to support the network.

**Acceptance Criteria:**
- Single binary (Rust) for relay/mix node
- RAM-only packet processing (no disk writes in packet path)
- Memory-locked process (mlock/mlockall, no swap)
- Configurable resource limits (bandwidth, connections)
- Runs on commodity Linux VPS

### US-14.2: Mix Node Operation
As a node operator, I want my node to participate in the mixnet with proper batching and shuffling.

**Acceptance Criteria:**
- Sphinx packet processing: decrypt one layer, add delay, shuffle, forward
- Configurable batch size and delay parameters
- Cover traffic generation between nodes
- Layer assignment via network consensus
- Geographic metadata published for jurisdiction routing

### US-14.3: VPN Node Operation
As a node operator, I want to run a Ghost VPN endpoint.

**Acceptance Criteria:**
- WireGuard server with ephemeral client keys
- No connection logging, no bandwidth logging per user
- RAM-only operation (no disk)
- Health reporting to node directory
- Auto-registration in node pool

### US-14.4: Zero-Log Enforcement
As a node operator, I want technical assurance that my node can't log user data even if I wanted to.

**Acceptance Criteria:**
- No filesystem writes in packet processing codepath
- Operational logs contain zero user-attributable data
- Process runs in memory-locked region (no swap)
- Crash = zero recoverable data
- Seized server = empty drive (no useful forensic artifacts)

### US-14.5: Warrant Canary
As a network participant, I want assurance that node operators haven't been compromised by legal orders.

**Acceptance Criteria:**
- Nodes publish signed "no warrants received" attestations
- Attestations refreshed on schedule (weekly)
- Absence of fresh attestation = automatic exclusion from node pool
- Attestation includes timestamp + node identity signature
- Client verifies attestation freshness before selecting node

### US-14.6: Node Directory
As an app user, I want the app to discover and select reliable nodes.

**Acceptance Criteria:**
- Signed node directory with health metrics
- Geographic metadata (country, jurisdiction info)
- Uptime and latency statistics
- Directory updated periodically (not on every connection)
- Multiple directory sources (prevent single point of failure)

## Technical Requirements
- Language: Rust (tokio async runtime)
- Networking: libp2p for node discovery
- Sphinx packet processing: same library as client
- Memory: mlock, zeroize, no allocation in hot path
- Binary size: <20MB target

## Dependencies
- Epic 03 (Scrambler) — packet format and routing
- Epic 09 (Ghost VPN) — VPN node protocol

## Architecture References
- [zero-log-doctrine.md](../architecture/zero-log-doctrine.md), [scrambler.md](../architecture/scrambler.md), [ghost-vpn.md](../architecture/ghost-vpn.md), [api-reference.md](../architecture/api-reference.md)
