# Burn Rooms — Self-Destructing Secret Chats

## Overview
Burn Rooms are conversations that self-destruct. Every message, every trace, every key — gone when the timer expires.

## Types

### 1-on-1 Burn Chat
- Two people, timed conversation
- Both parties must agree to timer setting
- Either party can trigger immediate burn

### Group Burn Room
- Up to 50 participants
- Creator sets timer
- One-time invite links (expire after single use)
- Anonymous participation option (no names shown, just messages)
- Room creator can trigger immediate burn for everyone

### Dead Man's Switch
- If no messages sent for X minutes, room auto-burns
- Prevents abandoned rooms from persisting
- Configurable: 30min / 1hr / 4hr inactivity trigger

## Timer Presets
- 15 minutes
- 60 minutes (default)
- 4 hours
- 24 hours
- Custom (minimum: 5 minutes)

## Properties
- All messages auto-delete after timer from SEND time
- Room itself deletes after timer from last message
- No screenshots allowed (FLAG_SECURE / capture detection)
- No copy/paste of messages
- No forwarding messages
- No media saving
- Typing notifications disabled
- Read receipts disabled
- No participant list visible (you see messages, not who's in the room)
- Invite link expires after use
- Max participants configurable

## Anti-Recovery Measures
- Messages encrypted with ephemeral room key
- Room key destroyed when room burns — even backed-up encrypted DB is unrecoverable
- Secure overwrite of message data in local database
- All participants' devices coordinate burn via signed "burn signal" through mixnet
- Dead drop queues for the room are purged
- Session keys destroyed

## Burn Signal Protocol
1. Timer expires (or manual burn triggered)
2. Burn signal generated: signed with room creator's key
3. Burn signal sent through Scrambler to all participants
4. Each device: securely wipes all room messages, destroys room key
5. Dead drop queues associated with room are abandoned (auto-purge on relay)
6. Room ceases to exist on all devices
7. No record it ever existed

## What Survives a Burn
Nothing. The room key is ephemeral and destroyed. Even if someone extracted the encrypted database before the burn, the key to decrypt those messages no longer exists anywhere.

Cross-references: [group-messaging.md](group-messaging.md), [access-control.md](access-control.md), [zero-log-doctrine.md](zero-log-doctrine.md), [scrambler.md](scrambler.md)
