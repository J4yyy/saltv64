use rand::Rng;
use base64::{Engine as _, engine::general_purpose};

fn main() {
    let mut rng = rand::thread_rng();

    // Generate salt
    let salt: Vec<u8> = (0..16).map(|_| rng.gen()).collect();
    let base64_salt = general_purpose::STANDARD.encode(&salt);

    // Generate IV
    let iv: Vec<u8> = (0..16).map(|_| rng.gen()).collect();
    let base64_iv = general_purpose::STANDARD.encode(&iv);

    println!("Base 64 Salt -> {base64_salt}\n");
    println!("Base 64 IV -> {base64_iv}");
}