//! Key derivation functions (KDF)
//!
//! Implements HKDF (HMAC-based Key Derivation Function) using SHA-256
//! for deriving encryption keys from shared secrets.

use ring::hkdf;
use ring::hmac;

use crate::error::{CryptoError, Result};

/// Derive keys using HKDF-SHA256
///
/// # Arguments
/// * `input_key_material` - The input key material (e.g., shared secret)
/// * `salt` - Optional salt value (use None for all-zero salt)
/// * `info` - Context and application-specific information
/// * `output_len` - Length of output key material in bytes
///
/// # Security
/// - Uses SHA-256 as the hash function
/// - Suitable for deriving multiple keys from a single shared secret
/// - Ensures independence between derived keys
pub fn hkdf_sha256(
    input_key_material: &[u8],
    salt: Option<&[u8]>,
    info: &[u8],
    output_len: usize,
) -> Result<Vec<u8>> {
    let salt = hkdf::Salt::new(hkdf::HKDF_SHA256, salt.unwrap_or(&[]));
    let prk = salt.extract(input_key_material);

    let info_slice = [info];
    let okm = prk
        .expand(&info_slice, MyLen(output_len))
        .map_err(|_| CryptoError::KeyDerivationFailed("HKDF expand failed".to_string()))?;

    let mut output = vec![0u8; output_len];
    okm.fill(&mut output)
        .map_err(|_| CryptoError::KeyDerivationFailed("HKDF fill failed".to_string()))?;

    Ok(output)
}

/// Derive a root key and chain key from a shared secret (for Double Ratchet)
///
/// Returns (root_key, chain_key) tuple
pub fn kdf_rk(root_key: &[u8], dh_output: &[u8]) -> Result<(Vec<u8>, Vec<u8>)> {
    let output = hkdf_sha256(dh_output, Some(root_key), b"WhisperRatchet", 64)?;

    let root_key = output[0..32].to_vec();
    let chain_key = output[32..64].to_vec();

    Ok((root_key, chain_key))
}

/// Derive a chain key and message key from a chain key (for Double Ratchet)
///
/// Returns (chain_key, message_key) tuple
pub fn kdf_ck(chain_key: &[u8]) -> Result<(Vec<u8>, Vec<u8>)> {
    let key = hmac::Key::new(hmac::HMAC_SHA256, chain_key);

    // Derive new chain key
    let new_chain_key = hmac::sign(&key, &[0x02]);

    // Derive message key
    let message_key = hmac::sign(&key, &[0x01]);

    Ok((
        new_chain_key.as_ref().to_vec(),
        message_key.as_ref().to_vec(),
    ))
}

/// Helper struct for HKDF output length
struct MyLen(usize);

impl hkdf::KeyType for MyLen {
    fn len(&self) -> usize {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hkdf_sha256() {
        let ikm = b"input key material";
        let salt = b"salt";
        let info = b"info";

        let result = hkdf_sha256(ikm, Some(salt), info, 32).unwrap();
        assert_eq!(result.len(), 32);
    }

    #[test]
    fn test_kdf_rk() {
        let root_key = vec![0u8; 32];
        let dh_output = vec![1u8; 32];

        let (new_root, chain) = kdf_rk(&root_key, &dh_output).unwrap();
        assert_eq!(new_root.len(), 32);
        assert_eq!(chain.len(), 32);
        assert_ne!(new_root, root_key);
    }

    #[test]
    fn test_kdf_ck() {
        let chain_key = vec![0u8; 32];

        let (new_chain, msg_key) = kdf_ck(&chain_key).unwrap();
        assert_eq!(new_chain.len(), 32);
        assert_eq!(msg_key.len(), 32);
        assert_ne!(new_chain, chain_key);
        assert_ne!(msg_key, chain_key);
    }

    #[test]
    fn test_kdf_deterministic() {
        let ikm = b"test input";
        let salt = b"test salt";
        let info = b"test info";

        let result1 = hkdf_sha256(ikm, Some(salt), info, 32).unwrap();
        let result2 = hkdf_sha256(ikm, Some(salt), info, 32).unwrap();

        assert_eq!(result1, result2);
    }
}
