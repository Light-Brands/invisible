//! Ethereum (ETH) wallet implementation

use crate::error::Result;
use crate::types::{Balance, Currency};

/// Ethereum wallet client
#[derive(Debug)]
pub struct EthereumWallet {}

impl EthereumWallet {
    /// Create a new Ethereum wallet
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }

    /// Get balance
    pub async fn get_balance(&self) -> Result<Balance> {
        Ok(Balance::zero(Currency::Ethereum))
    }
}

impl Default for EthereumWallet {
    fn default() -> Self {
        Self::new().unwrap()
    }
}
