//! Shadow Wallet Core
//!
//! Multi-currency wallet with privacy features.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::error::{Result, WalletError};
use crate::btc::BtcWallet;
use crate::xmr::XmrWallet;

/// Supported currencies
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Currency {
    /// Bitcoin
    BTC,
    /// Monero
    XMR,
    /// Zcash
    ZEC,
    /// Ethereum
    ETH,
}

/// Wallet configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletConfig {
    /// Wallet name
    pub name: String,
    /// Enabled currencies
    pub enabled_currencies: Vec<Currency>,
}

impl Default for WalletConfig {
    fn default() -> Self {
        Self {
            name: "Shadow Wallet".to_string(),
            enabled_currencies: vec![Currency::BTC, Currency::XMR],
        }
    }
}

/// Currency balance
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Balance {
    /// Available balance (smallest unit)
    pub available: u64,
    /// Pending balance (unconfirmed)
    pub pending: u64,
}

impl Balance {
    /// Create zero balance
    pub fn zero() -> Self {
        Self {
            available: 0,
            pending: 0,
        }
    }

    /// Total balance
    pub fn total(&self) -> u64 {
        self.available + self.pending
    }
}

/// Shadow Wallet
#[derive(Debug)]
pub struct ShadowWallet {
    config: WalletConfig,
    btc_wallet: Option<BtcWallet>,
    xmr_wallet: Option<XmrWallet>,
    balances: HashMap<Currency, Balance>,
}

impl ShadowWallet {
    /// Create new wallet
    pub fn new(config: WalletConfig) -> Result<Self> {
        let btc_wallet = if config.enabled_currencies.contains(&Currency::BTC) {
            Some(BtcWallet::new()?)
        } else {
            None
        };

        let xmr_wallet = if config.enabled_currencies.contains(&Currency::XMR) {
            Some(XmrWallet::new()?)
        } else {
            None
        };

        Ok(Self {
            config,
            btc_wallet,
            xmr_wallet,
            balances: HashMap::new(),
        })
    }

    /// Get balance for currency
    pub async fn get_balance(&mut self, currency: Currency) -> Result<Balance> {
        match currency {
            Currency::BTC => {
                if let Some(wallet) = &self.btc_wallet {
                    let balance = wallet.get_balance().await?;
                    self.balances.insert(currency, balance);
                    Ok(balance)
                } else {
                    Ok(Balance::zero())
                }
            }
            Currency::XMR => {
                if let Some(wallet) = &self.xmr_wallet {
                    let balance = wallet.get_balance().await?;
                    self.balances.insert(currency, balance);
                    Ok(balance)
                }  else {
                    Ok(Balance::zero())
                }
            }
            _ => Ok(Balance::zero()),
        }
    }

    /// Send transaction
    pub async fn send(
        &mut self,
        currency: Currency,
        to_address: &str,
        amount: u64,
    ) -> Result<String> {
        match currency {
            Currency::BTC => {
                let wallet = self.btc_wallet.as_ref()
                    .ok_or_else(|| WalletError::ConfigError("BTC not enabled".to_string()))?;
                wallet.send(to_address, amount).await
            }
            Currency::XMR => {
                let wallet = self.xmr_wallet.as_ref()
                    .ok_or_else(|| WalletError::ConfigError("XMR not enabled".to_string()))?;
                wallet.send(to_address, amount).await
            }
            _ => Err(WalletError::ConfigError(
                format!("{:?} not supported", currency)
            )),
        }
    }

    /// Get receiving address
    pub fn get_address(&self, currency: Currency) -> Result<String> {
        match currency {
            Currency::BTC => {
                let wallet = self.btc_wallet.as_ref()
                    .ok_or_else(|| WalletError::ConfigError("BTC not enabled".to_string()))?;
                Ok(wallet.get_address())
            }
            Currency::XMR => {
                let wallet = self.xmr_wallet.as_ref()
                    .ok_or_else(|| WalletError::ConfigError("XMR not enabled".to_string()))?;
                Ok(wallet.get_address())
            }
            _ => Err(WalletError::ConfigError(
                format!("{:?} not supported", currency)
            )),
        }
    }

    /// Refresh balances for all currencies
    pub async fn refresh_balances(&mut self) -> Result<()> {
        let currencies: Vec<Currency> = self.config.enabled_currencies.clone();
        for currency in currencies {
            self.get_balance(currency).await?;
        }
        Ok(())
    }

    /// Get all balances
    pub fn get_all_balances(&self) -> &HashMap<Currency, Balance> {
        &self.balances
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallet_creation() {
        let config = WalletConfig::default();
        let wallet = ShadowWallet::new(config);
        assert!(wallet.is_ok());
    }

    #[test]
    fn test_balance_operations() {
        let balance = Balance { available: 100, pending: 50 };
        assert_eq!(balance.total(), 150);

        let zero = Balance::zero();
        assert_eq!(zero.total(), 0);
    }

    #[tokio::test]
    async fn test_get_balance() {
        let config = WalletConfig::default();
        let mut wallet = ShadowWallet::new(config).unwrap();

        let balance = wallet.get_balance(Currency::BTC).await.unwrap();
        assert_eq!(balance.available, 0);
    }
}
