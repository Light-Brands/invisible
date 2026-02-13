//! # Invisible Client FFI
//!
//! C-compatible FFI bindings for Dart/Flutter.
//!
//! This library exposes Invisible's core functionality through a C API
//! that can be called from Dart via dart:ffi.

#![allow(clippy::missing_safety_doc)]

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::Mutex;

use once_cell::sync::Lazy;

mod messaging;
mod wallet;
mod network;

pub use messaging::*;
pub use wallet::*;
pub use network::*;

/// Global runtime for async operations
static RUNTIME: Lazy<tokio::runtime::Runtime> = Lazy::new(|| {
    tokio::runtime::Runtime::new().expect("Failed to create tokio runtime")
});

/// Initialize the Invisible client library
///
/// Must be called before any other functions.
///
/// # Safety
/// This function is safe to call multiple times.
#[no_mangle]
pub extern "C" fn invisible_init() -> i32 {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("invisible=debug")
        .try_init()
        .ok();

    tracing::info!("Invisible client library initialized");
    0 // Success
}

/// Get library version
///
/// Returns a null-terminated string containing the version.
/// The caller must NOT free the returned string.
///
/// # Safety
/// The returned pointer is valid for the lifetime of the program.
#[no_mangle]
pub extern "C" fn invisible_version() -> *const c_char {
    static VERSION: Lazy<CString> = Lazy::new(|| {
        CString::new(env!("CARGO_PKG_VERSION")).expect("Version string invalid")
    });
    VERSION.as_ptr()
}

/// Free a string allocated by the library
///
/// # Safety
/// The pointer must have been allocated by this library via CString::into_raw()
#[no_mangle]
pub unsafe extern "C" fn invisible_free_string(s: *mut c_char) {
    if !s.is_null() {
        let _ = CString::from_raw(s);
    }
}

/// Convert Rust string to C string
fn to_c_string(s: String) -> *mut c_char {
    CString::new(s)
        .expect("String contains null byte")
        .into_raw()
}

/// Convert C string to Rust string
///
/// # Safety
/// The pointer must be valid and point to a null-terminated string
unsafe fn from_c_string(s: *const c_char) -> Option<String> {
    if s.is_null() {
        return None;
    }
    CStr::from_ptr(s).to_str().ok().map(|s| s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let result = invisible_init();
        assert_eq!(result, 0);
    }

    #[test]
    fn test_version() {
        let version = unsafe {
            let ptr = invisible_version();
            assert!(!ptr.is_null());
            CStr::from_ptr(ptr).to_str().unwrap()
        };
        assert!(!version.is_empty());
    }
}
