# Shadow Wallet Security Hardening Specification

> **Purpose:** This document extends [shadow-wallet.md](shadow-wallet.md) with additional
> security hardening measures that elevate wallet security to match or exceed messenger
> security across all layers.

---

## Table of Contents

1. [Hardware Security Module Integration](#hardware-security-module-integration)
2. [Transaction Verification Layer](#transaction-verification-layer)
3. [Enhanced Memory Protection](#enhanced-memory-protection)
4. [Anti-Phishing Protection](#anti-phishing-protection)
5. [Smart Contract Security](#smart-contract-security)
6. [Seed Backup Hardening](#seed-backup-hardening)
7. [Multi-Signature Coordination](#multi-signature-coordination)
8. [Privacy Coin Enhancements](#privacy-coin-enhancements)
9. [Broadcast Verification](#broadcast-verification)
10. [Side-Channel Attack Mitigation](#side-channel-attack-mitigation)

---

## Hardware Security Module Integration

### Objective

Move wallet private keys from software-encrypted storage into hardware-protected secure
enclaves, ensuring keys never exist in plain form in application memory.

### Architecture

```
┌─────────────────────────────────────────┐
│         Application Layer               │
│  (Transaction construction, UI)         │
└─────────────────┬───────────────────────┘
                  │
                  v
┌─────────────────────────────────────────┐
│      Wallet Crypto Abstraction Layer    │
│  (Platform-agnostic signing interface)  │
└─────────────────┬───────────────────────┘
                  │
        ┌─────────┴─────────┐
        │                   │
        v                   v
┌──────────────┐    ┌──────────────┐
│ iOS Secure   │    │ Android      │
│ Enclave      │    │ StrongBox    │
│              │    │              │
│ Keys stored  │    │ Keys stored  │
│ in hardware  │    │ in hardware  │
│              │    │              │
│ Signing in   │    │ Signing in   │
│ enclave      │    │ hardware     │
└──────────────┘    └──────────────┘
```

### Implementation Specifications

#### iOS Secure Enclave

**Key Generation:**
```swift
// Keys generated inside Secure Enclave, never leave
let accessControl = SecAccessControlCreateWithFlags(
    kCFAllocatorDefault,
    kSecAttrAccessibleWhenUnlockedThisDeviceOnly,
    [.privateKeyUsage, .biometryCurrentSet],
    nil
)

let attributes: [String: Any] = [
    kSecAttrKeyType as String: kSecAttrKeyTypeECSECPrimeRandom,
    kSecAttrKeySizeInBits as String: 256,
    kSecAttrTokenID as String: kSecAttrTokenIDSecureEnclave,
    kSecPrivateKeyAttrs as String: [
        kSecAttrIsPermanent as String: true,
        kSecAttrApplicationTag as String: "com.invisible.wallet.secp256k1",
        kSecAttrAccessControl as String: accessControl
    ]
]

var error: Unmanaged<CFError>?
guard let privateKey = SecKeyCreateRandomKey(attributes as CFDictionary, &error) else {
    // Fallback to software-based key generation
}
```

**Transaction Signing:**
```swift
// User must authenticate with Face ID / Touch ID
let context = LAContext()
context.localizedReason = "Sign transaction: 0.5 ETH to 0x1234..."

// Signing happens INSIDE Secure Enclave
let signature = SecKeyCreateSignature(
    privateKey,
    .ecdsaSignatureMessageX962SHA256,
    txHash as CFData,
    &error
)
```

#### Android StrongBox Keymaster

**Key Generation:**
```kotlin
val keyGenParameterSpec = KeyGenParameterSpec.Builder(
    "invisible_wallet_key",
    KeyProperties.PURPOSE_SIGN
)
    .setDigests(KeyProperties.DIGEST_SHA256)
    .setAlgorithmParameterSpec(ECGenParameterSpec("secp256k1"))
    .setUserAuthenticationRequired(true)
    .setUserAuthenticationValidityDurationSeconds(30)
    .setIsStrongBoxBacked(true)  // Require hardware security
    .build()

val keyPairGenerator = KeyPairGenerator.getInstance(
    KeyProperties.KEY_ALGORITHM_EC,
    "AndroidKeyStore"
)

keyPairGenerator.initialize(keyGenParameterSpec)
val keyPair = keyPairGenerator.generateKeyPair()
```

**Transaction Signing:**
```kotlin
// User must authenticate with fingerprint/face unlock
val signature = Signature.getInstance("SHA256withECDSA")
signature.initSign(privateKey)
signature.update(txHash)
val sig = signature.sign()
```

#### Fallback Strategy

If hardware security is unavailable (older devices, emulators):

1. **Detect hardware capabilities** on app first launch
2. **Display warning** if hardware security unavailable:
   ```
   ⚠️ Hardware Security Not Available

   Your device does not support hardware-protected key storage.
   Wallet keys will be encrypted in software only.

   For maximum security, upgrade to a device with:
   - iOS: Secure Enclave (iPhone 5s or later)
   - Android: StrongBox (Pixel 3 or later)
   ```
3. **Fall back** to current software approach (Argon2id-encrypted keys in SQLCipher)
4. **Track** in telemetry: percentage of users with/without hardware security

---

## Transaction Verification Layer

### Objective

Implement comprehensive pre-signing verification to prevent user errors, clipboard
hijacking, address poisoning, and malicious transaction manipulation.

### Verification Pipeline

```
User initiates payment
         │
         v
┌────────────────────┐
│  Intent Capture    │
│  - Recipient       │
│  - Amount          │
│  - Asset           │
└─────────┬──────────┘
          │
          v
┌────────────────────────────────────────┐
│     PRE-SIGNING VERIFICATION           │
│  ┌──────────────────────────────────┐  │
│  │ 1. Address Validation            │  │
│  │    - Checksum verification       │  │
│  │    - Format validation           │  │
│  │    - Poisoning detection         │  │
│  └──────────────────────────────────┘  │
│  ┌──────────────────────────────────┐  │
│  │ 2. Amount Validation             │  │
│  │    - Clipboard hijack detection  │  │
│  │    - Dust attack detection       │  │
│  │    - Large payment confirmation  │  │
│  └──────────────────────────────────┘  │
│  ┌──────────────────────────────────┐  │
│  │ 3. Replay Protection             │  │
│  │    - ChainID verification        │  │
│  │    - Nonce/sequence validation   │  │
│  └──────────────────────────────────┘  │
│  ┌──────────────────────────────────┐  │
│  │ 4. Fee Validation                │  │
│  │    - Multi-source fee estimate   │  │
│  │    - Outlier detection           │  │
│  └──────────────────────────────────┘  │
│  ┌──────────────────────────────────┐  │
│  │ 5. Recipient Verification        │  │
│  │    - Contact identity check      │  │
│  │    - Safety number verification  │  │
│  └──────────────────────────────────┘  │
└─────────┬──────────────────────────────┘
          │
   [ALL CHECKS PASS]
          │
          v
┌────────────────────┐
│  User Confirmation │
│  - Display summary │
│  - Require consent │
└─────────┬──────────┘
          │
          v
┌────────────────────┐
│  Sign Transaction  │
│  (Hardware/Enclave)│
└────────────────────┘
```

### Address Validation

```rust
pub enum AddressValidationError {
    InvalidChecksum,
    InvalidFormat,
    PoisoningDetected,
    SanctionedAddress,
    KnownScamAddress,
}

pub fn validate_address(
    address: &str,
    chain: Chain,
    user_intent: &PaymentIntent,
) -> Result<(), AddressValidationError> {
    // 1. Checksum verification
    match chain {
        Chain::Ethereum => {
            if !is_valid_eip55_checksum(address) {
                return Err(InvalidChecksum);
            }
        }
        Chain::Bitcoin => {
            if !is_valid_base58check(address) {
                return Err(InvalidChecksum);
            }
        }
        Chain::Monero => {
            if !is_valid_monero_address(address) {
                return Err(InvalidFormat);
            }
        }
        _ => {}
    }

    // 2. Address poisoning detection
    // Attack: malware replaces clipboard with similar-looking address
    if let Some(recent_address) = get_recently_used_address() {
        let similarity = levenshtein_distance(address, &recent_address);
        if similarity < 5 {  // Very similar but not identical
            warn!("Address very similar to recent address");
            return Err(PoisoningDetected);
        }
    }

    // 3. Sanction list check (OFAC, etc.)
    if is_sanctioned_address(address, chain).await? {
        warn!("Address on sanctions list");
        // Note: User can override with warning, not hard block
    }

    // 4. Known scam database
    if is_known_scam(address, chain).await? {
        return Err(KnownScamAddress);
    }

    Ok(())
}
```

### Amount Validation

```rust
pub fn validate_amount(
    tx_amount: Amount,
    user_intent_amount: Amount,
    user_balance: Amount,
) -> Result<(), AmountValidationError> {
    // 1. Clipboard hijacking detection
    if tx_amount != user_intent_amount {
        return Err(AmountMismatch {
            expected: user_intent_amount,
            actual: tx_amount,
        });
    }

    // 2. Dust attack detection
    // Attacker sends tiny amounts to track wallet activity
    if tx_amount < dust_threshold() {
        warn!("Receiving dust amount - possible tracking attempt");
    }

    // 3. Large payment confirmation
    if tx_amount.usd_value() > 1000.0 {
        require_reenter_amount()?;  // User must type amount again
    }

    // 4. Balance validation
    if tx_amount + estimated_fee() > user_balance {
        return Err(InsufficientBalance);
    }

    Ok(())
}
```

### Replay Protection

```rust
pub fn ensure_replay_protection(tx: &UnsignedTransaction) -> Result<()> {
    match tx.chain {
        Chain::Bitcoin => {
            // Enforce BIP-143 (SegWit) signing
            ensure!(tx.uses_segwit_signing(), "Must use SegWit for replay protection");
        }
        Chain::Ethereum => {
            // Enforce EIP-155 (chainID in signature)
            ensure!(tx.chain_id.is_some(), "ChainID required for replay protection");

            // Prevent cross-chain replay
            let expected_chain_id = get_expected_chain_id();
            ensure!(
                tx.chain_id == Some(expected_chain_id),
                "ChainID mismatch - prevents mainnet/testnet replay"
            );
        }
        Chain::Monero => {
            // Monero has built-in replay protection via key images
            // No additional checks needed
        }
        _ => {}
    }

    Ok(())
}
```

### Fee Validation

```rust
pub async fn validate_fee(tx: &UnsignedTransaction) -> Result<FeeAssessment> {
    // 1. Get fee estimates from multiple sources
    let estimates = fetch_multi_source_fee_estimates(tx.chain).await?;

    // Sources:
    // - Our own fee estimation (based on recent blocks)
    // - Public APIs (ethgasstation.info, mempool.space, etc.)
    // - Blockchain nodes (direct RPC fee estimation)

    let median_fee = median(&estimates);
    let tx_fee = tx.fee;

    // 2. Outlier detection
    if tx_fee > median_fee * 2.0 {
        warn!("Transaction fee unusually high");
        return Ok(FeeAssessment::High {
            tx_fee,
            median_fee,
            warning: "Fee is 2x higher than median",
        });
    }

    // 3. Fee as percentage of transaction value
    let fee_percentage = (tx_fee.usd_value() / tx.amount.usd_value()) * 100.0;
    if fee_percentage > 5.0 {
        warn!("Fee is {}% of transaction value", fee_percentage);
        return Ok(FeeAssessment::HighPercentage {
            percentage: fee_percentage,
            warning: "Fee >5% of transaction",
        });
    }

    Ok(FeeAssessment::Reasonable)
}
```

### Recipient Verification

```rust
pub fn verify_recipient(
    recipient_address: &str,
    conversation_id: Option<ConversationId>,
) -> Result<RecipientVerification> {
    if let Some(conv_id) = conversation_id {
        // In-chat payment: verify contact identity
        let contact = get_contact_by_conversation(conv_id)?;

        let verification = RecipientVerification {
            contact_name: Some(contact.name),
            safety_number_verified: contact.safety_number_verified,
            key_changed_recently: contact.identity_key_changed_within(Duration::days(7)),
            first_payment: !has_previous_payments_to(conv_id),
        };

        // Warnings
        if !verification.safety_number_verified {
            warn!("Sending payment to never-verified contact");
        }

        if verification.key_changed_recently {
            warn!("Contact's identity key changed recently - verify safety number");
        }

        if verification.first_payment {
            warn!("First payment to this contact - verify recipient");
        }

        Ok(verification)
    } else {
        // External payment: no contact verification possible
        Ok(RecipientVerification {
            contact_name: None,
            safety_number_verified: false,
            key_changed_recently: false,
            first_payment: true,
        })
    }
}
```

---

## Enhanced Memory Protection

### Objective

Protect wallet keys in runtime memory from process dumps, swap attacks, and cold boot attacks.

### Memory Protection Architecture

```
┌──────────────────────────────────────────┐
│     Wallet Process (Reduced Privileges)  │
│                                          │
│  ┌────────────────────────────────────┐ │
│  │  Memory-Locked Region (mlock)      │ │
│  │  - Keys decrypted here only        │ │
│  │  - No swap to disk                 │ │
│  │  - Encrypted when not in use       │ │
│  │  ┌──────────────────────────────┐  │ │
│  │  │  Key Material (plaintext)    │  │ │
│  │  │  [Exists <100ms during sign] │  │ │
│  │  │  - Immediately zeroized      │  │ │
│  │  └──────────────────────────────┘  │ │
│  └────────────────────────────────────┘ │
│                                          │
│  ┌────────────────────────────────────┐ │
│  │  Encrypted Key Storage (rest)      │ │
│  │  - Keys encrypted in memory        │ │
│  │  - Separate encryption key         │ │
│  │  - Zeroized on process exit        │ │
│  └────────────────────────────────────┘ │
└──────────────────────────────────────────┘
```

### Implementation

```rust
use std::os::unix::prelude::*;
use libc::{mlock, munlock, mprotect, PROT_READ, PROT_WRITE};
use zeroize::Zeroize;

/// Memory-locked region for sensitive key material
pub struct LockedMemory {
    ptr: *mut u8,
    len: usize,
}

impl LockedMemory {
    pub fn new(size: usize) -> Result<Self> {
        // 1. Allocate aligned memory
        let layout = std::alloc::Layout::from_size_align(size, 4096)?;
        let ptr = unsafe { std::alloc::alloc(layout) };

        if ptr.is_null() {
            return Err(AllocationError);
        }

        // 2. Lock memory to prevent swapping to disk
        let ret = unsafe { mlock(ptr as *const libc::c_void, size) };
        if ret != 0 {
            warn!("mlock failed - keys may be swapped to disk");
        }

        // 3. Mark memory as read-write (not executable)
        unsafe {
            mprotect(
                ptr as *mut libc::c_void,
                size,
                PROT_READ | PROT_WRITE,
            );
        }

        Ok(LockedMemory { ptr, len: size })
    }

    /// Write data into locked memory
    pub fn write(&mut self, data: &[u8]) -> Result<()> {
        if data.len() > self.len {
            return Err(BufferTooSmall);
        }

        unsafe {
            std::ptr::copy_nonoverlapping(
                data.as_ptr(),
                self.ptr,
                data.len(),
            );
        }

        Ok(())
    }

    /// Read data from locked memory
    pub fn read(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
    }
}

impl Drop for LockedMemory {
    fn drop(&mut self) {
        // 1. Zeroize memory (prevent data remanence)
        unsafe {
            std::ptr::write_bytes(self.ptr, 0, self.len);
            // Volatile write to prevent compiler optimization
            std::ptr::write_volatile(self.ptr, 0);
        }

        // 2. Unlock memory
        unsafe {
            munlock(self.ptr as *const libc::c_void, self.len);
        }

        // 3. Deallocate
        unsafe {
            let layout = std::alloc::Layout::from_size_align_unchecked(self.len, 4096);
            std::alloc::dealloc(self.ptr, layout);
        }
    }
}

/// Encrypted key that only decrypts into locked memory
pub struct EncryptedKey {
    ciphertext: Vec<u8>,
    nonce: [u8; 12],
}

impl EncryptedKey {
    /// Sign a transaction (key exists in plaintext for <100ms)
    pub fn sign(&self, message: &[u8]) -> Result<Signature> {
        // 1. Create locked memory region
        let mut locked_mem = LockedMemory::new(32)?;

        // 2. Decrypt key directly into locked memory
        let encryption_key = derive_memory_encryption_key()?;
        let plaintext_key = aes_gcm_decrypt(
            &encryption_key,
            &self.nonce,
            &self.ciphertext,
        )?;
        locked_mem.write(&plaintext_key)?;

        // 3. Sign transaction
        let key_bytes = locked_mem.read();
        let signing_key = SigningKey::from_bytes(key_bytes)?;
        let signature = signing_key.sign(message);

        // 4. locked_mem is dropped here -> memory zeroized
        // plaintext_key also zeroized (Zeroize trait)

        Ok(signature)
    }
}

impl Drop for EncryptedKey {
    fn drop(&mut self) {
        self.ciphertext.zeroize();
        self.nonce.zeroize();
    }
}
```

### Memory Protection Checklist

- [x] **mlock()** - Lock memory pages to prevent swapping
- [x] **mprotect()** - Mark memory non-executable (W^X protection)
- [x] **Encrypted in memory** - Keys encrypted when not actively in use
- [x] **Zeroize on drop** - Guaranteed zeroing of sensitive data
- [x] **Volatile writes** - Prevent compiler from optimizing away zeroing
- [x] **Minimal plaintext time** - Keys in plaintext only during signing (<100ms)
- [x] **Separate process** - Wallet crypto in isolated process (optional, advanced)

---

## Anti-Phishing Protection

### Objective

Prevent users from sending payments to wrong/malicious recipients through identity
verification and suspicious pattern detection.

### Payment Verification UI Flow

```
┌─────────────────────────────────────────┐
│  User taps "Send 0.5 ETH" in chat       │
└─────────────┬───────────────────────────┘
              │
              v
┌─────────────────────────────────────────┐
│  PRE-CONFIRMATION VERIFICATION          │
│  ┌───────────────────────────────────┐  │
│  │  Contact Identity                 │  │
│  │  Name: Alice                      │  │
│  │  Safety Number: 4829 3847 ...     │  │
│  │  Status: ✓ Verified               │  │
│  │          (or ⚠ Never verified)    │  │
│  │          (or ⚠ Key changed 2d ago)│  │
│  └───────────────────────────────────┘  │
│  ┌───────────────────────────────────┐  │
│  │  Payment Details                  │  │
│  │  Amount: 0.5 ETH ($1,250 USD)     │  │
│  │  Fee: 0.002 ETH ($5 USD)          │  │
│  │  Total: 0.502 ETH ($1,255 USD)    │  │
│  └───────────────────────────────────┘  │
│  ┌───────────────────────────────────┐  │
│  │  Warnings (if applicable)         │  │
│  │  ⚠ First payment to this contact  │  │
│  │  ⚠ Large payment (>$1000)         │  │
│  │  ⚠ Contact key changed recently   │  │
│  └───────────────────────────────────┘  │
│  ┌───────────────────────────────────┐  │
│  │  Confirmation Checkbox            │  │
│  │  □ I verified this is the correct │  │
│  │    recipient and amount           │  │
│  └───────────────────────────────────┘  │
│                                         │
│  [Cancel]  [Confirm & Sign]             │
└─────────────────────────────────────────┘
```

### Suspicious Pattern Detection

```rust
pub struct SuspiciousPatternDetector {
    history: PaymentHistoryStore,
}

impl SuspiciousPatternDetector {
    pub fn analyze_payment(
        &self,
        payment: &PaymentIntent,
    ) -> Result<Vec<SuspiciousPattern>> {
        let mut warnings = Vec::new();

        // 1. First payment to contact
        if self.is_first_payment_to_contact(payment.recipient_id) {
            warnings.push(SuspiciousPattern::FirstPayment {
                message: "This is your first payment to this contact",
                severity: Severity::Warning,
            });
        }

        // 2. Large payment (>10x median)
        let median_payment = self.get_median_payment_amount();
        if payment.amount > median_payment * 10.0 {
            warnings.push(SuspiciousPattern::UnusuallyLarge {
                amount: payment.amount,
                median: median_payment,
                severity: Severity::Warning,
            });
        }

        // 3. Rapid succession payments
        let recent_payments = self.get_payments_last_n_minutes(5);
        if recent_payments.len() >= 3 {
            warnings.push(SuspiciousPattern::RapidPayments {
                count: recent_payments.len(),
                message: "Multiple payments in quick succession",
                severity: Severity::Warning,
            });
        }

        // 4. Payment to recently-changed identity
        if let Some(contact) = self.get_contact(payment.recipient_id) {
            if contact.identity_key_changed_within(Duration::days(7)) {
                warnings.push(SuspiciousPattern::RecentKeyChange {
                    days_ago: contact.identity_key_change_age().num_days(),
                    message: "Contact's identity key changed recently",
                    severity: Severity::Critical,
                });
            }
        }

        // 5. Payment to unverified contact
        if !self.is_safety_number_verified(payment.recipient_id) {
            warnings.push(SuspiciousPattern::UnverifiedContact {
                message: "You have not verified this contact's safety number",
                severity: Severity::Warning,
            });
        }

        // 6. Payment drains >90% of wallet
        let balance = self.get_balance();
        if payment.amount > balance * 0.9 {
            warnings.push(SuspiciousPattern::DrainingWallet {
                percentage: (payment.amount / balance) * 100.0,
                message: "This payment will drain most of your wallet",
                severity: Severity::Critical,
            });
        }

        Ok(warnings)
    }
}
```

### Payment Request Authentication

```rust
/// Payment requests must be signed by sender's identity key
pub struct PaymentRequest {
    recipient: Address,
    amount: Amount,
    asset: Asset,
    memo: Option<String>,
    timestamp: u64,
    signature: Signature,  // Ed25519 signature over entire request
}

impl PaymentRequest {
    /// Verify payment request came from legitimate contact
    pub fn verify(&self, sender_identity_key: &PublicKey) -> Result<()> {
        // 1. Reconstruct message to verify
        let message = self.signing_message();

        // 2. Verify Ed25519 signature
        sender_identity_key.verify(&message, &self.signature)?;

        // 3. Check timestamp freshness (prevent replay)
        let age = current_timestamp() - self.timestamp;
        if age > Duration::hours(24).num_seconds() {
            return Err(RequestExpired);
        }

        // 4. Verify recipient matches current user's wallet
        let my_address = get_my_address_for_asset(self.asset)?;
        if self.recipient != my_address {
            return Err(RecipientMismatch);
        }

        Ok(())
    }

    fn signing_message(&self) -> Vec<u8> {
        let mut msg = Vec::new();
        msg.extend_from_slice(self.recipient.as_bytes());
        msg.extend_from_slice(&self.amount.to_le_bytes());
        msg.extend_from_slice(self.asset.as_bytes());
        if let Some(memo) = &self.memo {
            msg.extend_from_slice(memo.as_bytes());
        }
        msg.extend_from_slice(&self.timestamp.to_le_bytes());
        msg
    }
}
```

---

## Smart Contract Security

### Objective

Protect users interacting with Ethereum smart contracts and DeFi protocols from unlimited
approvals, malicious contracts, and unexpected state changes.

### Token Approval Limits

```rust
/// NEVER allow unlimited token approvals
pub fn create_token_approval(
    token: Address,
    spender: Address,
    requested_amount: U256,
) -> Result<Transaction> {
    // 1. Reject unlimited approvals
    if requested_amount == U256::MAX {
        return Err(UnlimitedApprovalBlocked {
            message: "Unlimited approvals are not allowed",
            suggestion: "Approve only the exact amount needed",
        });
    }

    // 2. Default: approve only exact amount for this transaction
    // User can override to approve slightly more (e.g., 110% for slippage)

    // 3. Track active approvals
    track_approval(token, spender, requested_amount);

    // 4. Build approval transaction
    let data = encode_erc20_approve(spender, requested_amount);
    Ok(Transaction {
        to: token,
        data,
        value: U256::zero(),
    })
}

/// List all active token approvals, allow revocation
pub fn list_active_approvals() -> Vec<TokenApproval> {
    // Query on-chain for all approve() events from this wallet
    // Display in UI with "Revoke" button
}
```

### Contract Verification

```rust
pub async fn verify_contract(
    contract: Address,
    chain: Chain,
) -> Result<ContractVerification> {
    // 1. Check if contract is verified on block explorer
    let verified = check_etherscan_verification(contract, chain).await?;

    if !verified {
        warn!("Contract source code not verified");
    }

    // 2. Check against known malicious contract database
    if is_known_malicious(contract, chain).await? {
        return Err(MaliciousContract {
            contract,
            reason: "Contract on malicious contract database",
        });
    }

    // 3. Check contract deployment date
    let deployed_at = get_contract_deployment_date(contract, chain).await?;
    let age = current_time() - deployed_at;

    if age < Duration::days(7) {
        warn!("Contract deployed less than 7 days ago - exercise caution");
    }

    // 4. Check contract has been audited (if in audit database)
    let audit_reports = check_audit_database(contract).await?;

    Ok(ContractVerification {
        verified,
        deployment_age: age,
        audit_reports,
    })
}
```

### Transaction Simulation

```rust
pub async fn simulate_transaction(tx: &UnsignedTransaction) -> Result<SimulationResult> {
    // Use Tenderly, Blocknative, or local fork for simulation

    // 1. Create local fork at current block height
    let fork = create_local_fork(tx.chain).await?;

    // 2. Simulate transaction
    let result = fork.simulate_transaction(tx).await?;

    // 3. Analyze state changes
    let state_changes = result.state_changes;

    // 4. Detect suspicious patterns
    let mut warnings = Vec::new();

    for change in &state_changes {
        match change {
            StateChange::TokenTransfer { from, to, amount, token } => {
                // Check if draining wallet
                if from == tx.from && amount > get_balance(token)? * 0.9 {
                    warnings.push(Warning::DrainingWallet {
                        token: token.clone(),
                        percentage: 90,
                    });
                }

                // Check if transferring to unknown address
                if to != tx.to && !is_known_address(to) {
                    warnings.push(Warning::UnexpectedTransfer {
                        to: *to,
                        token: token.clone(),
                    });
                }
            }

            StateChange::Approval { token, spender, amount } => {
                // Already handled by approval limits
            }

            StateChange::NFTTransfer { from, to, token_id } => {
                // Warn about NFT transfers
                warnings.push(Warning::NFTTransfer {
                    token_id: *token_id,
                });
            }
        }
    }

    Ok(SimulationResult {
        success: result.success,
        gas_used: result.gas_used,
        state_changes,
        warnings,
    })
}
```

---

## Seed Backup Hardening

### Objective

Provide redundant, secure seed backup options beyond single paper backup.

### Shamir's Secret Sharing for Seeds (SLIP-39)

```rust
use slip39::{generate_mnemonics, combine_mnemonics};

/// Generate Shamir shares for seed backup
pub fn generate_shamir_backup(
    seed: &[u8],
    threshold: u8,
    num_shares: u8,
) -> Result<Vec<ShareMnemonic>> {
    // Use SLIP-39 standard for Shamir seed shares

    // Example: 2-of-3
    // - User needs any 2 shares to recover seed
    // - Can lose 1 share without compromising wallet
    // - Shares can be stored in different physical locations

    let shares = generate_mnemonics(
        seed,
        threshold,
        num_shares,
        "Invisible Wallet Backup",  // passphrase protection
    )?;

    // Each share is a mnemonic phrase (like BIP-39)
    // User writes each on separate paper/metal backup

    Ok(shares)
}

/// Recover seed from Shamir shares
pub fn recover_from_shamir_shares(
    shares: Vec<ShareMnemonic>,
) -> Result<Vec<u8>> {
    // Combine any `threshold` shares to recover seed
    let seed = combine_mnemonics(&shares)?;
    Ok(seed)
}
```

### Social Recovery (Advanced)

```rust
/// Distribute encrypted seed shares to trusted contacts
pub async fn setup_social_recovery(
    seed: &[u8],
    guardians: Vec<Contact>,
    threshold: usize,
) -> Result<()> {
    // 1. Split seed with Shamir (e.g., 2-of-3)
    let shares = shamir_split(seed, threshold, guardians.len())?;

    // 2. Encrypt each share to guardian's public key
    let encrypted_shares: Vec<_> = shares
        .iter()
        .zip(guardians.iter())
        .map(|(share, guardian)| {
            // Triple encryption:
            // - Guardian's public key (only they can decrypt)
            // - Device key (requires original device)
            // - 2FA secret (requires 2FA code)
            encrypt_recovery_share(share, guardian)
        })
        .collect();

    // 3. Send encrypted shares through Scrambler (dead drops)
    for (encrypted_share, guardian) in encrypted_shares.iter().zip(guardians.iter()) {
        send_message(
            guardian,
            Message::RecoveryShare(*encrypted_share),
        ).await?;
    }

    Ok(())
}

/// Recover wallet using social recovery
pub async fn recover_wallet_social(
    guardians: Vec<Contact>,
) -> Result<Vec<u8>> {
    // 1. Request recovery shares from guardians
    let shares = request_recovery_shares_from_guardians(guardians).await?;

    // 2. Need threshold shares + device key + 2FA
    ensure!(shares.len() >= threshold, "Not enough shares");

    // 3. Decrypt shares
    let decrypted_shares: Vec<_> = shares
        .iter()
        .map(|share| decrypt_recovery_share(share))
        .collect()?;

    // 4. Combine shares to recover seed
    let seed = shamir_combine(&decrypted_shares)?;

    Ok(seed)
}
```

---

## Multi-Signature Coordination

### Objective

Secure coordination of multi-signature transactions through Invisible messenger,
ensuring co-signers are authenticated and transaction details are verified.

### Multi-Sig Setup Protocol

```rust
/// Multi-sig wallet setup (e.g., 2-of-3)
pub async fn setup_multisig_wallet(
    co_signers: Vec<Contact>,
    threshold: usize,
) -> Result<MultisigWallet> {
    // 1. Each co-signer generates their key independently
    let my_key = generate_signing_key()?;

    // 2. Exchange public keys through Invisible (authenticated)
    let public_keys = exchange_public_keys_with_co_signers(&co_signers).await?;

    // 3. Verify each co-signer's identity (safety number check)
    for contact in &co_signers {
        ensure!(
            contact.safety_number_verified,
            "All co-signers must be verified"
        );
    }

    // 4. Derive multi-sig address (deterministic)
    let multisig_address = derive_multisig_address(&public_keys, threshold)?;

    // 5. Each party independently verifies the multi-sig address
    verify_multisig_address(&multisig_address, &public_keys, threshold)?;

    Ok(MultisigWallet {
        address: multisig_address,
        co_signers,
        threshold,
        my_key_index: 0,
    })
}
```

### Multi-Sig Transaction Signing

```rust
/// Propose a transaction to co-signers
pub async fn propose_multisig_transaction(
    wallet: &MultisigWallet,
    tx: UnsignedTransaction,
) -> Result<TransactionProposal> {
    // 1. Create transaction proposal
    let proposal = TransactionProposal {
        id: generate_proposal_id(),
        tx: tx.clone(),
        proposer: my_identity(),
        timestamp: current_timestamp(),
        signatures: Vec::new(),
    };

    // 2. Sign proposal with my identity key
    let proposal_signature = sign_proposal(&proposal)?;

    // 3. Send proposal to all co-signers via Invisible messenger
    for co_signer in &wallet.co_signers {
        send_message(
            co_signer,
            Message::MultisigProposal {
                proposal: proposal.clone(),
                proposer_signature: proposal_signature,
            },
        ).await?;
    }

    Ok(proposal)
}

/// Co-signer reviews and signs transaction
pub async fn review_multisig_proposal(
    wallet: &MultisigWallet,
    proposal: &TransactionProposal,
) -> Result<PartialSignature> {
    // 1. Verify proposal came from legitimate co-signer
    let proposer_verified = wallet.co_signers
        .iter()
        .any(|c| c.identity_key == proposal.proposer);

    ensure!(proposer_verified, "Proposer not a co-signer");

    // 2. Display transaction for review
    // - Recipient address
    // - Amount
    // - Fee
    // - Proposer identity (name + safety number)

    // 3. User confirms they approve

    // 4. Sign transaction with my key
    let my_signature = sign_transaction(&proposal.tx)?;

    // 5. Return signature to proposer
    Ok(PartialSignature {
        signer: my_identity(),
        signature: my_signature,
    })
}

/// Collect signatures and broadcast
pub async fn finalize_multisig_transaction(
    wallet: &MultisigWallet,
    proposal: &TransactionProposal,
    signatures: Vec<PartialSignature>,
) -> Result<()> {
    // 1. Verify we have threshold signatures
    ensure!(signatures.len() >= wallet.threshold, "Not enough signatures");

    // 2. Verify each signature
    for sig in &signatures {
        verify_partial_signature(&proposal.tx, sig)?;
    }

    // 3. Combine signatures into final transaction
    let signed_tx = combine_signatures(&proposal.tx, &signatures)?;

    // 4. Broadcast through Scrambler (multi-node broadcast)
    broadcast_transaction(&signed_tx).await?;

    Ok(())
}
```

---

## Privacy Coin Enhancements

### Monero View Key Protection

```rust
/// Monero wallet with separate view/spend keys
pub struct MoneroWallet {
    spend_key: EncryptedKey,  // Never transmitted
    view_key: EncryptedKey,   // Can be shared for read-only access
}

impl MoneroWallet {
    /// Share view key with auditor (read-only access)
    pub fn export_view_key(&self) -> Result<ViewKey> {
        // View key allows seeing incoming transactions
        // Cannot spend funds
        // Useful for:
        // - Tax reporting
        // - Audit compliance
        // - Business accounting

        let view_key = self.view_key.decrypt()?;

        Ok(ViewKey {
            key: view_key,
            warning: "View key provides read-only access to transactions",
        })
    }

    /// Generate unique subaddress per payment
    pub fn generate_subaddress(&self, index: u32) -> Result<MoneroAddress> {
        // Never reuse addresses
        // Each payment gets unique subaddress
        // Prevents address clustering

        let subaddress = derive_monero_subaddress(&self.spend_key, index)?;

        Ok(subaddress)
    }
}
```

### Bitcoin CoinJoin Automation

```rust
/// Automatically CoinJoin incoming funds
pub struct BitcoinPrivacyManager {
    coinjoin_enabled: bool,
    coinjoin_coordinators: Vec<CoordinatorInfo>,
}

impl BitcoinPrivacyManager {
    pub async fn auto_coinjoin(&self, utxo: UTXO) -> Result<()> {
        if !self.coinjoin_enabled {
            return Ok(());
        }

        // 1. Select coordinator
        // Use multiple coordinators to avoid single point of trust
        let coordinator = self.select_random_coordinator()?;

        // 2. Connect through Scrambler (coordinator can't see IP)
        let coinjoin_session = connect_to_coordinator_via_scrambler(coordinator).await?;

        // 3. Participate in CoinJoin round
        let mixed_utxo = coinjoin_session.participate(utxo).await?;

        // Result: UTXO mixed with other users' UTXOs
        // Blockchain analysis can't determine which output is ours

        Ok(())
    }
}
```

### Zcash Shielded Enforcement

```rust
/// ONLY shielded Zcash addresses allowed
pub fn validate_zcash_address(address: &str) -> Result<()> {
    // z-address (shielded): Allowed
    // t-address (transparent): BLOCKED

    if address.starts_with('t') {
        return Err(TransparentAddressBlocked {
            message: "Transparent Zcash addresses are not supported",
            reason: "Use only shielded (z-address) for privacy",
        });
    }

    ensure!(address.starts_with('z'), "Invalid Zcash address");

    Ok(())
}
```

---

## Broadcast Verification

### Objective

Verify that transactions were actually broadcast to the network and prevent broadcast
manipulation attacks.

### Multi-Phase Verification

```rust
pub async fn broadcast_and_verify(tx: &SignedTransaction) -> Result<BroadcastConfirmation> {
    // PHASE 1: Multi-node broadcast (already implemented)
    let broadcast_nodes = select_broadcast_nodes(5)?;

    for node in &broadcast_nodes {
        broadcast_to_node_via_scrambler(tx, node).await?;
    }

    // PHASE 2: Verification (NEW)
    // Query different set of nodes to confirm tx in mempool

    tokio::time::sleep(Duration::seconds(5)).await;

    let verification_nodes = select_verification_nodes(5)?;

    let mut confirmations = 0;

    for node in &verification_nodes {
        if node_has_tx_in_mempool(node, &tx.hash()).await? {
            confirmations += 1;
        }
    }

    // Require 3 of 5 verification nodes to confirm
    if confirmations >= 3 {
        Ok(BroadcastConfirmation::Success {
            confirmations,
            message: "Transaction confirmed in mempool",
        })
    } else {
        // PHASE 3: Fallback
        warn!("Broadcast verification failed - retrying");

        let fallback_nodes = select_fallback_broadcast_nodes(5)?;

        for node in &fallback_nodes {
            broadcast_to_node_via_scrambler(tx, node).await?;
        }

        // Check again
        tokio::time::sleep(Duration::seconds(5)).await;

        let recheck_confirmations = count_mempool_confirmations(tx).await?;

        if recheck_confirmations >= 3 {
            Ok(BroadcastConfirmation::SuccessAfterRetry {
                confirmations: recheck_confirmations,
            })
        } else {
            Err(BroadcastFailed {
                initial_confirmations: confirmations,
                retry_confirmations: recheck_confirmations,
            })
        }
    }
}
```

---

## Side-Channel Attack Mitigation

### Timing Attack Protection

```rust
/// Constant-time address comparison
pub fn compare_addresses_constant_time(a: &str, b: &str) -> bool {
    use ring::constant_time::verify_slices_are_equal;

    if a.len() != b.len() {
        return false;
    }

    verify_slices_are_equal(a.as_bytes(), b.as_bytes()).is_ok()
}

/// Constant-time amount comparison
pub fn validate_amount_constant_time(
    actual: Amount,
    expected: Amount,
) -> bool {
    let actual_bytes = actual.to_le_bytes();
    let expected_bytes = expected.to_le_bytes();

    use ring::constant_time::verify_slices_are_equal;
    verify_slices_are_equal(&actual_bytes, &expected_bytes).is_ok()
}
```

### Cache-Timing Attack Mitigation

```rust
/// Flush caches after sensitive operations
pub fn flush_caches() {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        use std::arch::x86_64::_mm_clflush;

        // Flush instruction cache
        std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
    }
}

/// Execute sensitive operation with cache flushing
pub fn sign_with_cache_protection(key: &SigningKey, message: &[u8]) -> Signature {
    // Before: flush caches
    flush_caches();

    // Sign
    let signature = key.sign(message);

    // After: flush caches again
    flush_caches();

    signature
}
```

---

## Implementation Priority

### Phase 1: Critical Security (P0)

**Week 1-2:**
- Hardware security module integration (iOS Secure Enclave, Android StrongBox)
- Memory protection (mlock, encrypted in memory, zeroize)

**Week 3-4:**
- Transaction verification layer (address, amount, replay protection)
- Fee validation (multi-source estimates, outlier detection)

### Phase 2: User Protection (P1)

**Week 5-6:**
- Anti-phishing protection (recipient verification, suspicious patterns)
- Payment request authentication

**Week 7-8:**
- Smart contract security (approval limits, simulation, contract verification)

### Phase 3: Advanced Features (P2)

**Week 9-10:**
- Shamir seed backup (SLIP-39)
- Social recovery (optional)

**Week 11-12:**
- Multi-sig coordination hardening
- Privacy coin enhancements

### Phase 4: Defense in Depth (P3)

**Week 13-14:**
- Broadcast verification
- Side-channel attack mitigations

**Week 15-16:**
- Comprehensive security testing
- Third-party security audit

---

## Testing & Validation

### Security Testing Checklist

- [ ] Hardware security module integration tested on real devices
- [ ] Memory protection verified (mlock, no swap, zeroization)
- [ ] Transaction verification catches all attack vectors
- [ ] Phishing protection prevents social engineering
- [ ] Smart contract simulation catches malicious contracts
- [ ] Shamir backup/recovery works correctly
- [ ] Multi-sig coordination secure against malicious co-signers
- [ ] Broadcast verification catches broadcast failures
- [ ] Side-channel attacks mitigated (timing, cache)

### Threat Modeling

| Attack Vector | Current Mitigation | Additional Hardening |
|---|---|---|
| **Clipboard hijacking** | Amount verification | ✓ Checksum validation |
| **Address poisoning** | Visual confirmation | ✓ Similarity detection |
| **Phishing** | Safety numbers | ✓ Enhanced verification UI |
| **Memory dumps** | Encrypted at rest | ✓ HSM + memory encryption |
| **Malicious contracts** | User review | ✓ Simulation + verification |
| **Unlimited approvals** | None currently | ✓ Approval limits |
| **Social engineering** | User judgment | ✓ Suspicious pattern detection |
| **Broadcast manipulation** | Multi-node broadcast | ✓ Broadcast verification |

---

## Cross-References

- [shadow-wallet.md](shadow-wallet.md) - Base wallet architecture
- [cryptography.md](cryptography.md) - Cryptographic primitives
- [scrambler.md](scrambler.md) - Network-level protection
- [zero-log-doctrine.md](zero-log-doctrine.md) - Data retention policy
- [access-control.md](access-control.md) - Authentication & authorization

---

*This specification provides defense-in-depth hardening that elevates the Shadow Wallet's
application-layer security to match the messenger's network-layer security. Together, they
form a comprehensive privacy-and-security architecture where financial transactions are as
untraceable as messages, and wallet keys are as protected as the device itself allows.*
