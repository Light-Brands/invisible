# Shadow Wallet Security Analysis & Hardening Plan

## Executive Summary

The Shadow Wallet already implements the **full 7-layer Scrambler protection** for all financial operations, matching the messenger's network-level security. However, there are additional hardening opportunities at the **cryptographic, hardware, and application layers** that can make financial operations even more secure.

---

## Current Security Posture: What's Already Protected

### ✅ Network & Metadata Protection (EXCELLENT)

The Shadow Wallet **already has** complete Scrambler parity:

1. **Layer 0: Ghost VPN** - All blockchain traffic through WireGuard tunnel
2. **Layer 1: Transaction Fragmentation** - Shamir's Secret Sharing (3-of-5) for transaction broadcasts
3. **Layer 2: Mixnet** - 5-layer mixnet with batching, delays, reordering
4. **Layer 3: Financial Cover Traffic** - Constant-rate dummy RPC queries, indistinguishable from real operations
5. **Layer 4: Jurisdiction Routing** - Multi-country routing for all blockchain interactions
6. **Layer 5: Protocol Camouflage** - obfs5/TLS for blockchain RPC calls
7. **Layer 6: Exit Nodes** - Multi-node broadcast to N blockchain nodes simultaneously
8. **Layer 7: Temporal Scrambling** - Random delay between "tap send" and blockchain broadcast

**Network isolation:** NO direct connection to any blockchain node, ever.

**Timing decorrelation:** In-chat payment message and on-chain transaction deliberately desynchronized.

---

## Identified Gaps & Hardening Opportunities

### 1. Hardware Security Module (HSM) Integration

**Current State:**
- Database key: `Argon2id(device_key + passphrase + 2FA_secret)`
- Device key is "hardware-bound where available" (Secure Enclave on iOS, StrongBox on Android)

**Gap:**
- Wallet private keys themselves are encrypted at rest but **not stored in hardware-protected memory**
- Transaction signing happens in software memory, vulnerable to memory dumps

**Hardening Recommendation:**

```
CRITICAL: Store wallet private keys in Secure Enclave/StrongBox, not just the database key

iOS:
- Use Secure Enclave for private key storage (kSecAttrTokenIDSecureEnclave)
- Transaction signing happens INSIDE the enclave (keys never leave)
- Face ID/Touch ID required for each transaction signature

Android:
- Use StrongBox Keymaster (hardware-backed keystore)
- Keys generated with SECURITY_LEVEL_STRONGBOX
- Biometric required for key usage (setUserAuthenticationRequired)

Fallback:
- If hardware unavailable: keep current software approach
- Display warning: "Hardware security not available"
```

**Implementation:**
```rust
// iOS Secure Enclave integration
fn sign_transaction_ios(tx: &Transaction) -> Result<Signature> {
    // Keys stored in Secure Enclave, signing happens inside
    // User must authenticate with Face ID/Touch ID
    secure_enclave::sign_with_biometric(tx)
}

// Android StrongBox integration
fn sign_transaction_android(tx: &Transaction) -> Result<Signature> {
    // Keys in StrongBox, signing requires biometric
    strongbox::sign_with_biometric(tx)
}
```

---

### 2. Transaction Verification & Malleability Protection

**Current State:**
- Transactions signed locally
- Broadcast through Scrambler to multiple nodes

**Gap:**
- No explicit mention of transaction verification BEFORE signing
- Bitcoin transaction malleability attack surface
- Ethereum gas price manipulation risk
- No mention of replay protection for multi-chain

**Hardening Recommendation:**

```
IMPLEMENT: Multi-stage transaction verification before signing

Pre-Signing Verification Checklist:
1. Address validation
   - Checksum verification (Bitcoin Base58Check, Ethereum EIP-55)
   - Address format validation per chain
   - Detect address poisoning (similar-looking addresses)
   - Warn if sending to exchange/smart contract (metadata leak risk)

2. Amount validation
   - Verify amount matches user intent (clipboard hijacking detection)
   - Dust attack detection (tiny payments with tracking intent)
   - Large payment confirmation (>$1000 requires re-entering amount)

3. Replay protection
   - Bitcoin: Enforce BIP-143 (segregated witness) transaction signing
   - Ethereum: Include chainID in signature (EIP-155)
   - Check nonce/sequence numbers to prevent replay across forks

4. Fee validation
   - Compare fee estimate against multiple sources (prevent fee manipulation)
   - Warn if fee >5% of transaction value (prevent fee attacks)
   - Allow manual fee override only with explicit warning

5. Recipient verification
   - Display recipient's Invisible contact name if payment in-chat
   - Show safety number verification status
   - Require confirmation if sending to never-verified contact
```

**Implementation:**
```rust
struct TxVerification {
    address_valid: bool,
    amount_matches_intent: bool,
    fee_reasonable: bool,
    replay_protected: bool,
    recipient_verified: bool,
}

fn verify_before_sign(tx: &UnsignedTx, user_intent: &PaymentIntent) -> Result<TxVerification> {
    // 1. Address validation
    validate_address_checksum(tx.recipient)?;
    detect_address_poisoning(tx.recipient, user_intent.recipient)?;

    // 2. Amount validation
    if tx.amount != user_intent.amount {
        return Err("Amount mismatch - possible clipboard hijack");
    }

    // 3. Replay protection
    ensure_replay_protection(tx)?;

    // 4. Fee validation
    let fee_estimates = fetch_fee_estimates_multi_source().await?;
    if tx.fee > median(fee_estimates) * 2 {
        warn!("Fee unusually high");
    }

    // 5. Recipient verification
    let recipient_verified = check_safety_number_verified(tx.recipient)?;

    Ok(TxVerification { ... })
}
```

---

### 3. Secure Key Deletion & Memory Protection

**Current State:**
- "Secure deletion protocol" for keys (overwrite with random, fsync, overwrite with zeros, fsync, unlink)
- Keys encrypted at rest

**Gap:**
- No mention of **runtime memory protection** for keys in use
- Process memory could be dumped while keys are decrypted
- No mention of **secure memory locking** (mlock/mprotect) for wallet operations

**Hardening Recommendation:**

```
IMPLEMENT: Defense-in-depth memory protection for keys

Runtime Protection:
1. Memory locking
   - mlock() all memory pages containing key material
   - Prevent keys from being swapped to disk
   - Use memory-locked regions for transaction signing

2. Memory encryption
   - Keep keys encrypted in memory when not actively in use
   - Decrypt only for signing operation, immediately re-encrypt
   - Use separate encryption key for in-memory protection

3. Process isolation
   - Wallet crypto operations run in separate process with reduced privileges
   - IPC for transaction signing (process boundary isolation)
   - Crash isolation: wallet crash doesn't expose main app memory

4. Secure zeroing on drop
   - Use `zeroize` crate (already mentioned in cryptography.md)
   - Ensure compiler doesn't optimize away zeroing
   - Volatile writes for secure cleanup
```

**Implementation:**
```rust
use zeroize::Zeroize;

struct SecureKey {
    encrypted_key: Vec<u8>,  // Encrypted at rest AND in memory
    mlock_region: MemoryLock,  // Memory-locked region
}

impl SecureKey {
    fn sign(&self, message: &[u8]) -> Result<Signature> {
        // 1. Decrypt key into locked memory
        let mut plaintext_key = self.decrypt_to_locked_memory()?;

        // 2. Sign (key is in locked memory, can't be swapped)
        let signature = sign_with_key(&plaintext_key, message)?;

        // 3. Immediately zeroize and drop
        plaintext_key.zeroize();

        Ok(signature)
    }
}

impl Drop for SecureKey {
    fn drop(&mut self) {
        self.encrypted_key.zeroize();
        // mlock_region automatically unmapped
    }
}
```

---

### 4. Phishing & Social Engineering Protection

**Current State:**
- Safety numbers for contact verification
- In-chat payments tied to conversations

**Gap:**
- No explicit protection against payment to wrong contact
- No verification that payment request came from legitimate contact
- No warning system for suspicious payment patterns

**Hardening Recommendation:**

```
IMPLEMENT: Payment verification workflow

Pre-Payment Verification:
1. Identity verification
   - Display contact's safety number BEFORE payment confirmation
   - Show verification status (✓ verified / ⚠ never verified / ⚠ key changed)
   - Require re-verification if contact's identity key changed recently

2. Payment request authentication
   - Payment requests signed with contact's identity key
   - Verify signature before displaying payment request
   - Detect and reject forged payment requests

3. Suspicious pattern detection
   - First payment to new contact: require extra confirmation
   - Payment >10x typical amount: require re-entering amount
   - Multiple payments in quick succession: warn user
   - Payment to contact with recent key change: strong warning

4. Visual confirmation
   - Show contact's profile picture/name prominently
   - Display full address (not truncated) for external payments
   - Require explicit checkbox: "I verified this is the correct recipient"
```

---

### 5. Smart Contract Interaction Security (Ethereum/DeFi)

**Current State:**
- WalletConnect v2 integration
- All RPC calls through Scrambler
- Transaction signing local

**Gap:**
- No explicit security model for smart contract interactions
- Token approvals could be unlimited (common attack vector)
- No simulation of transaction effects before signing
- No verification of contract authenticity

**Hardening Recommendation:**

```
IMPLEMENT: Smart contract security layer

DeFi Safety Checks:
1. Token approval limits
   - NEVER allow unlimited token approvals
   - Default: approve only exact amount needed for transaction
   - Display approval amount in user-friendly format
   - Track active approvals, allow revocation

2. Contract verification
   - Verify contract is on expected chain (prevent cross-chain phishing)
   - Check contract against known malicious contract database
   - Display contract verification status from Etherscan/Blockscout
   - Warn if interacting with unverified contract

3. Transaction simulation
   - Simulate transaction locally BEFORE signing
   - Show predicted state changes (token balance changes, approvals, etc.)
   - Detect suspicious patterns (e.g., draining entire wallet)
   - Block obviously malicious transactions (transfer all tokens to unknown address)

4. dApp connection security
   - Display dApp origin prominently during WalletConnect session
   - Require re-auth for each transaction (not just session approval)
   - Auto-disconnect WalletConnect after 1 hour of inactivity
   - Track and display all active dApp sessions
```

**Implementation:**
```rust
struct SmartContractTx {
    contract: Address,
    function: String,
    token_approvals: Vec<TokenApproval>,
    predicted_effects: Vec<StateChange>,
}

fn verify_smart_contract_tx(tx: &SmartContractTx) -> Result<SecurityAssessment> {
    // 1. Check for unlimited approvals
    for approval in &tx.token_approvals {
        if approval.amount == U256::MAX {
            return Err("Unlimited approval detected - rejecting");
        }
    }

    // 2. Simulate transaction
    let simulation = simulate_tx_locally(tx).await?;

    // 3. Analyze predicted effects
    if simulation.transfers_all_tokens() {
        return Err("Transaction would drain wallet - blocking");
    }

    // 4. Verify contract
    let contract_verified = check_contract_verification(tx.contract)?;
    if !contract_verified {
        warn!("Unverified contract");
    }

    Ok(SecurityAssessment { ... })
}
```

---

### 6. Backup & Recovery Security

**Current State:**
- Seed displayed once during setup for physical backup
- No cloud backup (by design)
- If device lost, wallet is lost

**Gap:**
- Single point of failure: lose paper backup = lose funds
- No recovery option if seed destroyed
- No multi-device wallet sync (by design, but users may want it)

**Hardening Recommendation:**

```
IMPLEMENT: Shamir's Secret Sharing for seed backup (optional)

Seed Backup Options:

Option 1: Standard (current)
- 24-word BIP-39 seed phrase
- User writes on paper/metal
- Stored securely offline

Option 2: Shamir Backup (NEW - optional)
- Split seed into M-of-N shares using Shamir's Secret Sharing
- Example: 2-of-3 (any 2 shares can recover seed)
- User can store shares in different physical locations
- Loss of 1 share doesn't compromise wallet
- Use SLIP-39 standard (Shamir's Secret Sharing for mnemonics)

Option 3: Social Recovery (NEW - optional, advanced)
- User selects 3-5 trusted contacts (Invisible contacts)
- Seed split into shares, each share encrypted to contact's public key
- Shares distributed through Scrambler (dead drops)
- Recovery: contact 2-of-3 friends, they return encrypted shares
- Shares only usable with device key + 2FA (3-factor recovery)
```

**Implementation:**
```rust
// SLIP-39 Shamir backup
fn generate_shamir_seed_backup(seed: &[u8], threshold: u8, shares: u8) -> Vec<ShareMnemonic> {
    // Use SLIP-39 to generate share mnemonics
    slip39::generate_mnemonics(seed, threshold, shares)
}

// Social recovery (advanced)
fn social_recovery_setup(seed: &[u8], contacts: Vec<Contact>) -> Result<()> {
    // 1. Split seed with Shamir (2-of-3)
    let shares = shamir_split(seed, 2, 3);

    // 2. Encrypt each share to contact's public key
    let encrypted_shares = shares.iter()
        .zip(contacts.iter())
        .map(|(share, contact)| encrypt_to_contact(share, contact))
        .collect();

    // 3. Send encrypted shares through Scrambler
    for (share, contact) in encrypted_shares.iter().zip(contacts.iter()) {
        send_recovery_share(share, contact).await?;
    }

    Ok(())
}
```

---

### 7. Side-Channel Attack Protection

**Current State:**
- Constant-time crypto operations mentioned (cryptography.md)
- Memory locking for crypto core

**Gap:**
- No explicit power analysis protection
- No timing attack protection beyond crypto primitives
- No mention of cache-timing attack protection

**Hardening Recommendation:**

```
IMPLEMENT: Comprehensive side-channel protection

Side-Channel Mitigations:

1. Timing attacks
   - Constant-time comparison for all secret-dependent operations
   - Already using ring::constant_time::verify_slices_are_equal (good!)
   - Extend to wallet operations: address comparison, amount validation

2. Power analysis (hardware wallets only, but good practice)
   - Randomize order of operations where possible
   - Add dummy operations to equalize power traces
   - Use blinding techniques for key operations

3. Cache-timing attacks
   - Flush caches after sensitive operations
   - Use cache-oblivious algorithms for key handling
   - Avoid secret-dependent memory access patterns

4. Acoustic/electromagnetic emanations
   - Limited practical defense, but:
   - Perform sensitive ops in quick bursts (reduce signal time)
   - Add random delays to desynchronize operations from external clock
```

---

### 8. Multi-Signature Wallet Hardening

**Current State:**
- "Multi-sig option: 2-of-3 signatures for large transactions (configurable threshold)"

**Gap:**
- No details on multi-sig implementation security
- How are co-signers coordinated?
- How to prevent malicious co-signer attacks?

**Hardening Recommendation:**

```
IMPLEMENT: Secure multi-sig coordination

Multi-Sig Security Model:

1. Setup phase
   - Each co-signer generates their own key independently
   - Public keys exchanged through Invisible messenger (verified identity)
   - Multi-sig address derived collaboratively
   - Each party verifies the multi-sig script independently

2. Transaction signing
   - Proposer creates unsigned transaction, sends to co-signers via Invisible
   - Each co-signer independently verifies transaction details
   - Co-signers sign with their own keys (never share private keys)
   - Signatures collected and combined

3. Co-signer verification
   - Each signature verified against expected co-signer identity
   - Detect if wrong co-signer tries to sign
   - Require all co-signers to have verified safety numbers

4. Time locks & spending limits
   - Optional: time lock for large transactions (24-hour delay)
   - Optional: daily spending limit (small payments single-sig, large multi-sig)
   - Optional: emergency recovery key (M-of-N+1 with trusted recovery contact)
```

---

### 9. Privacy Coin Integration Hardening

**Current State:**
- Monero: Ring signatures, stealth addresses, RingCT
- Zcash: zk-SNARKs, shielded only
- Bitcoin: CoinJoin/PayJoin + Silent Payments

**Gap:**
- No mention of Monero view key security
- No mention of Zcash viewing key handling
- No details on CoinJoin coordinator trust model

**Hardening Recommendation:**

```
ENHANCE: Privacy coin specific protections

Monero:
1. View key protection
   - View key allows seeing incoming transactions (not spending)
   - Encrypted separately from spend key
   - Optional: share view key with auditor (read-only access)
   - Never transmit view key over network

2. Subaddress usage
   - Generate unique subaddress per payment
   - Never reuse subaddresses (prevents address clustering)
   - Track subaddress->contact mapping locally

3. Ring signature size
   - Use maximum ring size supported (currently 16, soon FCMP++)
   - Larger ring = better anonymity set

Zcash:
1. Shielded-only enforcement
   - Block all transparent address interactions (already mentioned)
   - Warn if counterparty uses transparent address
   - Refuse to generate t-addresses

2. Viewing key handling
   - Separate viewing key from spending key
   - Allow read-only wallet on separate device

Bitcoin Privacy Enhancements:
1. CoinJoin automation
   - Auto-CoinJoin for all incoming funds (optional)
   - Use multiple CoinJoin coordinators (avoid single point)
   - Verify coordinator through Scrambler (coordinator can't see IP)

2. Silent Payments enforcement
   - Always use Silent Payments for receiving (BIP-352)
   - Never display static address to sender
   - One-time address per payment

3. Coin control
   - Advanced users can manually select UTXOs
   - Prevent address clustering by selective spending
   - Label UTXOs by source for privacy tracking
```

---

### 10. Transaction Broadcast Security

**Current State:**
- Multi-node broadcast to N nodes (default 5)
- Independent Scrambler paths per node
- First-seen attack mitigation

**Gap:**
- What if all N nodes are malicious/compromised?
- No verification that transaction was actually broadcast
- No fallback if broadcast fails

**Hardening Recommendation:**

```
IMPLEMENT: Broadcast verification & fallback

Enhanced Broadcast Security:

1. Broadcast confirmation
   - After multi-node broadcast, verify tx appeared in mempool
   - Query mempool from different set of nodes (not broadcast targets)
   - Confirm tx propagated to at least M of N query nodes (M=3, N=5)

2. Broadcast diversity
   - Select broadcast nodes from different operators/jurisdictions
   - Avoid selecting nodes run by same entity
   - Use node reputation system (track successful broadcasts)

3. Fallback mechanism
   - If broadcast fails, retry with different set of nodes
   - After 3 failed attempts, alert user
   - Option to broadcast through alternative channel (Tor, I2P)

4. Double-spend detection
   - Monitor for conflicting transactions
   - Alert if multiple txs spending same inputs detected
   - Possible attack: malicious node broadcasts modified tx
```

---

## Priority Ranking

### P0 - Critical (Implement Immediately)

1. **Hardware Security Module integration** (Secure Enclave/StrongBox for keys)
2. **Transaction verification before signing** (address validation, amount verification, replay protection)
3. **Memory protection** (mlock, memory encryption, secure zeroing)

### P1 - High Priority

4. **Phishing protection** (identity verification, payment request authentication)
5. **Smart contract security** (approval limits, simulation, contract verification)
6. **Multi-sig hardening** (secure coordination, co-signer verification)

### P2 - Medium Priority

7. **Backup improvements** (Shamir seed backup, social recovery)
8. **Privacy coin enhancements** (view key protection, auto-CoinJoin)
9. **Broadcast verification** (confirm propagation, fallback)

### P3 - Lower Priority (Defense in Depth)

10. **Side-channel protections** (timing, cache, power analysis)

---

## Implementation Roadmap

### Phase 1: Foundation (Weeks 1-4)
- Hardware security module integration
- Transaction verification framework
- Memory protection hardening

### Phase 2: User-Facing Security (Weeks 5-8)
- Phishing protection UI
- Multi-sig coordination
- Smart contract safety

### Phase 3: Advanced Features (Weeks 9-12)
- Shamir seed backup
- Enhanced privacy coin support
- Broadcast verification

### Phase 4: Defense in Depth (Weeks 13-16)
- Side-channel protections
- Security monitoring & alerting
- Comprehensive security audit

---

## Comparison: Messenger vs Wallet Security

| Security Layer | Messenger | Shadow Wallet | Gap |
|---|---|---|---|
| **Network Anonymity** | 7-layer Scrambler | ✓ Full parity | None |
| **End-to-End Encryption** | Double Ratchet + PQ | N/A (crypto = public ledger) | Expected |
| **Key Storage** | Encrypted DB | Encrypted DB | **→ Add HSM** |
| **Hardware Protection** | DB key in enclave | DB key in enclave | **→ Keys in enclave** |
| **Memory Protection** | ✓ mlock for crypto | Mentioned | **→ Extend to wallet** |
| **Transaction Verification** | Message auth | Basic | **→ Comprehensive checks** |
| **Phishing Protection** | Safety numbers | Safety numbers | **→ Payment-specific** |
| **Backup/Recovery** | None (device-only) | Seed phrase | **→ Shamir backup** |
| **Auto-Purge** | ✓ 24h-90d | ✓ Transaction history | Equivalent |

---

## Conclusion

The Shadow Wallet **already has excellent network-level protection** through full Scrambler integration. The remaining gaps are primarily at the **application and cryptographic layers**:

1. **HSM integration** - Move from software-encrypted keys to hardware-protected keys
2. **Transaction verification** - Prevent user errors and attacks before signing
3. **Memory hardening** - Protect keys in runtime memory, not just at rest
4. **Smart contract security** - Specific protections for DeFi interactions
5. **Enhanced backup** - Shamir Secret Sharing for seed redundancy

Implementing the P0 and P1 recommendations would bring the Shadow Wallet to a **security posture that equals or exceeds** the messenger, with the network anonymity already at parity and the application-layer security elevated to best-in-class.
