//! Decode an AVIF file into an image and save it in another format.
//!
//! Reads the bundled AVIF, decodes it to a [`image::DynamicImage`] with [`avif::decode`], and saves it as a PNG in the
//! `assets/` directory.
//!
//! Run with:
//!
//! ```text
//! cargo run --example decode
//! ```

use std::error::Error;

const SOURCE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/image.avif");

fn main() -> Result<(), Box<dyn Error>> {
    let bytes = std::fs::read(SOURCE)?;

    // Decode AVIF bytes into a DynamicImage.
    let img = avif::decode(&bytes)?;

    // `image` infers the output format from the file extension.
    let out = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/decode-example.png");
    img.save(out)?;

    println!("decoded to PNG -> {}", out);
    Ok(())
}
