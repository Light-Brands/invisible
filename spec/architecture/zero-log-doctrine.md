# Zero-Log Doctrine — Nothing Exists Unless You're Holding It

## Overview
This is a foundational architectural principle. No remote system in the Invisible network stores any data, ever.

## Remote Hosts: Absolute Zero

### Relay / Mix Node Guarantees
- No message logs
- No connection logs
- No IP address logs
- No timestamp logs
- No error logs containing user data
- No metrics that could identify users
- No disk writes of any packet content
- No database — relay nodes are stateless
- RAM-only packet processing
- Packets decrypted in memory, forwarded, gone
- Crash = zero recoverable data
- Seized server = empty hard drive

### Dead Drops
- Ephemeral RAM queues only
- Messages held max 72 hours, then purged
- No delivery receipts stored
- Queue IDs are random, rotate per session
- Server operator cannot inspect queue contents

### Technical Enforcement
- Relay software runs in memory-locked process (mlock/mlockall) — prevents swap to disk
- No filesystem writes in relay codepath
- Operational logs (uptime, bandwidth) contain zero user-attributable data
- Node operators cannot comply with data requests — nothing exists to hand over

## Device Isolation
- Messages exist ONLY on the device where received
- No cloud sync, no cross-device history
- No backup/restore mechanism (by design)
- No "linked devices" sharing message history
- Keys generated per-device, never exported
- Lose your phone = messages gone (that's the point)

## Auto-Purge System
- Ghost Mode: 24 hours (default)
- Shadow Mode: 7 days
- Archive Mode: 30 days
- Vault Mode: 90 days (maximum)
- Burn Now: manual instant wipe
- No option for "forever"

### What Gets Purged
- All message content
- All media/attachments
- Message metadata (timestamps, read status)
- Expired session keys
- Dead drop queue identifiers

### What Survives (until manually deleted)
- Contact list (encrypted)
- Active identity keys
- App settings/preferences

### Purge Method
- Secure overwrite: data overwritten with random bytes before filesystem deletion
- Prevents forensic recovery from flash storage
- Purge runs on background timer + every app launch

## Local Encryption at Rest
- SQLCipher: AES-256 encrypted SQLite
- Database key: Argon2id(device_hardware_key + passphrase + 2FA_secret)
- Argon2id params: 256MB memory, 4 iterations, 4 parallelism
- Individual messages additionally encrypted with per-conversation keys
- Double encryption: conversation-level E2EE + storage-level database encryption
- Even with physical device access + bypassed lock screen: DB is AES-256 encrypted and needs passphrase + 2FA

Cross-references: [data-models.md](data-models.md), [access-control.md](access-control.md), [cryptography.md](cryptography.md)
