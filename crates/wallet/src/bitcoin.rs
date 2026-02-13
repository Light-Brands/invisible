//! Bitcoin (BTC) wallet implementation

use crate::error::Result;
use crate::types::{Balance, Currency};

/// Bitcoin wallet client
#[derive(Debug)]
pub struct BitcoinWallet {}

impl BitcoinWallet {
    /// Create a new Bitcoin wallet
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }

    /// Get balance
    pub async fn get_balance(&self) -> Result<Balance> {
        Ok(Balance::zero(Currency::Bitcoin))
    }
}

impl Default for BitcoinWallet {
    fn default() -> Self {
        Self::new().unwrap()
    }
}
