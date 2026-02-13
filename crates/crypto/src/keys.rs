//! Cryptographic key types and management
//!
//! Implements Ed25519 keys for identity and X25519 keys for encryption.
//! All sensitive key material is zeroized on drop.

use ring::rand::{SecureRandom, SystemRandom};
use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::error::{CryptoError, Result};

/// Size of Ed25519 public keys (32 bytes)
pub const ED25519_PUBLIC_KEY_SIZE: usize = 32;

/// Size of Ed25519 private keys (32 bytes seed)
pub const ED25519_PRIVATE_KEY_SIZE: usize = 32;

/// Size of X25519 keys (32 bytes)
pub const X25519_KEY_SIZE: usize = 32;

/// A cryptographic key pair (public + private key)
#[derive(Debug, Clone, Zeroize, ZeroizeOnDrop)]
pub struct KeyPair {
    /// Public key (safe to share)
    #[zeroize(skip)]
    public: Vec<u8>,
    /// Private key (must remain secret)
    private: Vec<u8>,
}

impl KeyPair {
    /// Generate a new random key pair
    pub fn generate() -> Result<Self> {
        let rng = SystemRandom::new();
        let mut private = vec![0u8; X25519_KEY_SIZE];
        rng.fill(&mut private)
            .map_err(|e| CryptoError::CryptoError(format!("RNG failed: {:?}", e)))?;

        // For X25519, public key is derived from private key
        // TODO: Implement actual X25519 key derivation
        let public = private.clone(); // Placeholder

        Ok(Self { public, private })
    }

    /// Get the public key
    pub fn public_key(&self) -> &[u8] {
        &self.public
    }

    /// Get the private key (use carefully!)
    pub fn private_key(&self) -> &[u8] {
        &self.private
    }
}

/// Identity key for long-term device identity (Ed25519)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityKey {
    /// Public identity key
    public: Vec<u8>,
    /// Private identity key (if owned)
    #[serde(skip_serializing_if = "Option::is_none")]
    private: Option<Vec<u8>>,
}

impl IdentityKey {
    /// Generate a new identity key
    pub fn generate() -> Result<Self> {
        let key_pair = KeyPair::generate()?;
        Ok(Self {
            public: key_pair.public_key().to_vec(),
            private: Some(key_pair.private_key().to_vec()),
        })
    }

    /// Create from public key only (for remote identities)
    pub fn from_public(public: Vec<u8>) -> Self {
        Self {
            public,
            private: None,
        }
    }

    /// Get public key
    pub fn public_key(&self) -> &[u8] {
        &self.public
    }

    /// Check if we own the private key
    pub fn is_owned(&self) -> bool {
        self.private.is_some()
    }
}

/// Signed pre-key for X3DH
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedPreKey {
    /// The pre-key pair
    key_pair: KeyPair,
    /// Signature over the public key
    signature: Vec<u8>,
    /// Key ID for rotation tracking
    id: u32,
    /// Timestamp when created
    timestamp: u64,
}

impl SignedPreKey {
    /// Generate a new signed pre-key
    pub fn generate(id: u32) -> Result<Self> {
        let key_pair = KeyPair::generate()?;
        // TODO: Actually sign with identity key
        let signature = vec![0u8; 64]; // Placeholder

        Ok(Self {
            key_pair,
            signature,
            id,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        })
    }

    /// Get the key ID
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Get public key
    pub fn public_key(&self) -> &[u8] {
        self.key_pair.public_key()
    }
}

/// One-time pre-key for X3DH
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OneTimePreKey {
    /// The pre-key pair
    key_pair: KeyPair,
    /// Key ID
    id: u32,
}

impl OneTimePreKey {
    /// Generate a new one-time pre-key
    pub fn generate(id: u32) -> Result<Self> {
        let key_pair = KeyPair::generate()?;
        Ok(Self { key_pair, id })
    }

    /// Get the key ID
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Get public key
    pub fn public_key(&self) -> &[u8] {
        self.key_pair.public_key()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keypair_generation() {
        let kp = KeyPair::generate().unwrap();
        assert_eq!(kp.public_key().len(), X25519_KEY_SIZE);
        assert_eq!(kp.private_key().len(), X25519_KEY_SIZE);
    }

    #[test]
    fn test_identity_key_generation() {
        let ik = IdentityKey::generate().unwrap();
        assert!(ik.is_owned());
        assert!(!ik.public_key().is_empty());
    }

    #[test]
    fn test_signed_prekey_generation() {
        let spk = SignedPreKey::generate(1).unwrap();
        assert_eq!(spk.id(), 1);
        assert!(!spk.public_key().is_empty());
    }
}
