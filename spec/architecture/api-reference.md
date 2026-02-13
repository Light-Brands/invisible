# API Reference — Relay Node Protocol

## Overview
Invisible relay nodes provide stateless message routing. All communication uses encrypted Sphinx packets over the Scrambler network. This documents the relay node API for packet submission, retrieval, and key bundle management.

## Transport
- Default: obfs5 (pluggable transport) over TCP
- Fallback: TLS 1.3 with uTLS fingerprint mimicry
- All payloads are Sphinx packets (exactly 2KB)
- No plaintext metadata in any request

## Endpoints

### POST /submit
Submit an encrypted Sphinx packet for routing through the mixnet.

**Request:** Raw Sphinx packet (2048 bytes)
**Response:** 204 No Content (accepted) or 503 (overloaded)
**Notes:**
- No authentication required (packets are self-authenticating via Sphinx)
- Relay cannot inspect packet content or determine final destination
- Packet enters the mix pool for batching and forwarding

### GET /retrieve/{queue_id}
Retrieve pending messages from a dead drop queue.

**Request:** queue_id is a random identifier known only to sender/recipient
**Response:** Array of encrypted Sphinx packets (or empty array)
**Notes:**
- Queue ID is ephemeral (rotates per session)
- Relay cannot determine who owns the queue
- Messages auto-purge after 72 hours

### POST /keys/publish
Publish a key bundle for async key exchange (X3DH).

**Request Body:**
```json
{
  "identity_key": "<Ed25519 public key>",
  "signed_prekey": "<X25519 public key + signature>",
  "pq_prekey": "<ML-KEM-1024 public key>",
  "one_time_prekeys": ["<X25519 key>", ...]
}
```
**Response:** 201 Created
**Notes:**
- Key bundles are stored ephemerally (RAM-only, TTL 7 days)
- One-time prekeys are consumed on retrieval (single use)
- No identity information attached to key bundles

### GET /keys/{identity_key_hash}
Retrieve a key bundle for initiating a conversation.

**Request:** SHA-256 hash of the target's identity key (shared out-of-band)
**Response:** Key bundle JSON (one-time prekey removed after retrieval)

### GET /health
Health check endpoint for node monitoring.

**Response:** `{"status": "ok", "uptime": 12345, "version": "0.1.0"}`
**Notes:**
- No user-attributable data in health responses
- Used by network for node quality monitoring

### POST /cover
Submit a cover traffic packet (loop traffic).

**Request:** Raw Sphinx packet (2048 bytes, cover flag encrypted inside)
**Response:** 204 No Content
**Notes:**
- Indistinguishable from real packets at the network level
- Cover flag only readable by the loop endpoint (sender)

## Error Handling
- All errors return generic responses (no information leakage)
- 400: Malformed packet
- 503: Node overloaded (client should retry with different node)
- No error messages containing user data or packet details

## Rate Limiting
- Per-IP rate limiting to prevent abuse (configurable per node)
- Rate limits are generous to avoid disrupting cover traffic
- Clients behind VPN share IPs — rate limits account for this

Cross-references: [scrambler.md](scrambler.md), [zero-log-doctrine.md](zero-log-doctrine.md), [cryptography.md](cryptography.md)
