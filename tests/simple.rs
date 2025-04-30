extern crate reed_solomon;

use reed_solomon::Decoder;
use reed_solomon::Encoder;

const ECC_LEN: usize = 8;

#[test]
fn helloworld() {
    let data = b"Hello, World!";

    // Create encoder and decoder
    let enc = Encoder::<ECC_LEN>::new();
    let dec = Decoder::<ECC_LEN>::new();

    // Encode data
    let encoded = enc.encode(&data[..]);

    // Simulate some transmission errors
    let mut corrupted = *encoded;
    for i in 0..4 {
        corrupted[i] = 0x0;
    }

    // Try to recover data
    let recovered = dec.correct(&mut corrupted, None).unwrap();

    assert_eq!(data, recovered.data());
}

#[test]
fn with_erasures() {
    let data = b"Hello, World!";

    // Create encoder and decoder
    let enc = Encoder::<ECC_LEN>::new();
    let dec = Decoder::<ECC_LEN>::new();

    // Encode data
    let encoded = enc.encode(&data[..]);

    // Simulate some transmission errors
    let mut corrupted = *encoded;
    for i in 0..4 {
        corrupted[i] = 0x0;
    }

    // Try to recover data
    let known_erasures = [0, 1, 2];
    let recovered = dec.correct(&mut corrupted, Some(&known_erasures)).unwrap();

    assert_eq!(data, recovered.data());
}
