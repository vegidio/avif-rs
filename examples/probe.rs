//! Read an AVIF header without decoding its pixels using [`avif::probe`].
//!
//! Returns an [`avif::ImageInfo`] with dimensions, color type, and bit depth — useful for validation or thumbnailing
//! pipelines where you don't need the decoded image.
//!
//! Run with:
//!
//! ```text
//! cargo run --example probe
//! ```

use std::error::Error;

const SOURCE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/image.avif");

fn main() -> Result<(), Box<dyn Error>> {
    let bytes = std::fs::read(SOURCE)?;

    let info = avif::probe(&bytes)?;

    println!("dimensions: {}x{}", info.width, info.height);
    println!("color type: {:?}", info.color_type);
    println!("bit depth:  {:?}", info.bit_depth);
    Ok(())
}
