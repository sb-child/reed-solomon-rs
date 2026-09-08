# Reed-Solomon BCH
[![license](https://img.shields.io/github/license/mashape/apistatus.svg)]()
[![Build Status](https://travis-ci.org/mersinvald/reed-solomon-rs.svg?branch=master)](https://travis-ci.org/mersinvald/reed-solomon-rs)
[![Crates.io](https://img.shields.io/crates/v/reed-solomon.svg)](https://crates.io/crates/reed-solomon)
[![Documentation](https://docs.rs/reed-solomon/badge.svg)](https://docs.rs/reed-solomon)

Reed-Solomon BCH encoder and decoder implemented in Rust.
This is a port of python implementation from [Wikiversity](https://en.wikiversity.org/wiki/Reed–Solomon_codes_for_coders)

## Setup 

```toml
[dependencies]
reed-solomon = "0.2"
```

```rust
extern crate reed_solomon
```

## Example

```rust
extern crate reed_solomon;

use reed_solomon::Encoder;
use reed_solomon::Decoder;

fn main() {
    let data = b"Hello World!";

    // Length of error correction code
    let ecc_len = 8;

    // Create encoder and decoder with 
    let enc = Encoder::new(ecc_len);
    let dec = Decoder::new(ecc_len);

    // Encode data
    let encoded = enc.encode(&data[..]);

    // Simulate some transmission errors
    let mut corrupted = *encoded;
    for i in 0..4 {
        corrupted[i] = 0x0;
    }

    // Try to recover data
    let known_erasures = [0];
    let recovered = dec.correct(&mut corrupted, Some(&known_erasures)).unwrap();

    let orig_str = std::str::from_utf8(data).unwrap();
    let recv_str = std::str::from_utf8(recovered.data()).unwrap();

    println!("message:               {:?}", orig_str);
    println!("original data:         {:?}", data);
    println!("error correction code: {:?}", encoded.ecc());
    println!("corrupted:             {:?}", corrupted);
    println!("repaired:              {:?}", recv_str);
}
```

## Fixed (compile-time) API

Besides the runtime `Encoder`/`Decoder` above (ecc length is a `usize` argument),
this crate also ships const-generic variants `FixedEncoder<ECCLEN>` and
`FixedDecoder<ECCLEN>` where the ecc length is fixed at compile time:

```rust
use reed_solomon::FixedEncoder;
use reed_solomon::FixedDecoder;

fn main() {
    let data = b"Hello World!";

    // ECC length is a const generic parameter
    let enc = FixedEncoder::<8>::new();
    let dec = FixedDecoder::<8>::new();

    let encoded = enc.encode(&data[..]);

    // Simulate some transmission errors
    let mut corrupted = *encoded;
    for x in corrupted.iter_mut().take(4) {
        *x = 0x0;
    }

    // Try to recover data
    let known_erasures = [0];
    let recovered = dec.correct(&mut corrupted, Some(&known_erasures)).unwrap();

    let recv_str = std::str::from_utf8(recovered.data()).unwrap();
    println!("repaired: {:?}", recv_str);
}
```

Pick the runtime API when the ecc length is only known at run time; pick the
fixed API when you want the ecc length baked into the type (e.g. for
`no_std`/embedded code or to rule out mismatched encoder/decoder lengths).
