# Group Messaging — MLS-Based Secure Groups

## Overview
Invisible uses the Message Layer Security (MLS) protocol (RFC 9420) for group key management, providing forward secrecy and post-compromise security at the group level.

## Why MLS
- Scales to large groups (hundreds of members) without per-pair key exchanges
- TreeKEM: logarithmic key update cost (not linear like sender-keys)
- Standardized (IETF RFC 9420) with formal security proofs
- Forward secrecy and post-compromise security for groups

## Group Architecture
- Group state is a ratchet tree (binary tree of DH key pairs)
- Each member holds a leaf in the tree
- Group secrets derived from tree root
- Adding/removing members requires tree update (logarithmic cost)

## Encrypted Member Lists
- Server never sees group membership
- Member list encrypted with group key, stored locally on each device
- Group identifiers are random and unlinkable to members
- Relay nodes route group messages without knowing group composition

## Key Management
- Group key rotated on every member add/remove
- Forward secrecy: removing a member revokes their access to future messages
- Post-compromise security: one ratchet step after compromise restores security
- Each member generates fresh key material for tree updates

## Message Flow
1. Sender encrypts message with current group key (AES-256-GCM)
2. Message split via Shamir and routed through Scrambler (same as 1-on-1)
3. Each member retrieves from their dead drop queue
4. Members decrypt with their copy of the group key

## Group Operations
- **Create**: Founder generates initial tree, invites members via QR/link
- **Add member**: Existing member creates Welcome message + tree Update
- **Remove member**: Any member can propose removal, requires group commit
- **Leave**: Member removes self, triggers tree update
- **Update keys**: Members periodically update their leaf keys

## Group Limits
- Maximum 1000 members (MLS tree depth limit)
- Recommended: ≤50 for best performance
- Large groups may experience higher latency due to tree updates

## Burn Room Groups
- Groups can be configured as burn rooms (see [burn-rooms.md](burn-rooms.md))
- Ephemeral group key destroyed on burn timer expiry
- All members' local copies wiped simultaneously via burn signal

Cross-references: [cryptography.md](cryptography.md), [burn-rooms.md](burn-rooms.md), [scrambler.md](scrambler.md)
