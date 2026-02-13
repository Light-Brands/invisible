//! X3DH (Extended Triple Diffie-Hellman) Key Agreement Protocol
//!
//! Implements the Signal Protocol's X3DH key exchange for establishing
//! shared secrets between parties who have not previously communicated.
//!
//! ## Protocol Overview
//!
//! 1. Bob publishes identity key (IK_B), signed pre-key (SPK_B), and one-time pre-keys (OPK_B)
//! 2. Alice fetches Bob's key bundle
//! 3. Alice generates ephemeral key (EK_A) and computes DH operations
//! 4. Alice derives shared secret SK = KDF(DH1 || DH2 || DH3 || DH4)
//! 5. Alice sends initial message with IK_A, EK_A, and optionally used OPK_B id
//! 6. Bob computes same shared secret from received keys
//!
//! ## Security Properties
//!
//! - **Forward Secrecy:** Compromise of long-term keys doesn't reveal past session keys
//! - **Deniability:** No cryptographic proof of who sent messages
//! - **Asynchronous:** Parties can establish shared secret without being online simultaneously

use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::error::{CryptoError, Result};
use crate::kdf::hkdf_sha256;
use crate::keys::{IdentityKey, KeyPair, OneTimePreKey, SignedPreKey};
use crate::utils::concat;

/// X3DH session that holds the derived shared secret
#[derive(Debug, Zeroize, ZeroizeOnDrop)]
pub struct X3DHSession {
    /// Shared secret derived from X3DH
    shared_secret: Vec<u8>,
    /// Associated data for this session
    associated_data: Vec<u8>,
}

impl X3DHSession {
    /// Get the shared secret
    pub fn shared_secret(&self) -> &[u8] {
        &self.shared_secret
    }

    /// Get the associated data
    pub fn associated_data(&self) -> &[u8] {
        &self.associated_data
    }
}

/// Pre-key bundle published by Bob for others to initiate sessions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreKeyBundle {
    /// Bob's identity key (public)
    pub identity_key: IdentityKey,
    /// Bob's signed pre-key (public)
    pub signed_pre_key: SignedPreKey,
    /// Bob's one-time pre-key (public, optional)
    pub one_time_pre_key: Option<OneTimePreKey>,
}

/// Initial message from Alice to Bob containing her public keys
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitialMessage {
    /// Alice's identity key (public)
    pub identity_key: IdentityKey,
    /// Alice's ephemeral key (public)
    pub ephemeral_key: Vec<u8>,
    /// ID of the one-time pre-key used (if any)
    pub one_time_pre_key_id: Option<u32>,
}

/// Initiator side of X3DH (Alice)
pub struct X3DHInitiator {
    /// Alice's identity key
    identity_key: IdentityKey,
}

impl X3DHInitiator {
    /// Create a new X3DH initiator with an identity key
    pub fn new(identity_key: IdentityKey) -> Self {
        Self { identity_key }
    }

    /// Perform X3DH key agreement as the initiator (Alice)
    ///
    /// # Arguments
    /// * `bundle` - Bob's pre-key bundle
    ///
    /// # Returns
    /// * `X3DHSession` - The established session with shared secret
    /// * `InitialMessage` - Message to send to Bob
    pub fn initiate(&self, bundle: &PreKeyBundle) -> Result<(X3DHSession, InitialMessage)> {
        // Generate ephemeral key for this session
        let ephemeral_key = KeyPair::generate()?;

        // Perform Diffie-Hellman operations
        // DH1 = DH(IK_A, SPK_B)
        // DH2 = DH(EK_A, IK_B)
        // DH3 = DH(EK_A, SPK_B)
        // DH4 = DH(EK_A, OPK_B) if OPK_B exists

        // TODO: Implement actual X25519 DH operations
        // For now, using placeholders
        let dh1 = self.dh(
            self.identity_key.public_key(),
            bundle.signed_pre_key.public_key(),
        )?;
        let dh2 = self.dh(
            ephemeral_key.public_key(),
            bundle.identity_key.public_key(),
        )?;
        let dh3 = self.dh(
            ephemeral_key.public_key(),
            bundle.signed_pre_key.public_key(),
        )?;

        let mut dh_outputs = vec![dh1, dh2, dh3];

        let one_time_pre_key_id = if let Some(ref opk) = bundle.one_time_pre_key {
            let dh4 = self.dh(ephemeral_key.public_key(), opk.public_key())?;
            dh_outputs.push(dh4);
            Some(opk.id())
        } else {
            None
        };

        // Concatenate all DH outputs
        let dh_concat = concat(&dh_outputs.iter().map(|v| v.as_slice()).collect::<Vec<_>>());

        // Derive shared secret using HKDF
        let shared_secret = hkdf_sha256(&dh_concat, None, b"X3DHv1", 32)?;

        // Build associated data (IK_A || IK_B)
        let associated_data = concat(&[
            self.identity_key.public_key(),
            bundle.identity_key.public_key(),
        ]);

        let session = X3DHSession {
            shared_secret,
            associated_data,
        };

        let initial_message = InitialMessage {
            identity_key: self.identity_key.clone(),
            ephemeral_key: ephemeral_key.public_key().to_vec(),
            one_time_pre_key_id,
        };

        Ok((session, initial_message))
    }

    /// Perform Diffie-Hellman key agreement
    fn dh(&self, our_public: &[u8], their_public: &[u8]) -> Result<Vec<u8>> {
        use x25519_dalek::{PublicKey as X25519PublicKey, StaticSecret};
        use rand::rngs::OsRng;

        // Generate temp key for DH
        // TODO: In production, use actual stored private keys
        let our_secret = StaticSecret::random_from_rng(OsRng);

        let their_pub_bytes: [u8; 32] = their_public
            .try_into()
            .map_err(|_| CryptoError::InvalidKey("Invalid public key length".to_string()))?;
        let their_public_key = X25519PublicKey::from(their_pub_bytes);

        let shared_secret = our_secret.diffie_hellman(&their_public_key);
        Ok(shared_secret.as_bytes().to_vec())
    }
}

/// Responder side of X3DH (Bob)
pub struct X3DHResponder {
    /// Bob's identity key
    identity_key: IdentityKey,
    /// Bob's signed pre-key
    signed_pre_key: SignedPreKey,
    /// Bob's one-time pre-keys
    one_time_pre_keys: Vec<OneTimePreKey>,
}

impl X3DHResponder {
    /// Create a new X3DH responder with key material
    pub fn new(
        identity_key: IdentityKey,
        signed_pre_key: SignedPreKey,
        one_time_pre_keys: Vec<OneTimePreKey>,
    ) -> Self {
        Self {
            identity_key,
            signed_pre_key,
            one_time_pre_keys,
        }
    }

    /// Get the pre-key bundle to publish
    pub fn get_bundle(&self) -> PreKeyBundle {
        PreKeyBundle {
            identity_key: self.identity_key.clone(),
            signed_pre_key: self.signed_pre_key.clone(),
            one_time_pre_key: self.one_time_pre_keys.first().cloned(),
        }
    }

    /// Respond to an initial message from Alice
    ///
    /// # Arguments
    /// * `msg` - The initial message from Alice
    ///
    /// # Returns
    /// * `X3DHSession` - The established session with shared secret
    pub fn respond(&self, msg: &InitialMessage) -> Result<X3DHSession> {
        // Perform the same DH operations as Alice
        // DH1 = DH(SPK_B, IK_A)
        // DH2 = DH(IK_B, EK_A)
        // DH3 = DH(SPK_B, EK_A)
        // DH4 = DH(OPK_B, EK_A) if OPK was used

        // TODO: Implement actual X25519 DH operations
        let dh1 = self.dh(
            self.signed_pre_key.public_key(),
            msg.identity_key.public_key(),
        )?;
        let dh2 = self.dh(self.identity_key.public_key(), &msg.ephemeral_key)?;
        let dh3 = self.dh(self.signed_pre_key.public_key(), &msg.ephemeral_key)?;

        let mut dh_outputs = vec![dh1, dh2, dh3];

        if let Some(opk_id) = msg.one_time_pre_key_id {
            // Find the one-time pre-key that was used
            if let Some(opk) = self.one_time_pre_keys.iter().find(|k| k.id() == opk_id) {
                let dh4 = self.dh(opk.public_key(), &msg.ephemeral_key)?;
                dh_outputs.push(dh4);

                // TODO: Remove used one-time pre-key from storage
            }
        }

        // Concatenate all DH outputs (must be in same order as Alice)
        let dh_concat = concat(&dh_outputs.iter().map(|v| v.as_slice()).collect::<Vec<_>>());

        // Derive shared secret using HKDF
        let shared_secret = hkdf_sha256(&dh_concat, None, b"X3DHv1", 32)?;

        // Build associated data (IK_A || IK_B)
        let associated_data = concat(&[
            msg.identity_key.public_key(),
            self.identity_key.public_key(),
        ]);

        Ok(X3DHSession {
            shared_secret,
            associated_data,
        })
    }

    /// Perform Diffie-Hellman key agreement
    fn dh(&self, our_public: &[u8], their_public: &[u8]) -> Result<Vec<u8>> {
        use x25519_dalek::{PublicKey as X25519PublicKey, StaticSecret};
        use rand::rngs::OsRng;

        // Generate temp key for DH
        // TODO: In production, use actual stored private keys
        let our_secret = StaticSecret::random_from_rng(OsRng);

        let their_pub_bytes: [u8; 32] = their_public
            .try_into()
            .map_err(|_| CryptoError::InvalidKey("Invalid public key length".to_string()))?;
        let their_public_key = X25519PublicKey::from(their_pub_bytes);

        let shared_secret = our_secret.diffie_hellman(&their_public_key);
        Ok(shared_secret.as_bytes().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prekey_bundle_creation() {
        let identity = IdentityKey::generate().unwrap();
        let signed_pre_key = SignedPreKey::generate(1).unwrap();
        let one_time_pre_key = OneTimePreKey::generate(1).unwrap();

        let bundle = PreKeyBundle {
            identity_key: identity,
            signed_pre_key,
            one_time_pre_key: Some(one_time_pre_key),
        };

        assert!(!bundle.identity_key.public_key().is_empty());
        assert!(!bundle.signed_pre_key.public_key().is_empty());
        assert!(bundle.one_time_pre_key.is_some());
    }

    #[test]
    fn test_x3dh_session_creation() {
        let shared_secret = vec![1u8; 32];
        let associated_data = vec![2u8; 64];

        let session = X3DHSession {
            shared_secret: shared_secret.clone(),
            associated_data: associated_data.clone(),
        };

        assert_eq!(session.shared_secret(), &shared_secret);
        assert_eq!(session.associated_data(), &associated_data);
    }

    #[test]
    fn test_x3dh_initiator_creation() {
        let identity = IdentityKey::generate().unwrap();
        let _initiator = X3DHInitiator::new(identity);
    }

    #[test]
    fn test_x3dh_responder_bundle() {
        let identity = IdentityKey::generate().unwrap();
        let signed_pre_key = SignedPreKey::generate(1).unwrap();
        let one_time_pre_keys = vec![OneTimePreKey::generate(1).unwrap()];

        let responder = X3DHResponder::new(identity, signed_pre_key, one_time_pre_keys);
        let bundle = responder.get_bundle();

        assert!(!bundle.identity_key.public_key().is_empty());
        assert!(!bundle.signed_pre_key.public_key().is_empty());
        assert!(bundle.one_time_pre_key.is_some());
    }
}
