//! Common wallet types

use serde::{Deserialize, Serialize};
use std::fmt;

/// Supported cryptocurrencies
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Currency {
    /// Monero (XMR)
    Monero,
    /// Zcash (ZEC)
    Zcash,
    /// Bitcoin (BTC)
    Bitcoin,
    /// Ethereum (ETH)
    Ethereum,
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Currency::Monero => write!(f, "XMR"),
            Currency::Zcash => write!(f, "ZEC"),
            Currency::Bitcoin => write!(f, "BTC"),
            Currency::Ethereum => write!(f, "ETH"),
        }
    }
}

/// Account balance for a currency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Balance {
    /// Currency type
    pub currency: Currency,
    /// Available balance (in smallest unit)
    pub available: u64,
    /// Pending balance (unconfirmed)
    pub pending: u64,
    /// Total balance
    pub total: u64,
}

impl Balance {
    /// Create a new balance
    pub fn new(currency: Currency, available: u64, pending: u64) -> Self {
        Self {
            currency,
            available,
            pending,
            total: available + pending,
        }
    }

    /// Create a zero balance
    pub fn zero(currency: Currency) -> Self {
        Self {
            currency,
            available: 0,
            pending: 0,
            total: 0,
        }
    }
}

/// Transaction status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionStatus {
    /// Transaction pending (in mempool)
    Pending,
    /// Transaction confirmed with N blocks
    Confirmed { blocks: u32 },
    /// Transaction failed
    Failed,
}

/// Transaction direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionDirection {
    /// Incoming transaction
    Incoming,
    /// Outgoing transaction
    Outgoing,
}

/// Transaction record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    /// Transaction ID (hash)
    pub id: String,
    /// Currency
    pub currency: Currency,
    /// Direction (incoming/outgoing)
    pub direction: TransactionDirection,
    /// Amount (in smallest unit)
    pub amount: u64,
    /// Fee paid (in smallest unit)
    pub fee: u64,
    /// Status
    pub status: TransactionStatus,
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Source address (if incoming)
    pub from_address: Option<String>,
    /// Destination address (if outgoing)
    pub to_address: Option<String>,
    /// Memo/note
    pub memo: Option<String>,
}

/// Wallet address
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    /// Currency
    pub currency: Currency,
    /// Address string
    pub address: String,
    /// Label/name
    pub label: Option<String>,
    /// Derivation path (for HD wallets)
    pub derivation_path: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_balance_creation() {
        let balance = Balance::new(Currency::Bitcoin, 1000, 500);
        assert_eq!(balance.available, 1000);
        assert_eq!(balance.pending, 500);
        assert_eq!(balance.total, 1500);
    }

    #[test]
    fn test_zero_balance() {
        let balance = Balance::zero(Currency::Monero);
        assert_eq!(balance.available, 0);
        assert_eq!(balance.total, 0);
    }

    #[test]
    fn test_currency_display() {
        assert_eq!(Currency::Monero.to_string(), "XMR");
        assert_eq!(Currency::Bitcoin.to_string(), "BTC");
    }
}
