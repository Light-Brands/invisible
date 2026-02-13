//! Wallet storage operations

use rusqlite::params;
use serde::{Deserialize, Serialize};

use crate::database::Database;
use crate::error::Result;

/// Stored wallet account
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredAccount {
    /// Account ID
    pub id: String,
    /// Currency (BTC, ETH, XMR, ZEC)
    pub currency: String,
    /// Available balance
    pub balance_available: u64,
    /// Pending balance
    pub balance_pending: u64,
    /// Primary address
    pub address: String,
    /// Created timestamp
    pub created_at: i64,
}

impl Database {
    /// Store wallet account
    pub fn store_account(&self, account: &StoredAccount) -> Result<()> {
        self.connection().execute(
            "INSERT OR REPLACE INTO wallet_accounts 
             (id, currency, balance_available, balance_pending, address, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                &account.id,
                &account.currency,
                account.balance_available,
                account.balance_pending,
                &account.address,
                account.created_at,
            ],
        )?;
        Ok(())
    }

    /// Get all wallet accounts
    pub fn get_accounts(&self) -> Result<Vec<StoredAccount>> {
        let mut stmt = self.connection().prepare(
            "SELECT id, currency, balance_available, balance_pending, address, created_at
             FROM wallet_accounts ORDER BY created_at",
        )?;

        let accounts = stmt
            .query_map([], |row| {
                Ok(StoredAccount {
                    id: row.get(0)?,
                    currency: row.get(1)?,
                    balance_available: row.get(2)?,
                    balance_pending: row.get(3)?,
                    address: row.get(4)?,
                    created_at: row.get(5)?,
                })
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;

        Ok(accounts)
    }

    /// Update account balance
    pub fn update_balance(&self, account_id: &str, available: u64, pending: u64) -> Result<()> {
        self.connection().execute(
            "UPDATE wallet_accounts SET balance_available = ?1, balance_pending = ?2 WHERE id = ?3",
            params![available, pending, account_id],
        )?;
        Ok(())
    }
}
