# Product Vision — The Messenger That Doesn't Exist

## The Problem

Every mainstream messenger is fundamentally broken for privacy:

- **Signal**: Requires a phone number. Centralized servers. US jurisdiction. Metadata visible to Signal Foundation.
- **Telegram**: Not E2EE by default. Cloud-stored messages. Russian origin, UAE operations. Cooperates with governments.
- **WhatsApp**: Owned by Meta. Metadata harvested. Backdoor pressure from governments.
- **iMessage**: Apple holds encryption keys for iCloud backups. US jurisdiction. Closed source.

Even the best options (Signal) leak metadata: who you talk to, when, how often, from where. This metadata is often more dangerous than message content.

## The Vision

**Invisible is the messenger that doesn't exist.**

No trace that you used it. No record of who you talked to. No metadata for anyone to harvest. No server to subpoena. No company to pressure. No jurisdiction to compel.

It's not just encrypted messages — it's an entire privacy infrastructure:
- The network can't see who's talking to whom (Scrambler)
- Your ISP can't see you're using Invisible (Ghost VPN + Protocol Camouflage)
- Your device reveals nothing if seized (2FA-encrypted storage + panic wipe)
- Your money moves as privately as your words (Shadow Wallet)

## Design Philosophy

### Privacy is not a feature — it's the architecture
Every technical decision starts with: "What does an adversary learn?" If the answer is anything other than "nothing," we redesign.

### Zero trust by default
We don't trust ourselves. The system is designed so that even Invisible's developers, if compelled, have nothing to provide. No keys, no logs, no metadata, no user list.

### Usability without compromise
The hardest part of privacy tools is making them usable. Invisible must be as easy to use as iMessage while providing stronger privacy than anything on the market. If privacy requires user effort, we've failed.

### Open everything
Every line of client code, relay code, and protocol specification is open source. Reproducible builds ensure the binary matches the source. Trust is verified, not assumed.

## Market Position

### Who Invisible Is For
- Journalists protecting sources
- Human rights workers in authoritarian regimes
- Lawyers with attorney-client privilege obligations
- Activists organizing in hostile environments
- Business professionals with trade secrets
- Anyone who believes private communication is a human right
- Crypto-native users who want financial privacy
- Regular people who are tired of being the product

### Who Invisible Is NOT For
- People who want to sync messages across 5 devices effortlessly (each device is an island)
- People who want cloud backup (there is none)
- People who want social features, stories, channels (this is a messenger, not a social network)
- People unwilling to use 2FA (it's mandatory)

## Purpose and Monetization

### Internal Team Use
Invisible is built for internal team use. All features are available to every team member with no feature gating. Privacy is the default, not a premium.

### If We Choose to Monetize
Should Invisible be released publicly, the model would be open-core:

**Free (Personal Use)**
- Full E2EE messaging with all Scrambler layers
- Ghost VPN
- Shadow Wallet (basic)
- Burn rooms
- All privacy features

**Paid (Teams / Enterprise)**
- Priority relay nodes (lower latency)
- Team key management
- Admin controls for team deployment
- SLA guarantees for relay infrastructure
- Advanced Phantom Swap (higher volume)
- Priority support

**Infrastructure Incentives**
- Community node operators earn fees from network traffic
- Decentralized — revenue flows to infrastructure operators, not a central company

## Success Metrics
We don't track users (we can't). Success is measured by:
- Number of community relay/mix/VPN nodes
- Network uptime and latency
- Security audit results
- Open-source contributor count
- Protocol adoption by other projects
- Absence of successful deanonymization attacks

## Competitive Landscape

| | Signal | Telegram | SimpleX | Session | Nym | **Invisible** |
|---|---|---|---|---|---|---|
| E2EE default | Yes | No | Yes | Yes | N/A | **Yes** |
| Post-quantum | Yes | No | No | No | No | **Yes** |
| Zero identifiers | No | No | Yes | Yes | No | **Yes** |
| Mixnet | No | No | No | No | Yes | **Yes** |
| Built-in VPN | No | No | No | No | Yes* | **Yes** |
| Crypto wallet | No | Yes* | No | No | No | **Yes** |
| Atomic swaps | No | No | No | No | No | **Yes** |
| Open source (all) | Partial | No | Yes | Yes | Yes | **Yes** |
| Decentralized | No | No | Yes | Yes | Yes | **Yes** |

Invisible combines the best of Signal (E2EE), SimpleX (zero identifiers), Nym (mixnet), and adds layers that no competitor offers: mandatory VPN, Shamir fragmentation, jurisdiction routing, protocol camouflage, burn rooms, and a full privacy-first financial layer.
