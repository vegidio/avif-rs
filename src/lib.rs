//! `avif-rust` — encode and decode AVIF images via libavif.
//!
//! The libavif C library (plus its codec/support dependencies) is downloaded as a
//! prebuilt **static** library at build time and linked directly into this crate, so
//! consumers do not need libavif installed on the host. See `build.rs`.
//!
//! This crate currently exposes only the raw FFI layer ([`sys`]); the safe, ergonomic
//! encode/decode API will be added later.

mod sys;

use std::ffi::CStr;

/// Returns the version string of the linked libavif library, e.g. `"1.4.2"`.
pub fn libavif_version() -> String {
    // SAFETY: `avifVersion` returns a pointer to a static, NUL-terminated C string.
    unsafe {
        let ptr = sys::avifVersion();
        CStr::from_ptr(ptr).to_string_lossy().into_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Smoke test: calling into libavif proves the static binaries are linked and
    /// callable end-to-end.
    #[test]
    fn reports_libavif_version() {
        let version = libavif_version();
        println!("linked libavif version: {version}");
        assert!(!version.is_empty());
    }
}
