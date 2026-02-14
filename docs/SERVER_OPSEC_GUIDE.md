# Server Infrastructure - Operational Security Guide

**Can your servers compromise your users' privacy? Let's find out.**

## TL;DR: The Answer

**No, compromised servers CANNOT reveal user identities** - by design.

Here's why:

1. ✅ **Mix nodes** never see sender/receiver - only encrypted Sphinx packets
2. ✅ **Dead drops** only store encrypted fragments with access tokens
3. ✅ **VPN servers** only see encrypted traffic to mix nodes
4. ✅ **Shamir secret sharing** means any single server has ZERO information
5. ✅ **Zero-knowledge architecture** - servers process but never learn

**But you still need OpSec for deployment** - let me show you how.

---

## Threat Model: What Can Adversaries Learn?

### Scenario 1: Someone Compromises Your Mix Node

**What they get:**
- Encrypted Sphinx packets flowing through
- Timing of packets (when received/forwarded)
- Network addresses of previous/next hops

**What they CANNOT get:**
- Original sender identity ❌
- Final recipient ❌
- Message contents ❌ (encrypted)
- Full route ❌ (only knows prev/next hop)

**Why:** Sphinx packets use onion encryption. Each layer only reveals the next hop.

```
Original: Alice → [Layer0 → Layer1 → Layer2 → Layer3 → Layer4] → Bob

If Layer2 is compromised:
- Sees: "Traffic from Layer1 → Layer3"
- Doesn't know: Alice exists, Bob exists, message content
- Learns: NOTHING about sender/receiver
```

### Scenario 2: Someone Compromises Your Dead Drop

**What they get:**
- Encrypted message fragments
- Access tokens (random 32-byte values)
- Expiration times

**What they CANNOT get:**
- Who sent the message ❌
- Who will retrieve it ❌
- Message contents ❌ (encrypted + fragmented)
- Full message ❌ (only has 1 of K fragments)

**Why:** Shamir secret sharing means ANY single fragment reveals ZERO information.

```
Original message: "Hello Bob"
Split into 5 shares (need 3 to reconstruct):

Dead Drop 1: [0x9a3f...] ← Looks like random noise
Dead Drop 2: [0x2c7e...] ← Completely different random noise
Dead Drop 3: [0xb81d...] ← Also random

Compromise ANY 2 drops? You learn NOTHING.
Need 3? You can reconstruct message (but still encrypted end-to-end).
```

### Scenario 3: Adversary Controls Multiple Servers

**If they control 2 out of 5 mix nodes:**
- Can observe traffic correlation at those 2 points
- Still cannot decrypt messages (E2EE)
- Cannot determine sender/receiver (Sphinx unlinkability)
- Timing analysis is mitigated by cover traffic

**If they control 2 out of 5 dead drops:**
- Have 2 Shamir shares (need 3 minimum)
- Learn ZERO information about message (information-theoretic security)

**Why it's safe:** Platform designed for K-of-N threshold security.

### Scenario 4: Global Passive Adversary (Worst Case)

**If they monitor ALL internet traffic:**
- See encrypted packets entering VPN
- See encrypted Sphinx packets between mix nodes
- See encrypted fragments stored in dead drops
- Can attempt timing correlation attacks

**Mitigations:**
- ✅ Cover traffic (constant-rate dummy packets)
- ✅ Temporal delays (Poisson distribution)
- ✅ Batch mixing (shuffle with other users' packets)
- ✅ Multi-path routing (different routes per fragment)

**Result:** Correlation attacks are computationally infeasible.

---

## Privacy-Preserving Server Deployment

### Goal: Deploy servers WITHOUT linking them to your identity

### Step 1: Anonymous Payment (CRITICAL)

#### ✅ Option A: Monero (Best Privacy)

**Why Monero:**
- Ring signatures (sender anonymity)
- Stealth addresses (receiver anonymity)
- Confidential amounts (transaction amounts hidden)
- No blockchain analysis possible

**Providers accepting Monero:**
- **Njalla** (privacy-focused hosting): https://njal.la
- **1984 Hosting** (Iceland, privacy laws): https://1984.hosting
- **FlokiNET** (Romania/Iceland): https://flokinet.is

**Payment flow:**
```bash
# 1. Buy Monero with cash (LocalMonero)
# 2. Send to your wallet (privacy preserved)
# 3. Pay hosting provider
# 4. Provider CANNOT link payment to you
```

#### ✅ Option B: Bitcoin + CoinJoin

**If Monero not available:**
```bash
# 1. Buy Bitcoin
# 2. Mix through Whirlpool/CoinJoin
# 3. Send to hosting provider
# 4. Use Tor for all communications

# Providers accepting Bitcoin:
# - Njalla
# - BitLaunch (resells DigitalOcean/Vultr with crypto)
```

#### ❌ Option C: Traditional Payment (NOT RECOMMENDED)

**Credit card/PayPal:**
- Links servers to your real identity
- Subpoena reveals your infrastructure
- **Only use if you're okay being identified as operator**

### Step 2: Anonymous Registration

```bash
# GOOD OpSec:
1. Use Tor browser for ALL provider communications
2. Create throwaway email (ProtonMail via Tor)
3. Use fake/pseudonymous registration details
4. Pay with Monero/mixed Bitcoin
5. Never access servers from your real IP

# BAD OpSec:
1. Register with real name/email ❌
2. Pay with credit card ❌
3. SSH from home IP ❌
4. Use personal GitHub to deploy code ❌
```

### Step 3: Server Hardening (After Deployment)

```bash
# On each server:

# 1. Disable password authentication (keys only)
sudo sed -i 's/PasswordAuthentication yes/PasswordAuthentication no/' /etc/ssh/sshd_config
sudo systemctl restart sshd

# 2. Enable UFW firewall (deny all except needed ports)
ufw default deny incoming
ufw default allow outgoing
ufw allow 9001/tcp  # Mix node port
ufw allow 22/tcp    # SSH (consider changing port)
ufw enable

# 3. Install fail2ban (blocks brute force)
apt install -y fail2ban
systemctl enable fail2ban

# 4. Automatic security updates
apt install -y unattended-upgrades
dpkg-reconfigure -plow unattended-upgrades

# 5. Disable root login
sudo passwd -l root

# 6. Remove unnecessary services
systemctl disable snapd
systemctl stop snapd
apt remove --purge snapd

# 7. Install intrusion detection (optional)
apt install -y aide
aideinit
```

### Step 4: Access Servers Safely

```bash
# ALWAYS use Tor or VPN when accessing servers

# Option A: SSH through Tor
# Add to ~/.ssh/config:
Host mix-node-*
    ProxyCommand nc -x localhost:9050 %h %p

# Then SSH normally (goes through Tor):
ssh root@server-address

# Option B: Use VPN
# Connect to VPN BEFORE SSHing
sudo wg-quick up wg0
ssh root@server-address
```

---

## Deployment OpSec Checklist

### Before Deployment

- [ ] Buy Monero/mixed Bitcoin with cash
- [ ] Create anonymous email (ProtonMail via Tor)
- [ ] Choose privacy-focused hosting (Njalla/1984/FlokiNET)
- [ ] Register via Tor browser only
- [ ] Use pseudonymous registration details

### During Deployment

- [ ] Pay with Monero (or mixed Bitcoin)
- [ ] Access server setup ONLY via Tor/VPN
- [ ] Use SSH keys (never passwords)
- [ ] Harden servers (firewall, fail2ban, updates)
- [ ] Deploy code from anonymous git (not personal GitHub)

### After Deployment

- [ ] Verify no logs linking you to servers
- [ ] Test server access ONLY via Tor/VPN
- [ ] Enable automatic security updates
- [ ] Monitor for intrusions (fail2ban logs)
- [ ] Regularly rotate servers (every 6-12 months)

---

## What Adversaries Can Learn (Summary Table)

| Adversary | What They Control | What They Learn | Your Identity Safe? |
|-----------|------------------|----------------|-------------------|
| **Single Mix Node** | 1 of 5 layers | Previous/next hop only | ✅ YES |
| **Multiple Mix Nodes** | 2-3 of 5 layers | Partial routes, timing | ✅ YES (with cover traffic) |
| **All Mix Nodes** | Entire mixnet | Full routes (but encrypted) | ✅ YES (E2EE protects) |
| **Dead Drop** | 1-2 of 5 fragments | Random encrypted data | ✅ YES (Shamir threshold) |
| **VPN Provider** | VPN traffic | Encrypted Sphinx packets | ✅ YES (nested encryption) |
| **Hosting Provider** | Physical servers | IP addresses, network traffic | ⚠️ ONLY if paid anonymously |
| **Global Passive** | All internet traffic | Encrypted flows, timing | ⚠️ Need cover traffic + delays |

---

## Recommended Deployment Strategy

### Tier 1: Maximum Privacy (Recommended)

**Infrastructure:**
- VPN: 3 servers via **Njalla** (Monero payment)
- Mix Nodes: 5 servers via **1984 Hosting** (Monero payment)
- Dead Drops: 3 servers via **FlokiNET** (Monero payment)

**OpSec:**
- All payments: Monero (untraceable)
- All access: Via Tor
- Registration: Pseudonymous
- Server location: Multiple jurisdictions (Iceland, Romania, Switzerland)

**Result:** Your identity is NOT linked to infrastructure.

### Tier 2: Good Privacy (Budget-Friendly)

**Infrastructure:**
- VPN: Use **Mullvad** (accepts Monero)
- Mix Nodes: 3 servers via **BitLaunch** (Bitcoin + CoinJoin)
- Dead Drops: 1 server via **BitLaunch**

**OpSec:**
- Payments: Mixed Bitcoin
- Access: Via Mullvad VPN
- Registration: Throwaway email

**Result:** Identity linkage requires subpoena + blockchain analysis.

### Tier 3: Basic Privacy (NOT Recommended for Sensitive Use)

**Infrastructure:**
- VPN: DigitalOcean ($5/month)
- Mix Nodes: DigitalOcean ($15/month)
- Dead Drops: DigitalOcean ($5/month)

**OpSec:**
- Payment: Credit card (🚨 links to you)
- Access: SSH from home IP (🚨 links to you)

**Result:** Hosting provider knows you operate infrastructure.

---

## Privacy-Focused Hosting Providers

| Provider | Payment | Privacy | Locations | Cost |
|----------|---------|---------|-----------|------|
| **Njalla** | XMR, BTC | ⭐⭐⭐⭐⭐ | Sweden, NL | $15/mo |
| **1984 Hosting** | XMR, BTC | ⭐⭐⭐⭐⭐ | Iceland | $10/mo |
| **FlokiNET** | XMR, BTC | ⭐⭐⭐⭐⭐ | Iceland, Romania | $12/mo |
| **BitLaunch** | BTC | ⭐⭐⭐⭐ | Resells DO/Vultr | $5/mo |
| **Mullvad VPN** | XMR, BTC, Cash | ⭐⭐⭐⭐⭐ | VPN only | $5/mo |
| **DigitalOcean** | Card | ⭐ | Global | $5/mo |
| **Vultr** | Card, BTC | ⭐⭐ | Global | $5/mo |

---

## Is This Overkill?

**It depends on your threat model:**

### Scenario A: Personal Project / Testing
**Threat:** Curious hackers, casual surveillance
**Need Anonymous Deployment?** No - regular hosting is fine
**Why:** Platform's cryptography protects users even if you're identified

### Scenario B: Privacy Service for Others
**Threat:** Law enforcement, state actors
**Need Anonymous Deployment?** YES - Monero + Tor required
**Why:** You could be pressured to reveal infrastructure or surveil users

### Scenario C: High-Risk Activists / Journalists
**Threat:** Nation-state adversaries
**Need Anonymous Deployment?** ABSOLUTELY - maximum OpSec
**Why:** Compromise of servers could enable targeted attacks

---

## Key Insight: Defense in Depth

**Even if an adversary:**
- Knows you operate the servers
- Compromises ALL your servers
- Has unlimited computing power

**They still CANNOT:**
- Decrypt end-to-end encrypted messages (E2EE)
- Link senders to receivers (Sphinx unlinkability)
- Reconstruct messages from partial Shamir shares
- Break the cryptography (information-theoretic security)

**Why it works:**
```
Layer 1: E2EE (can't read messages)
Layer 2: Sphinx (can't trace routes)
Layer 3: Shamir (can't reconstruct from partial shares)
Layer 4: Cover traffic (can't correlate timing)
Layer 5: Multi-path (different routes per fragment)
```

**Bottom line:** The platform is designed so that **even you, the operator, cannot compromise user privacy**.

---

## Recommended Reading

1. **Threat Modeling:**
   - "Threat Modeling: Designing for Security" - Adam Shostack
   - EFF Surveillance Self-Defense: https://ssd.eff.org

2. **Operational Security:**
   - "The Grugq's OPSEC" series
   - Whonix documentation: https://whonix.org

3. **Anonymous Hosting:**
   - "Privacy for Server Operators" - Riseup
   - Tor Project: https://torproject.org

---

## Final Recommendation

**For most users deploying Invisible:**

1. **Use privacy-focused hosting** (Njalla, 1984, FlokiNET)
2. **Pay with Monero** (untraceable)
3. **Access via Tor** (no IP linkage)
4. **Use pseudonymous registration**
5. **Spread servers across jurisdictions**

**Cost:** ~$30-50/month for Tier 1 privacy
**Time:** 2-3 hours setup (same as regular deployment)
**Result:** Infrastructure NOT linked to your identity

**The platform's cryptography protects users even if servers are compromised.**
**But anonymous deployment protects YOU from being identified as the operator.**

---

Need help with anonymous deployment? Let me know which tier you want and I'll create the exact commands!
