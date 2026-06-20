//! Encode at 10-bit depth using [`avif::EncoderConfig`].
//!
//! Bit depth is the one encoder setting without a builder method, so it's set by constructing an
//! [`avif::EncoderConfig`] and passing it to [`avif::AvifEncoder::new_with_config`]. After encoding we probe the output
//! to confirm the stored depth is 10-bit.
//!
//! Run with:
//!
//! ```text
//! cargo run --example high_bit_depth
//! ```

use std::error::Error;

use avif::{AvifEncoder, BitDepth, EncoderConfig};

const SOURCE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/image.jpg");

fn main() -> Result<(), Box<dyn Error>> {
    let img = image::open(SOURCE)?;

    let config = EncoderConfig {
        bit_depth: BitDepth::Ten,
        ..Default::default()
    };

    let mut bytes = Vec::new();
    img.write_with_encoder(AvifEncoder::new_with_config(&mut bytes, config))?;

    // Confirm the encoded stream really is 10-bit.
    let info = avif::probe(&bytes)?;
    println!("encoded {} bytes at {:?}", bytes.len(), info.bit_depth);

    let out = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/high-bit-depth-example.avif");
    std::fs::write(out, &bytes)?;
    println!("saved -> {}", out);
    Ok(())
}
