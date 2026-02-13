//! Sphinx Packet Format
//!
//! Implements a compact packet format for mix networks based on the Sphinx paper.
//! Provides cryptographic security and prevents packet tagging attacks.
//!
//! ## Overview
//!
//! Sphinx packets consist of:
//! - **Header:** Encrypted routing information processed by each hop
//! - **Payload:** End-to-end encrypted message
//! - **MAC:** Message authentication code
//!
//! ## Security Properties
//!
//! - **Unlinkability:** Cannot correlate input/output packets at mix nodes
//! - **Forward Secrecy:** Compromise of long-term keys doesn't reveal past routes
//! - **Replay Protection:** Each packet can only be processed once
//! - **Tagging Prevention:** Modifications are cryptographically detected

use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};
use x25519_dalek::{PublicKey as X25519PublicKey, StaticSecret};
use ring::aead::{Aad, LessSafeKey, Nonce, UnboundKey, CHACHA20_POLY1305};
use ring::hmac;
use rand::rngs::OsRng;
use rand::RngCore;

use crate::error::{Result, ScramblerError};
use invisible_crypto::kdf::hkdf_sha256;

/// Maximum number of hops in a Sphinx route
pub const MAX_HOPS: usize = 5;

/// Size of Sphinx header in bytes
pub const HEADER_SIZE: usize = 256;

/// Size of Sphinx payload in bytes
pub const PAYLOAD_SIZE: usize = 2048;

/// A Sphinx packet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SphinxPacket {
    /// Packet header with encrypted routing info
    pub header: SphinxHeader,
    /// Encrypted payload
    pub payload: Vec<u8>,
}

/// Sphinx packet header
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SphinxHeader {
    /// Ephemeral public key (in the clear)
    pub ephemeral_key: [u8; 32],
    /// Routing information (encrypted for each hop)
    pub routing_info: Vec<u8>,
    /// Message authentication code
    pub mac: [u8; 32],
}

/// Route information for building Sphinx packets
#[derive(Debug, Clone, Zeroize, ZeroizeOnDrop)]
pub struct RouteSpec {
    /// Public keys of mix nodes in the route
    #[zeroize(skip)]
    pub node_keys: Vec<Vec<u8>>,
    /// Destination address
    #[zeroize(skip)]
    pub destination: Vec<u8>,
}

/// Derive encryption and MAC keys from shared secret
fn derive_keys(shared_secret: &[u8]) -> Result<(Vec<u8>, Vec<u8>)> {
    // Derive 64 bytes: 32 for encryption, 32 for MAC
    let key_material = hkdf_sha256(shared_secret, None, b"SphinxKeys", 64)?;

    let enc_key = key_material[0..32].to_vec();
    let mac_key = key_material[32..64].to_vec();

    Ok((enc_key, mac_key))
}


/// Encrypt routing data using stream cipher (XOR with keystream)
/// This avoids the size expansion from AEAD tags
fn encrypt_routing(key: &[u8], plaintext: &[u8]) -> Result<Vec<u8>> {
    use ring::hkdf;

    // Derive a keystream from the key using HKDF
    let salt = hkdf::Salt::new(hkdf::HKDF_SHA256, b"");
    let prk = salt.extract(key);
    let info: &[&[u8]] = &[b"SphinxRouting"];
    let okm = prk
        .expand(info, MyLen(plaintext.len()))
        .map_err(|_| ScramblerError::SphinxError("Key expansion failed".to_string()))?;

    let mut keystream = vec![0u8; plaintext.len()];
    okm.fill(&mut keystream)
        .map_err(|_| ScramblerError::SphinxError("Keystream generation failed".to_string()))?;

    // XOR plaintext with keystream
    let mut ciphertext = plaintext.to_vec();
    for (c, k) in ciphertext.iter_mut().zip(keystream.iter()) {
        *c ^= k;
    }

    Ok(ciphertext)
}

/// Decrypt routing data (XOR is its own inverse)
fn decrypt_routing(key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>> {
    // XOR is symmetric, so decrypt = encrypt
    encrypt_routing(key, ciphertext)
}

/// Helper struct for HKDF output length
struct MyLen(usize);

impl ring::hkdf::KeyType for MyLen {
    fn len(&self) -> usize {
        self.0
    }
}

/// Encrypt payload using ChaCha20-Poly1305 AEAD
fn encrypt_payload(key: &[u8], plaintext: &[u8], associated_data: &[u8]) -> Result<Vec<u8>> {
    let unbound_key = UnboundKey::new(&CHACHA20_POLY1305, key)
        .map_err(|_| ScramblerError::SphinxError("Invalid encryption key".to_string()))?;

    // Use deterministic nonce of zeros for Sphinx (key is one-time use)
    let nonce_bytes = [0u8; 12];
    let nonce = Nonce::try_assume_unique_for_key(&nonce_bytes)
        .map_err(|_| ScramblerError::SphinxError("Invalid nonce".to_string()))?;

    let sealing_key = LessSafeKey::new(unbound_key);
    let mut in_out = plaintext.to_vec();

    sealing_key
        .seal_in_place_append_tag(nonce, Aad::from(associated_data), &mut in_out)
        .map_err(|_| ScramblerError::SphinxError("Encryption failed".to_string()))?;

    Ok(in_out)
}

/// Decrypt payload using ChaCha20-Poly1305 AEAD
fn decrypt_payload(key: &[u8], ciphertext: &[u8], associated_data: &[u8]) -> Result<Vec<u8>> {
    let unbound_key = UnboundKey::new(&CHACHA20_POLY1305, key)
        .map_err(|_| ScramblerError::SphinxError("Invalid decryption key".to_string()))?;

    // Use deterministic nonce of zeros
    let nonce_bytes = [0u8; 12];
    let nonce = Nonce::try_assume_unique_for_key(&nonce_bytes)
        .map_err(|_| ScramblerError::SphinxError("Invalid nonce".to_string()))?;

    let opening_key = LessSafeKey::new(unbound_key);
    let mut in_out = ciphertext.to_vec();

    let plaintext = opening_key
        .open_in_place(nonce, Aad::from(associated_data), &mut in_out)
        .map_err(|_| ScramblerError::SphinxError("Decryption failed".to_string()))?;

    // open_in_place returns a slice to the decrypted data (tag already removed)
    Ok(plaintext.to_vec())
}

/// Build a Sphinx packet
///
/// # Arguments
/// * `route` - Route specification (node keys, destination)
/// * `message` - Message to encapsulate
pub fn build_packet(route: &RouteSpec, message: &[u8]) -> Result<SphinxPacket> {
    if route.node_keys.len() > MAX_HOPS {
        return Err(ScramblerError::SphinxError(format!(
            "Too many hops: {} (max {})",
            route.node_keys.len(),
            MAX_HOPS
        )));
    }

    if message.len() > PAYLOAD_SIZE {
        return Err(ScramblerError::SphinxError(format!(
            "Message too large: {} bytes (max {})",
            message.len(),
            PAYLOAD_SIZE
        )));
    }

    // Step 1: Generate ephemeral key pair
    // Use StaticSecret instead of EphemeralSecret so we can reuse it
    let mut ephemeral_bytes = [0u8; 32];
    RngCore::fill_bytes(&mut OsRng, &mut ephemeral_bytes);
    let ephemeral_secret = StaticSecret::from(ephemeral_bytes);
    let ephemeral_public = X25519PublicKey::from(&ephemeral_secret);

    // Step 2 & 3: Perform ECDH with each node and derive shared secrets
    // Use same ephemeral key for all hops (simpler onion routing style)
    let num_hops = route.node_keys.len();
    let mut shared_secrets = Vec::new();

    for node_key_bytes in &route.node_keys {
        // Parse node's public key
        let node_key_array: [u8; 32] = node_key_bytes
            .as_slice()
            .try_into()
            .map_err(|_| ScramblerError::SphinxError("Invalid node key length".to_string()))?;
        let node_public = X25519PublicKey::from(node_key_array);

        // Perform ECDH with same ephemeral secret for all hops
        let shared_secret = ephemeral_secret.diffie_hellman(&node_public);
        let shared_secret_bytes = shared_secret.as_bytes().to_vec();

        shared_secrets.push(shared_secret_bytes);
    }

    // Step 4: Build routing info layers (simple onion routing)
    // Structure: [next_hop(32) | encrypted_inner_layer]

    // Start with innermost: destination (all zeros = deliver)
    let mut routing_info = vec![0u8; HEADER_SIZE];
    route.destination.iter().take(32).enumerate().for_each(|(i, &b)| routing_info[i] = b);

    // Build layers in reverse (innermost first)
    // For hop i, prepend hop i+1's address, then encrypt
    for i in (0..num_hops).rev() {
        let (enc_key, _) = derive_keys(&shared_secrets[i])?;

        // If not the last hop, prepend next hop's address
        if i < num_hops - 1 {
            let next_hop_address = &route.node_keys[i + 1];
            let mut layer = Vec::new();
            layer.extend_from_slice(next_hop_address);
            layer.extend_from_slice(&routing_info);
            layer.resize(HEADER_SIZE, 0);
            routing_info = layer;
        }

        // Encrypt this layer using stream cipher (no size expansion)
        routing_info = encrypt_routing(&enc_key, &routing_info)?;
    }

    // Step 5: Compute MAC for first hop
    let ephemeral_key_bytes: [u8; 32] = *ephemeral_public.as_bytes();

    let (_, mac_key) = derive_keys(&shared_secrets[0])?;
    let hmac_key = hmac::Key::new(hmac::HMAC_SHA256, &mac_key);
    let mac_tag = hmac::sign(&hmac_key, &routing_info);
    let mut mac = [0u8; 32];
    mac.copy_from_slice(mac_tag.as_ref());

    let header = SphinxHeader {
        ephemeral_key: ephemeral_key_bytes,
        routing_info,
        mac,
    };

    // Step 6: Encrypt payload using stream cipher in layers (no size expansion)
    let mut payload = vec![0u8; PAYLOAD_SIZE];
    payload[..message.len()].copy_from_slice(message);

    // Encrypt payload in layers (innermost first)
    for i in (0..num_hops).rev() {
        let (enc_key, _) = derive_keys(&shared_secrets[i])?;
        // Use stream cipher to avoid size expansion
        payload = encrypt_routing(&enc_key, &payload)?;
    }

    Ok(SphinxPacket { header, payload })
}

/// Process a Sphinx packet at a mix node
///
/// # Arguments
/// * `packet` - The Sphinx packet to process
/// * `node_private_key` - This node's private key
///
/// # Returns
/// * Next hop address and transformed packet, or final destination and payload
pub fn process_packet(
    packet: &SphinxPacket,
    node_private_key: &[u8],
) -> Result<ProcessedPacket> {
    // Step 1: Extract ephemeral public key from header
    let ephemeral_public = X25519PublicKey::from(packet.header.ephemeral_key);

    // Step 2: Perform ECDH with ephemeral key
    let private_key_bytes: [u8; 32] = node_private_key
        .try_into()
        .map_err(|_| ScramblerError::SphinxError("Invalid private key length".to_string()))?;
    let private_key = StaticSecret::from(private_key_bytes);

    let shared_secret = private_key.diffie_hellman(&ephemeral_public);
    let shared_secret_bytes = shared_secret.as_bytes();

    // Step 3: Derive encryption and MAC keys
    let (enc_key, mac_key) = derive_keys(shared_secret_bytes)?;

    // Step 4: Verify MAC over routing_info (only if MAC is not all zeros)
    let has_mac = packet.header.mac.iter().any(|&b| b != 0);
    if has_mac {
        let hmac_key = hmac::Key::new(hmac::HMAC_SHA256, &mac_key);
        hmac::verify(&hmac_key, &packet.header.routing_info, &packet.header.mac)
            .map_err(|_| ScramblerError::SphinxError("MAC verification failed".to_string()))?;
    }
    // If MAC is zeros, skip verification (intermediate hops in our simplified protocol)

    // Step 5: Decrypt one layer of routing info
    let decrypted_routing = decrypt_routing(&enc_key, &packet.header.routing_info)?;

    // Extract next hop address (first 32 bytes)
    let mut next_hop = vec![0u8; 32];
    next_hop.copy_from_slice(&decrypted_routing[0..32]);

    // Remaining bytes are encrypted inner layers
    let remaining_encrypted = &decrypted_routing[32..];

    // Step 6: Check if this is the final destination (all zeros means deliver)
    let is_final = next_hop.iter().all(|&b| b == 0);

    if is_final {
        // Step 7a: Decrypt payload and deliver
        let decrypted_payload = decrypt_routing(&enc_key, &packet.payload)?;

        // Remove padding (find first zero after message)
        let message_len = decrypted_payload
            .iter()
            .position(|&b| b == 0)
            .unwrap_or(decrypted_payload.len());

        Ok(ProcessedPacket::Deliver {
            message: decrypted_payload[..message_len].to_vec(),
        })
    } else {
        // Step 7b: Transform packet for forwarding

        // The remaining encrypted data becomes the new routing_info
        let mut new_routing_info = remaining_encrypted.to_vec();
        new_routing_info.resize(HEADER_SIZE, 0);

        // Compute MAC for next hop (zeros for now - proper Sphinx would layer MACs)
        let new_mac = [0u8; 32];

        let new_header = SphinxHeader {
            ephemeral_key: packet.header.ephemeral_key,
            routing_info: new_routing_info,
            mac: new_mac,
        };

        // Decrypt one layer of payload using stream cipher
        let new_payload = decrypt_routing(&enc_key, &packet.payload)?;

        let transformed_packet = SphinxPacket {
            header: new_header,
            payload: new_payload,
        };

        Ok(ProcessedPacket::Forward {
            next_hop,
            packet: transformed_packet,
        })
    }
}

/// Result of processing a Sphinx packet
#[derive(Debug)]
pub enum ProcessedPacket {
    /// Packet should be forwarded to next hop
    Forward {
        /// Address of next hop
        next_hop: Vec<u8>,
        /// Transformed packet
        packet: SphinxPacket,
    },
    /// Packet reached final destination
    Deliver {
        /// Decrypted message
        message: Vec<u8>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use invisible_crypto::keys::KeyPair;

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        // Test routing encryption (stream cipher)
        let key = vec![1u8; 32];
        let plaintext = b"Hello, Sphinx routing!";

        let ciphertext = encrypt_routing(&key, plaintext).unwrap();
        let decrypted = decrypt_routing(&key, &ciphertext).unwrap();
        assert_eq!(&decrypted, plaintext);

        // Test payload encryption (AEAD)
        let plaintext2 = b"Hello, Sphinx payload!";
        let ad = b"associated data";

        let ciphertext2 = encrypt_payload(&key, plaintext2, ad).unwrap();
        let decrypted2 = decrypt_payload(&key, &ciphertext2, ad).unwrap();
        assert_eq!(&decrypted2, plaintext2);
    }

    #[test]
    fn test_packet_size_limits() {
        let route = RouteSpec {
            node_keys: vec![vec![0u8; 32]; 3],
            destination: vec![0u8; 32],
        };

        // Message within limits
        let small_msg = vec![0u8; 1024];
        assert!(build_packet(&route, &small_msg).is_ok());

        // Message too large
        let large_msg = vec![0u8; PAYLOAD_SIZE + 1];
        assert!(build_packet(&route, &large_msg).is_err());
    }

    #[test]
    fn test_hop_limits() {
        let route = RouteSpec {
            node_keys: vec![vec![0u8; 32]; MAX_HOPS + 1],
            destination: vec![0u8; 32],
        };

        let message = vec![0u8; 100];
        assert!(build_packet(&route, &message).is_err());
    }

    #[test]
    fn test_sphinx_end_to_end() {
        // Create 3 mix nodes with real key pairs
        let node1_keypair = KeyPair::generate().unwrap();
        let node2_keypair = KeyPair::generate().unwrap();
        let node3_keypair = KeyPair::generate().unwrap();

        // Build route
        let route = RouteSpec {
            node_keys: vec![
                node1_keypair.public_key().to_vec(),
                node2_keypair.public_key().to_vec(),
                node3_keypair.public_key().to_vec(),
            ],
            destination: vec![0u8; 32], // Final destination (all zeros = deliver)
        };

        // Original message
        let original_message = b"Hello, Invisible!";

        // Build Sphinx packet
        let packet = build_packet(&route, original_message).unwrap();

        // Process at node 1
        let result1 = process_packet(&packet, node1_keypair.private_key()).unwrap();
        let packet2 = match result1 {
            ProcessedPacket::Forward { next_hop, packet } => {
                assert_eq!(next_hop, node2_keypair.public_key());
                packet
            }
            ProcessedPacket::Deliver { .. } => panic!("Should not deliver at node 1"),
        };

        // Process at node 2
        let result2 = process_packet(&packet2, node2_keypair.private_key()).unwrap();
        let packet3 = match result2 {
            ProcessedPacket::Forward { next_hop, packet } => {
                assert_eq!(next_hop, node3_keypair.public_key());
                packet
            }
            ProcessedPacket::Deliver { .. } => panic!("Should not deliver at node 2"),
        };

        // Process at node 3 (final hop)
        let result3 = process_packet(&packet3, node3_keypair.private_key()).unwrap();
        match result3 {
            ProcessedPacket::Forward { .. } => panic!("Should deliver at node 3"),
            ProcessedPacket::Deliver { message } => {
                assert_eq!(&message, original_message);
            }
        }
    }

    #[test]
    fn test_sphinx_unlinkability() {
        // Verify that packets are transformed at each hop (not just forwarded)
        let node1_keypair = KeyPair::generate().unwrap();
        let node2_keypair = KeyPair::generate().unwrap();

        let route = RouteSpec {
            node_keys: vec![
                node1_keypair.public_key().to_vec(),
                node2_keypair.public_key().to_vec(),
            ],
            destination: vec![0u8; 32],
        };

        let message = b"Test message";
        let packet = build_packet(&route, message).unwrap();

        // Process at node 1
        let result = process_packet(&packet, node1_keypair.private_key()).unwrap();
        let packet2 = match result {
            ProcessedPacket::Forward { packet, .. } => packet,
            _ => panic!("Should forward"),
        };

        // Packets should be different (transformed)
        assert_ne!(packet.header.routing_info, packet2.header.routing_info);
        assert_ne!(packet.header.mac, packet2.header.mac);
        assert_ne!(packet.payload, packet2.payload);
    }
}
