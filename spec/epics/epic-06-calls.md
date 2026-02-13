# Epic 06: Calls — E2EE Voice & Video

## Overview
Implement end-to-end encrypted voice and video calls using WebRTC with custom SRTP key exchange routed through the Scrambler.

## User Stories

### US-06.1: E2EE Voice Calls
As a user, I want to make end-to-end encrypted voice calls.

**Acceptance Criteria:**
- WebRTC with custom SRTP key exchange (not standard SRTP-DTLS)
- Call key derived from existing Double Ratchet session
- Opus codec for voice
- Call metadata (who called whom, duration) stored locally only
- Call signaling routed through Scrambler

### US-06.2: E2EE Video Calls
As a user, I want to make end-to-end encrypted video calls.

**Acceptance Criteria:**
- VP9/AV1 codec for video
- Same key exchange as voice calls
- Camera preview shows locally before call connects
- No recording capability built into the app
- FLAG_SECURE / capture prevention active during calls

### US-06.3: Call Security Verification
As a user, I want to verify my call is not being intercepted.

**Acceptance Criteria:**
- Short Authentication String (SAS) displayed to both parties
- SAS derived from SRTP master key
- Users can verbally compare SAS for verification
- Visual indicator for verified vs unverified calls

### US-06.4: Call Through Scrambler
As a user, I want my call traffic routed through the privacy network.

**Acceptance Criteria:**
- Call signaling (setup/teardown) through full Scrambler
- Media stream through Ghost VPN + reduced mixnet (1-2 hops for latency)
- Configurable: full mixnet (higher latency, max privacy) vs VPN-only (lower latency)
- IP addresses never directly exchanged between peers

### US-06.5: Group Calls
As a user, I want encrypted group voice/video calls.

**Acceptance Criteria:**
- Up to 8 participants (video) / 32 participants (voice)
- SFU (Selective Forwarding Unit) model — SFU sees only encrypted streams
- Per-participant SRTP keys
- SFU runs on community relay nodes

## Technical Requirements
- WebRTC: Flutter WebRTC plugin
- Custom SRTP key exchange (bypass SRTP-DTLS, use Invisible key agreement)
- Codecs: Opus (audio), VP9/AV1 (video)
- Latency target: <300ms for acceptable call quality

## Dependencies
- Epic 00 (Foundation), Epic 01 (Messaging), Epic 03 (Scrambler), Epic 09 (Ghost VPN)

## Architecture References
- [cryptography.md](../architecture/cryptography.md), [scrambler.md](../architecture/scrambler.md), [ghost-vpn.md](../architecture/ghost-vpn.md)
