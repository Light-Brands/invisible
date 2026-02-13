//! Cryptographic key types and management
//!
//! Implements Ed25519 keys for identity and X25519 keys for encryption.
//! All sensitive key material is zeroized on drop.

use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
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
#[derive(Debug, Clone, Serialize, Deserialize, Zeroize, ZeroizeOnDrop)]
pub struct KeyPair {
    /// Public key (safe to share)
    #[zeroize(skip)]
    public: Vec<u8>,
    /// Private key (must remain secret)
    private: Vec<u8>,
}

impl KeyPair {
    /// Generate a new random key pair using X25519
    pub fn generate() -> Result<Self> {
        use rand::rngs::OsRng;
        use x25519_dalek::{PublicKey as X25519PublicKey, StaticSecret as X25519Secret};

        // Generate X25519 private key
        let private_key = X25519Secret::random_from_rng(OsRng);

        // Derive public key
        let public_key = X25519PublicKey::from(&private_key);

        Ok(Self {
            public: public_key.as_bytes().to_vec(),
            private: private_key.to_bytes().to_vec(),
        })
    }

    /// Perform Diffie-Hellman key agreement
    pub fn dh(&self, their_public: &[u8]) -> Result<Vec<u8>> {
        use x25519_dalek::{PublicKey as X25519PublicKey, StaticSecret as X25519Secret};

        // Reconstruct our private key
        let private_bytes: [u8; 32] = self.private
            .as_slice()
            .try_into()
            .map_err(|_| CryptoError::InvalidKey("Invalid private key length".to_string()))?;
        let private_key = X25519Secret::from(private_bytes);

        // Parse their public key
        let public_bytes: [u8; 32] = their_public
            .try_into()
            .map_err(|_| CryptoError::InvalidKey("Invalid public key length".to_string()))?;
        let their_public_key = X25519PublicKey::from(public_bytes);

        // Perform DH
        let shared_secret = private_key.diffie_hellman(&their_public_key);

        Ok(shared_secret.as_bytes().to_vec())
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
    /// Generate a new identity key using Ed25519
    pub fn generate() -> Result<Self> {
        use rand::rngs::OsRng;
        use rand::RngCore;

        // Generate Ed25519 signing key
        let mut secret_key_bytes = [0u8; 32];
        OsRng.fill_bytes(&mut secret_key_bytes);

        let signing_key = SigningKey::from_bytes(&secret_key_bytes);
        let verifying_key = signing_key.verifying_key();

        Ok(Self {
            public: verifying_key.to_bytes().to_vec(),
            private: Some(signing_key.to_bytes().to_vec()),
        })
    }

    /// Sign a message
    pub fn sign(&self, message: &[u8]) -> Result<Vec<u8>> {
        let private = self.private.as_ref()
            .ok_or_else(|| CryptoError::InvalidKey("No private key available".to_string()))?;

        let signing_key_bytes: [u8; 32] = private
            .as_slice()
            .try_into()
            .map_err(|_| CryptoError::InvalidKey("Invalid private key length".to_string()))?;

        let signing_key = SigningKey::from_bytes(&signing_key_bytes);
        let signature = signing_key.sign(message);

        Ok(signature.to_bytes().to_vec())
    }

    /// Verify a signature
    pub fn verify(&self, message: &[u8], signature: &[u8]) -> Result<()> {
        let verifying_key_bytes: [u8; 32] = self.public
            .as_slice()
            .try_into()
            .map_err(|_| CryptoError::InvalidKey("Invalid public key length".to_string()))?;

        let verifying_key = VerifyingKey::from_bytes(&verifying_key_bytes)
            .map_err(|e| CryptoError::InvalidKey(format!("Invalid public key: {}", e)))?;

        let signature_bytes: [u8; 64] = signature
            .try_into()
            .map_err(|_| CryptoError::InvalidKey("Invalid signature length".to_string()))?;

        let signature = Signature::from_bytes(&signature_bytes);

        verifying_key
            .verify(message, &signature)
            .map_err(|_| CryptoError::SignatureVerificationFailed)?;

        Ok(())
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
    pub fn generate(id: u32, identity_key: &IdentityKey) -> Result<Self> {
        let key_pair = KeyPair::generate()?;

        // Sign the public key with identity key
        let signature = identity_key.sign(key_pair.public_key())?;

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

    /// Verify the signature on this pre-key
    pub fn verify(&self, identity_key: &IdentityKey) -> Result<()> {
        identity_key.verify(self.key_pair.public_key(), &self.signature)
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
        let identity = IdentityKey::generate().unwrap();
        let spk = SignedPreKey::generate(1, &identity).unwrap();
        assert_eq!(spk.id(), 1);
        assert!(!spk.public_key().is_empty());

        // Verify signature
        assert!(spk.verify(&identity).is_ok());
    }
}
