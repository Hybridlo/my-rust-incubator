use rand::{distributions::Alphanumeric, Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;

const SALT_LENGTH: usize = 20;

/// returns both the hash and the salt
pub fn hash_password<S: AsRef<str>>(pw: S) -> Result<String, String> {
    let mut rng = ChaCha20Rng::from_entropy();
    let salt: String = (0..SALT_LENGTH)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect();

    let config = argon2::Config::default();
    let hash = argon2::hash_encoded(pw.as_ref().as_bytes(), salt.as_bytes(), &config)
        .map_err(|_| "Password hashing failed")?;

    Ok(hash)
}

pub fn validate_hash<S: AsRef<str>>(hash: S, pw: S) -> Result<bool, String> {
    argon2::verify_encoded(hash.as_ref(), pw.as_ref().as_bytes())
        .map_err(|_| "Authorization failed!".to_string())
}
