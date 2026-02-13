# Data Models — Local-Only Encrypted Storage

## Overview
All data lives exclusively on the user's device. No server-side database exists. Relays are stateless or ephemeral-store only.

## Storage Engine
- SQLCipher (AES-256 encrypted SQLite)
- Database key: Argon2id(device_hardware_key + user_passphrase + 2FA_secret)
- Argon2id parameters: 256MB memory, 4 iterations, 4 parallelism
- All tables encrypted at rest — raw DB file is unreadable without credentials

## Schema

### messages
| Column | Type | Description |
|--------|------|-------------|
| id | TEXT (UUID) | Unique message ID |
| conversation_id | TEXT | Pairwise conversation identifier |
| content_encrypted | BLOB | AES-256-GCM encrypted message body |
| content_nonce | BLOB | 96-bit GCM nonce |
| sent_at | INTEGER | Unix timestamp (local) |
| received_at | INTEGER | Unix timestamp (local) |
| expires_at | INTEGER | Auto-purge timestamp |
| direction | INTEGER | 0=sent, 1=received |
| status | INTEGER | 0=pending, 1=delivered, 2=read |

### conversations
| Column | Type | Description |
|--------|------|-------------|
| id | TEXT | Pairwise conversation identifier |
| contact_id | TEXT | Reference to contact |
| created_at | INTEGER | Conversation creation time |
| last_message_at | INTEGER | Last message timestamp |
| retention_policy | INTEGER | 0=24h, 1=7d, 2=30d, 3=90d |
| is_burn_room | INTEGER | 1 if burn room |
| burn_timer_seconds | INTEGER | Burn room duration |

### contacts
| Column | Type | Description |
|--------|------|-------------|
| id | TEXT (UUID) | Local contact ID |
| pairwise_id | TEXT | Pairwise identifier for this contact |
| identity_key_public | BLOB | Contact's Ed25519 public key |
| display_name_encrypted | BLOB | User-chosen name (encrypted) |
| verified | INTEGER | Safety number verified flag |
| created_at | INTEGER | First contact time |

### key_store
| Column | Type | Description |
|--------|------|-------------|
| id | TEXT | Key identifier |
| key_type | TEXT | identity/signed_prekey/one_time/ratchet/daily_master |
| key_data_encrypted | BLOB | Encrypted key material |
| created_at | INTEGER | Key creation time |
| expires_at | INTEGER | Key expiry (NULL for identity) |
| conversation_id | TEXT | Associated conversation (NULL for global) |

### dead_drop_queues
| Column | Type | Description |
|--------|------|-------------|
| id | TEXT | Dead drop queue identifier |
| conversation_id | TEXT | Associated conversation |
| relay_address | TEXT | Relay node address |
| created_at | INTEGER | Queue creation time |
| session_id | TEXT | Current session (rotates) |

## Auto-Purge System
- Background timer runs purge on schedule
- Purge also runs on every app launch
- Secure deletion: random byte overwrite before filesystem unlink
- Retention options: 24h (default), 7d, 30d, 90d — no "forever" option
- Contacts and active keys survive purge (until manual deletion)

## No Server-Side Storage
- Relay nodes are stateless — process packets in RAM only
- Dead drops hold messages max 72 hours in RAM queues, then purge
- No server-side database, no message store, no user table

Cross-references: [cryptography.md](cryptography.md), [identity-system.md](identity-system.md), [zero-log-doctrine.md](zero-log-doctrine.md)
