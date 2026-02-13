//! Messaging FFI
//!
//! C API for end-to-end encrypted messaging.

use std::os::raw::c_char;
use std::sync::Mutex;
use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::{to_c_string, from_c_string};

/// Message handle (opaque pointer)
pub type MessageHandle = usize;

/// Global message store
static MESSAGES: Lazy<Mutex<HashMap<MessageHandle, String>>> = 
    Lazy::new(|| Mutex::new(HashMap::new()));

static NEXT_HANDLE: Lazy<Mutex<MessageHandle>> = Lazy::new(|| Mutex::new(1));

/// Send an encrypted message
///
/// # Arguments
/// * `recipient_id` - Recipient's user ID (null-terminated string)
/// * `message` - Message text (null-terminated string)
///
/// # Returns
/// * Message handle on success, 0 on failure
///
/// # Safety
/// Both pointers must be valid null-terminated strings
#[no_mangle]
pub unsafe extern "C" fn invisible_send_message(
    recipient_id: *const c_char,
    message: *const c_char,
) -> MessageHandle {
    let recipient = match from_c_string(recipient_id) {
        Some(s) => s,
        None => return 0,
    };

    let msg = match from_c_string(message) {
        Some(s) => s,
        None => return 0,
    };

    tracing::info!("Sending message to {}: {}", recipient, msg);

    // TODO: Actual message encryption and sending
    // For now, just store it
    let mut messages = MESSAGES.lock().unwrap();
    let mut handle = NEXT_HANDLE.lock().unwrap();
    
    let current_handle = *handle;
    messages.insert(current_handle, format!("To {}: {}", recipient, msg));
    *handle += 1;

    current_handle
}

/// Receive messages
///
/// Returns a JSON array of messages as a null-terminated string.
/// The caller must free the returned string with invisible_free_string().
///
/// # Returns
/// * Pointer to JSON string on success, null on failure
///
/// # Safety
/// The returned pointer must be freed with invisible_free_string()
#[no_mangle]
pub extern "C" fn invisible_receive_messages() -> *mut c_char {
    tracing::debug!("Receiving messages");

    // TODO: Actual message retrieval from relay nodes
    let messages = vec![
        serde_json::json!({
            "id": "msg_001",
            "from": "user_123",
            "text": "Hello from Invisible!",
            "timestamp": 1234567890,
        }),
    ];

    let json = serde_json::to_string(&messages).unwrap_or_else(|_| "[]".to_string());
    to_c_string(json)
}

/// Get message status
///
/// # Arguments
/// * `handle` - Message handle from send_message
///
/// # Returns
/// * 0 = pending, 1 = sent, 2 = delivered, 3 = failed, -1 = invalid handle
#[no_mangle]
pub extern "C" fn invisible_message_status(handle: MessageHandle) -> i32 {
    let messages = MESSAGES.lock().unwrap();
    
    if messages.contains_key(&handle) {
        1 // Sent (placeholder)
    } else {
        -1 // Invalid handle
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::{CStr, CString};

    #[test]
    fn test_send_message() {
        let recipient = CString::new("user_123").unwrap();
        let message = CString::new("Hello").unwrap();

        unsafe {
            let handle = invisible_send_message(recipient.as_ptr(), message.as_ptr());
            assert_ne!(handle, 0);
            
            let status = invisible_message_status(handle);
            assert_eq!(status, 1);
        }
    }

    #[test]
    fn test_receive_messages() {
        let messages_ptr = invisible_receive_messages();
        assert!(!messages_ptr.is_null());

        unsafe {
            let messages_str = CStr::from_ptr(messages_ptr).to_str().unwrap();
            assert!(messages_str.contains("Hello from Invisible!"));
            
            crate::invisible_free_string(messages_ptr);
        }
    }
}
