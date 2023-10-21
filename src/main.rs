use reed_solomon::Encoder;
use reed_solomon::Decoder;
use rand::prelude::*;

extern crate reed_solomon;

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

    for i in 1..4 {
        corrupted[i] = thread_rng().next_u32() as u8;
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
