#![warn(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::print_stdout,
    clippy::print_stderr,
    missing_debug_implementations,
    clippy::pedantic,
)]
#![allow(
    clippy::redundant_closure_for_method_calls,
    clippy::wildcard_imports,
    clippy::too_many_lines,
    clippy::field_reassign_with_default,
    clippy::match_bool
)]
#![allow(unsafe_code)] // Explicitly allowing unsafe code for FFI

pub mod command;
pub mod error;
pub mod gamestate;
pub mod misc;
pub mod response;
#[cfg(feature = "session")]
pub mod session;
pub mod simulate;
#[cfg(feature = "sso")]
pub mod sso;

/// Represents the numerical ID of a player on a server.
pub type PlayerId = u32;


pub use crate::error::SFError;
pub use crate::session::{ServerConnection, Session};
pub use crate::command::Command;
pub use crate::response::Response;

use std::ffi::{CStr, CString};
use std::ptr;
use tokio::runtime::Runtime;

/// Creates a new session instance
#[no_mangle]
pub extern "C" fn sf_session_new(username: *const i8, password: *const i8, server_url: *const i8) -> *mut Session {
    let user = unsafe { CStr::from_ptr(username).to_str().unwrap_or("").to_string() };
    let pass = unsafe { CStr::from_ptr(password).to_str().unwrap_or("").to_string() };
    let server = unsafe { CStr::from_ptr(server_url).to_str().unwrap_or("").to_string() };

    let server_connection = match ServerConnection::new(&server) {
        Some(conn) => conn,
        None => return ptr::null_mut(),
    };

    let session = Session::new(&user, &pass, server_connection);
    Box::into_raw(Box::new(session))
}

/// Logs in using a session
#[no_mangle]
pub extern "C" fn sf_session_login(session: *mut Session) -> bool {
    if session.is_null() {
        return false;
    }
    let session = unsafe { &mut *session };
    let runtime = Runtime::new().expect("Failed to create Tokio runtime");
    runtime.block_on(session.login()).is_ok()
}

/// Frees a session instance
#[no_mangle]
pub extern "C" fn sf_session_free(session: *mut Session) {
    if !session.is_null() {
        unsafe { drop(Box::from_raw(session)) };
    }
}

/// Executes a command on the server
#[no_mangle]
pub extern "C" fn sf_command_execute(session: *mut Session, command_id: u32) -> bool {
    if session.is_null() {
        return false;
    }

    let command = match Command::from_id(command_id) {
        Some(cmd) => cmd,
        None => return false, // Unknown command
    };

    let session = unsafe { &mut *session };
    let runtime = Runtime::new().expect("Failed to create Tokio runtime");

    match runtime.block_on(session.send_command(command)) {
        Ok(_) => true,
        Err(e) => {
            eprintln!("sf_command_execute: Failed to execute command: {:?}", e);
            false
        }
    }
}

/// Parses a response from the server
#[no_mangle]
pub extern "C" fn sf_response_parse(response_str: *const i8) -> *mut Response {
    if response_str.is_null() {
        return ptr::null_mut();
    }
    let response_cstr = unsafe { CStr::from_ptr(response_str) };
    let response_str = response_cstr.to_str().unwrap_or("").to_string();
    match Response::parse(response_str, chrono::Local::now().naive_utc()) {
        Ok(response) => Box::into_raw(Box::new(response)),
        Err(_) => ptr::null_mut(),
    }
}

/// Frees a response object
#[no_mangle]
pub extern "C" fn sf_response_free(response: *mut Response) {
    if !response.is_null() {
        unsafe { drop(Box::from_raw(response)) };
    }
}

/// Retrieves a value from a parsed response
#[no_mangle]
pub extern "C" fn sf_response_get_value(response: *mut Response, key: *const i8) -> *const i8 {
    if response.is_null() || key.is_null() {
        return ptr::null();
    }
    let key_cstr = unsafe { CStr::from_ptr(key) };
    let key_str = key_cstr.to_str().unwrap_or("");
    let response = unsafe { &*response };
    if let Some(value) = response.values().get(key_str) {
        let c_string = CString::new(value.as_str()).unwrap();
        return c_string.into_raw();
    }
    ptr::null()
}
