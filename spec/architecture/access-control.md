# Access Control — App Security

## Overview
Invisible enforces mandatory 2FA with no bypass. 2FA is architecturally required — the 2FA secret is part of the database encryption key derivation.

## Every App Open
1. Biometric (Face ID / Fingerprint) OR device PIN
2. TOTP code (authenticator app) OR hardware security key (FIDO2/WebAuthn)
3. App unlocks, database decrypted with derived key

Fail either step → app stays locked.

## Why 2FA is Mandatory
- The 2FA TOTP secret is an input to Argon2id key derivation
- Without the correct 2FA code, the database encryption key cannot be derived
- This is not a UI gate — it's a cryptographic requirement
- Cannot be disabled, cannot be skipped, cannot be "remembered"

## Supported Second Factors
- TOTP: Google Authenticator, Authy, any RFC 6238 compatible app
- FIDO2: YubiKey, other WebAuthn hardware security keys
- No SMS-based 2FA (vulnerable to SIM swap attacks)

## Lockout Policy
- 3 failed attempts → 5 minute lockout
- 6 failed attempts → 1 hour lockout
- 9 failed attempts → 24 hour lockout
- Configurable: user can set stricter policies

## Auto-Lock
- App locks after configurable inactivity (default: 2 minutes)
- Lock requires full 2FA re-authentication
- No "remember this device" option
- No "skip for now" option

## Duress Features

### Duress PIN
- Secondary PIN that looks like it unlocks the app
- Actually triggers instant data wipe
- Wipe is silent — appears to open normally but with empty state
- Attacker believes they've gained access; in reality, all data is destroyed

### Panic Gesture
- Configurable gesture (e.g., shake phone 5 times) triggers immediate wipe
- Works even from lock screen
- No confirmation dialog — immediate action

### Remote Wipe
- Send a signed wipe command to other devices through the mixnet
- Uses special ephemeral key pair for wipe authorization
- Wipe command authenticated via device identity key

### Kill Switch
- If 2FA fails N times (configurable), auto-wipe all data
- Default: disabled (to prevent accidental wipe)
- Recommended for high-risk users

## Anti-Forensics Integration
- No thumbnails generated for media
- No clipboard integration — text cannot be copied from Invisible
- No app preview in task switcher (blank/decoy screen)
- No notification content — push says "New message" only (no preview, no sender)
- RAM-based media viewer — media never written to disk
- Secure keyboard option — built-in keyboard that doesn't log keystrokes

## Screen Capture Prevention
- Android: FLAG_SECURE blocks screenshots and screen recording
- iOS: UIScreen capturedDidChange detection (warns if screen recording starts)
- Watermarking option for group chats (invisible per-user watermark to trace leaks)

Cross-references: [cryptography.md](cryptography.md), [zero-log-doctrine.md](zero-log-doctrine.md), [burn-rooms.md](burn-rooms.md)
