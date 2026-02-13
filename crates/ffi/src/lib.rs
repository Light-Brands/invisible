//! FFI Bindings for Flutter
//!
//! Provides safe Rust-Flutter communication for:
//! - Cryptographic operations
//! - Wallet management
//! - Message encryption/decryption
//! - Local storage

#![allow(clippy::not_unsafe_ptr_arg_deref)]

use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};

pub mod crypto;
pub mod wallet;
pub mod storage;
pub mod error;

pub use error::{FfiError, FfiResult};

/// Global FFI context
static CONTEXT: Lazy<Arc<Mutex<FfiContext>>> = Lazy::new(|| {
    Arc::new(Mutex::new(FfiContext::new()))
});

/// FFI context holding state
pub struct FfiContext {
    /// Storage database (if initialized)
    pub storage: Option<invisible_storage::Database>,
    /// Active wallets by currency
    pub wallets: std::collections::HashMap<String, Arc<Mutex<dyn std::any::Any + Send>>>,
}

impl FfiContext {
    fn new() -> Self {
        Self {
            storage: None,
            wallets: std::collections::HashMap::new(),
        }
    }
}

/// Initialize FFI subsystem
#[no_mangle]
pub extern "C" fn invisible_ffi_init() -> i32 {
    // Initialize logging
    tracing_subscriber::fmt::init();
    0 // Success
}

/// Get FFI version
#[no_mangle]
pub extern "C" fn invisible_ffi_version() -> *const std::os::raw::c_char {
    std::ffi::CString::new(env!("CARGO_PKG_VERSION"))
        .unwrap()
        .into_raw()
}

/// Free a string allocated by Rust
#[no_mangle]
pub unsafe extern "C" fn invisible_ffi_free_string(s: *mut std::os::raw::c_char) {
    if !s.is_null() {
        drop(std::ffi::CString::from_raw(s));
    }
}
