//! Wallet FFI
//!
//! C API for cryptocurrency wallet operations.

use std::os::raw::c_char;
use std::sync::Mutex;

use once_cell::sync::Lazy;

use crate::{to_c_string, from_c_string};

/// Wallet instance
static WALLET: Lazy<Mutex<Option<WalletState>>> = Lazy::new(|| Mutex::new(None));

struct WalletState {
    btc_balance: u64,
    xmr_balance: u64,
}

/// Create a new wallet
///
/// # Returns
/// * 0 on success, -1 on failure
#[no_mangle]
pub extern "C" fn invisible_wallet_create() -> i32 {
    tracing::info!("Creating wallet");

    // TODO: Create actual wallet with invisible-wallet crate
    let mut wallet = WALLET.lock().unwrap();
    *wallet = Some(WalletState {
        btc_balance: 0,
        xmr_balance: 0,
    });

    0 // Success
}

/// Get wallet balance for a currency
///
/// # Arguments
/// * `currency` - Currency code ("BTC", "XMR", "ZEC", "ETH")
///
/// # Returns
/// * Balance in satoshis/smallest unit, -1 on error
///
/// # Safety
/// The currency pointer must be a valid null-terminated string
#[no_mangle]
pub unsafe extern "C" fn invisible_wallet_get_balance(currency: *const c_char) -> i64 {
    let currency_str = match from_c_string(currency) {
        Some(s) => s,
        None => return -1,
    };

    let wallet = WALLET.lock().unwrap();
    let state = match wallet.as_ref() {
        Some(w) => w,
        None => return -1,
    };

    match currency_str.as_str() {
        "BTC" => state.btc_balance as i64,
        "XMR" => state.xmr_balance as i64,
        _ => 0,
    }
}

/// Send cryptocurrency
///
/// # Arguments
/// * `currency` - Currency code
/// * `to_address` - Recipient address
/// * `amount` - Amount in satoshis/smallest unit
///
/// # Returns
/// * Transaction ID as null-terminated string, null on failure
///
/// # Safety
/// Both pointers must be valid null-terminated strings.
/// The returned pointer must be freed with invisible_free_string()
#[no_mangle]
pub unsafe extern "C" fn invisible_wallet_send(
    currency: *const c_char,
    to_address: *const c_char,
    amount: u64,
) -> *mut c_char {
    let currency_str = match from_c_string(currency) {
        Some(s) => s,
        None => return std::ptr::null_mut(),
    };

    let address = match from_c_string(to_address) {
        Some(s) => s,
        None => return std::ptr::null_mut(),
    };

    tracing::info!(
        "Sending {} {} to {}",
        amount,
        currency_str,
        address
    );

    // TODO: Actual transaction creation and broadcasting
    let txid = format!("tx_{}_placeholder", currency_str.to_lowercase());
    to_c_string(txid)
}

/// Get receiving address for a currency
///
/// # Arguments
/// * `currency` - Currency code
///
/// # Returns
/// * Address as null-terminated string, null on failure
///
/// # Safety
/// The currency pointer must be a valid null-terminated string.
/// The returned pointer must be freed with invisible_free_string()
#[no_mangle]
pub unsafe extern "C" fn invisible_wallet_get_address(
    currency: *const c_char,
) -> *mut c_char {
    let currency_str = match from_c_string(currency) {
        Some(s) => s,
        None => return std::ptr::null_mut(),
    };

    // TODO: Get actual address from wallet
    let address = match currency_str.as_str() {
        "BTC" => "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh",
        "XMR" => "4AdUndXHHZ6cfufTMvppY6JwXNouMBzSkbLYfpAV5Usx...",
        _ => return std::ptr::null_mut(),
    };

    to_c_string(address.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::{CStr, CString};

    #[test]
    fn test_wallet_create() {
        let result = invisible_wallet_create();
        assert_eq!(result, 0);
    }

    #[test]
    fn test_wallet_balance() {
        invisible_wallet_create();

        let currency = CString::new("BTC").unwrap();
        unsafe {
            let balance = invisible_wallet_get_balance(currency.as_ptr());
            assert_eq!(balance, 0);
        }
    }

    #[test]
    fn test_wallet_address() {
        invisible_wallet_create();

        let currency = CString::new("BTC").unwrap();
        unsafe {
            let addr_ptr = invisible_wallet_get_address(currency.as_ptr());
            assert!(!addr_ptr.is_null());

            let addr_str = CStr::from_ptr(addr_ptr).to_str().unwrap();
            assert!(addr_str.starts_with("bc1"));

            crate::invisible_free_string(addr_ptr);
        }
    }
}
