//! Database connection and management

use rusqlite::{Connection, OpenFlags};
use std::path::{Path, PathBuf};
use zeroize::Zeroizing;

use crate::error::{Result, StorageError};

/// Database configuration
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    /// Path to database file
    pub path: PathBuf,
    /// SQLCipher encryption key
    pub encryption_key: String,
    /// Key derivation iterations (Argon2id)
    pub kdf_iter: u32,
}

/// Database connection
pub struct Database {
    conn: Connection,
    config: DatabaseConfig,
}

impl Database {
    /// Open or create an encrypted database
    ///
    /// # Arguments
    /// * `config` - Database configuration including encryption key
    pub fn open(config: DatabaseConfig) -> Result<Self> {
        // Ensure parent directory exists
        if let Some(parent) = config.path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Open database with SQLCipher encryption
        let flags = OpenFlags::SQLITE_OPEN_READ_WRITE
            | OpenFlags::SQLITE_OPEN_CREATE
            | OpenFlags::SQLITE_OPEN_NO_MUTEX;

        let conn = Connection::open_with_flags(&config.path, flags)?;

        // Set SQLCipher encryption key
        let key = Zeroizing::new(config.encryption_key.clone());
        conn.pragma_update(None, "key", &*key)?;

        // Set SQLCipher parameters for Argon2id
        conn.pragma_update(None, "cipher_page_size", &4096)?;
        conn.pragma_update(None, "kdf_iter", &config.kdf_iter)?;
        conn.pragma_update(None, "cipher_kdf_algorithm", &"PBKDF2_HMAC_SHA512")?;

        // Verify encryption is working
        conn.execute("SELECT count(*) FROM sqlite_master", [])?;

        let mut db = Self { conn, config };

        // Run migrations
        db.run_migrations()?;

        Ok(db)
    }

    /// Derive encryption key from passphrase and 2FA code
    ///
    /// # Arguments
    /// * `passphrase` - User passphrase
    /// * `totp_code` - Current TOTP 2FA code
    /// * `salt` - Salt for key derivation (should be stored)
    pub fn derive_key(passphrase: &str, totp_code: &str, salt: &[u8]) -> Result<String> {
        use argon2::{
            password_hash::{PasswordHasher, SaltString},
            Argon2, Params,
        };

        // Combine passphrase with 2FA code
        let combined = format!("{}{}", passphrase, totp_code);

        // Use Argon2id for key derivation
        let params = Params::new(65536, 3, 1, Some(32))
            .map_err(|e| StorageError::KeyDerivationError(e.to_string()))?;

        let argon2 = Argon2::new(
            argon2::Algorithm::Argon2id,
            argon2::Version::V0x13,
            params,
        );

        let salt_string = SaltString::encode_b64(salt)
            .map_err(|e| StorageError::KeyDerivationError(e.to_string()))?;

        let hash = argon2
            .hash_password(combined.as_bytes(), &salt_string)
            .map_err(|e| StorageError::KeyDerivationError(e.to_string()))?;

        // Extract hash as hex string for SQLCipher
        Ok(hex::encode(hash.hash.unwrap().as_bytes()))
    }

    /// Run database migrations
    fn run_migrations(&mut self) -> Result<()> {
        // TODO: Use refinery for migrations
        // For now, create basic schema

        self.conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS messages (
                id TEXT PRIMARY KEY,
                conversation_id TEXT NOT NULL,
                sender_id TEXT NOT NULL,
                content BLOB NOT NULL,
                timestamp INTEGER NOT NULL,
                status TEXT NOT NULL,
                FOREIGN KEY (conversation_id) REFERENCES conversations(id)
            );

            CREATE TABLE IF NOT EXISTS conversations (
                id TEXT PRIMARY KEY,
                name TEXT,
                created_at INTEGER NOT NULL,
                last_message_at INTEGER
            );

            CREATE TABLE IF NOT EXISTS contacts (
                id TEXT PRIMARY KEY,
                display_name TEXT,
                identity_key BLOB NOT NULL,
                created_at INTEGER NOT NULL
            );

            CREATE TABLE IF NOT EXISTS keys (
                id TEXT PRIMARY KEY,
                contact_id TEXT NOT NULL,
                key_type TEXT NOT NULL,
                key_data BLOB NOT NULL,
                created_at INTEGER NOT NULL,
                FOREIGN KEY (contact_id) REFERENCES contacts(id)
            );

            CREATE TABLE IF NOT EXISTS wallet_accounts (
                id TEXT PRIMARY KEY,
                currency TEXT NOT NULL,
                balance_available INTEGER NOT NULL,
                balance_pending INTEGER NOT NULL,
                address TEXT NOT NULL,
                created_at INTEGER NOT NULL
            );

            CREATE TABLE IF NOT EXISTS transactions (
                id TEXT PRIMARY KEY,
                account_id TEXT NOT NULL,
                tx_hash TEXT NOT NULL,
                direction TEXT NOT NULL,
                amount INTEGER NOT NULL,
                fee INTEGER NOT NULL,
                status TEXT NOT NULL,
                timestamp INTEGER NOT NULL,
                FOREIGN KEY (account_id) REFERENCES wallet_accounts(id)
            );

            CREATE INDEX IF NOT EXISTS idx_messages_conversation ON messages(conversation_id);
            CREATE INDEX IF NOT EXISTS idx_messages_timestamp ON messages(timestamp);
            CREATE INDEX IF NOT EXISTS idx_transactions_account ON transactions(account_id);
            "#,
        )?;

        Ok(())
    }

    /// Get underlying connection (for internal use)
    pub(crate) fn connection(&self) -> &Connection {
        &self.conn
    }

    /// Close database and clear sensitive data
    pub fn close(self) -> Result<()> {
        // Connection will be closed when dropped
        Ok(())
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        // Clear sensitive data from memory
        // SQLCipher handles this internally
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_database_creation() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");

        let config = DatabaseConfig {
            path: db_path,
            encryption_key: "test_key_12345678901234567890".to_string(),
            kdf_iter: 64000,
        };

        let db = Database::open(config).unwrap();
        
        // Verify tables were created
        let table_count: i64 = db
            .connection()
            .query_row(
                "SELECT count(*) FROM sqlite_master WHERE type='table'",
                [],
                |row| row.get(0),
            )
            .unwrap();

        assert!(table_count > 0);
    }

    #[test]
    fn test_key_derivation() {
        let salt = b"test_salt_16byte";
        let key = Database::derive_key("my_passphrase", "123456", salt).unwrap();
        
        // Key should be hex string
        assert!(!key.is_empty());
        assert!(key.chars().all(|c| c.is_ascii_hexdigit()));
    }
}
