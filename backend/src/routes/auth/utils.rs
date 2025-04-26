use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use rand_core::OsRng;

/// Hashes a plaintext password using the Argon2id algorithm with a freshly‐generated random salt.
///
/// This function returns the password hash in the standard PHC string format:
/// `"$argon2id$v=19$m=65536,t=2,p=1$<salt>$<hash>"`.
///
/// # Arguments
///
/// * `password` – The user’s plaintext password to be hashed.
///
/// # Returns
///
/// * `Ok(String)` containing the PHC‐formatted Argon2 hash on success.
/// * `Err(String)` with a descriptive error message if hashing fails.
///
/// # Examples
///
/// ```
/// use backend::routes::auth::utils::hash_password;
/// use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
/// use tokio::runtime::Runtime;
/// let password_plain = "correct horse battery staple";
/// Runtime::new().unwrap().block_on(async move {
///     let password_hash_string = hash_password(password_plain).await.unwrap();
///     let password_hash = PasswordHash::new(&password_hash_string).unwrap();
///     let argon2 = Argon2::default();
///     let is_valid = argon2.verify_password(password_plain.as_bytes(), &password_hash.into()).map_or(false, |_| true);
///     assert!(is_valid);
/// });
/// ```
pub async fn hash_password(password: &str) -> Result<String, String> {
    // 1) generate a fresh cryptographically‐secure salt
    let salt = SaltString::generate(&mut OsRng);

    // 2) perform Argon2id hashing
    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| format!("Error while hashing password: {}", e))?
        // Convert to the PHC string format
        .to_string();

    Ok(hash)
}
