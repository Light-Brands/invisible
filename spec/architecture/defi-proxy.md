# DeFi Proxy — Anonymous DeFi Access

## Overview

Invisible provides anonymous access to decentralized finance (DeFi) protocols. All
blockchain interactions pass through the full 7-layer Scrambler stack — the same
architecture that protects messaging. "Proxied through the Scrambler" does not mean a
simple RPC relay. It means every DeFi operation — every RPC call, every transaction
broadcast, every balance query — receives the complete treatment: fragmentation, mixnet
routing, cover traffic, jurisdiction scrambling, protocol camouflage, dead drop relay, and
temporal scrambling. No exceptions. No shortcuts. No reduced layer count because "it's
just a balance check."

The threat model is identical to messaging: a global passive adversary who controls the
internet backbone, every ISP, every data center, and every blockchain node provider
cannot determine that a user is interacting with any DeFi protocol, which protocols they
use, what positions they hold, or when they trade.

## Architecture

### RPC Proxy Layer

- All Ethereum/Solana/etc. RPC calls routed through the full Scrambler stack (all 7 layers)
- Your IP never reaches Infura, Alchemy, or any blockchain node provider
- RPC requests are encrypted Sphinx packets — identical to messaging traffic at every hop
- Response caching for frequently-queried data (block height, gas prices) to reduce
  unnecessary network round-trips without leaking query patterns

### How It Works

1. User connects external wallet via WalletConnect v2
2. dApp sends transaction request
3. Transaction signing happens locally on device (private keys never leave)
4. Signed transaction enters the DeFi Privacy Pipeline (see below) — full 7-layer treatment
5. Transaction broadcast to multiple blockchain nodes via separate Scrambler exit paths
6. Blockchain node sees the transaction but not the sender's IP, location, timing, or
   any network-level metadata
7. dApp frontend never sees real IP (all traffic proxied through Scrambler)

---

## DeFi Privacy Pipeline

Every DeFi operation — RPC query, transaction submission, position monitoring, price
feed, ENS lookup — passes through all 7 Scrambler layers. This section maps each layer
to its DeFi-specific role. For full technical detail on each layer's cryptographic
foundations, see [scrambler.md](scrambler.md).

### Layer 1: Fragmentation (Shamir's Secret Sharing)

Transaction submissions are split into Shamir shares and broadcast to multiple
blockchain nodes via separate Scrambler exit paths. No single exit node sees the complete
signed transaction. Balance queries can be split across multiple RPC endpoints for
cross-verification without revealing the full query pattern to any single endpoint.

A swap on Uniswap becomes 5 Shamir shares, each taking an independent path through the
mixnet, each exiting through a different jurisdiction, each arriving at a different
blockchain node. No single point in the network sees the complete transaction.

### Layer 2: Mixnet (5-Layer Sphinx Routing)

All RPC traffic passes through the 5-layer mixnet. RPC requests and responses are Sphinx
packets — fixed 2048-byte, cryptographically indistinguishable from messaging traffic.
A mix node processing a batch of 50+ packets cannot determine which are chat messages,
which are Ethereum RPC calls, and which are cover traffic. They are all identical 2KB
Sphinx packets.

This is critical: DeFi traffic and messaging traffic are not separate streams. They are
the same stream. An observer monitoring the mixnet sees one undifferentiated flow of
Sphinx packets. There is no "DeFi mode" that activates a distinct traffic pattern.

### Layer 3: Cover Traffic (Constant-Rate Dummy RPC Queries)

The Invisible client sends a constant stream of dummy RPC queries at the same rate as
messaging cover traffic. Dummy queries include:

- Balance checks for random addresses (not the user's wallets)
- Gas price queries across multiple chains
- Block height requests
- Token price feed queries
- Transaction receipt lookups for random transaction hashes

Real DeFi queries replace dummy queries in the traffic stream, exactly as real messages
replace cover packets in messaging. The observable traffic rate does not change.

An observer cannot distinguish "user is actively trading on Uniswap" from "app is
generating cover traffic." This is the critical property. Without cover traffic, RPC
query volume is a binary indicator of DeFi activity — a spike in RPC calls reveals that
the user opened their wallet or submitted a trade. With cover traffic, the signal is
constant. There is no spike. There is no silence. There is only noise.

Cover traffic starts when the app launches (via Ghost VPN), not when the user opens the
wallet UI. Opening the wallet does not create any observable traffic change. Closing the
wallet does not reduce traffic. The traffic stream is invariant to user behavior.

### Layer 4: Jurisdiction Routing

RPC requests to blockchain nodes cross multi-jurisdiction Scrambler paths, identical to
messaging. The same hard constraints apply:

- No two consecutive hops in the same country
- No two consecutive hops in the same intelligence alliance
- Minimum 5 distinct countries per path
- Maximum 1 Five Eyes node per path
- Preference for privacy-protective jurisdictions (Switzerland, Iceland, Romania, Panama)

An Ethereum RPC call from a user in the US might traverse: VPN endpoint in Switzerland,
mix nodes in Iceland, Romania, Panama, and Singapore, exiting to an Alchemy endpoint
from a Brazilian exit gateway. No single jurisdiction can compel the complete path.

### Layer 5: Protocol Camouflage (Pluggable Transports)

RPC over JSON-RPC/WebSocket is wrapped in pluggable transports before leaving the
client. Deep packet inspection cannot distinguish "Ethereum RPC call" from "normal web
browsing" or "video streaming."

- **obfs5** (default): RPC traffic appears as random bytes. No JSON-RPC signatures, no
  WebSocket upgrade headers, no Ethereum-specific byte patterns visible on the wire.
- **TLS camouflage** (fallback): RPC traffic appears as standard HTTPS to a legitimate
  domain. TLS fingerprint matches Chrome/Firefox.
- **Domain fronting** (emergency): RPC traffic appears as CDN content requests.

The camouflage applies to the entire Scrambler tunnel, not just the RPC payload. An ISP
performing DPI sees the same traffic whether the user is sending a chat message, checking
an ETH balance, or doing nothing at all.

### Layer 6: Dead Drops (Asynchronous DeFi Operations)

For asynchronous DeFi operations — waiting for transaction confirmation, monitoring
open positions, tracking liquidation thresholds — polling happens through the dead
drop relay architecture. The pattern mirrors messaging:

1. The user's signed transaction (or RPC query) is deposited at a dead drop by the
   Scrambler exit node after reaching the blockchain node and obtaining a response.
2. The user's client polls dead drops on a schedule with jitter, retrieving RPC
   responses the same way it retrieves message shares.
3. The polling node does not know which user is checking which transaction.
4. The dead drop relay cannot correlate the RPC request deposit with the response
   retrieval — they arrive at different times via different mixnet paths.

This is particularly important for position monitoring. A user with an open Aave borrow
position needs periodic health factor checks. These checks go through dead drops. The
relay handling the health factor response does not know which user's position is being
monitored, which protocol is being queried, or whether the query is real or cover
traffic.

### Layer 7: Temporal Scrambling (Mandatory Random Delays)

Mandatory random delay between "user approves transaction in UI" and "signed transaction
broadcast to blockchain." This prevents timing correlation between UI interaction and
on-chain activity. The delay follows the same Poisson distribution used for messaging:

| Urgency Mode | Transaction Broadcast Delay | Typical Range |
|---|---|---|
| Maximum | Poisson(15s) | 5-45s |
| High (default) | Poisson(5s) | 1-15s |
| Standard | Poisson(2s) | 0.5-8s |
| Low latency | Poisson(500ms) | 0.1-3s |

An adversary who observes a user's device activity (screen on, biometric unlock) and
correlates it with an on-chain transaction timestamp gains nothing, because the delay
between intent and broadcast is random and unpredictable.

DeFi position monitoring queries are sent on a randomized schedule, not in response to
price movements. If ETH drops 10% and the user's Aave position approaches liquidation,
the health factor check does not fire immediately — it fires on the next scheduled poll
cycle, with jitter. This prevents an adversary from correlating "ETH price crash at
14:32:07" with "RPC query spike from this IP at 14:32:08" and inferring that the user
holds a leveraged ETH position. The query would have fired at the same time regardless
of price action, because it is on a fixed schedule with random jitter.

---

## RPC Cover Traffic

### Constant-Rate Dummy RPC Requests

The Invisible client maintains a constant stream of dummy RPC requests to multiple
chains simultaneously, independent of user activity. This is not optional. It is the
default behavior whenever the app is running.

### Dummy Query Types

| Query Type | Example | Purpose |
|---|---|---|
| Balance checks | `eth_getBalance` for random addresses | Mask real balance queries |
| Gas price | `eth_gasPrice` on ETH, Polygon, Arbitrum | Mask pre-transaction gas estimation |
| Block height | `eth_blockNumber` across chains | Mask chain activity monitoring |
| Token prices | DEX price queries for random token pairs | Mask trading intent |
| Transaction receipts | `eth_getTransactionReceipt` for random tx hashes | Mask confirmation monitoring |
| Contract state | `eth_call` to random verified contracts | Mask smart contract interactions |

### Indistinguishability Properties

- Real queries use the same Sphinx packet format, size (2048 bytes), and timing as
  dummy queries
- Real queries replace dummy queries in the traffic stream — the rate does not change
- Query targets (RPC endpoints) are randomized for both real and dummy queries
- Response sizes are padded to uniform Sphinx packet size — a balance response and a
  swap simulation response look identical on the wire

### Lifecycle

- Cover traffic starts on app launch (Ghost VPN connect), before the user opens any
  wallet UI
- Cover traffic rate is configurable, defaulting to the same rate as messaging cover
  traffic (1 packet/second)
- Opening the wallet UI: no observable traffic change
- Submitting a trade: no observable traffic change
- Closing the wallet UI: no observable traffic change
- The traffic stream is invariant to user behavior at every stage

---

## Multi-Node Transaction Broadcast

### Architecture

Signed DeFi transactions (swaps, liquidity provision, borrows, repayments, approvals)
are broadcast to N blockchain nodes via N separate Scrambler exit paths. This is a
direct application of Layer 1 (fragmentation) to transaction submission.

### Purpose

Prevents first-seen geolocation attacks. When a transaction appears on-chain, blockchain
analytics firms examine which node first propagated it. If a single node is the
exclusive source, the node operator (or anyone monitoring that node's peer connections)
can narrow down the submitter's network location. Broadcasting through multiple
Scrambler exits from multiple jurisdictions makes first-seen analysis meaningless.

### Mechanism

```
    SIGNED TRANSACTION
           |
    +------+------+
    |  Scrambler   |
    |  Layer 1:    |
    |  Fragment    |
    +------+------+
           |
    +------+------+------+------+
    |      |      |      |      |
  Path A Path B Path C Path D Path E
    |      |      |      |      |
    v      v      v      v      v
  Exit:  Exit:  Exit:  Exit:  Exit:
  CH     IS     RO     PA     SG
    |      |      |      |      |
    v      v      v      v      v
  Node1  Node2  Node3  Node4  Node5
  (Infura)(Alchemy)(Ankr)(Custom)(Custom)

  Each path traverses a different jurisdiction.
  Each exit node submits to a different blockchain node.
  First-seen analysis sees 5 independent origins across 5 countries.
```

### Query Isolation

Different RPC endpoints are used for different query types to prevent a single endpoint
from building a complete picture of user activity:

- **Balance queries**: Endpoint set A (rotated per session)
- **Transaction submission**: Endpoint set B (rotated per transaction)
- **Transaction confirmation**: Endpoint set C (rotated per query)
- **Contract state reads**: Endpoint set D (rotated per session)

No single RPC provider sees balance checks AND transaction submissions AND confirmation
polling for the same wallet. The complete picture exists only on the user's device.

---

## WalletConnect v2 Integration

- Standard WalletConnect v2 protocol
- Connect any compatible wallet: MetaMask, Trust Wallet, Phantom, Rabby, etc.
- WalletConnect relay traffic itself goes through the full Scrambler stack — the relay
  server does not see the user's IP, location, or timing patterns
- Session management: connections auto-expire on app lock
- No persistent wallet connection state on any server

---

## Supported Protocols

- Ethereum mainnet + L2s (Arbitrum, Optimism, Base, zkSync)
- Solana
- Polygon
- Any EVM-compatible chain
- Custom RPC endpoints supported (routed through Scrambler like all others)

---

## Privacy Enhancements

- **ENS/naming lookups** go through Scrambler (no DNS leak)
- **Token balance queries** proxied through full 7-layer stack (no one sees which wallets
  you are checking)
- **Transaction simulation** proxied (no one sees what you are about to do)
- **dApp metadata** stripped from requests
- **No referrer headers, no cookies, no fingerprinting data** sent to dApps
- **WalletConnect relay traffic** goes through Scrambler — the relay server sees only
  anonymous Sphinx packets, not the user's IP or connection metadata
- **dApp WebSocket connections** are multiplexed through Scrambler to prevent connection
  fingerprinting — multiple dApp connections appear as a single undifferentiated Sphinx
  packet stream
- **Smart contract interaction patterns** are obscured by batching multiple operations
  with random timing — an observer cannot determine the sequence or cadence of contract
  calls
- **No direct connection to any blockchain node, ever** — all traffic goes through
  Ghost VPN + full Scrambler stack. The client never opens a TCP connection to an RPC
  endpoint. Every byte between the client and any blockchain infrastructure passes
  through all 7 layers plus Layer 0

---

## DeFi Operations

- Swap on DEXs (Uniswap, Curve, etc.) anonymously — full 7-layer treatment
- Provide liquidity without IP exposure — transaction broadcast via multi-node exit
- Borrow/lend on Aave, Compound with privacy — position monitoring through dead drops
- NFT interactions with hidden identity — metadata queries through cover traffic stream
- All operations go through Ghost VPN + full Scrambler stack — no exceptions

---

## Security

- Transaction signing is always local — never on a remote server
- WalletConnect sessions are encrypted end-to-end
- No transaction history stored on any server
- Local transaction log follows same auto-purge policy as messages
- RPC responses are verified against multiple endpoints (via fragmented queries) to
  detect compromised or malicious RPC nodes

---

## Limitations

On-chain transactions are still visible on the blockchain. Addresses are pseudonymous.
Invisible does not make on-chain activity disappear — it makes the connection between a
human being and that on-chain activity undetectable at the network level.

What Invisible eliminates for DeFi:

- **IP address correlation**: No blockchain node, RPC provider, or dApp frontend ever
  sees the user's real IP. Chain analysis firms that cross-reference on-chain addresses
  with IP logs from node operators find nothing.
- **Timing correlation**: The random delay between UI interaction and on-chain broadcast
  (Layer 7) prevents correlating "user touched phone at 14:32" with "transaction
  appeared on-chain at 14:32."
- **Geographic inference**: Multi-jurisdiction Scrambler paths (Layer 4) and multi-node
  broadcast prevent first-seen geolocation.
- **Traffic analysis**: Cover traffic (Layer 3) prevents inferring DeFi activity from
  network traffic patterns. An observer cannot determine whether the user is actively
  trading, passively holding, or not using DeFi at all.
- **Connection fingerprinting**: Protocol camouflage (Layer 5) prevents identifying
  DeFi-specific traffic patterns via DPI.

What Invisible does not eliminate:

- **On-chain heuristics**: Clustering analysis, common-input-ownership assumptions, and
  change address detection operate on blockchain data, not network metadata. These are
  outside the Scrambler's scope.
- **Smart contract interaction patterns**: Once a transaction is on-chain, the contract
  calls and token movements are public. The Scrambler hides who submitted the
  transaction, not what the transaction does.

For true on-chain privacy — where the transactions themselves are opaque — use Monero
(XMR) or Zcash (ZEC), where amounts, senders, and recipients are cryptographically
hidden on-chain. See [shadow-wallet.md](shadow-wallet.md). For bridging between
transparent and private chains without leaving a trail, use XMR Hop via Phantom Swap.
See [phantom-swap.md](phantom-swap.md).

---

Cross-references: [shadow-wallet.md](shadow-wallet.md), [phantom-swap.md](phantom-swap.md), [scrambler.md](scrambler.md), [ghost-vpn.md](ghost-vpn.md)
