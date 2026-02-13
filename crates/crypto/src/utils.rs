//! Cryptographic utility functions
//!
//! Provides constant-time operations and other security-critical utilities.

use subtle::ConstantTimeEq;
use zeroize::Zeroize;

use crate::error::{CryptoError, Result};

/// Constant-time comparison of byte slices
///
/// # Security
/// Uses constant-time comparison to prevent timing attacks
pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.ct_eq(b).into()
}

/// Securely compare two byte slices and return Result
///
/// Returns Ok(()) if equal, Err if not equal or different lengths
pub fn verify_constant_time(a: &[u8], b: &[u8]) -> Result<()> {
    if constant_time_eq(a, b) {
        Ok(())
    } else {
        Err(CryptoError::AuthenticationFailed)
    }
}

/// Generate random bytes using the system's secure random number generator
pub fn random_bytes(len: usize) -> Result<Vec<u8>> {
    use ring::rand::{SecureRandom, SystemRandom};

    let rng = SystemRandom::new();
    let mut bytes = vec![0u8; len];
    rng.fill(&mut bytes)
        .map_err(|e| CryptoError::CryptoError(format!("Random generation failed: {:?}", e)))?;

    Ok(bytes)
}

/// Concatenate multiple byte slices into a single Vec
///
/// Useful for building messages to sign or authenticate
pub fn concat(slices: &[&[u8]]) -> Vec<u8> {
    slices.iter().flat_map(|s| s.iter()).copied().collect()
}

/// Split a byte slice into chunks of a specific size
///
/// Returns Vec of slices, last chunk may be shorter
pub fn chunk_bytes(data: &[u8], chunk_size: usize) -> Vec<&[u8]> {
    data.chunks(chunk_size).collect()
}

/// Secure key wrapper that zeroizes on drop
#[derive(Clone, Zeroize)]
#[zeroize(drop)]
pub struct SecureKey {
    key: Vec<u8>,
}

impl SecureKey {
    /// Create a new secure key from bytes
    pub fn new(key: Vec<u8>) -> Self {
        Self { key }
    }

    /// Get a reference to the key bytes
    pub fn as_bytes(&self) -> &[u8] {
        &self.key
    }

    /// Get the key length
    pub fn len(&self) -> usize {
        self.key.len()
    }

    /// Check if the key is empty
    pub fn is_empty(&self) -> bool {
        self.key.is_empty()
    }
}

impl std::fmt::Debug for SecureKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SecureKey")
            .field("len", &self.key.len())
            .finish()
    }
}

/// Convert hex string to bytes
pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>> {
    hex::decode(hex).map_err(|e| CryptoError::InvalidKey(format!("Invalid hex: {}", e)))
}

/// Convert bytes to hex string
pub fn bytes_to_hex(bytes: &[u8]) -> String {
    hex::encode(bytes)
}

/// Convert base64 string to bytes
pub fn base64_to_bytes(b64: &str) -> Result<Vec<u8>> {
    use base64::Engine;
    base64::engine::general_purpose::STANDARD
        .decode(b64)
        .map_err(|e| CryptoError::InvalidKey(format!("Invalid base64: {}", e)))
}

/// Convert bytes to base64 string
pub fn bytes_to_base64(bytes: &[u8]) -> String {
    use base64::Engine;
    base64::engine::general_purpose::STANDARD.encode(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant_time_eq() {
        let a = b"test";
        let b = b"test";
        let c = b"fail";

        assert!(constant_time_eq(a, b));
        assert!(!constant_time_eq(a, c));
        assert!(!constant_time_eq(a, b"different length"));
    }

    #[test]
    fn test_verify_constant_time() {
        let a = b"test";
        let b = b"test";
        let c = b"fail";

        assert!(verify_constant_time(a, b).is_ok());
        assert!(verify_constant_time(a, c).is_err());
    }

    #[test]
    fn test_random_bytes() {
        let bytes1 = random_bytes(32).unwrap();
        let bytes2 = random_bytes(32).unwrap();

        assert_eq!(bytes1.len(), 32);
        assert_eq!(bytes2.len(), 32);
        assert_ne!(bytes1, bytes2); // Should be different
    }

    #[test]
    fn test_concat() {
        let a = b"hello";
        let b = b" ";
        let c = b"world";

        let result = concat(&[a, b, c]);
        assert_eq!(result, b"hello world");
    }

    #[test]
    fn test_chunk_bytes() {
        let data = b"0123456789";
        let chunks = chunk_bytes(data, 3);

        assert_eq!(chunks.len(), 4);
        assert_eq!(chunks[0], b"012");
        assert_eq!(chunks[1], b"345");
        assert_eq!(chunks[2], b"678");
        assert_eq!(chunks[3], b"9");
    }

    #[test]
    fn test_secure_key() {
        let key_data = vec![1, 2, 3, 4];
        let key = SecureKey::new(key_data.clone());

        assert_eq!(key.as_bytes(), &[1, 2, 3, 4]);
        assert_eq!(key.len(), 4);
        assert!(!key.is_empty());

        // Key should be zeroized when dropped
        drop(key);
    }

    #[test]
    fn test_hex_conversion() {
        let bytes = vec![0xde, 0xad, 0xbe, 0xef];
        let hex = bytes_to_hex(&bytes);
        assert_eq!(hex, "deadbeef");

        let decoded = hex_to_bytes(&hex).unwrap();
        assert_eq!(decoded, bytes);
    }

    #[test]
    fn test_base64_conversion() {
        let bytes = b"hello world";
        let b64 = bytes_to_base64(bytes);

        let decoded = base64_to_bytes(&b64).unwrap();
        assert_eq!(decoded, bytes);
    }
}
