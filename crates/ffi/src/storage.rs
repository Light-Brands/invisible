//! Storage FFI bindings

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::path::PathBuf;

use invisible_storage::{Database, DatabaseConfig};

use crate::CONTEXT;

/// Initialize database
#[no_mangle]
pub unsafe extern "C" fn storage_init(
    db_path: *const c_char,
    encryption_key: *const c_char,
) -> i32 {
    let path_str = match CStr::from_ptr(db_path).to_str() {
        Ok(s) => s,
        Err(_) => return -1,
    };

    let key_str = match CStr::from_ptr(encryption_key).to_str() {
        Ok(s) => s,
        Err(_) => return -1,
    };

    let config = DatabaseConfig {
        path: PathBuf::from(path_str),
        encryption_key: key_str.to_string(),
        kdf_iter: 64000,
    };

    match Database::open(config) {
        Ok(db) => {
            let mut context = CONTEXT.lock().unwrap();
            context.storage = Some(db);
            0 // Success
        }
        Err(_) => -1, // Error
    }
}

/// Store a message
#[no_mangle]
pub unsafe extern "C" fn storage_store_message(
    id: *const c_char,
    conversation_id: *const c_char,
    sender_id: *const c_char,
    content: *const u8,
    content_len: usize,
) -> i32 {
    let context = CONTEXT.lock().unwrap();
    let db = match &context.storage {
        Some(db) => db,
        None => return -1,
    };

    let id_str = match CStr::from_ptr(id).to_str() {
        Ok(s) => s.to_string(),
        Err(_) => return -1,
    };

    let conv_id_str = match CStr::from_ptr(conversation_id).to_str() {
        Ok(s) => s.to_string(),
        Err(_) => return -1,
    };

    let sender_id_str = match CStr::from_ptr(sender_id).to_str() {
        Ok(s) => s.to_string(),
        Err(_) => return -1,
    };

    let content_vec = std::slice::from_raw_parts(content, content_len).to_vec();

    let message = invisible_storage::messages::StoredMessage {
        id: id_str,
        conversation_id: conv_id_str,
        sender_id: sender_id_str,
        content: content_vec,
        timestamp: chrono::Utc::now().timestamp(),
        status: "sent".to_string(),
    };

    match db.store_message(&message) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

/// Get messages for conversation
#[no_mangle]
pub unsafe extern "C" fn storage_get_messages(
    conversation_id: *const c_char,
    limit: usize,
) -> *mut c_char {
    let context = CONTEXT.lock().unwrap();
    let db = match &context.storage {
        Some(db) => db,
        None => {
            let error = serde_json::json!({ "error": "Storage not initialized" });
            return CString::new(error.to_string()).unwrap().into_raw();
        }
    };

    let conv_id_str = match CStr::from_ptr(conversation_id).to_str() {
        Ok(s) => s,
        Err(e) => {
            let error = serde_json::json!({ "error": e.to_string() });
            return CString::new(error.to_string()).unwrap().into_raw();
        }
    };

    match db.get_messages(conv_id_str, limit) {
        Ok(messages) => {
            let json = serde_json::to_string(&messages).unwrap();
            CString::new(json).unwrap().into_raw()
        }
        Err(e) => {
            let error = serde_json::json!({ "error": e.to_string() });
            CString::new(error.to_string()).unwrap().into_raw()
        }
    }
}
