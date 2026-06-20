//! Encode several images concurrently — the *safe* way.
//!
//! The bundled SVT-AV1 keeps a global state that is set per-encode, so running concurrent encodes with **different**
//! configurations in one process corrupts that state and segfaults. Concurrent encodes that all use the **same**
//! configuration (here: defaults) are safe and were verified stable under load.
//!
//! If you need different settings across threads, serialize the encoding behind a lock instead (see the README
//! "Troubleshooting" section).
//!
//! Run with:
//!
//! ```text
//! cargo run --example parallel_encode
//! ```

use std::error::Error;
use std::thread;

const SOURCE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/image.jpg");

fn main() -> Result<(), Box<dyn Error>> {
    let img = image::open(SOURCE)?;

    // Spawn several encodes at once. Every thread uses the SAME config (defaults), which is the documented-safe
    // concurrency model.
    let handles: Vec<_> = (0..4)
        .map(|i| {
            let img = img.clone();
            thread::spawn(move || (i, avif::encode(&img).expect("encode")))
        })
        .collect();

    for handle in handles {
        let (i, bytes) = handle.join().expect("thread panicked");
        println!("thread {i}: encoded {} bytes", bytes.len());
    }

    Ok(())
}
