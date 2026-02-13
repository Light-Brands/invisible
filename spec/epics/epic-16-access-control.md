# Epic 16: Access Control — Mandatory 2FA & Panic Features

## Overview
Implement mandatory two-factor authentication (architecturally required for database decryption), duress PIN, panic wipe, and biometric gating with no bypass option.

## User Stories

### US-16.1: Mandatory 2FA Setup
As a new user, I must set up 2FA during initial app setup — there is no way to skip this.

**Acceptance Criteria:**
- 2FA setup is part of first-launch flow (cannot be skipped or deferred)
- Supported: TOTP (RFC 6238) or FIDO2 hardware key
- No SMS-based 2FA (SIM swap vulnerable)
- 2FA secret becomes input to database encryption key derivation
- Without correct 2FA code, database cannot be decrypted (cryptographic, not just UI)

### US-16.2: Every-Open Authentication
As a user, I must authenticate with biometric + 2FA every time I open the app.

**Acceptance Criteria:**
- Step 1: Biometric (Face ID / Touch ID) OR device PIN
- Step 2: TOTP code OR hardware security key
- Both steps required — fail either = app stays locked
- No "remember this device" option
- No "skip for now" option
- 2FA required even if app was just backgrounded and returned

### US-16.3: Lockout Policy
As a user, I want failed attempts to trigger escalating lockouts.

**Acceptance Criteria:**
- 3 failed attempts → 5 minute lockout
- 6 failed attempts → 1 hour lockout
- 9 failed attempts → 24 hour lockout
- User can configure stricter policies
- Lockout timer stored in secure enclave (can't be bypassed by reinstall)

### US-16.4: Duress PIN
As a user, I want a secondary PIN that silently wipes all data when entered under coercion.

**Acceptance Criteria:**
- User configures a "duress PIN" during setup (optional but recommended)
- Entering duress PIN appears to unlock the app (empty/decoy state)
- Actually triggers silent, complete data wipe in background
- Attacker believes they've gained access; all real data is destroyed
- No visual indication of wipe occurring

### US-16.5: Panic Gesture
As a user, I want a physical gesture that instantly wipes all data.

**Acceptance Criteria:**
- Configurable gesture (e.g., shake phone 5 times rapidly)
- Works even from lock screen
- No confirmation dialog — immediate wipe
- Wipe method: secure overwrite + database key destruction
- All keys destroyed first (makes encrypted data unrecoverable), then data overwritten

### US-16.6: Remote Wipe
As a user, I want to remotely wipe my other devices.

**Acceptance Criteria:**
- Send signed wipe command through Scrambler to other devices
- Wipe command authenticated via device identity key
- Target device executes wipe on next connection
- Confirmation sent back (if target device still accessible)

### US-16.7: Kill Switch
As a user, I want an option to auto-wipe after N failed 2FA attempts.

**Acceptance Criteria:**
- Configurable threshold (default: disabled)
- Recommended for high-risk users
- After N failed attempts, all data wiped
- Kill switch setting itself protected by 2FA

### US-16.8: Auto-Lock
As a user, I want the app to auto-lock after brief inactivity.

**Acceptance Criteria:**
- Configurable: 30s / 1min / 2min (default) / 5min
- Lock requires full 2FA re-authentication
- Auto-lock independent of VPN timeout (shorter)

## Technical Requirements
- TOTP: RFC 6238 implementation
- FIDO2: WebAuthn/CTAP2 via platform APIs
- Biometric: Flutter local_auth plugin
- Key derivation: Argon2id with 2FA secret as input
- Secure enclave: Keychain (iOS) / Keystore (Android) for lockout state

## Dependencies
- Epic 00 (Foundation) — for database encryption integration

## Architecture References
- [access-control.md](../architecture/access-control.md), [cryptography.md](../architecture/cryptography.md), [zero-log-doctrine.md](../architecture/zero-log-doctrine.md)
