# The Scrambler -- Deep Network Obfuscation Architecture

> The crown jewel of Invisible. A 7-layer system (plus Ghost VPN as Layer 0) engineered
> to defeat even a global passive adversary. The Scrambler makes it mathematically
> infeasible to determine who is talking to whom, when, or about what.

---

## Table of Contents

1. [Overview](#overview)
2. [Design Principles](#design-principles)
3. [Layer 0: Ghost VPN](#layer-0-ghost-vpn-mandatory-encrypted-tunnel)
4. [Layer 1: Message Fragmentation + Secret Sharing](#layer-1-message-fragmentation--secret-sharing)
5. [Layer 2: Mixnet](#layer-2-mixnet-nym-style)
6. [Layer 3: Cover Traffic](#layer-3-cover-traffic--constant-noise)
7. [Layer 4: Geographic Jurisdiction Scrambling](#layer-4-geographic-jurisdiction-scrambling)
8. [Layer 5: Protocol Camouflage](#layer-5-protocol-camouflage-pluggable-transports)
9. [Layer 6: Dead Drop Architecture](#layer-6-dead-drop-architecture)
10. [Layer 7: Temporal Scrambling](#layer-7-temporal-scrambling)
11. [Complete Flow](#complete-flow-all-layers-combined)
12. [Adversary Analysis](#adversary-analysis)
13. [Configuration Reference](#configuration-reference)
14. [Implementation Notes](#implementation-notes)
15. [Cross-References](#cross-references)

---

## Overview

Every secure messenger encrypts message content. That is table stakes. The hard problem
-- the one almost nobody solves -- is metadata. Who talked to whom. When. How often. For
how long. Metadata is surveillance. Metadata kills.

The Scrambler exists to destroy metadata at every observable layer of the network stack.
It does not rely on any single technique. It composes eight independent obfuscation
layers, each of which independently raises the cost of correlation attacks by orders of
magnitude. Together they produce a system where even an adversary who controls the
entire internet backbone, every ISP, every data center, and every nation-state signals
intelligence agency cannot determine that Alice is communicating with Bob.

This is not theoretical. Each layer is grounded in peer-reviewed cryptographic research.
The composition is what makes it unprecedented.

### Why Not Just Use Tor?

Tor provides onion routing, which hides routing information behind layers of encryption.
But Tor has known weaknesses:

- **Traffic analysis**: Entry and exit nodes can be correlated by timing.
- **No batching**: Packets flow through in real time, preserving timing patterns.
- **No cover traffic**: Silence leaks information -- when you are not sending, an
  observer knows you are not communicating.
- **Volunteer network**: Node operators are not vetted or incentivized.
- **Circuit-based**: A compromised circuit exposes the full path.

The Scrambler replaces onion routing with a mixnet, adds cover traffic, fragments
messages across independent paths, scrambles timing, enforces jurisdictional diversity,
camouflages the wire protocol, and interposes dead drops between sender and recipient.
Tor solves one problem. The Scrambler solves eight.

### Threat Model

The Scrambler is designed against the strongest realistic adversary:

- **Global Passive Adversary (GPA)**: Can observe all network traffic worldwide.
- **Partial Active Adversary**: Can compromise some (but not all) nodes in any layer.
- **Nation-State Level**: Has legal compulsion power within its jurisdiction.
- **ISP-Level DPI**: Can perform deep packet inspection on all traffic.

The Scrambler does NOT defend against:

- Endpoint compromise (malware on the user's device).
- Rubber-hose cryptanalysis (physical coercion of the user).
- Quantum computing attacks on current cryptographic primitives (addressed separately
  in [cryptography.md](cryptography.md) post-quantum migration plan).

---

## Design Principles

1. **Defense in depth.** No single layer is trusted to provide anonymity alone. Every
   layer independently degrades an adversary's ability to correlate traffic.

2. **Information-theoretic security where possible.** Shamir's Secret Sharing and
   cover traffic provide security guarantees that hold regardless of computational
   power.

3. **Composability.** Each layer is an independent module. Layers can be tested,
   audited, upgraded, or replaced without affecting others.

4. **Constant observable behavior.** From any external vantage point, an Invisible
   client looks the same whether idle, sending, or receiving. Traffic rate, packet
   size, and timing patterns are invariant.

5. **Jurisdictional independence.** No single government can compel disclosure of a
   complete message path. The path always spans multiple non-cooperating jurisdictions.

6. **Zero trust in infrastructure.** Every node -- VPN endpoint, mix node, dead drop
   relay -- is assumed to be potentially compromised. The protocol's security holds
   even if a significant minority of nodes are adversarial.

---

## Layer 0: Ghost VPN (Mandatory Encrypted Tunnel)

Covered in full detail in [ghost-vpn.md](ghost-vpn.md). Summary of role within The
Scrambler:

### Purpose

The Ghost VPN is the outermost shield. It ensures that the user's ISP, local network
operator, and any on-path observer between the client and the VPN endpoint sees only
encrypted VPN traffic. They cannot determine that the user is running Invisible, let
alone what they are doing with it.

### Key Properties

- **WireGuard protocol.** Modern, minimal attack surface, formally verified
  cryptographic core (Noise IK pattern).
- **Auto-connect on app launch.** No user action required. The VPN is always on when
  Invisible is running. There is no "unprotected" state.
- **Random global endpoint selection.** 50+ VPN endpoints across 30+ countries. Each
  session connects to a randomly selected endpoint. Never the same endpoint twice in
  a row. Endpoint selection is weighted by current load and latency, with hard
  diversity constraints.
- **No account linkage.** VPN authentication uses ephemeral tokens derived from the
  user's identity key. The VPN infrastructure cannot link sessions to a persistent
  user identity.
- **Key rotation.** WireGuard session keys rotate every 2 minutes. Compromise of a
  session key exposes at most 2 minutes of traffic metadata.

### What the ISP Sees

```
[User Device] ---- WireGuard (UDP, encrypted) ----> [VPN Endpoint in Country X]
```

The ISP sees UDP packets to an IP address. The IP may or may not be known as a VPN
endpoint. The packet contents are indistinguishable from random bytes. The ISP cannot
determine:

- That the user is running Invisible (vs. any other WireGuard VPN).
- What protocol is inside the tunnel.
- Who the user is communicating with.
- Whether the user is actively communicating at all.

---

## Layer 1: Message Fragmentation + Secret Sharing

### Purpose

A single message traveling a single path is a single point of correlation. Layer 1
eliminates this by splitting every message into multiple shares using Shamir's Secret
Sharing scheme, then routing each share through a completely independent network path.
An adversary who intercepts fewer than K shares learns absolutely nothing about the
message content -- not a single bit.

### Mechanism

**Shamir's Secret Sharing (SSS)** is an information-theoretic secret sharing scheme
published by Adi Shamir in 1979. It splits a secret S into N shares such that:

- Any K shares can reconstruct S (threshold reconstruction).
- Any K-1 or fewer shares reveal zero information about S.
- Security is information-theoretic: it holds against adversaries with unlimited
  computational power.

The scheme works over a finite field GF(p) where p is a large prime. A random
polynomial of degree K-1 is generated with the secret as the constant term. Each share
is an evaluation of this polynomial at a distinct point.

### Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| K (threshold) | 3 | Minimum shares needed to reconstruct |
| N (total shares) | 5 | Total shares generated |
| Field size | 256-bit prime | Finite field for SSS computation |
| Packet size | 2048 bytes | Uniform Sphinx packet payload |

### Process

1. **Serialize** the plaintext message (already encrypted end-to-end at a higher layer;
   see [cryptography.md](cryptography.md)).
2. **Pad** the serialized message to a multiple of the share payload size. Padding uses
   ISO/IEC 7816-4 (random bytes with length delimiter) to prevent length leakage.
3. **Split** each padded block using Shamir's Secret Sharing with parameters (K, N).
4. **Package** each share into a Sphinx packet (exactly 2048 bytes, including header,
   routing info, and payload). All packets are cryptographically indistinguishable
   from each other and from cover traffic packets.
5. **Assign** each share an independent path through the mixnet, selected by the
   client's path selection algorithm (see Layer 4).

### Fragmentation Diagram

```
                        ORIGINAL MESSAGE
                    (encrypted, padded to 10KB)
                              |
                    +---------+---------+
                    |  Shamir's Secret  |
                    |  Sharing (3-of-5) |
                    +---------+---------+
                              |
          +-------+-------+-------+-------+-------+
          |       |       |       |       |       |
       Share 1 Share 2 Share 3 Share 4 Share 5
       (2KB)   (2KB)   (2KB)   (2KB)   (2KB)
          |       |       |       |       |
          v       v       v       v       v
       Path A  Path B  Path C  Path D  Path E
          |       |       |       |       |
          v       v       v       v       v
       +-----+ +-----+ +-----+ +-----+ +-----+
       | CH  | | IS  | | RO  | | PA  | | SG  |
       | Mix | | Mix | | Mix | | Mix | | Mix |
       +-----+ +-----+ +-----+ +-----+ +-----+
          |       |       |       |       |
          v       v       v       v       v
       Dead    Dead    Dead    Dead    Dead
       Drop A  Drop B  Drop C  Drop D  Drop E

    CH = Switzerland    IS = Iceland      RO = Romania
    PA = Panama         SG = Singapore

    Intercepting 1 or 2 shares reveals ZERO information.
    Recipient needs only 3 of 5 shares to reconstruct.
```

### Security Properties

- **Information-theoretic confidentiality.** K-1 shares leak zero bits. This is
  provable, not merely computationally hard.
- **Path independence.** Each share traverses a geographically and jurisdictionally
  distinct path. Compromising one path does not help with others.
- **Redundancy.** With 3-of-5, up to 2 shares can be lost (network failure, node
  compromise, censorship) and the message still arrives.
- **No single point of interception.** There is no single network location where the
  complete message (or enough shares to reconstruct it) passes through.

---

## Layer 2: Mixnet (Nym-Style)

### Purpose

The mixnet is the core anonymity engine. Unlike onion routing (Tor), a mixnet does not
simply relay packets in real time. It **collects, delays, reorders, and releases packets
in batches**, destroying the temporal correlation between input and output that makes
traffic analysis possible.

### Why Mixnets, Not Onion Routing

| Property | Onion Routing (Tor) | Mixnet (Scrambler) |
|----------|--------------------|--------------------|
| Packet flow | Real-time relay | Batched, delayed, reordered |
| Timing attacks | Vulnerable | Resistant (random delays) |
| Cover traffic | None | Continuous (Layer 3) |
| Packet size | Variable | Fixed 2KB Sphinx |
| Batch processing | No | Yes (collect-shuffle-release) |
| Global passive adversary | Partially vulnerable | Resistant |

### Academic Foundation

The Scrambler's mixnet is based on the **Loopix** architecture (Piotrowska, Hayes,
Elahi, Meiser, Danezis; ACM CCS 2017, University College London). Loopix provides:

- Provable anonymity guarantees under the Anytrust model.
- Poisson mix strategy (continuous-time mixing with tunable delay).
- Loop cover traffic for sender and receiver unobservability.
- Provider-based architecture for offline message retrieval.

We extend Loopix with Sphinx packet format, Shamir fragmentation integration, and
jurisdictional routing constraints.

### Sphinx Packet Format

All packets in the mixnet use the **Sphinx** packet format (Danezis & Goldberg, IEEE
S&P 2009). Properties:

- **Fixed size**: Every packet is exactly 2048 bytes, regardless of payload.
- **Bitwise indistinguishability**: A Sphinx packet containing a real message share is
  cryptographically indistinguishable from a cover traffic packet, a loop packet, or
  any other Sphinx packet.
- **Single-use header**: Each mix node peels one layer of encryption from the header
  to learn only the next hop. The header is re-randomized at each hop, preventing
  correlation between incoming and outgoing headers.
- **No replay**: Built-in replay detection via tag caching at each node.

### Mixnet Topology

The mixnet uses a **stratified topology** with 5 layers:

```
                    ENTRY GATEWAYS (Layer 1)
                   /    |    |    |    \
                  /     |    |    |     \
            MIX NODES (Layer 2) -- dozens per layer
                  \     |    |    |     /
                   \    |    |    |    /
            MIX NODES (Layer 3) -- dozens per layer
                  /     |    |    |     \
                 /      |    |    |      \
            MIX NODES (Layer 4) -- dozens per layer
                  \     |    |    |     /
                   \    |    |    |    /
                    EXIT GATEWAYS (Layer 5)
```

Each layer contains dozens of geographically distributed nodes. A packet traverses
exactly one node per layer, selected by the sender's path selection algorithm.

### Mix Node Operation

Each mix node performs the following at each processing cycle:

```
    INCOMING PACKETS (from previous layer or clients)
          |
          v
    +---------------------------+
    |    COLLECT INTO BATCH     |
    |  (accumulate for T ms)    |
    +---------------------------+
          |
          v
    +---------------------------+
    |   STRIP ENCRYPTION LAYER  |
    |  (Sphinx header unwrap)   |
    |  Learn ONLY next hop      |
    +---------------------------+
          |
          v
    +---------------------------+
    |     ADD RANDOM DELAY      |
    |  (Poisson, mean = lambda) |
    |  Range: 50ms - 5000ms     |
    +---------------------------+
          |
          v
    +---------------------------+
    |   REORDER RANDOMLY        |
    |  (cryptographic shuffle)  |
    +---------------------------+
          |
          v
    +---------------------------+
    |    PAD TO UNIFORM SIZE    |
    |  (all packets = 2048B)    |
    +---------------------------+
          |
          v
    +---------------------------+
    |  ADD COVER TRAFFIC        |
    |  (inter-node loop pkts)   |
    +---------------------------+
          |
          v
    OUTGOING PACKETS (to next layer nodes)
```

### Mixing Visualization

```
    TIME -->

    INCOMING:     |A|  |B|   |C| |D|    |E|  |F|
                  ================================
                  |        MIX POOL               |
                  |   collect, decrypt headers,    |
                  |   add delays, shuffle,         |
                  |   add cover packets (X,Y)      |
                  ================================
    OUTGOING:        |D|  |X| |B|   |F| |Y| |A|  |E|  |C|

    - Order is randomized.
    - Timing is randomized.
    - Cover packets (X, Y) are injected.
    - An observer cannot correlate input to output.
```

### Node Requirements

- **RAM-only processing.** No packet data ever touches persistent storage. All
  cryptographic operations, batch collection, and shuffling happen in memory.
- **Minimum batch size.** A node will not release packets until the batch contains at
  least M packets (configurable, default 50). This prevents low-traffic correlation.
- **Heartbeat cover traffic.** Even when no real traffic flows, nodes exchange cover
  packets to maintain a constant observable traffic pattern between nodes.

---

## Layer 3: Cover Traffic -- Constant Noise

### Purpose

Silence is information. If an observer can detect when a user is sending or receiving
messages versus when they are idle, they can perform intersection attacks over time. Cover
traffic eliminates this signal by ensuring that every connected client produces a
constant, indistinguishable stream of packets at all times.

### Mechanism

Every connected Invisible client sends Sphinx packets at a constant rate, regardless of
whether the user is actively messaging. These packets are one of three types:

1. **Real message shares**: Actual Shamir shares of a real message, inserted into the
   traffic stream in place of a cover packet.
2. **Loop cover packets**: Packets routed through the full mixnet and back to the
   sender. They look identical to real packets at every hop. They allow the sender to
   detect network health and node compromise.
3. **Drop cover packets**: Packets sent into the mixnet that are silently discarded by
   the exit gateway. They contribute to the overall traffic volume without creating
   detectable dead drops or recipient activity.

### Traffic Pattern Comparison

```
    WITHOUT COVER TRAFFIC (traditional messenger):

    User activity:    idle----SEND---idle--------RECV--idle---SEND-SEND---idle
    Network traffic:  ........||||...............||..........||||||||.........
                      ^                                                    ^
                      Observable pattern reveals communication timing

    =========================================================================

    WITH COVER TRAFFIC (The Scrambler):

    User activity:    idle----SEND---idle--------RECV--idle---SEND-SEND---idle
    Network traffic:  ||||||||||||||||||||||||||||||||||||||||||||||||||||||||
                      ^                                                    ^
                      Constant rate. No observable difference.
                      Real packets replace cover packets seamlessly.
```

### Parameters

| Parameter | Default | Effect |
|-----------|---------|--------|
| Client send rate | 1 packet/second | Higher = better anonymity, more bandwidth |
| Loop fraction | 60% | Percentage of cover that loops back to sender |
| Drop fraction | 40% | Percentage of cover that is silently dropped |
| Rate jitter | +/- 10% | Slight randomization to avoid clock fingerprinting |

### Bandwidth Cost

At 1 packet/second with 2KB packets:

- **Upload**: ~2 KB/s = ~16 kbps (negligible on any modern connection)
- **Monthly**: ~5.2 GB (within typical mobile data plans)
- **Configurable**: Users on metered connections can reduce to 0.5 pkt/s (~2.6 GB/month)

### Security Properties

- **Sender unobservability.** An observer cannot determine when a user transitions from
  idle to actively messaging. The traffic stream looks the same.
- **Receiver unobservability.** Receipt of a message does not cause any observable
  change in the recipient's traffic pattern. The recipient polls dead drops on their
  own schedule (see Layer 6).
- **Statistical unlinkability.** Over time, the constant traffic rate means that an
  adversary performing intersection attacks gains no additional information. The
  anonymity set does not shrink with observation time.

---

## Layer 4: Geographic Jurisdiction Scrambling

### Purpose

Cryptographic and network-level protections can be undermined by legal compulsion. If
every node in a message's path falls within a single government's jurisdiction, that
government can compel all node operators to log traffic and perform correlation attacks.
Layer 4 prevents this by enforcing strict jurisdictional diversity requirements on every
message path.

### Intelligence Alliance Map

The following intelligence-sharing alliances represent cooperative surveillance
capabilities. The Scrambler's routing algorithm treats each alliance as a single
jurisdiction for diversity purposes.

```
    FIVE EYES (signals intelligence sharing):
    +-------+-------+-------+-------+-------+
    |  USA  |  GBR  |  CAN  |  AUS  |  NZL  |
    +-------+-------+-------+-------+-------+

    NINE EYES (Five Eyes + ):
    +-------+-------+-------+-------+
    |  DNK  |  FRA  |  NLD  |  NOR  |
    +-------+-------+-------+-------+

    FOURTEEN EYES (Nine Eyes + ):
    +-------+-------+-------+-------+-------+
    |  DEU  |  BEL  |  ITA  |  SWE  |  ESP  |
    +-------+-------+-------+-------+-------+
```

### Routing Constraints

The path selection algorithm enforces the following hard constraints:

1. **No two consecutive nodes in the same country.** Adjacent hops must cross a
   national border.
2. **No two consecutive nodes in the same intelligence alliance.** A Five Eyes node
   cannot be followed by another Five Eyes node.
3. **Minimum 5 distinct countries per complete path.** Across the full mixnet
   traversal (5 layers), at least 5 different countries must be represented.
4. **Maximum 1 Five Eyes node per path.** At most one hop may traverse a Five Eyes
   country.
5. **Prefer privacy-protective jurisdictions.** Routing weight favors countries with:
   - Constitutional privacy protections (Switzerland, Iceland)
   - No mandatory data retention laws (Romania, Panama)
   - Strong judicial oversight requirements for surveillance (Germany for domestic)
   - No mutual legal assistance treaty (MLAT) with the user's country

### Jurisdiction Metadata

Every node in the Scrambler's directory carries jurisdiction metadata:

```
{
  "node_id": "abc123...",
  "country": "CH",
  "jurisdiction": "switzerland",
  "alliance": "none",
  "data_retention": false,
  "privacy_constitution": true,
  "mlat_partners": ["EU"],
  "operator_jurisdiction": "CH",
  "hosting_jurisdiction": "CH",
  "legal_risk_score": 0.15
}
```

The `legal_risk_score` is a composite metric (0.0 = lowest risk, 1.0 = highest)
incorporating all jurisdictional factors. The routing algorithm uses this score as a
negative weight in path selection.

### Example Path

```
    Sender (USA)
        |
        | [Layer 0: Ghost VPN]
        v
    VPN Endpoint: Zurich, SWITZERLAND (neutral, privacy constitution)
        |
        | [Layer 2: Mixnet Entry Gateway]
        v
    Mix Layer 1: Reykjavik, ICELAND (no data retention, strong privacy laws)
        |
        v
    Mix Layer 2: Bucharest, ROMANIA (no data retention, non-Fourteen Eyes)
        |
        v
    Mix Layer 3: Panama City, PANAMA (no MLAT with most countries, banking secrecy)
        |
        v
    Mix Layer 4: Singapore, SINGAPORE (independent judiciary, non-alliance)
        |
        | [Layer 2: Mixnet Exit Gateway]
        v
    Dead Drop Relay: Sao Paulo, BRAZIL (non-alliance, independent jurisdiction)
        |
        | [Recipient retrieves from dead drop]
        v
    Recipient (Germany)

    Countries traversed: 6 (CH, IS, RO, PA, SG, BR)
    Intelligence alliances crossed: 0 (none of these are Five/Nine/Fourteen Eyes)
    Jurisdictions that would need to cooperate for compulsion: 6
    Probability of simultaneous cooperation: effectively zero
```

### Path Selection Algorithm

Path selection runs entirely on the client. No central authority assigns paths.

1. Fetch current node directory (signed, timestamped, distributed via mixnet).
2. For each Shamir share, independently select a path:
   a. Filter nodes by layer assignment.
   b. Apply hard constraints (no consecutive same-country, no consecutive same-alliance).
   c. Weight remaining candidates by: latency, load, legal_risk_score (inverted),
      uptime history, and random jitter.
   d. Select path via weighted random sampling.
3. Verify path meets minimum jurisdiction diversity (5+ countries).
4. If constraints cannot be satisfied (e.g., insufficient node diversity), the client
   delays transmission and retries with updated directory.

---

## Layer 5: Protocol Camouflage (Pluggable Transports)

### Purpose

Even with a VPN, an adversary performing deep packet inspection (DPI) might identify
the VPN protocol and flag the traffic for enhanced monitoring. Layer 5 makes the wire
protocol itself indistinguishable from common internet traffic. The Scrambler's bytes on
the wire look like normal web browsing, video streaming, or random noise -- depending on
the selected transport.

### Pluggable Transport Framework

The transport layer is modular. The client and its first-hop node negotiate a transport
at connection time. Transports can be upgraded, added, or replaced without changing any
other Scrambler layer.

### Available Transports

#### obfs5 (Default)

- **Appearance to DPI**: Random bytes. No identifiable protocol signature, magic bytes,
  or statistical fingerprint.
- **Mechanism**: Elliptic-curve Diffie-Hellman key exchange produces a shared secret.
  All subsequent bytes are encrypted with a stream cipher. Packet lengths are randomized
  within configured bounds. Timing is jittered.
- **Resistance**: Defeats signature-based DPI, protocol whitelisting (appears as
  "unknown protocol"), and statistical analysis of byte distributions.
- **Limitation**: Some censors block all unidentifiable protocols. In these environments,
  fall back to TLS camouflage.

#### TLS Camouflage via uTLS (Fallback)

- **Appearance to DPI**: Standard HTTPS traffic to a legitimate website. Indistinguishable
  from Chrome or Firefox browsing.
- **Mechanism**: Uses the uTLS library to precisely replicate the TLS Client Hello
  fingerprint of a target browser (Chrome 120, Firefox 121, Safari 17, etc.). The TLS
  connection is established to a cooperating front server or CDN edge node. Application
  data is tunneled inside the TLS connection.
- **SNI**: Server Name Indication field shows a legitimate, popular domain name
  (e.g., `cdn.example.com`). The actual Scrambler endpoint receives the traffic via
  virtual hosting or CDN routing.
- **Resistance**: Defeats DPI that whitelists only known protocols. Passes TLS
  fingerprint checks. Appears as normal HTTPS in traffic logs.

#### Domain Fronting via CDN (Emergency)

- **Appearance to DPI**: HTTPS requests to a major CDN (e.g., a large cloud provider's
  CDN endpoint). Completely indistinguishable from normal CDN-hosted web content access.
- **Mechanism**: The TLS SNI and HTTP Host header point to a legitimate, high-traffic
  domain hosted on the CDN. The encrypted HTTP request inside the TLS tunnel specifies
  the actual Scrambler endpoint as an inner Host header. The CDN routes the request to
  the Scrambler frontend.
- **Resistance**: Blocking this traffic requires blocking the entire CDN, which would
  break access to thousands of legitimate websites. This is the "nuclear option" for
  censorship circumvention.
- **Limitation**: Requires cooperating CDN infrastructure. Available CDN providers may
  change. Maintained as emergency fallback.

### Transport Negotiation

```
    Client                                      First-Hop Node
      |                                              |
      |  1. Probe: attempt obfs5 handshake           |
      |--------------------------------------------->|
      |                                              |
      |  2a. If obfs5 succeeds:                      |
      |<---------------------------------------------|
      |     [obfs5 session established]              |
      |                                              |
      |  2b. If obfs5 blocked/timeout:               |
      |  3. Probe: attempt uTLS (Chrome profile)     |
      |--------------------------------------------->|
      |                                              |
      |  4a. If TLS succeeds:                        |
      |<---------------------------------------------|
      |     [TLS camouflage session established]     |
      |                                              |
      |  4b. If TLS blocked/timeout:                 |
      |  5. Domain fronting via CDN                   |
      |--------------------------------------------->|
      |     [CDN fronted session established]        |
      |                                              |
```

### Packet Shaping

Regardless of transport, the Scrambler shapes packet sizes and timing to match the
profile of the selected camouflage protocol:

- **TLS mode**: Packet sizes match typical HTTPS response distributions. Timing mimics
  web page loading patterns (burst of requests, pause, burst).
- **obfs5 mode**: Packet sizes drawn from uniform random distribution within configured
  bounds.
- **CDN mode**: Mimics chunked transfer encoding patterns typical of CDN content delivery.

---

## Layer 6: Dead Drop Architecture

### Purpose

In a traditional messenger, the sender's network transmits to the recipient's network.
Even with perfect encryption and anonymity, the fact that traffic flows from A's
infrastructure to B's infrastructure is a correlation signal. Layer 6 eliminates this
by ensuring that sender and recipient never connect to the same network node.
Communication is mediated through anonymous dead drops -- ephemeral relay mailboxes that
cannot correlate depositor and retriever.

### Inspiration

The dead drop architecture draws from the **SimpleX Messaging Protocol** (SMP) queue
design, adapted for the mixnet context. SimpleX demonstrated that unidirectional queues
with separate sender and receiver connections provide strong metadata protection without
requiring user identifiers.

### Mechanism

```
    SENDER                    DEAD DROP RELAY                 RECIPIENT
      |                            |                              |
      |  (via mixnet, anonymous)   |                              |
      |  1. Deposit share at       |                              |
      |     dead drop ID: x7k9m   |                              |
      |--------------------------->|                              |
      |                            |  [share stored in RAM]       |
      |                            |  [no sender metadata kept]   |
      |                            |                              |
      |                            |   2. Recipient polls         |
      |                            |      dead drop ID: x7k9m    |
      |                            |<-----------------------------|
      |                            |                              |
      |                            |   3. Deliver share           |
      |                            |------------------------------>|
      |                            |                              |
      |                            |   4. Delete from RAM         |
      |                            |                              |
      |  SENDER NEVER CONNECTS     |    RECIPIENT NEVER CONNECTS  |
      |  TO RECIPIENT'S NODE  <----|-->  TO SENDER'S NODE         |
      |                            |                              |

    The dead drop relay sees:
      - An anonymous deposit from the mixnet (no sender identity)
      - An anonymous retrieval from the mixnet (no recipient identity)
      - It CANNOT correlate them (different connections, different times)
```

### Dead Drop Properties

- **Ephemeral IDs.** Each dead drop ID is a random 256-bit identifier, derived from a
  shared secret between sender and recipient. IDs rotate every session (or every N
  messages, whichever comes first). There are no long-term dead drop addresses that
  could be surveilled.

- **RAM-only storage.** Dead drop relays store deposited shares only in volatile memory.
  If the relay is seized or powered down, all stored data is lost. There is no
  persistent database to subpoena.

- **No correlation capability.** The dead drop relay receives deposits and retrievals as
  anonymous Sphinx packets from the mixnet. It does not know the sender's identity,
  the recipient's identity, or that a particular deposit and retrieval are related.
  Deposits and retrievals happen at different times via different mixnet paths.

- **Multiple dead drops per conversation.** Each Shamir share of a single message is
  deposited at a different dead drop relay. Even compromising one dead drop reveals
  only one share (which, per Layer 1, reveals zero information about the message).

- **Time-to-live (TTL).** Shares stored at a dead drop expire and are deleted after a
  configurable TTL (default: 24 hours). This limits the window for retrieval attacks
  and bounds the relay's memory usage.

- **Redundant dead drops.** For reliability, each share may be deposited at 2 dead
  drops simultaneously. The recipient retrieves from whichever responds first.

### Dead Drop ID Derivation

Dead drop IDs are derived deterministically by both sender and recipient from their
shared conversation secret:

```
dead_drop_id = HKDF(
    ikm = conversation_root_key,
    salt = epoch_counter || share_index,
    info = "invisible-dead-drop-v1",
    length = 32
)
```

Both parties can independently compute the same dead drop ID for each share of each
message without any additional communication. The epoch counter advances with each
message, ensuring dead drop IDs are never reused.

---

## Layer 7: Temporal Scrambling

### Purpose

Even with mixing, cover traffic, and dead drops, timing is the most dangerous metadata
dimension. If a sender types a message at 14:32:07 and the recipient's device shows
activity at 14:32:09, a sufficiently powerful observer might correlate these events.
Layer 7 destroys this temporal correlation by introducing random delays at every stage of
the pipeline.

### Delay Sources

Temporal scrambling is not a single delay. It is the composition of multiple independent
random delays:

1. **Client-side pre-delay.** Before entering the mixnet, each Shamir share is held
   locally for a random duration drawn from a Poisson distribution. This prevents the
   "typing then immediate send" timing correlation.

2. **Mix node delays.** Each of the 5 mix nodes in the path adds an independent random
   delay (Poisson distribution, node-specific lambda parameter). This is integral to
   the mixnet's operation (Layer 2).

3. **Dead drop residence time.** Shares sit at the dead drop until the recipient polls.
   The recipient polls on their own schedule, not triggered by incoming messages. This
   adds an unpredictable delay that the sender cannot control or predict.

4. **Client-side polling jitter.** The recipient's polling interval includes random
   jitter to prevent polling-time fingerprinting.

### Delay Budget

```
    Source                      Distribution       Typical Range
    -----------------------------------------------------------------
    Client pre-delay            Poisson(5s)        0s - 15s
    Mix node 1 delay            Poisson(1s)        50ms - 5s
    Mix node 2 delay            Poisson(1s)        50ms - 5s
    Mix node 3 delay            Poisson(1s)        50ms - 5s
    Mix node 4 delay            Poisson(1s)        50ms - 5s
    Mix node 5 delay            Poisson(1s)        50ms - 5s
    Dead drop residence         Uniform            0s - polling interval
    Recipient poll jitter       Uniform(+/-20%)    variable
    -----------------------------------------------------------------
    TOTAL (typical)                                2s - 45s
    TOTAL (95th percentile)                        < 60s
```

### Urgency Modes

Users can configure the tradeoff between latency and anonymity:

| Mode | Client Pre-Delay | Mix Delays | Polling Interval | Typical E2E Latency |
|------|-----------------|------------|------------------|---------------------|
| Maximum | Poisson(15s) | Poisson(2s) per node | 30s | 30-90s |
| High (default) | Poisson(5s) | Poisson(1s) per node | 10s | 5-45s |
| Standard | Poisson(2s) | Poisson(500ms) per node | 5s | 2-20s |
| Low latency | Poisson(500ms) | Poisson(200ms) per node | 2s | 1-8s |
| Instant* | 0 | Poisson(100ms) per node | 1s | 0.5-3s |

*Instant mode reduces temporal anonymity significantly. A warning is displayed to the
user. Mixnet batching and cover traffic still provide substantial protection, but
temporal correlation becomes feasible for a GPA. Use only when latency is critical.

### Why Poisson?

The Poisson distribution is chosen because it is the maximum-entropy distribution for
inter-event times in a memoryless process. This means:

- An observer gains no information from the elapsed time since the last packet about
  when the next packet will arrive.
- The delay process is statistically indistinguishable from a natural Poisson process
  (such as user-initiated web requests).
- The composition of multiple independent Poisson delays is well-understood and
  analyzable.

---

## Complete Flow (All Layers Combined)

The following documents the complete end-to-end journey of a single message through all
eight layers of The Scrambler.

### Step-by-Step

```
SENDER'S DEVICE
===============

 1. [App Layer]      Sender opens Invisible app.
                     Biometric/passphrase authentication.
                     Ghost VPN auto-connects to random endpoint (Layer 0).

 2. [App Layer]      Sender composes message: "Meeting at 3pm"

 3. [Crypto Layer]   Message encrypted with Double Ratchet + post-quantum KEM.
                     (See cryptography.md for details.)
                     Result: ciphertext blob, ~500 bytes.

 4. [Layer 1]        Ciphertext padded to uniform block size.
                     Shamir's Secret Sharing splits into 5 shares (K=3).
                     Each share: 2KB Sphinx packet payload.

 5. [Layer 7]        Each share assigned independent random pre-delay.
                     Share 1: wait 2.3s
                     Share 2: wait 7.1s
                     Share 3: wait 0.8s
                     Share 4: wait 4.5s
                     Share 5: wait 11.2s

 6. [Layer 3]        Shares queued for insertion into cover traffic stream.
                     At the scheduled moment, a share replaces one cover packet.
                     External observer sees: constant 1 pkt/s stream, no change.

 7. [Layer 5]        Each outgoing packet wrapped in pluggable transport.
                     obfs5: all bytes appear random to DPI.

 8. [Layer 0]        Packet exits device through WireGuard VPN tunnel.
                     ISP sees: UDP packets to VPN endpoint in random country.


NETWORK (per share, 5 parallel paths)
======================================

 9. [VPN Endpoint]   VPN decapsulates. Sees Sphinx packet. No user identity.
                     Forwards to mixnet entry gateway.

10. [Layer 2]        ENTRY GATEWAY (e.g., Switzerland):
                     Receives Sphinx packet.
                     Strips first encryption layer.
                     Learns only: next hop is Mix Layer 1 node in Iceland.
                     Adds random delay (Poisson, ~1s).
                     Releases into outgoing batch.

11. [Layer 2]        MIX NODE 1 (e.g., Iceland):
                     Collects batch of 50+ packets from various sources.
                     Strips encryption layer. Learns only: next hop is Romania.
                     Adds random delay. Shuffles batch. Releases.

12. [Layer 2]        MIX NODE 2 (e.g., Romania):
                     Same process. Next hop: Panama.

13. [Layer 2]        MIX NODE 3 (e.g., Panama):
                     Same process. Next hop: Exit Gateway in Singapore.

14. [Layer 2]        EXIT GATEWAY (e.g., Singapore):
                     Strips final encryption layer.
                     Sees: Sphinx payload destined for dead drop ID x7k9m.
                     Forwards to dead drop relay.

15. [Layer 4]        JURISDICTION CHECK (enforced at step 4, path selection):
                     Path verified: CH -> IS -> RO -> PA -> SG
                     5 countries, 0 Five Eyes, 0 consecutive same-alliance.
                     Constraint satisfied.

16. [Layer 6]        DEAD DROP RELAY (e.g., Brazil):
                     Receives anonymous Sphinx payload.
                     Stores in RAM at dead drop ID x7k9m.
                     Does not know sender. Does not know recipient.
                     TTL: 24 hours.


RECIPIENT'S DEVICE
==================

17. [Layer 6]        Recipient's app polls known dead drop IDs on schedule.
                     Poll interval: 10s +/- 2s jitter.
                     Poll goes through recipient's own VPN + mixnet path.
                     Dead drop relay returns stored share. Deletes from RAM.

18. [Layer 1]        After collecting 3 of 5 shares (from 3 different dead drops,
                     via 3 different mixnet paths):
                     Shamir reconstruction recovers the ciphertext.

19. [Crypto Layer]   Double Ratchet + post-quantum KEM decryption.
                     Plaintext recovered: "Meeting at 3pm"

20. [App Layer]      Message displayed to recipient.
                     No notification was triggered by message arrival --
                     the app discovered the message during routine polling.
```

### End-to-End Timing

```
    Sender types     Share 1     Share 3     Share 2       Recipient
    message          enters      enters      enters        retrieves
    |                mixnet      mixnet      mixnet        3 shares
    |                |           |           |             |
    v                v           v           v             v
    T+0s             T+0.8s      T+2.3s      T+7.1s        T+12s - T+45s
                                                           (depends on
                                                            polling schedule
                                                            and mix delays)
```

Total end-to-end latency: typically 5-45 seconds in "High" urgency mode. This is the
price of metadata annihilation. The Scrambler trades latency for anonymity, and it is a
trade worth making.

---

## Adversary Analysis

### What Each Adversary Sees

| Adversary Position | Observable Information | Cannot Determine |
|---|---|---|
| **User's ISP** | WireGuard UDP packets to a VPN IP in a random country | That the user runs Invisible; who they talk to; when they talk; message content |
| **VPN endpoint operator** | Encrypted Sphinx packets from a VPN session; no user identity | Who the user is; who they talk to; message content; message destination |
| **Mixnet entry gateway** | Sphinx packet from VPN IP; next hop only | Sender identity; final destination; message content; relationship to other packets |
| **Interior mix node** | Sea of identical 2KB Sphinx packets; previous hop and next hop only | Original sender; final recipient; which packets are real vs cover; message content |
| **Mixnet exit gateway** | Sphinx payload destined for a dead drop ID | Who deposited the share; who will retrieve it; message content (encrypted); which message this share belongs to |
| **Dead drop relay** | Anonymous deposit and anonymous retrieval of a 2KB blob | Sender identity; recipient identity; that deposit and retrieval are related; message content |
| **Recipient's ISP** | WireGuard UDP packets to a VPN IP | That the recipient uses Invisible; that a message was received; sender identity |

### Global Passive Adversary (GPA)

A GPA can observe all network traffic worldwide simultaneously. Against The Scrambler, a
GPA would need to:

1. **Identify Invisible traffic** despite pluggable transport camouflage (Layer 5).
   With obfs5 or TLS camouflage, this is not reliably possible.

2. **Correlate VPN sessions to users** despite ephemeral VPN authentication and random
   endpoint selection (Layer 0). Requires compromising the VPN infrastructure.

3. **Trace packets through the 5-layer mixnet** despite batching, shuffling, random
   delays, and cover traffic (Layers 2, 3, 7). With N packets per batch, K mix nodes,
   and cover traffic, the number of possible packet orderings at each node is N!.
   Across 5 nodes: (N!)^5. With N=50, this is approximately 10^320. Intractable.

4. **Correlate across Shamir shares** routed through 5 independent paths (Layer 1).
   Each share takes a different path through different nodes in different countries.
   The GPA must independently trace all 5 paths AND determine they belong to the same
   message. With no shared metadata between shares, this requires solving 5
   independent instances of the mixnet tracing problem simultaneously.

5. **Link dead drop deposits to retrievals** (Layer 6). Deposits and retrievals happen
   at different times via different mixnet paths. The dead drop relay reveals nothing.
   The GPA must correlate timing, but Layer 7 temporal scrambling makes timing
   unreliable.

6. **Overcome jurisdictional diversity** (Layer 4). Even if the GPA is also a legal
   adversary, compelling cooperation across 5+ non-allied jurisdictions simultaneously
   is operationally impractical.

**Conclusion**: Each layer independently raises the cost of correlation by orders of
magnitude. Their composition produces a system where correlation is not merely
computationally expensive -- it is mathematically infeasible under standard cryptographic
and information-theoretic assumptions.

### Formal Security Properties

Under the Anytrust assumption (at least one honest node per mixnet layer) and with
sufficient cover traffic, The Scrambler provides:

- **Sender anonymity**: An adversary cannot determine which user sent a given message.
- **Recipient anonymity**: An adversary cannot determine which user received a given
  message.
- **Sender-recipient unlinkability**: An adversary cannot determine that two users are
  communicating, even given arbitrary observation time.
- **Unobservability**: An adversary cannot determine whether a user is communicating or
  idle.

These properties hold against a global passive adversary and a partial active adversary
(controlling up to a minority of nodes in each mixnet layer).

---

## Configuration Reference

### Client-Side Parameters

| Parameter | Default | Range | Effect |
|-----------|---------|-------|--------|
| `shamir_k` | 3 | 2-5 | Threshold for share reconstruction |
| `shamir_n` | 5 | 3-8 | Total shares generated per message |
| `mixnet_layers` | 5 | 3-7 | Number of mix node layers traversed |
| `cover_traffic_rate` | 1.0 pkt/s | 0.5-5.0 pkt/s | Cover traffic send rate |
| `cover_loop_fraction` | 0.6 | 0.0-1.0 | Fraction of cover as loop traffic |
| `temporal_delay_mean` | 5.0s | 0.0-30.0s | Mean client-side pre-delay |
| `min_jurisdictions` | 5 | 3-8 | Minimum countries per path |
| `max_five_eyes_hops` | 1 | 0-2 | Maximum Five Eyes nodes per path |
| `dead_drop_ttl` | 86400s | 3600-604800s | Dead drop share expiration |
| `polling_interval` | 10s | 1-60s | Dead drop polling frequency |
| `polling_jitter` | 0.2 | 0.0-0.5 | Polling interval randomization |
| `urgency_mode` | "high" | instant/low/standard/high/maximum | Latency-anonymity tradeoff |
| `transport` | "auto" | auto/obfs5/tls/cdn | Pluggable transport selection |

### Mix Node Parameters

| Parameter | Default | Range | Effect |
|-----------|---------|-------|--------|
| `batch_min_size` | 50 | 20-200 | Minimum packets before release |
| `mix_delay_lambda` | 1.0s | 0.1-5.0s | Poisson delay parameter |
| `inter_node_cover_rate` | 5.0 pkt/s | 1.0-20.0 pkt/s | Cover traffic between nodes |
| `max_memory_mb` | 512 | 128-2048 | RAM limit for packet storage |
| `replay_cache_ttl` | 3600s | 600-86400s | Sphinx tag cache duration |

### Recommended Profiles

| Profile | Use Case | Latency | Bandwidth | Anonymity |
|---------|----------|---------|-----------|-----------|
| **Paranoid** | Whistleblowers, journalists in hostile states | 30-90s | ~10 KB/s | Maximum |
| **Private** (default) | Standard private messaging | 5-45s | ~2 KB/s | Very high |
| **Balanced** | Users who need faster responses | 2-20s | ~2 KB/s | High |
| **Responsive** | Time-sensitive communication | 1-8s | ~2 KB/s | Moderate-high |

---

## Implementation Notes

### Core Libraries

| Component | Language | Crate/Library | Notes |
|-----------|----------|--------------|-------|
| Sphinx packets | Rust | `sphinx-packet` | Custom implementation, formally verified core |
| Shamir's Secret Sharing | Rust | `sss` (sharks) | GF(256) arithmetic, constant-time |
| Mix node | Rust | `tokio` runtime | Async I/O, RAM-only, zero-copy where possible |
| Path selection | Rust | Custom | Client-side, jurisdiction-aware weighted sampling |
| Pluggable transports | Rust | `obfs4-rs`, `utls-rs` | Modular transport trait |
| WireGuard VPN | Rust | `boringtun` | Userspace WireGuard implementation |
| Dead drop relay | Rust | `tokio` + custom | RAM-only, anonymous mailbox with TTL eviction |
| Cover traffic | Rust | Custom | Poisson-distributed packet scheduler |
| Client core | Rust | Combined | Cross-compiled for iOS (via Swift FFI) and Android (via JNI) |

### Testability

Each layer is independently testable:

- **Layer 0**: VPN connectivity, endpoint rotation, key rotation timing.
- **Layer 1**: Share generation, reconstruction with K shares, failure with K-1.
- **Layer 2**: Batch collection, shuffle verification, delay distribution validation.
- **Layer 3**: Traffic rate constancy measurement, cover/real indistinguishability.
- **Layer 4**: Path constraint verification, jurisdiction diversity validation.
- **Layer 5**: DPI evasion testing against commercial DPI appliances.
- **Layer 6**: Dead drop deposit/retrieval isolation, TTL enforcement, RAM-only verification.
- **Layer 7**: Delay distribution validation, end-to-end timing analysis.

### Integration Testing

Full-stack integration tests deploy a local mixnet (3 layers, 3 nodes per layer) with
simulated jurisdiction metadata and verify:

- Messages reconstruct correctly with K of N shares.
- Messages fail to reconstruct with K-1 shares.
- Cover traffic is indistinguishable from real traffic in packet captures.
- Path constraints are enforced in all generated paths.
- End-to-end timing falls within configured bounds.
- Dead drop isolation holds under concurrent load.

### Performance Targets

| Metric | Target | Notes |
|--------|--------|-------|
| Message delivery (P50) | < 15s | "Private" profile |
| Message delivery (P99) | < 60s | "Private" profile |
| Client battery impact | < 3% per hour | Cover traffic + polling |
| Client bandwidth | < 2 KB/s sustained | At default cover rate |
| Mix node throughput | > 10,000 pkt/s | Per node, commodity hardware |
| Dead drop capacity | > 1M shares in RAM | Per relay, 2GB RAM allocation |

---

## Cross-References

- [ghost-vpn.md](ghost-vpn.md) -- Layer 0 in detail: VPN architecture, endpoint network,
  key management.
- [cryptography.md](cryptography.md) -- End-to-end encryption (Double Ratchet,
  post-quantum KEM), key exchange, identity keys.
- [zero-log-doctrine.md](zero-log-doctrine.md) -- The legal and technical guarantee that
  no component in the system retains logs, metadata, or user-identifying information.
- [api-reference.md](api-reference.md) -- Client API for interacting with The Scrambler
  programmatically.

---

*The Scrambler is not a feature. It is the reason Invisible exists. Every design decision
in this system answers a single question: can an adversary with unlimited resources
determine who is talking to whom? The answer, across every layer, is no.*
