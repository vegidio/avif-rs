//! Encode with custom settings via the [`avif::AvifEncoder`] builder.
//!
//! Instead of the one-line [`avif::encode`] facade, use `AvifEncoder` directly (through `image`'s `ImageEncoder` trait)
//! to tune quality, speed, and threading. Builder methods: `with_quality` / `with_quality_alpha`
//! (0–100, higher = better), `with_speed` (0–10, slower = better compression), `with_threads`, `with_bit_depth`.
//!
//! Run with:
//!
//! ```text
//! cargo run --example custom_encoder
//! ```

use std::error::Error;

use avif::AvifEncoder;

const SOURCE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/image.jpg");

fn main() -> Result<(), Box<dyn Error>> {
    let img = image::open(SOURCE)?;

    let mut bytes = Vec::new();
    img.write_with_encoder(
        AvifEncoder::new(&mut bytes)
            .with_quality(80) // 0–100, higher = better quality
            .with_speed(4)    // 0–10, slower = better compression
            .with_threads(4), // worker threads (omit for auto-detect)
    )?;

    let out = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/custom-encoder-example.avif");
    std::fs::write(out, &bytes)?;

    println!("encoded {} bytes (quality 80, speed 4) -> {}", bytes.len(), out);
    Ok(())
}
