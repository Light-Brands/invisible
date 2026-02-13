//! Contact storage operations

use rusqlite::params;
use serde::{Deserialize, Serialize};

use crate::database::Database;
use crate::error::Result;

/// Stored contact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredContact {
    /// Contact ID
    pub id: String,
    /// Display name
    pub display_name: Option<String>,
    /// Identity key (public)
    pub identity_key: Vec<u8>,
    /// Created timestamp
    pub created_at: i64,
}

impl Database {
    /// Store a contact
    pub fn store_contact(&self, contact: &StoredContact) -> Result<()> {
        self.connection().execute(
            "INSERT OR REPLACE INTO contacts (id, display_name, identity_key, created_at)
             VALUES (?1, ?2, ?3, ?4)",
            params![
                &contact.id,
                &contact.display_name,
                &contact.identity_key,
                contact.created_at,
            ],
        )?;
        Ok(())
    }

    /// Get all contacts
    pub fn get_contacts(&self) -> Result<Vec<StoredContact>> {
        let mut stmt = self.connection().prepare(
            "SELECT id, display_name, identity_key, created_at FROM contacts ORDER BY created_at DESC",
        )?;

        let contacts = stmt
            .query_map([], |row| {
                Ok(StoredContact {
                    id: row.get(0)?,
                    display_name: row.get(1)?,
                    identity_key: row.get(2)?,
                    created_at: row.get(3)?,
                })
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;

        Ok(contacts)
    }

    /// Get contact by ID
    pub fn get_contact(&self, id: &str) -> Result<Option<StoredContact>> {
        let mut stmt = self.connection().prepare(
            "SELECT id, display_name, identity_key, created_at FROM contacts WHERE id = ?1",
        )?;

        let contact = stmt
            .query_row(params![id], |row| {
                Ok(StoredContact {
                    id: row.get(0)?,
                    display_name: row.get(1)?,
                    identity_key: row.get(2)?,
                    created_at: row.get(3)?,
                })
            })
            .optional()?;

        Ok(contact)
    }

    /// Delete a contact
    pub fn delete_contact(&self, id: &str) -> Result<()> {
        self.connection()
            .execute("DELETE FROM contacts WHERE id = ?1", params![id])?;
        Ok(())
    }
}
