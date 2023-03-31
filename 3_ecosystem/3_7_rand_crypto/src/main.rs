use std::{path::Path, fs};

use rand::{SeedableRng, seq::{IteratorRandom, SliceRandom}, thread_rng, distributions::Alphanumeric, Rng};
use rand_chacha::ChaCha20Rng;
use sha3::{Digest, Sha3_512};

fn generate_password<S: AsRef<str>>(pw_len: u64, symbol_set: S) -> Option<String> {
    if symbol_set.as_ref().is_empty() {
        return None;
    }
    
    let mut rng = ChaCha20Rng::from_entropy();

    let mut pw = String::new();

    for _ in 0..pw_len {
                                                             // checked for empty symbol_set before
        pw.push(symbol_set.as_ref().chars().choose(&mut rng).unwrap())
    }

    Some(pw)
}

fn select_rand_val<T>(values: &[T]) -> Option<&T> {
    let mut rng = thread_rng();

    values.choose(&mut rng)
}

const ACCESS_TOKEN_LENGTH: u64 = 64;

fn new_access_token() -> String {    
    let mut rng = ChaCha20Rng::from_entropy();

    (0..ACCESS_TOKEN_LENGTH).map(|_| rng.sample( Alphanumeric) as char).collect()
}

fn get_file_hash(file_path: &Path) -> Result<Vec<u8>, ()> {
    let file = fs::read(file_path).map_err(|_| ())?;

    let mut hasher = Sha3_512::new();
    
    hasher.update(file);

    Ok(hasher.finalize().to_vec())
}

const ALPHANUMERIC_SYMBOLS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

fn hash_password<S: AsRef<str>>(pw: S) -> Result<String, ()> {
    let config = argon2::Config::default();
    let salt = generate_password(10, ALPHANUMERIC_SYMBOLS).ok_or(())?;

    argon2::hash_encoded(pw.as_ref().as_bytes(), salt.as_bytes(), &config).map_err(|_| ())
}

fn main() {
    let password = generate_password(15, "abcdefghijklmnopqrstuvwxyz").unwrap();
    println!("password: {}", password);

    let rand_val = select_rand_val(&[5, 10, 13, 33, 6]).unwrap();
    println!("rand value: {}", rand_val);

    let access_token = new_access_token();
    println!("access token: {}", access_token);

    let file_hash = get_file_hash(Path::new("src/main.rs")).unwrap();
    println!("main hash: {:x?}", file_hash);

    let pw_hash = hash_password(&password).unwrap();
    assert!(argon2::verify_encoded(&pw_hash, password.as_bytes()).unwrap());
}
