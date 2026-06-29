//! Encode an image to AVIF with default settings.
//!
//! Opens the bundled JPEG, encodes it to AVIF bytes with [`avif::encode`], and writes the result to a file in the
//! `assets/` directory.
//!
//! Run with:
//!
//! ```text
//! cargo run --example encode
//! ```

use std::error::Error;

const SOURCE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/image.jpg");

fn main() -> Result<(), Box<dyn Error>> {
    // Load any format the `image` crate understands.
    let img = image::open(SOURCE)?;

    // Encode to AVIF with sensible defaults (SVT-AV1, quality 60, speed 6).
    let bytes = avif::encode(&img)?;

    let out = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/example_encode.avif");
    std::fs::write(out, &bytes)?;

    println!("encoded {} bytes -> {}", bytes.len(), out);
    Ok(())
}
