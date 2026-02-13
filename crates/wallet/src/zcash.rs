//! Zcash (ZEC) wallet implementation

use crate::error::Result;
use crate::types::{Balance, Currency};

/// Zcash wallet client
#[derive(Debug)]
pub struct ZcashWallet {}

impl ZcashWallet {
    /// Create a new Zcash wallet
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }

    /// Get balance
    pub async fn get_balance(&self) -> Result<Balance> {
        Ok(Balance::zero(Currency::Zcash))
    }
}

impl Default for ZcashWallet {
    fn default() -> Self {
        Self::new().unwrap()
    }
}
