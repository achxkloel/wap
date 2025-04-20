use crate::routes::auth::models::{LoginUserSchema, RegisterUserRequestSchema, User};
use crate::routes::auth::utils::hash_password;
use anyhow::Result;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use async_trait::async_trait;
use sqlx::PgPool;

#[async_trait]
pub trait AuthService: Send + Sync + 'static {
    async fn register(&self, request: RegisterUserRequestSchema) -> Result<User>;
    async fn login(&self, request: LoginUserSchema) -> Result<User>;
    async fn refresh(&self, user_id: i32) -> Result<User>;
}

#[derive(Clone)]
pub struct PgAuthService {
    pub db: PgPool,
    pub settings: crate::config::WapSettings,
}

impl PgAuthService {
    pub fn new(db: PgPool, settings: crate::config::WapSettings) -> Self {
        Self { db, settings }
    }
}

#[async_trait]
impl AuthService for PgAuthService {
    async fn register(&self, request: RegisterUserRequestSchema) -> Result<User> {
        let hashed_password = hash_password(request.password.as_str())
            .await
            .map_err(|e| anyhow::anyhow!("Error while hashing password: {}", e))?
            .to_string();

        let user = sqlx::query_as!(
            User,
            "INSERT INTO users (email, password_hash) VALUES ($1, $2) RETURNING id, email, password_hash, created_at, updated_at",
            request.email.to_ascii_lowercase(),
            hashed_password
        )
        .fetch_one(&self.db)
        .await?;

        // Insert new settings
        sqlx::query!("INSERT INTO settings (user_id) VALUES ($1)", user.id)
            .execute(&self.db)
            .await?;

        Ok(user)
    }

    async fn login(&self, request: LoginUserSchema) -> Result<User> {
        let email = request.email.to_ascii_lowercase();
        let user = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE email = $1",
            email
        )
        .fetch_optional(&self.db)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Invalid email or password"))?;

        let is_valid = match PasswordHash::new(&user.clone().password_hash.unwrap()) {
            Ok(parsed_hash) => Argon2::default()
                .verify_password(request.password.as_bytes(), &parsed_hash)
                .map_or(false, |_| true),
            Err(_) => false,
        };

        if !is_valid {
            return Err(anyhow::anyhow!("Invalid email or password"));
        }

        Ok(user.clone())
    }

    async fn refresh(&self, user_id: i32) -> Result<User> {
        let user = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE id = $1",
            user_id
        )
        .fetch_optional(&self.db)
        .await?
        .ok_or_else(|| anyhow::anyhow!("User not found"))?;

        Ok(user)
    }
} 
