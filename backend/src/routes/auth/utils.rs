use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use rand_core::OsRng;

pub async fn hash_password(password: &str) -> Result<String, String> {
    // 1) generate a fresh salt
    let salt = SaltString::generate(&mut OsRng);

    // 2) hash it
    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| format!("Error while hashing password: {}", e))?
        // `to_string()` yields the PHC‐format string: "$argon2id$v=19$m=…$…$…"
        .to_string();

    Ok(hash)
}
