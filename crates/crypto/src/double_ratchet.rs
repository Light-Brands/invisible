//! Double Ratchet Algorithm
//!
//! Implements the Signal Protocol's Double Ratchet algorithm for secure messaging.
//! Provides forward secrecy and post-compromise security through continuous
//! key derivation and Diffie-Hellman ratcheting.
//!
//! ## Overview
//!
//! The Double Ratchet combines:
//! - **DH Ratchet:** Generates new DH key pairs and performs new DH exchanges
//! - **Symmetric Key Ratchet:** Derives new message keys from chain keys
//!
//! ## Security Properties
//!
//! - **Forward Secrecy:** Past messages remain secure even if current keys compromised
//! - **Post-Compromise Security:** New DH ratchet step restores security after compromise
//! - **Message Loss Resilience:** Can handle out-of-order or lost messages

use ring::aead::{self, Aad, LessSafeKey, Nonce, UnboundKey, AES_256_GCM};
use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::error::{CryptoError, Result};
use crate::kdf::{kdf_ck, kdf_rk};
use crate::keys::KeyPair;
use crate::utils::{concat, random_bytes};

/// Maximum number of skipped message keys to store
const MAX_SKIP: usize = 1000;

/// Double Ratchet state
#[derive(Debug, Zeroize, ZeroizeOnDrop)]
pub struct DoubleRatchet {
    /// DH ratchet key pair
    #[zeroize(skip)]
    dh_self: KeyPair,
    /// Remote party's current DH public key
    #[zeroize(skip)]
    dh_remote: Vec<u8>,
    /// Root chain key
    root_key: Vec<u8>,
    /// Sending chain key
    chain_key_send: Vec<u8>,
    /// Receiving chain key
    chain_key_recv: Vec<u8>,
    /// Number of messages sent in current sending chain
    send_count: u32,
    /// Number of messages received in current receiving chain
    recv_count: u32,
    /// Previous sending chain length (for header)
    prev_chain_len: u32,
    /// Skipped message keys for out-of-order messages
    #[zeroize(skip)]
    skipped_keys: Vec<SkippedKey>,
}

/// A skipped message key for handling out-of-order messages
#[derive(Debug, Clone, Zeroize, ZeroizeOnDrop)]
struct SkippedKey {
    /// The DH public key used
    #[zeroize(skip)]
    public_key: Vec<u8>,
    /// The message number
    message_num: u32,
    /// The message key
    message_key: Vec<u8>,
}

/// Message header containing ratchet information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageHeader {
    /// Sender's current DH ratchet public key
    pub public_key: Vec<u8>,
    /// Number of messages in previous sending chain
    pub prev_chain_len: u32,
    /// Message number in current sending chain
    pub message_num: u32,
}

/// Encrypted message with header
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedMessage {
    /// Message header
    pub header: MessageHeader,
    /// Ciphertext
    pub ciphertext: Vec<u8>,
}

impl DoubleRatchet {
    /// Initialize Double Ratchet from X3DH shared secret (Alice - sending first)
    ///
    /// # Arguments
    /// * `shared_secret` - Shared secret from X3DH
    /// * `remote_public_key` - Bob's signed pre-key public key
    pub fn init_alice(shared_secret: &[u8], remote_public_key: Vec<u8>) -> Result<Self> {
        // Initialize with shared secret as root key
        let root_key = shared_secret.to_vec();

        // Generate initial DH key pair
        let dh_self = KeyPair::generate()?;

        // Perform initial DH ratchet step with remote public key
        let dh_output = dh_self.dh(&remote_public_key)?;

        // Derive new root key and sending chain key
        let (new_root_key, chain_key_send) = kdf_rk(&root_key, &dh_output)?;

        Ok(Self {
            dh_self,
            dh_remote: remote_public_key,
            root_key: new_root_key,
            chain_key_send,
            chain_key_recv: vec![0u8; 32], // Will be initialized on first receive
            send_count: 0,
            recv_count: 0,
            prev_chain_len: 0,
            skipped_keys: Vec::new(),
        })
    }

    /// Initialize Double Ratchet from X3DH shared secret (Bob - receiving first)
    ///
    /// # Arguments
    /// * `shared_secret` - Shared secret from X3DH
    /// * `keypair` - Bob's signed pre-key pair
    pub fn init_bob(shared_secret: &[u8], keypair: KeyPair) -> Result<Self> {
        let root_key = shared_secret.to_vec();

        Ok(Self {
            dh_self: keypair,
            dh_remote: vec![], // Will be set when receiving first message
            root_key,
            chain_key_send: vec![0u8; 32], // Will be initialized on first DH ratchet
            chain_key_recv: vec![0u8; 32], // Will be initialized on first receive
            send_count: 0,
            recv_count: 0,
            prev_chain_len: 0,
            skipped_keys: Vec::new(),
        })
    }

    /// Encrypt a message
    ///
    /// # Arguments
    /// * `plaintext` - The message to encrypt
    /// * `associated_data` - Additional authenticated data
    pub fn encrypt(&mut self, plaintext: &[u8], associated_data: &[u8]) -> Result<EncryptedMessage> {
        // Derive message key from sending chain
        let (new_chain_key, message_key) = kdf_ck(&self.chain_key_send)?;
        self.chain_key_send = new_chain_key;

        // Encrypt plaintext with message key
        let ciphertext = self.aead_encrypt(&message_key, plaintext, associated_data)?;

        // Build message header
        let header = MessageHeader {
            public_key: self.dh_self.public_key().to_vec(),
            prev_chain_len: self.prev_chain_len,
            message_num: self.send_count,
        };

        self.send_count += 1;

        Ok(EncryptedMessage { header, ciphertext })
    }

    /// Decrypt a message
    ///
    /// # Arguments
    /// * `message` - The encrypted message to decrypt
    /// * `associated_data` - Additional authenticated data
    pub fn decrypt(
        &mut self,
        message: &EncryptedMessage,
        associated_data: &[u8],
    ) -> Result<Vec<u8>> {
        // Check if this is a message with a new DH ratchet key
        if message.header.public_key != self.dh_remote {
            self.dh_ratchet(&message.header)?;
        }

        // Check if we need to skip messages
        if message.header.message_num > self.recv_count {
            self.skip_message_keys(message.header.message_num)?;
        }

        // Try to get message key (might be skipped key)
        let message_key = if let Some(key) = self.try_skipped_keys(message) {
            key
        } else {
            // Derive message key from receiving chain
            let (new_chain_key, msg_key) = kdf_ck(&self.chain_key_recv)?;
            self.chain_key_recv = new_chain_key;
            self.recv_count += 1;
            msg_key
        };

        // Decrypt ciphertext
        self.aead_decrypt(&message_key, &message.ciphertext, associated_data)
    }

    /// Perform DH ratchet step when receiving message with new DH key
    fn dh_ratchet(&mut self, header: &MessageHeader) -> Result<()> {
        // Store previous chain length
        self.prev_chain_len = self.send_count;
        self.send_count = 0;
        self.recv_count = 0;

        // Update remote DH public key
        self.dh_remote = header.public_key.clone();

        // Perform DH with new remote key using our current DH keypair
        let dh_output = self.dh_self.dh(&self.dh_remote)?;

        // Derive new root key and receiving chain key
        let (new_root_key, chain_key_recv) = kdf_rk(&self.root_key, &dh_output)?;
        self.root_key = new_root_key;
        self.chain_key_recv = chain_key_recv;

        // Generate new DH key pair for next ratchet
        self.dh_self = KeyPair::generate()?;

        // Perform DH with new key pair and remote key
        let dh_output2 = self.dh_self.dh(&self.dh_remote)?;

        // Derive new root key and sending chain key
        let (new_root_key2, chain_key_send) = kdf_rk(&self.root_key, &dh_output2)?;
        self.root_key = new_root_key2;
        self.chain_key_send = chain_key_send;

        Ok(())
    }

    /// Skip message keys for out-of-order messages
    fn skip_message_keys(&mut self, until: u32) -> Result<()> {
        if self.recv_count + MAX_SKIP as u32 < until {
            return Err(CryptoError::RatchetStateError(
                "Too many skipped messages".to_string(),
            ));
        }

        while self.recv_count < until {
            let (new_chain_key, message_key) = kdf_ck(&self.chain_key_recv)?;

            self.skipped_keys.push(SkippedKey {
                public_key: self.dh_remote.clone(),
                message_num: self.recv_count,
                message_key,
            });

            self.chain_key_recv = new_chain_key;
            self.recv_count += 1;
        }

        Ok(())
    }

    /// Try to find a skipped message key
    fn try_skipped_keys(&mut self, message: &EncryptedMessage) -> Option<Vec<u8>> {
        if let Some(pos) = self.skipped_keys.iter().position(|k| {
            k.public_key == message.header.public_key && k.message_num == message.header.message_num
        }) {
            Some(self.skipped_keys.remove(pos).message_key)
        } else {
            None
        }
    }

    /// Encrypt with AEAD (AES-256-GCM)
    fn aead_encrypt(&self, key: &[u8], plaintext: &[u8], ad: &[u8]) -> Result<Vec<u8>> {
        let unbound_key = UnboundKey::new(&AES_256_GCM, key)
            .map_err(|_| CryptoError::EncryptionFailed("Invalid key".to_string()))?;

        let nonce_bytes = random_bytes(12)?;
        let nonce = Nonce::try_assume_unique_for_key(&nonce_bytes)
            .map_err(|_| CryptoError::EncryptionFailed("Invalid nonce".to_string()))?;

        let sealing_key = LessSafeKey::new(unbound_key);
        let mut in_out = plaintext.to_vec();

        sealing_key
            .seal_in_place_append_tag(nonce, Aad::from(ad), &mut in_out)
            .map_err(|_| CryptoError::EncryptionFailed("Encryption failed".to_string()))?;

        // Prepend nonce to ciphertext
        let mut result = nonce_bytes;
        result.extend_from_slice(&in_out);

        Ok(result)
    }

    /// Decrypt with AEAD (AES-256-GCM)
    fn aead_decrypt(&self, key: &[u8], ciphertext: &[u8], ad: &[u8]) -> Result<Vec<u8>> {
        if ciphertext.len() < 12 {
            return Err(CryptoError::DecryptionFailed(
                "Ciphertext too short".to_string(),
            ));
        }

        // Extract nonce and ciphertext
        let (nonce_bytes, ct) = ciphertext.split_at(12);

        let unbound_key = UnboundKey::new(&AES_256_GCM, key)
            .map_err(|_| CryptoError::DecryptionFailed("Invalid key".to_string()))?;

        let nonce = Nonce::try_assume_unique_for_key(nonce_bytes)
            .map_err(|_| CryptoError::DecryptionFailed("Invalid nonce".to_string()))?;

        let opening_key = LessSafeKey::new(unbound_key);
        let mut in_out = ct.to_vec();

        opening_key
            .open_in_place(nonce, Aad::from(ad), &mut in_out)
            .map_err(|_| CryptoError::DecryptionFailed("Decryption failed".to_string()))?;

        // Remove tag
        let plaintext_len = in_out.len() - AES_256_GCM.tag_len();
        in_out.truncate(plaintext_len);

        Ok(in_out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_ratchet_init() {
        let shared_secret = vec![1u8; 32];
        let remote_key = vec![2u8; 32];

        let alice = DoubleRatchet::init_alice(&shared_secret, remote_key).unwrap();
        assert_eq!(alice.send_count, 0);
        assert_eq!(alice.recv_count, 0);
    }

    #[test]
    fn test_message_header_serialization() {
        let header = MessageHeader {
            public_key: vec![1, 2, 3, 4],
            prev_chain_len: 5,
            message_num: 10,
        };

        let serialized = serde_json::to_string(&header).unwrap();
        let deserialized: MessageHeader = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.public_key, header.public_key);
        assert_eq!(deserialized.prev_chain_len, header.prev_chain_len);
        assert_eq!(deserialized.message_num, header.message_num);
    }

    #[test]
    fn test_aead_encrypt_decrypt() {
        let shared_secret = vec![1u8; 32];
        let remote_key = vec![2u8; 32];
        let mut alice = DoubleRatchet::init_alice(&shared_secret, remote_key).unwrap();

        let key = vec![0u8; 32];
        let plaintext = b"Hello, World!";
        let ad = b"associated data";

        let ciphertext = alice.aead_encrypt(&key, plaintext, ad).unwrap();
        let decrypted = alice.aead_decrypt(&key, &ciphertext, ad).unwrap();

        assert_eq!(decrypted, plaintext);
    }
}
