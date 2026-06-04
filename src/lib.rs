//! Rust FFI bindings for the makerom utility
//!
//! This library provides safe Rust wrappers around the C makerom library
//! for creating 3DS ROM files in various formats (CCI, CIA, CXI, CFA).

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod ffi {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use ffi::*;

/// A safe wrapper around the C user_settings structure
pub struct UserSettings {
    inner: *mut ffi::user_settings,
}

impl UserSettings {
    /// Create a new user settings instance
    pub fn new() -> Result<Self, &'static str> {
        unsafe {
            let settings = libc::calloc(1, std::mem::size_of::<ffi::user_settings>());
            if settings.is_null() {
                return Err("Failed to allocate memory for user settings");
            }
            let settings = settings as *mut ffi::user_settings;
            // ffi::init_UserSettings(settings);
            Ok(UserSettings { inner: settings })
        }
    }

    /// Get a mutable reference to the inner settings
    pub fn inner_mut(&mut self) -> *mut ffi::user_settings {
        self.inner
    }

    /// Get an immutable reference to the inner settings
    pub fn inner(&self) -> *const ffi::user_settings {
        self.inner
    }
}

impl Drop for UserSettings {
    fn drop(&mut self) {
        unsafe {
            // ffi::free_UserSettings(self.inner);
            libc::free(self.inner as *mut libc::c_void);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_settings_creation() {
        let _settings = UserSettings::new().expect("Failed to create settings");
    }
}
