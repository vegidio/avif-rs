//! Encoder-focused integration tests.

mod common;

use common::{is_avif, source_image};

#[test]
fn encode_produces_valid_avif() {
    let img = source_image();
    let bytes = avif::encode(&img).expect("encode");
    assert!(!bytes.is_empty(), "encoded output should not be empty");
    assert!(is_avif(&bytes), "output should be a valid AVIF stream");
}

/// Fires several encodes at once so their SVT-AV1 first-time RTCD init overlaps (issue #1). Every thread must produce
/// valid AVIF bytes; the fix also keeps this from flooding stderr with `Pointer "..." is set before!`.
#[test]
fn concurrent_encodes_all_succeed() {
    let img = source_image();
    let handles: Vec<_> = (0..8)
        .map(|_| {
            let img = img.clone();
            std::thread::spawn(move || avif::encode(&img).expect("encode"))
        })
        .collect();

    for handle in handles {
        let bytes = handle.join().expect("thread panicked");
        assert!(!bytes.is_empty(), "encoded output should not be empty");
        assert!(is_avif(&bytes), "output should be a valid AVIF stream");
    }
}
