//! Crypto FFI bindings

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use invisible_crypto::{IdentityKey, KeyPair};

/// Generate a new identity key pair
#[no_mangle]
pub extern "C" fn crypto_generate_identity_key() -> *mut c_char {
    match IdentityKey::generate() {
        Ok(key) => {
            let json = serde_json::json!({
                "public_key": hex::encode(key.public_key()),
            });
            CString::new(json.to_string()).unwrap().into_raw()
        }
        Err(e) => {
            let error = serde_json::json!({ "error": e.to_string() });
            CString::new(error.to_string()).unwrap().into_raw()
        }
    }
}

/// Generate a random key pair
#[no_mangle]
pub extern "C" fn crypto_generate_keypair() -> *mut c_char {
    match KeyPair::generate() {
        Ok(key) => {
            let json = serde_json::json!({
                "public_key": hex::encode(key.public_key()),
            });
            CString::new(json.to_string()).unwrap().into_raw()
        }
        Err(e) => {
            let error = serde_json::json!({ "error": e.to_string() });
            CString::new(error.to_string()).unwrap().into_raw()
        }
    }
}

/// Generate random bytes
#[no_mangle]
pub unsafe extern "C" fn crypto_random_bytes(length: usize) -> *mut c_char {
    match invisible_crypto::utils::random_bytes(length) {
        Ok(bytes) => {
            let hex = hex::encode(bytes);
            CString::new(hex).unwrap().into_raw()
        }
        Err(e) => {
            let error = serde_json::json!({ "error": e.to_string() });
            CString::new(error.to_string()).unwrap().into_raw()
        }
    }
}
