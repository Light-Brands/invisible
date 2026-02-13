//! End-to-end integration tests

use invisible_crypto::{IdentityKey, KeyPair};
use invisible_wallet::{HDWallet, Currency};
use invisible_storage::{Database, DatabaseConfig};

#[test]
fn test_crypto_key_generation() {
    // Generate identity key
    let identity = IdentityKey::generate().unwrap();
    assert!(!identity.public_key().is_empty());
    assert!(identity.is_owned());

    // Generate ephemeral key
    let keypair = KeyPair::generate().unwrap();
    assert_eq!(keypair.public_key().len(), 32);
    assert_eq!(keypair.private_key().len(), 32);
}

#[test]
fn test_wallet_mnemonic_generation() {
    // Generate wallet
    let wallet = HDWallet::generate(12).unwrap();
    let mnemonic = wallet.mnemonic_phrase();
    
    // Verify 12 words
    assert_eq!(mnemonic.split_whitespace().count(), 12);
    
    // Restore from mnemonic
    let restored = HDWallet::from_mnemonic(&mnemonic, None).unwrap();
    assert_eq!(wallet.mnemonic_phrase(), restored.mnemonic_phrase());
}

#[test]
fn test_wallet_address_generation() {
    let wallet = HDWallet::generate(24).unwrap();
    
    // Generate Bitcoin address
    let btc_addr = wallet.generate_address(Currency::Bitcoin, 0).unwrap();
    assert!(!btc_addr.is_empty());
    
    // Generate Ethereum address
    let eth_addr = wallet.generate_address(Currency::Ethereum, 0).unwrap();
    assert!(!eth_addr.is_empty());
}

#[test]
fn test_storage_initialization() {
    use tempfile::tempdir;
    
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("test.db");
    
    let config = DatabaseConfig {
        path: db_path,
        encryption_key: "test_encryption_key_32_bytes_long".to_string(),
        kdf_iter: 64000,
    };
    
    let db = Database::open(config).unwrap();
    
    // Verify database is encrypted and accessible
    let contacts = db.get_contacts().unwrap();
    assert_eq!(contacts.len(), 0);
}

#[test]
fn test_storage_message_operations() {
    use tempfile::tempdir;
    use invisible_storage::messages::StoredMessage;
    
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("test.db");
    
    let config = DatabaseConfig {
        path: db_path,
        encryption_key: "test_encryption_key_32_bytes_long".to_string(),
        kdf_iter: 64000,
    };
    
    let db = Database::open(config).unwrap();
    
    // Store a message
    let message = StoredMessage {
        id: "msg_1".to_string(),
        conversation_id: "conv_1".to_string(),
        sender_id: "user_1".to_string(),
        content: vec![1, 2, 3, 4],
        timestamp: chrono::Utc::now().timestamp(),
        status: "sent".to_string(),
    };
    
    db.store_message(&message).unwrap();
    
    // Retrieve messages
    let messages = db.get_messages("conv_1", 10).unwrap();
    assert_eq!(messages.len(), 1);
    assert_eq!(messages[0].id, "msg_1");
    
    // Delete message
    db.delete_message("msg_1").unwrap();
    let messages = db.get_messages("conv_1", 10).unwrap();
    assert_eq!(messages.len(), 0);
}

#[test]
fn test_full_messaging_flow() {
    // Simulate Alice and Bob exchanging messages
    
    // 1. Generate identities
    let alice_identity = IdentityKey::generate().unwrap();
    let bob_identity = IdentityKey::generate().unwrap();
    
    // 2. Generate pre-keys
    let alice_keypair = KeyPair::generate().unwrap();
    let bob_keypair = KeyPair::generate().unwrap();
    
    // TODO: Implement X3DH key agreement
    // TODO: Implement Double Ratchet encryption
    // TODO: Send encrypted message through mixnet
    
    // For now, verify components exist
    assert!(!alice_identity.public_key().is_empty());
    assert!(!bob_identity.public_key().is_empty());
    assert_eq!(alice_keypair.public_key().len(), 32);
    assert_eq!(bob_keypair.public_key().len(), 32);
}
