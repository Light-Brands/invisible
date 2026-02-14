# Simple Guide: What Servers Do You Need?

**A visual breakdown of every server and what it does.**

---

## The 3 Types of Servers

```
YOUR PHONE/COMPUTER
        ↓
   [VPN SERVER] ← Encrypts your internet connection
        ↓
   [MIX NODE 1] ← Receives your encrypted packet
        ↓
   [MIX NODE 2] ← Shuffles with other packets
        ↓
   [MIX NODE 3] ← Routes to next hop
        ↓
   [MIX NODE 4] ← More shuffling
        ↓
   [MIX NODE 5] ← Final mix before destination
        ↓
  [DEAD DROP 1] ← Stores message fragment #1
  [DEAD DROP 2] ← Stores message fragment #2
  [DEAD DROP 3] ← Stores message fragment #3
```

---

## Server Type 1: VPN Servers

**What:** WireGuard endpoints that encrypt all your traffic

**Purpose:**
- Hide your real IP address
- Encrypt traffic before it enters the internet
- Mandatory first layer of protection

**How many:** 3-5 servers (or use Mullvad)

**What they see:**
- ✅ Your real IP connecting
- ✅ Encrypted Sphinx packets going out
- ❌ What's in the packets (encrypted)
- ❌ Who you're talking to (hidden by mix network)

**Can they compromise you?**
- If you pay anonymously (Monero): NO
- If you pay with credit card: They know YOU use VPN
- Either way: They can't read your messages (E2EE)

**Cost:** $5/month (Mullvad) or $15/month (3 DIY servers)

---

## Server Type 2: Mix Node Relays

**What:** Servers that shuffle and forward encrypted packets

**Purpose:**
- Break link between sender and receiver
- Mix your packets with others' packets
- Prevent traffic analysis

**How many:** 3-5 servers (5 recommended for full privacy)

**What they see:**
- ✅ Encrypted packet arrived from previous hop
- ✅ Encrypted packet forwarded to next hop
- ❌ Who sent the original packet (onion routing)
- ❌ Who will receive it (hidden in layers)
- ❌ Message contents (encrypted)

**Can they compromise you?**
- Single node: NO - only sees prev/next hop
- Multiple nodes: NO - Sphinx prevents correlation
- All nodes: Still NO - messages are E2EE

**Cost:** $15-50/month (3-5 servers at $5-10 each)

---

## Server Type 3: Dead Drop Relays

**What:** Temporary storage for message fragments

**Purpose:**
- Anonymous message pickup
- No direct connection between sender/receiver
- Like a mailbox with a secret access code

**How many:** 3 servers (minimum 1, recommended 3)

**What they see:**
- ✅ Encrypted message fragment
- ✅ Random access token (32 random bytes)
- ❌ Who stored it (came via mix network)
- ❌ Who will retrieve it (access token is random)
- ❌ What's in the fragment (Shamir share = random noise)

**Can they compromise you?**
- Single dead drop: NO - fragment is random noise
- Two dead drops: NO - still can't reconstruct (need 3)
- Three dead drops: Can reconstruct message, but it's still E2EE

**Cost:** $5-30/month (1-3 servers at $5-10 each)

---

## Minimum Setup (Testing)

**Total: $25/month**

```
1 VPN:        Mullvad subscription         $5/month
3 Mix Nodes:  DigitalOcean droplets       $15/month
1 Dead Drop:  DigitalOcean droplet         $5/month
────────────────────────────────────────────────────
TOTAL:                                    $25/month
```

**What this gives you:**
- ✅ Full privacy stack operational
- ✅ 3-layer mixnet (minimum viable)
- ✅ Anonymous message delivery
- ✅ End-to-end encryption
- ⚠️ Reduced redundancy (3 layers vs 5)

---

## Recommended Setup (Production)

**Total: $50-80/month**

```
3 VPNs:       Njalla servers (Monero)     $45/month
5 Mix Nodes:  1984 Hosting (Monero)       $50/month
3 Dead Drops: FlokiNET (Monero)           $36/month
────────────────────────────────────────────────────
TOTAL:                                   $131/month

OR (Budget Option):
1 VPN:        Mullvad (Monero)             $5/month
5 Mix Nodes:  BitLaunch                   $25/month
3 Dead Drops: BitLaunch                   $15/month
────────────────────────────────────────────────────
TOTAL:                                    $45/month
```

**What this gives you:**
- ✅ Full 5-layer mixnet
- ✅ Redundant dead drops (fault tolerance)
- ✅ Anonymous payment (can't be traced to you)
- ✅ Multiple VPN endpoints (failover)
- ✅ Geographic diversity (jurisdiction routing)

---

## Privacy Protection Per Server Type

### VPN Servers - What Adversaries Learn

| Adversary | What They See | Can They Identify You? |
|-----------|--------------|----------------------|
| VPN Provider | Your real IP + encrypted traffic | ✅ YES (if credit card payment) |
| VPN Provider | Your real IP + encrypted traffic | ❌ NO (if Monero payment) |
| Hacker | Encrypted VPN tunnel | ❌ NO |
| Government | VPN traffic exists | ⚠️ Knows you use VPN, can't read |

### Mix Nodes - What Adversaries Learn

| Adversary | What They See | Can They Trace You? |
|-----------|--------------|-------------------|
| Single Mix Node | Previous hop → Next hop | ❌ NO |
| 2-3 Mix Nodes | Partial route | ❌ NO (with cover traffic) |
| All 5 Mix Nodes | Full route | ❌ NO (E2EE still protects) |
| Hacker | Encrypted Sphinx packets | ❌ NO |

### Dead Drops - What Adversaries Learn

| Adversary | What They See | Can They Read Messages? |
|-----------|--------------|----------------------|
| Single Dead Drop | 1 Shamir share (random) | ❌ NO |
| 2 Dead Drops | 2 Shamir shares | ❌ NO (need 3 minimum) |
| 3 Dead Drops | Full message (E2EE) | ❌ NO (still encrypted) |
| Hacker | Encrypted fragments | ❌ NO |

---

## The Critical Question: Can Servers Sniff Your Identity?

**Short answer: NO - by design.**

### Why Server Compromise Doesn't Reveal Identity:

#### Layer 1: End-to-End Encryption (E2EE)
```
Alice sends: "Hello Bob"
Servers see: [0x9a2f3c7e1d...] ← Random encrypted bytes
```
**Even if they hack ALL servers, they can't decrypt messages.**

#### Layer 2: Sphinx Onion Routing
```
Mix Node 2 sees:
  - Packet came from: Mix Node 1
  - Packet going to: Mix Node 3
  - Original sender: ??? (unknown)
  - Final receiver: ??? (unknown)
```
**Even if they control multiple nodes, they can't trace routes.**

#### Layer 3: Shamir Secret Sharing
```
Message split into 5 shares (need 3 to reconstruct):

Dead Drop 1: [0x3a9f...]  ← Looks random
Dead Drop 2: [0x7c2e...]  ← Completely different
Dead Drop 3: [0xb81d...]  ← Also random

Hack 2 servers? Learn NOTHING (need 3).
Hack 3 servers? Get encrypted message (still can't read).
```
**Information-theoretic security - not breakable even with infinite computing power.**

---

## Anonymous Server Payment Guide

### Why Pay Anonymously?

**Scenario A: You pay with credit card**
- Hosting provider knows: "John Smith operates these servers"
- Government subpoena → They identify you
- **BUT**: Still can't read user messages (E2EE protects users)

**Scenario B: You pay with Monero**
- Hosting provider knows: "Someone operates these servers"
- Government subpoena → They get nothing useful
- **Result**: You're anonymous, users are anonymous

### How to Pay with Monero (Step-by-Step)

```bash
# Step 1: Get Monero anonymously
# - Buy with cash on LocalMonero.co
# - Mine it yourself
# - Trade services for XMR

# Step 2: Install Monero wallet
# Download from: getmonero.org

# Step 3: Choose privacy-focused hosting
# - Njalla.la (Sweden)
# - 1984.hosting (Iceland)
# - FlokiNET.is (Iceland/Romania)

# Step 4: Register via Tor
# - Open Tor browser
# - Go to hosting provider
# - Create account with throwaway email
# - Pay with Monero

# Step 5: Deploy servers
# - Access ONLY via Tor
# - Use SSH keys (never passwords)
# - Deploy code from anonymous git
```

**Result:** No link between you and servers.

---

## Is Anonymous Deployment Overkill?

**It depends:**

### You DON'T need anonymous deployment if:
- ✅ It's a personal project / testing
- ✅ You're okay being identified as operator
- ✅ Users trust you won't surveil them
- ✅ No legal risk in your jurisdiction

**Why it's safe:** Platform cryptography protects users even if you're identified.

### You NEED anonymous deployment if:
- ⚠️ Operating a public privacy service
- ⚠️ Legal gray area in your country
- ⚠️ Serving high-risk users (activists/journalists)
- ⚠️ Want plausible deniability

**Why it matters:** Protects YOU from legal/physical risk.

---

## Quick Decision Matrix

**Choose your threat model:**

| Your Situation | Payment Method | Access Method | Cost |
|---------------|---------------|---------------|------|
| **Personal Testing** | Credit card OK | Regular SSH | $25/mo |
| **Small Privacy Service** | Mixed Bitcoin | VPN access | $45/mo |
| **Public Service** | Monero | Tor only | $50-130/mo |
| **High-Risk Users** | Monero + Cash | Tor + airgap | $130/mo+ |

---

## Summary: The Truth About Server Privacy

### What Servers CAN Learn:
- ✅ Traffic is flowing through them
- ✅ Approximate timing of packets
- ✅ Network addresses of adjacent hops

### What Servers CANNOT Learn:
- ❌ Who sent a message (Sphinx hiding)
- ❌ Who receives a message (onion routing)
- ❌ Message contents (E2EE)
- ❌ Full routing paths (only see prev/next)
- ❌ Anything from partial fragments (Shamir)

### What Anonymous Deployment Protects:
- ✅ YOUR identity as operator (not linked to servers)
- ✅ YOUR location (Tor/VPN access)
- ✅ YOUR payment info (Monero untraceable)

### What It Doesn't Change:
- Platform still secure even if you're identified
- Users are protected by cryptography, not operational security
- Your OpSec protects YOU, not your users

---

## Bottom Line

**The platform is designed so that even YOU, the operator, cannot compromise user privacy.**

**Anonymous deployment protects YOU from being identified as the operator.**

**Choose your deployment tier based on YOUR risk tolerance, not your users' privacy needs.**

---

Ready to deploy? See:
- `INFRASTRUCTURE_DEPLOYMENT.md` - How to deploy servers
- `SERVER_OPSEC_GUIDE.md` - Full OpSec details
- `QUICK_START.md` - Get started in 5 minutes
