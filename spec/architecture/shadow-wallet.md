# Shadow Wallet -- Private Crypto Built Into The Messenger

## Overview

Every Invisible user gets a non-custodial, privacy-first crypto wallet built directly
into the app. Send money as anonymously as you send messages. No KYC, no account, no
history server.

Financial transactions are the highest-value metadata an adversary can target. A leaked
payment reveals not just who communicated, but the economic relationship between parties,
transaction amounts, and timing of real-world activity. The Shadow Wallet treats every
financial operation -- transaction broadcasts, balance queries, fee estimation, RPC calls,
swap negotiations -- as traffic that demands the same 7-layer Scrambler protection that
messages receive. There are no shortcuts, no direct connections, no "good enough"
approximations.

## Core Principles

- Non-custodial: user holds the keys, Invisible never touches them
- No KYC: no name, no email, no phone, no ID, ever
- No account: wallet generated locally from device seed
- No history server: transaction history is local-only
- All financial operations route through the full 7-layer Scrambler pipeline
- No direct connection to any blockchain node, ever

## Multi-Chain Support

- Monero (XMR) -- PRIMARY (privacy native, ring signatures, stealth addresses, RingCT)
- Zcash shielded (ZEC) -- zk-SNARKs, only z-addresses supported
- Bitcoin (BTC) -- via CoinJoin/PayJoin + Silent Payments (BIP-352)
- Ethereum (ETH) -- via privacy layers / ZK rollups
- Stablecoins (USDC/USDT) -- via ZK rollup privacy layers

## Wallet Generation

- BIP-39 mnemonic seed generated locally with CSPRNG
- Seed encrypted at rest with same 2FA-derived key as message DB
- Private keys derived per-chain from master seed
- Keys never leave device, never backed up to cloud, never transmitted
- Seed displayed once during setup for physical backup (paper/metal)

---

## Financial Privacy Pipeline

Every financial operation passes through the same 7-layer Scrambler architecture that
protects messages. This is not a simplified version or a "lightweight" mode. It is the
same pipeline, applied to a different payload type. The Scrambler does not distinguish
between a chat message and a signed Ethereum transaction -- both are Sphinx packets
traveling through the same mixnet, subject to the same delays, the same cover traffic,
the same jurisdictional constraints.

### Layer 1: Transaction Fragmentation

Transaction broadcasts are fragmented using Shamir's Secret Sharing, identical to message
fragmentation. The signed transaction is split into N shares (default 5, threshold K=3).
Each share is packaged into a standard 2048-byte Sphinx packet -- cryptographically
indistinguishable from a message share or a cover traffic packet.

Each share is broadcast to a different blockchain node via a separate, independent
Scrambler path. No single node is the first to see the complete transaction. This defeats
first-seen heuristics -- the technique where an adversary monitors many blockchain nodes
and geolocates the sender by identifying which node received the transaction first. When
5 different nodes in 5 different countries each receive the transaction simultaneously via
5 different Scrambler exit paths, first-seen analysis produces noise, not signal.

For balance queries and other read operations, the query is sent through a single
Scrambler path (fragmentation is optional for non-sensitive reads), but the full mixnet
and cover traffic protections still apply.

### Layer 2: Mixnet Routing

All financial traffic passes through the 5-layer mixnet exactly as message traffic does.
This includes:

- **Transaction broadcasts** (signed transaction payloads)
- **Balance queries** (RPC calls to check account state)
- **Fee estimation requests** (gas price queries, fee rate lookups)
- **Block height checks** (chain sync operations)
- **Swap negotiations** (atomic swap protocol messages)
- **Smart contract interactions** (DeFi transactions, token approvals)

Each packet is collected, delayed, reordered, and released in batches at every mix node.
An observer watching the mixnet sees a sea of identical 2KB Sphinx packets. They cannot
determine which packets are chat messages, which are transaction broadcasts, which are
balance queries, and which are cover traffic. Financial traffic is structurally invisible
within the mixnet.

### Layer 3: Financial Cover Traffic

The Scrambler's constant-rate cover traffic stream includes **financial cover traffic** --
dummy RPC queries and dummy transaction-shaped packets that are indistinguishable from
real financial operations. This is critical because financial operations have distinct
traffic patterns (a balance check is a request-response pair; a transaction broadcast is
a fire-and-forget payload). Without financial cover traffic, an observer could distinguish
"user is checking their balance" from "user is sending a message" by packet timing
patterns.

Financial cover traffic operates as follows:

- **Dummy RPC queries**: The client periodically generates fake balance queries, fee
  estimation requests, and block height checks. These are Sphinx packets routed through
  the mixnet to Scrambler exit nodes, which silently discard them. They are
  indistinguishable from real RPC queries at every hop.
- **Dummy transaction-shaped packets**: Packets sized and structured identically to
  transaction broadcast packets, routed through the full pipeline and discarded at exit.
- **Replacement model**: When the user initiates a real financial operation (balance
  check, transaction broadcast), the real packet replaces a cover packet in the outgoing
  stream. The observable traffic rate does not change. An adversary cannot distinguish
  "user just sent a payment" from "another cover packet in the constant stream."

### Layer 4: Jurisdiction Routing

Transaction broadcasts and all blockchain interactions are subject to the same
jurisdictional diversity constraints as messages:

- No two consecutive nodes in the same country.
- No two consecutive nodes in the same intelligence alliance.
- Minimum 5 distinct countries per complete path.
- Maximum 1 Five Eyes node per path.
- Preference for privacy-protective jurisdictions (Switzerland, Iceland, Romania, Panama,
  Singapore).

Financial traffic is not exempt from these constraints. A transaction broadcast that
passes through nodes in 5 different non-allied jurisdictions cannot be compelled by any
single government. No court order, no subpoena, no national security letter can force
simultaneous cooperation across Switzerland, Iceland, Romania, Panama, and Singapore to
trace a single Bitcoin transaction back to its sender.

### Layer 5: Protocol Camouflage

RPC calls to blockchain nodes are wrapped in the same pluggable transport as all other
Scrambler traffic. Deep packet inspection cannot distinguish "user is interacting with
Ethereum" from "user is browsing the web" from "user is sending a chat message." The
transport layer does not change based on payload type.

- **obfs5**: RPC calls appear as random bytes. No JSON-RPC signatures, no Ethereum
  protocol fingerprints, no Monero daemon wire format. Everything is encrypted before
  it reaches the transport layer.
- **TLS camouflage**: Blockchain interactions look like HTTPS requests to a CDN. An
  ISP sees `cdn.example.com` in the SNI field, not `mainnet.infura.io`.
- **Domain fronting**: In censored environments, blockchain RPC calls are tunneled
  through CDN infrastructure. Blocking access to blockchain nodes requires blocking the
  entire CDN.

This is particularly important for blockchain interactions because many RPC endpoints
(Infura, Alchemy, public Monero nodes) are well-known. Without protocol camouflage, an
adversary performing DPI could identify "this user is talking to a Monero RPC endpoint"
even through a VPN. With Layer 5, they cannot.

### Layer 6: Dead Drops and Exit Nodes

Financial operations use two variants of the dead drop architecture depending on the
operation type:

**For in-chat payments** (payment confirmations, payment request messages): The payment
notification message uses dead drops exactly like any chat message. The recipient polls
their dead drops on schedule and discovers the payment notification alongside any other
messages. The dead drop relay does not know that the deposited share contains a payment
notification versus a text message versus a photograph. They are all identical Sphinx
payloads.

**For transaction broadcasts** (submitting signed transactions to blockchain networks):
The Scrambler exit node submits the reconstructed signed transaction to the target
blockchain node. The exit node functions as an anonymous broadcast relay -- it knows it
is submitting a transaction, but it does not know who initiated it. The transaction
arrived as an anonymous Sphinx packet through the mixnet. The exit node's IP address
appears on the blockchain network as the transaction source, not the user's. With
multi-node broadcast (see below), multiple exit nodes in multiple countries submit the
same transaction simultaneously, further dissolving any single point of attribution.

**For RPC queries** (balance checks, fee estimation): The query exits the Scrambler via
an exit node that forwards the RPC call to the target blockchain endpoint. The response
travels back through a separate Scrambler path to the client. The RPC endpoint sees a
request from a Scrambler exit node in a random country. It does not know who asked.

### Layer 7: Temporal Scrambling for Financial Operations

**Mandatory random delay between payment intent and blockchain broadcast.** When a user
taps "send payment," the following sequence occurs:

1. The transaction is signed immediately on the local device.
2. The signed transaction is held locally for a random Poisson-distributed delay
   (configurable, using the same delay parameters as message pre-delays).
3. After the delay, the transaction enters the Scrambler pipeline as Sphinx packets.
4. Each Sphinx packet is subject to additional delays at every mix node (same Poisson
   delays as message packets).
5. The transaction reaches exit nodes and is broadcast to the blockchain.

This prevents timing correlation between "Alice tapped send in her chat" and "transaction
X appeared in the mempool." Without temporal scrambling, an adversary watching both the
Scrambler network and the blockchain mempool could correlate the timing of a user's
Scrambler activity spike with the appearance of a new transaction. With Layer 7, the
delay between intent and broadcast is random and unpredictable -- typically 2 to 45
seconds depending on urgency mode, identical to message latency.

The same urgency modes available for messages apply to financial operations:

| Mode | Typical Broadcast Delay | Use Case |
|------|------------------------|----------|
| Maximum | 30-90s | High-value transactions, maximum anonymity |
| High (default) | 5-45s | Standard payments |
| Standard | 2-20s | Time-sensitive transfers |
| Low latency | 1-8s | Urgent payments (reduced temporal anonymity) |

---

## Cross-Layer Timing Decorrelation

In-chat payment messages and on-chain transaction broadcasts are **deliberately
desynchronized**. This is a critical design decision that prevents a class of correlation
attacks unique to payment systems built into messengers.

### The Attack

An adversary with access to both the Scrambler network (as a partial observer) and the
blockchain (which is public) could attempt the following:

1. Observe that Alice's Scrambler client produced activity at time T.
2. Observe that a new transaction appeared in the blockchain mempool at time T + delta.
3. If delta is small and consistent, correlate Alice with the transaction.
4. Simultaneously, observe that Bob's Scrambler client received a message at time T + delta2.
5. Infer that Alice paid Bob.

### The Defense

The Shadow Wallet splits every in-chat payment into two independent operations that take
completely separate paths through the Scrambler:

**Operation 1: The chat message.** An encrypted message is sent through the Scrambler
to the recipient, confirming the payment. This message says (in encrypted form) "I sent
you X amount, here is the transaction reference." It travels through the standard message
pipeline -- fragmented, mixed, delayed, deposited at dead drops, retrieved by the
recipient on their polling schedule.

**Operation 2: The blockchain transaction.** The signed transaction is broadcast through
a completely separate set of Scrambler paths to blockchain nodes via exit nodes. It takes
different routes, passes through different mix nodes, exits through different countries,
and arrives at different times.

These two operations are:

- **Path-independent.** They use different Scrambler paths, different mix nodes, different
  exit points. No network node sees both operations.
- **Temporally independent.** Each has its own Poisson pre-delay, its own mix node delays,
  its own dead drop/exit timing. There is no fixed relationship between when the chat
  message arrives and when the transaction appears on-chain.
- **Structurally independent.** The chat message is a Sphinx packet deposited at a dead
  drop. The transaction is a Sphinx packet submitted to a blockchain node via an exit
  relay. They do not share infrastructure at the delivery layer.

An adversary watching both the Scrambler and the blockchain sees: a constant stream of
identical Sphinx packets in the Scrambler (cover traffic makes activity undetectable),
and transactions appearing in the mempool at random times from random Scrambler exit
nodes in random countries. There is no timing signal to correlate.

---

## Multi-Node Transaction Broadcast

For non-privacy coins (BTC, ETH, stablecoins), where transactions are visible on a
public blockchain, the Shadow Wallet employs multi-node simultaneous broadcast to defeat
first-seen geolocation attacks.

### The Attack

Blockchain surveillance firms operate nodes in many geographic locations. When a
transaction first appears in the mempool, the firm notes which of its nodes saw it first.
The node that received it first is likely geographically closest to (or directly connected
to) the broadcasting node. By triangulating first-seen times across many monitoring nodes,
the firm can estimate the geographic origin of the transaction.

### The Defense

The Shadow Wallet broadcasts each signed transaction to **N different blockchain nodes
simultaneously** (default N=5, configurable up to 8), using N independent Scrambler exit
paths. Each exit path:

- Traverses a different set of mix nodes.
- Exits through a different Scrambler exit node in a different country.
- Connects to a different blockchain full node.
- Has independent Poisson delays at every stage.

The result: a blockchain surveillance firm's monitoring nodes see the transaction appear
"simultaneously" (within the noise of network propagation) from multiple unrelated
geographic locations. First-seen analysis produces contradictory results -- the transaction
appears to originate from Switzerland and Singapore and Brazil and Iceland at the same
time. The signal is destroyed.

### Privacy Coin Considerations

**Monero (XMR):** The Monero daemon already implements Dandelion++ for transaction
propagation, which provides its own first-seen protection by routing new transactions
through a random "stem" path before broadcasting. The Scrambler adds an additional layer
on top -- the Dandelion++ stem begins at a Scrambler exit node, not at the user's device.
The user's network identity is protected by both the Scrambler pipeline and Dandelion++.

**Zcash shielded (ZEC):** Shielded transactions reveal no sender, recipient, or amount
information on-chain. Multi-node broadcast adds geolocation resistance for the broadcast
event itself, complementing Zcash's on-chain privacy.

### Parameters

| Parameter | Default | Range | Description |
|-----------|---------|-------|-------------|
| `broadcast_fanout` | 5 | 3-8 | Number of simultaneous broadcast paths |
| `broadcast_jitter` | 2s | 0-5s | Additional random jitter between broadcasts |
| `min_broadcast_jurisdictions` | 3 | 2-5 | Minimum distinct countries for exit nodes |

---

## In-Chat Payments

- Send/receive crypto inline in any chat (1-on-1 or group)
- Payment requests: request specific amount, one-tap to pay
- Group splits: split a bill among group members, auto-calculated
- No payment processor. Transactions are signed locally and broadcast through the full
  Scrambler pipeline -- same 7-layer protection as messages. The on-chain transaction
  and the in-chat payment notification are deliberately desynchronized to prevent timing
  correlation.
- Payment history is local only, encrypted with SQLCipher + Argon2id

### Payment Flow

```
    SENDER TAPS "SEND 0.5 XMR"
              |
              v
    [1] Transaction signed locally (private key never leaves device)
              |
              v
    [2] Poisson pre-delay (Layer 7) -- random wait before broadcast
              |
              +-----> [3a] Signed tx enters Scrambler as Sphinx packets
              |             Fragmented (Layer 1), mixed (Layer 2),
              |             covered (Layer 3), jurisdiction-routed (Layer 4),
              |             camouflaged (Layer 5), exits via N exit nodes
              |             to N blockchain nodes (Layer 6)
              |
              +-----> [3b] Encrypted payment confirmation enters Scrambler
                            as separate Sphinx packets on separate paths.
                            Deposited at dead drops (Layer 6).
                            Recipient retrieves on polling schedule.

    [3a] and [3b] take DIFFERENT paths, DIFFERENT delays, DIFFERENT exits.
    An adversary cannot correlate them by timing, path, or structure.
```

---

## Privacy Coin Integration

### Monero (XMR) -- The Default Currency

- Ring Signatures: transaction mixed with 16+ decoys
- Stealth Addresses: one-time address per transaction
- RingCT: transaction amounts hidden (Pedersen commitments)
- FCMP++ (2026): full chain membership proofs, anonymity set = entire blockchain
- Dandelion++: transaction propagation privacy (complemented by Scrambler)
- Result: sender hidden, recipient hidden, amount hidden, untraceable
- Scrambler adds: broadcast origin hidden, network identity hidden, timing decorrelated

### Zcash Shielded (ZEC)

- zk-SNARKs: zero-knowledge proofs of transaction validity
- Only shielded (z-address) transactions supported -- transparent addresses blocked
- Sapling/Orchard shielded pools
- Scrambler adds: broadcast origin hidden from chain surveillance

### Bitcoin (BTC) -- Privacy-Enhanced

- CoinJoin: transactions mixed with other users
- PayJoin: sender and recipient both contribute inputs
- Silent Payments (BIP-352): one-time addresses per transaction
- Optional: auto-swap BTC -> XMR -> BTC via atomic swap
- Multi-node broadcast: defeats first-seen geolocation (see above)

### Ethereum & Stablecoins

- Transactions routed through ZK-rollup privacy layers
- All RPC calls go through full Scrambler pipeline (no IP leaks to Infura/Alchemy)
- Multi-node broadcast for transaction submission
- Smart contract interactions (approvals, swaps, bridges) receive same Scrambler treatment

---

## Wallet Security

- Seed phrase: generated locally, encrypted at rest
- Private keys: NEVER leave device, NEVER backed up to cloud, NEVER transmitted
- Seed displayed once during setup (user's responsibility to back up physically)
- Multi-sig option: 2-of-3 signatures for large transactions (configurable threshold)
- Auto-purge: wallet transaction history follows same retention policy as messages
  (24h-90d)
- Balances can be re-derived from chain if history is purged

### Network Isolation

All blockchain network interactions pass through Ghost VPN (Layer 0) and the full
Scrambler stack. This is not optional. There is no code path in the Shadow Wallet that
makes a direct connection to any blockchain node, RPC endpoint, or network service.

- **Transaction broadcasts**: Through Scrambler to exit nodes to blockchain nodes.
- **Balance queries**: Through Scrambler to exit nodes to RPC endpoints.
- **Fee estimation**: Through Scrambler to exit nodes to RPC endpoints.
- **Block height checks**: Through Scrambler to exit nodes to blockchain nodes.
- **ENS/naming lookups**: Through Scrambler to exit nodes to resolution services.
- **Token metadata**: Through Scrambler to exit nodes to contract calls.

No direct connection to any blockchain node, ever. RPC endpoints are accessed only
through Scrambler exit nodes. The user's IP address never appears in any blockchain
node's connection log. The user's ISP never sees traffic to any blockchain-related IP
address. The only outbound connection from the user's device is the WireGuard tunnel to
a randomly selected Ghost VPN endpoint.

---

## DeFi Wallet Connect

- WalletConnect v2 integration
- All RPC calls routed through full Scrambler pipeline
- Transaction signing happens locally
- dApp frontends never see real IP
- Connect MetaMask, Trust Wallet, Phantom, Rabby, etc.
- ENS lookups go through Scrambler
- All DeFi interactions (swaps, liquidity provision, lending, borrowing) subject to same
  7-layer privacy pipeline as direct payments

---

Cross-references: [phantom-swap.md](phantom-swap.md), [defi-proxy.md](defi-proxy.md), [cryptography.md](cryptography.md), [scrambler.md](scrambler.md)
