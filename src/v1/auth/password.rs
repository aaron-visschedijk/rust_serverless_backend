use argon2::{self, Config};
use rand::prelude::*;
use std::time::{Instant};

pub fn hash(password: &str) -> String {
    let start = Instant::now();
    let config = Config::default();
    let mut rng = rand::thread_rng();
    let mut salt: [u8; 128] = [0; 128];
    rng.fill_bytes(&mut salt);
    let hash = argon2::hash_encoded(password.as_bytes(), &salt, &config).unwrap();
    let elapsed = start.elapsed();
    println!("Hashing took {} milliseconds", elapsed.as_millis());
    hash
}

pub fn verify(password: &str, hash: &str) -> bool {
    let start = Instant::now();
    let verified = argon2::verify_encoded(&hash, password.as_bytes()).unwrap();
    let elapsed = start.elapsed();
    println!("Hashing took {} milliseconds", elapsed.as_millis());
    verified
}