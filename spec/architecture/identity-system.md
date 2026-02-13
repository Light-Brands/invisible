# Identity System — Zero-Identifier Design

## Overview
Invisible has NO phone numbers, NO emails, NO usernames — ever. Identity is purely cryptographic and pairwise.

## Core Principles
- Zero global identifiers — no way to look up or enumerate users
- Pairwise anonymous identifiers per conversation (different ID for each contact)
- Device-local identity keys that never leave the device
- Contact exchange only via QR code or one-time link
- Plausible deniability built into the protocol

## Identity Architecture
- Each device generates an Ed25519 identity key pair on first launch
- Identity key is the root of all cryptographic operations
- No registration server — no account creation step
- No phone number verification, no email confirmation

## Pairwise Identifiers
- For each conversation, a unique pairwise identifier is derived
- Identifier = HKDF(identity_key_A, identity_key_B, "pairwise")
- Contact A sees a different identifier for the same user than Contact B
- No way to correlate identifiers across conversations
- Relay nodes see only pairwise queue identifiers, never identity keys

## Contact Exchange
- QR code: encodes one-time key bundle (identity key + signed prekey + one-time prekey)
- One-time link: HTTPS URL containing encrypted key bundle, expires after single use
- Out-of-band verification: safety number comparison (numeric or QR)
- No contact discovery from address book (no phone number matching)

## Key Verification
- Safety numbers: SHA-256 hash of both parties' identity keys, displayed as 60-digit number or QR code
- Users can verify in person or via trusted channel
- Key change alerts: if a contact's identity key changes, conversation is flagged

## Plausible Deniability
- Triple Diffie-Hellman provides offline deniability
- No cryptographic proof that a specific user sent a specific message
- Messages can be forged by anyone who knows the session key
- Important for legal protection in sensitive contexts

## Multi-Device
- Each device is an independent identity (separate key pair)
- No linked device sharing message history
- New device = blank slate (no message migration)
- Contacts must re-verify new devices

Cross-references: [cryptography.md](cryptography.md), [data-models.md](data-models.md), [api-reference.md](api-reference.md)
