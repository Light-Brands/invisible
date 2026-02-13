//! Wallet FFI bindings

use std::ffi::CString;
use std::os::raw::c_char;

use invisible_wallet::{HDWallet, Currency};

/// Generate a new HD wallet with mnemonic
#[no_mangle]
pub extern "C" fn wallet_generate(word_count: usize) -> *mut c_char {
    match HDWallet::generate(word_count) {
        Ok(wallet) => {
            let json = serde_json::json!({
                "mnemonic": wallet.mnemonic_phrase(),
            });
            CString::new(json.to_string()).unwrap().into_raw()
        }
        Err(e) => {
            let error = serde_json::json!({ "error": e.to_string() });
            CString::new(error.to_string()).unwrap().into_raw()
        }
    }
}

/// Restore HD wallet from mnemonic
#[no_mangle]
pub unsafe extern "C" fn wallet_restore(mnemonic: *const c_char) -> *mut c_char {
    let mnemonic_str = match std::ffi::CStr::from_ptr(mnemonic).to_str() {
        Ok(s) => s,
        Err(e) => {
            let error = serde_json::json!({ "error": e.to_string() });
            return CString::new(error.to_string()).unwrap().into_raw();
        }
    };

    match HDWallet::from_mnemonic(mnemonic_str, None) {
        Ok(_wallet) => {
            let json = serde_json::json!({ "success": true });
            CString::new(json.to_string()).unwrap().into_raw()
        }
        Err(e) => {
            let error = serde_json::json!({ "error": e.to_string() });
            CString::new(error.to_string()).unwrap().into_raw()
        }
    }
}

/// Generate address for currency
#[no_mangle]
pub unsafe extern "C" fn wallet_generate_address(
    mnemonic: *const c_char,
    currency: *const c_char,
    account: u32,
) -> *mut c_char {
    let mnemonic_str = match std::ffi::CStr::from_ptr(mnemonic).to_str() {
        Ok(s) => s,
        Err(e) => {
            let error = serde_json::json!({ "error": e.to_string() });
            return CString::new(error.to_string()).unwrap().into_raw();
        }
    };

    let currency_str = match std::ffi::CStr::from_ptr(currency).to_str() {
        Ok(s) => s,
        Err(e) => {
            let error = serde_json::json!({ "error": e.to_string() });
            return CString::new(error.to_string()).unwrap().into_raw();
        }
    };

    let currency_enum = match currency_str {
        "BTC" => Currency::Bitcoin,
        "ETH" => Currency::Ethereum,
        "XMR" => Currency::Monero,
        "ZEC" => Currency::Zcash,
        _ => {
            let error = serde_json::json!({ "error": "Invalid currency" });
            return CString::new(error.to_string()).unwrap().into_raw();
        }
    };

    match HDWallet::from_mnemonic(mnemonic_str, None) {
        Ok(wallet) => match wallet.generate_address(currency_enum, account) {
            Ok(address) => {
                let json = serde_json::json!({ "address": address });
                CString::new(json.to_string()).unwrap().into_raw()
            }
            Err(e) => {
                let error = serde_json::json!({ "error": e.to_string() });
                CString::new(error.to_string()).unwrap().into_raw()
            }
        },
        Err(e) => {
            let error = serde_json::json!({ "error": e.to_string() });
            CString::new(error.to_string()).unwrap().into_raw()
        }
    }
}
