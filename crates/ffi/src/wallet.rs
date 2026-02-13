//! Wallet FFI bindings

use std::ffi::CString;
use std::os::raw::c_char;

use invisible_wallet::{ShadowWallet, Currency};

/// Generate a new HD wallet with mnemonic
#[no_mangle]
pub extern "C" fn wallet_generate(_word_count: usize) -> *mut c_char {
    // TODO: Implement HD wallet generation
    let error = serde_json::json!({ "error": "HD wallet generation not yet implemented" });
    CString::new(error.to_string()).unwrap().into_raw()
}

/// Restore HD wallet from mnemonic
#[no_mangle]
pub unsafe extern "C" fn wallet_restore(_mnemonic: *const c_char) -> *mut c_char {
    // TODO: Implement HD wallet restore
    let error = serde_json::json!({ "error": "HD wallet restore not yet implemented" });
    CString::new(error.to_string()).unwrap().into_raw()
}

/// Generate address for currency
#[no_mangle]
pub unsafe extern "C" fn wallet_generate_address(
    _mnemonic: *const c_char,
    _currency: *const c_char,
    _account: u32,
) -> *mut c_char {
    // TODO: Implement address generation from HD wallet
    let error = serde_json::json!({ "error": "Address generation not yet implemented" });
    CString::new(error.to_string()).unwrap().into_raw()
}
