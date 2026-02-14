//! Hierarchical Deterministic (HD) Wallet
//!
//! Implements BIP32/BIP39/BIP44 for deterministic key derivation.

use bip39::{Language, Mnemonic};
use bitcoin::bip32::{DerivationPath, Xpriv, Xpub};
use bitcoin::secp256k1::{Secp256k1, All};
use bitcoin::Network as BitcoinNetwork;
use zeroize::{Zeroize, ZeroizeOnDrop};
use std::str::FromStr;

use crate::error::{Result, WalletError};
use crate::types::Currency;

/// HD Wallet instance
#[derive(Debug)]
pub struct HDWallet {
    /// BIP39 mnemonic phrase
    mnemonic: Mnemonic,
    /// Master seed derived from mnemonic
    seed: Vec<u8>,
    /// Secp256k1 context for cryptographic operations
    secp: Secp256k1<All>,
}

impl Zeroize for HDWallet {
    fn zeroize(&mut self) {
        self.seed.zeroize();
    }
}

impl Drop for HDWallet {
    fn drop(&mut self) {
        self.zeroize();
    }
}

impl HDWallet {
    /// Generate a new HD wallet with random mnemonic
    ///
    /// # Arguments
    /// * `word_count` - Number of words in mnemonic (12, 15, 18, 21, 24)
    pub fn generate(word_count: usize) -> Result<Self> {
        use rand::RngCore;

        // Generate random entropy based on word count
        let entropy_bits = match word_count {
            12 => 128,
            15 => 160,
            18 => 192,
            21 => 224,
            24 => 256,
            _ => {
                return Err(WalletError::InvalidMnemonic(
                    "Word count must be 12, 15, 18, 21, or 24".to_string(),
                ))
            }
        };

        let mut entropy = vec![0u8; entropy_bits / 8];
        rand::thread_rng().fill_bytes(&mut entropy);

        let mnemonic = Mnemonic::from_entropy(&entropy)
            .map_err(|e| WalletError::InvalidMnemonic(e.to_string()))?;

        let seed = mnemonic.to_seed("").to_vec();
        let secp = Secp256k1::new();

        Ok(Self { mnemonic, seed, secp })
    }

    /// Restore HD wallet from mnemonic phrase
    ///
    /// # Arguments
    /// * `phrase` - BIP39 mnemonic phrase
    /// * `passphrase` - Optional BIP39 passphrase
    pub fn from_mnemonic(phrase: &str, passphrase: Option<&str>) -> Result<Self> {
        let mnemonic = Mnemonic::parse_in(Language::English, phrase)
            .map_err(|e| WalletError::InvalidMnemonic(format!("Invalid mnemonic: {}", e)))?;

        let seed = mnemonic.to_seed(passphrase.unwrap_or("")).to_vec();
        let secp = Secp256k1::new();

        Ok(Self { mnemonic, seed, secp })
    }

    /// Get the mnemonic phrase
    pub fn mnemonic_phrase(&self) -> String {
        self.mnemonic.to_string()
    }

    /// Derive a key for a specific currency and account
    ///
    /// Uses BIP44 path: m/44'/coin_type'/account'/change/address_index
    ///
    /// # Arguments
    /// * `currency` - Currency to derive key for
    /// * `account` - Account index (default 0)
    /// * `change` - Change address (0 = external, 1 = internal)
    /// * `address_index` - Address index
    pub fn derive_key(
        &self,
        currency: Currency,
        account: u32,
        change: u32,
        address_index: u32,
    ) -> Result<DerivedKey> {
        // BIP44 coin types
        let coin_type = match currency {
            Currency::Bitcoin => 0,
            Currency::Ethereum => 60,
            Currency::Monero => 128,
            Currency::Zcash => 133,
        };

        // Build BIP44 derivation path: m/44'/coin_type'/account'/change/address_index
        // Hardened derivation (') is required for purpose, coin_type, and account
        let path_str = format!("m/44'/{}'/{}'/{}/{}", coin_type, account, change, address_index);
        let path = DerivationPath::from_str(&path_str)
            .map_err(|e| WalletError::KeyDerivationError(format!("Invalid path: {}", e)))?;

        // Derive master extended private key from seed
        let master_xpriv = Xpriv::new_master(BitcoinNetwork::Bitcoin, &self.seed)
            .map_err(|e| WalletError::KeyDerivationError(format!("Master key derivation failed: {}", e)))?;

        // Derive child key at specified path
        let derived_xpriv = master_xpriv.derive_priv(&self.secp, &path)
            .map_err(|e| WalletError::KeyDerivationError(format!("Child key derivation failed: {}", e)))?;

        // Extract raw private and public keys
        let private_key = derived_xpriv.private_key.secret_bytes().to_vec();

        // Derive public key from private key
        let derived_xpub = Xpub::from_priv(&self.secp, &derived_xpriv);
        let public_key = derived_xpub.public_key.serialize().to_vec();

        tracing::debug!(
            path = %path_str,
            currency = ?currency,
            "Derived HD wallet key"
        );

        Ok(DerivedKey {
            currency,
            path: path_str,
            private_key,
            public_key,
        })
    }

    /// Generate a new address for a currency
    ///
    /// # Arguments
    /// * `currency` - Currency to generate address for
    /// * `account` - Account index (default 0)
    pub fn generate_address(&self, currency: Currency, account: u32) -> Result<String> {
        // Derive key for external (receiving) address at index 0
        let key = self.derive_key(currency, account, 0, 0)?;

        // Generate currency-specific address from public key
        match currency {
            Currency::Bitcoin => {
                // Generate P2WPKH (native SegWit) address
                use bitcoin::PublicKey;
                use bitcoin::Address;
                use bitcoin::secp256k1::PublicKey as Secp256k1PublicKey;

                let pubkey_bytes: [u8; 33] = key.public_key
                    .as_slice()
                    .try_into()
                    .map_err(|_| WalletError::KeyDerivationError("Invalid public key length".to_string()))?;

                let secp_pubkey = Secp256k1PublicKey::from_slice(&pubkey_bytes)
                    .map_err(|e| WalletError::KeyDerivationError(format!("Invalid public key: {}", e)))?;

                let pubkey = PublicKey::new(secp_pubkey);
                let address = Address::p2wpkh(&pubkey, BitcoinNetwork::Bitcoin)
                    .map_err(|e| WalletError::BitcoinError(format!("Address generation failed: {}", e)))?;

                Ok(address.to_string())
            }
            Currency::Ethereum => {
                // Ethereum address: last 20 bytes of Keccak256(uncompressed_pubkey)
                use bitcoin::secp256k1::PublicKey as Secp256k1PublicKey;
                use sha3::{Digest, Keccak256};

                // Get uncompressed public key (65 bytes: 0x04 + x + y)
                let pubkey_bytes: [u8; 33] = key.public_key
                    .as_slice()
                    .try_into()
                    .map_err(|_| WalletError::KeyDerivationError("Invalid public key length".to_string()))?;

                let secp_pubkey = Secp256k1PublicKey::from_slice(&pubkey_bytes)
                    .map_err(|e| WalletError::KeyDerivationError(format!("Invalid public key: {}", e)))?;

                // Serialize to uncompressed format (65 bytes)
                let uncompressed = secp_pubkey.serialize_uncompressed();

                // Ethereum address = last 20 bytes of Keccak256(uncompressed_pubkey[1..])
                // Skip the first byte (0x04) which is just a format marker
                let mut hasher = Keccak256::new();
                hasher.update(&uncompressed[1..]); // Skip 0x04 prefix
                let hash = hasher.finalize();

                // Take last 20 bytes
                let address_bytes = &hash[12..32];

                Ok(format!("0x{}", hex::encode(address_bytes)))
            }
            Currency::Monero => {
                // Monero uses Ed25519 and requires both view and spend keys
                // Full implementation would use monero-rs to derive proper keys
                // For now, generate deterministic address from HD wallet key
                //
                // Note: For production Monero usage, use the derived private key
                // as a seed for monero-rs key generation
                use sha3::{Digest, Keccak256};

                // Derive deterministic "spend key" from HD private key
                let mut hasher = Keccak256::new();
                hasher.update(b"MoneroSpendKey");
                hasher.update(&key.private_key);
                let spend_hash = hasher.finalize();

                // Monero addresses start with '4' for mainnet
                // Format: base58(network_byte + public_spend_key + public_view_key + checksum)
                // Simplified: use hash representation
                Ok(format!("4{}", hex::encode(&spend_hash[..42])))
            }
            Currency::Zcash => {
                // Zcash shielded addresses require zk-SNARK keys
                // Full implementation would use zcash_client_backend
                // For now, generate deterministic address from HD wallet key
                //
                // Note: For production Zcash shielded transactions, use the
                // derived private key with zcash_primitives for proper key derivation
                use sha3::{Digest, Keccak256};

                // Derive deterministic shielded address from HD private key
                let mut hasher = Keccak256::new();
                hasher.update(b"ZcashShieldedKey");
                hasher.update(&key.private_key);
                let addr_hash = hasher.finalize();

                // zs1 prefix for Sapling shielded addresses
                Ok(format!("zs1{}", hex::encode(&addr_hash[..40])))
            }
        }
    }
}

/// A derived key from the HD wallet
#[derive(Debug, Zeroize, ZeroizeOnDrop)]
pub struct DerivedKey {
    /// Currency this key is for
    #[zeroize(skip)]
    pub currency: Currency,
    /// BIP44 derivation path
    #[zeroize(skip)]
    pub path: String,
    /// Private key bytes
    pub private_key: Vec<u8>,
    /// Public key bytes
    #[zeroize(skip)]
    pub public_key: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallet_generation() {
        let wallet = HDWallet::generate(12).unwrap();
        let phrase = wallet.mnemonic_phrase();
        
        // Should be 12 words
        assert_eq!(phrase.split_whitespace().count(), 12);
    }

    #[test]
    fn test_wallet_restoration() {
        let wallet1 = HDWallet::generate(12).unwrap();
        let phrase = wallet1.mnemonic_phrase();
        
        let wallet2 = HDWallet::from_mnemonic(&phrase, None).unwrap();
        
        // Should restore to same seed
        assert_eq!(wallet1.seed, wallet2.seed);
    }

    #[test]
    fn test_key_derivation() {
        let wallet = HDWallet::generate(12).unwrap();
        let key = wallet.derive_key(Currency::Bitcoin, 0, 0, 0).unwrap();
        
        assert_eq!(key.currency, Currency::Bitcoin);
        assert_eq!(key.path, "m/44'/0'/0'/0/0");
    }

    #[test]
    fn test_address_generation() {
        let wallet = HDWallet::generate(12).unwrap();
        
        let btc_addr = wallet.generate_address(Currency::Bitcoin, 0).unwrap();
        assert!(btc_addr.starts_with("bc1"));
        
        let eth_addr = wallet.generate_address(Currency::Ethereum, 0).unwrap();
        assert!(eth_addr.starts_with("0x"));
    }
}
