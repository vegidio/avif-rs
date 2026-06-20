//! Print the version of the statically linked libavif library.
//!
//! This is the smallest possible call into the native library — if it prints a version string, the prebuilt static
//! binaries were downloaded and linked correctly.
//!
//! Run with:
//!
//! ```text
//! cargo run --example version
//! ```

fn main() {
    println!("linked libavif version: {}", avif::libavif_version());
}
