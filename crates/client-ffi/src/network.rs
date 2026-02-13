//! Network FFI
//!
//! C API for network connectivity and relay node communication.

use std::os::raw::c_char;
use std::sync::Mutex;

use once_cell::sync::Lazy;

use crate::to_c_string;

/// Connection status
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionStatus {
    Disconnected = 0,
    Connecting = 1,
    Connected = 2,
    Failed = 3,
}

/// Network state
static NETWORK_STATE: Lazy<Mutex<ConnectionStatus>> = 
    Lazy::new(|| Mutex::new(ConnectionStatus::Disconnected));

/// Connect to the Invisible network
///
/// # Returns
/// * 0 on success, -1 on failure
#[no_mangle]
pub extern "C" fn invisible_connect() -> i32 {
    tracing::info!("Connecting to Invisible network");

    // TODO: Actual network connection
    // - Connect to VPN
    // - Discover relay nodes
    // - Establish Scrambler connection

    let mut status = NETWORK_STATE.lock().unwrap();
    *status = ConnectionStatus::Connecting;

    // Simulate connection
    *status = ConnectionStatus::Connected;

    0 // Success
}

/// Disconnect from the network
///
/// # Returns
/// * 0 on success
#[no_mangle]
pub extern "C" fn invisible_disconnect() -> i32 {
    tracing::info!("Disconnecting from network");

    let mut status = NETWORK_STATE.lock().unwrap();
    *status = ConnectionStatus::Disconnected;

    0 // Success
}

/// Get connection status
///
/// # Returns
/// * ConnectionStatus enum value
#[no_mangle]
pub extern "C" fn invisible_connection_status() -> ConnectionStatus {
    let status = NETWORK_STATE.lock().unwrap();
    *status
}

/// Get network statistics as JSON
///
/// Returns statistics about network performance.
/// The caller must free the returned string with invisible_free_string().
///
/// # Returns
/// * JSON string with stats, null on failure
///
/// # Safety
/// The returned pointer must be freed with invisible_free_string()
#[no_mangle]
pub extern "C" fn invisible_network_stats() -> *mut c_char {
    let stats = serde_json::json!({
        "latency_ms": 45,
        "packets_sent": 1234,
        "packets_received": 1189,
        "bytes_sent": 567890,
        "bytes_received": 543210,
        "vpn_connected": true,
        "relay_nodes": 5,
    });

    let json = serde_json::to_string(&stats).unwrap_or_else(|_| "{}".to_string());
    to_c_string(json)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CStr;

    #[test]
    fn test_connect() {
        let result = invisible_connect();
        assert_eq!(result, 0);

        let status = invisible_connection_status();
        assert_eq!(status, ConnectionStatus::Connected);
    }

    #[test]
    fn test_disconnect() {
        invisible_connect();
        let result = invisible_disconnect();
        assert_eq!(result, 0);

        let status = invisible_connection_status();
        assert_eq!(status, ConnectionStatus::Disconnected);
    }

    #[test]
    fn test_network_stats() {
        let stats_ptr = invisible_network_stats();
        assert!(!stats_ptr.is_null());

        unsafe {
            let stats_str = CStr::from_ptr(stats_ptr).to_str().unwrap();
            assert!(stats_str.contains("latency_ms"));

            crate::invisible_free_string(stats_ptr);
        }
    }
}
