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
#![allow(unsafe_code)] // Allowing unsafe code explicitly

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

use std::ffi::CStr;
use std::ptr;
use tokio::runtime::Runtime;

/// Creates a new session instance
#[no_mangle]
pub extern "C" fn sf_session_new(username: *const u8, password: *const u8, server_url: *const u8) -> *mut Session {
    let user = unsafe {
        if username.is_null() {
            return ptr::null_mut();
        }
        CStr::from_ptr(username as *const i8)
            .to_str()
            .unwrap_or("")
            .to_string()
    };

    let pass = unsafe {
        if password.is_null() {
            return ptr::null_mut();
        }
        CStr::from_ptr(password as *const i8)
            .to_str()
            .unwrap_or("")
            .to_string()
    };

    let server = unsafe {
        if server_url.is_null() {
            return ptr::null_mut();
        }
        CStr::from_ptr(server_url as *const i8)
            .to_str()
            .unwrap_or("")
            .to_string()
    };

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

    // Use a Tokio runtime to execute the async login function
    let runtime = Runtime::new().expect("Failed to create Tokio runtime");

    match runtime.block_on(session.login()) {
        Ok(_) => true,
        Err(e) => {
            eprintln!("sf_session_login: Failed to log in: {:?}", e);
            false
        }
    }
}

/// Frees a session instance
#[no_mangle]
pub extern "C" fn sf_session_free(session: *mut Session) {
    if session.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(session));
    }
}
