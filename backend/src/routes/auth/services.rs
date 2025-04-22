use crate::config::WapSettings;
use crate::routes::auth::models;
use crate::routes::auth::models::{
    GoogleUser, LoginUserSchema, RefreshError, RegisterUserRequestSchema, TokenClaims,
    TokenResponse, UserDb,
};
use crate::routes::auth::utils::hash_password;
use anyhow::Result;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use async_trait::async_trait;
use axum::http::StatusCode;
use axum::Json;
use futures_util::FutureExt;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPool, Row};

#[async_trait]
pub trait AuthServiceImpl: Send + Sync + 'static + JwtConfigImpl {
    async fn token_claim(
        &self,
        token: &str,
    ) -> Result<TokenClaims, (StatusCode, Json<RefreshError>)>;
    async fn register(&self, request: &RegisterUserRequestSchema) -> Result<UserDb>;
    async fn login(&self, request: LoginUserSchema) -> Result<UserDb>;
    async fn refresh(&self, user_id: i32) -> Result<UserDb, (StatusCode, Json<RefreshError>)>;
}

#[derive(Clone)]
pub struct AuthService {
    pub db: PgPool,
    pub settings: WapSettings,
    pub http: Client,
}

impl AuthService {
    pub fn new(db: PgPool, settings: &WapSettings) -> Self {
        AuthService {
            db,
            settings: settings.clone(),
            http: Client::default(),
        }
    }
}

#[async_trait]
impl JwtConfigImpl for AuthService {
    async fn jwt_secret(&self) -> String {
        self.settings.jwt_secret.clone()
    }

    async fn create_jwt_token(&self, user_id: &str, exp: usize) -> String {
        let secret = self.settings.jwt_secret.as_ref();
        let now = chrono::Utc::now();
        let iat = now.timestamp() as usize;
        let claims: models::TokenClaims = models::TokenClaims {
            sub: user_id.to_string(),
            exp,
            iat,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret),
        )
        .unwrap()
    }
}

#[async_trait]
impl AuthServiceImpl for AuthService {
    async fn register(&self, request: &RegisterUserRequestSchema) -> Result<UserDb> {
        let hashed_password = hash_password(request.password.as_str())
            .await
            .map_err(|e| anyhow::anyhow!("Error while hashing password: {}", e))?
            .to_string();

        let user = sqlx::query_as!(
            UserDb,
            "INSERT INTO users (email, password_hash) VALUES ($1, $2) RETURNING *",
            request.email.to_ascii_lowercase(),
            hashed_password
        )
        .fetch_one(&self.db)
        .await?;

        // Insert new settings
        sqlx::query!("INSERT INTO settings (user_id) VALUES ($1)", user.id.0)
            .execute(&self.db)
            .await?;

        Ok(user)
    }

    async fn token_claim(
        &self,
        token: &str,
    ) -> Result<TokenClaims, (StatusCode, Json<RefreshError>)> {
        let token_data = decode::<TokenClaims>(
            token,
            &DecodingKey::from_secret(self.settings.jwt_secret.as_ref()),
            &Validation::default(),
        )
        .map_err(|_| {
            let err = RefreshError {
                status: "fail".to_string(),
                message: "Invalid token".to_string(),
            };
            (StatusCode::UNAUTHORIZED, Json(err))
        })?;

        Ok(token_data.claims)
    }

    async fn login(&self, request: LoginUserSchema) -> Result<UserDb> {
        let email = request.email.to_ascii_lowercase();
        let user = sqlx::query_as!(UserDb, "SELECT * FROM users WHERE email = $1", email)
            .fetch_optional(&self.db)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Invalid email or password"))?;

        let is_valid = match PasswordHash::new(&user.clone().password_hash) {
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

    async fn refresh(&self, user_id: i32) -> Result<UserDb, (StatusCode, Json<RefreshError>)> {
        let user = sqlx::query_as!(UserDb, "SELECT * FROM users WHERE id = $1", user_id)
            .fetch_optional(&self.db)
            .await
            .map_err(|e| {
                let err = RefreshError {
                    status: "fail".to_string(),
                    message: "Invalid token".to_string(),
                };
                (StatusCode::INTERNAL_SERVER_ERROR, Json(err))
            })?
            .unwrap();

        Ok(user)
    }
}

#[async_trait]
pub trait JwtConfigImpl: Sync + Send + 'static {
    /// Return the HMAC secret for signing.
    async fn jwt_secret(&self) -> String;

    /// How many minutes an access token lives.
    async fn access_expires_minutes(&self) -> i64 {
        60
    }
    /// How many days a refresh token lives.
    async fn refresh_expires_days(&self) -> i64 {
        30
    }

    async fn create_jwt_token(&self, user_id: &str, exp: usize) -> String;
}

#[async_trait]
pub trait GoogleAuthService: Send + Sync + 'static + JwtConfigImpl {
    async fn request_token(&self, code: &str, state: &str) -> Result<TokenResponse>;
    async fn get_google_user(&self, access_token: &str, id_token: &str) -> Result<GoogleUser>;
    async fn upsert_google_user(&self, google_user: &GoogleUser) -> Result<UserDb>;
}

#[async_trait]
impl GoogleAuthService for AuthService {
    // async fn create_jwt_token(&self, user_id: &str, exp: usize) -> String {
    //     create_jwt_token(user_id, exp, self.settings.jwt_secret.as_str()).await
    // }

    async fn request_token(&self, code: &str, state: &str) -> Result<TokenResponse> {
        let params = [
            ("code", code),
            (
                "client_id",
                &self.settings.google_oauth_client_id.clone().unwrap(),
            ),
            (
                "client_secret",
                &self.settings.google_oauth_client_secret.clone().unwrap(),
            ),
            (
                "redirect_uri",
                &self.settings.google_oauth_redirect_url.clone().unwrap(),
            ),
            ("grant_type", "authorization_code"),
        ];
        let resp = self
            .http
            .post("https://oauth2.googleapis.com/token")
            .form(&params)
            .send()
            .await?
            .error_for_status()?;
        let token: TokenResponse = resp.json().await?;
        Ok(token)
    }

    async fn get_google_user(&self, access_token: &str, _id_token: &str) -> Result<GoogleUser> {
        let resp = self
            .http
            .get("https://www.googleapis.com/oauth2/v3/userinfo")
            .bearer_auth(access_token)
            .send()
            .await?
            .error_for_status()?;
        let user: GoogleUser = resp.json().await?;
        Ok(user)
    }

    /// Insert or update a Google‐authenticated user, returning the full UserDb.
    async fn upsert_google_user(&self, google_user: &GoogleUser) -> Result<UserDb> {
        let user: UserDb = sqlx::query_as!(
            UserDb,
            r#"
            INSERT INTO users
                (email, password_hash, first_name, last_name, image_url, google_id, created_at, updated_at)
            VALUES
                ($1, '', NULL, NULL, $2, $3, NOW(), NOW())
            ON CONFLICT (google_id) DO UPDATE
                SET email        = EXCLUDED.email,
                    image_url    = EXCLUDED.image_url,
                    updated_at   = NOW()
            RETURNING *
            "#,
            google_user.email,
            google_user.picture,
            google_user.id,
        )
            .fetch_one(&self.db)
            .await?;

        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::WapSettings;
    use crate::routes::auth::models::{LoginUserSchema, RegisterUserRequestSchema};
    use crate::routes::auth::services::AuthService;
    use sqlx::PgPool;

    /// A fake verifier to drive GoogleAuthService tests
    #[derive(Clone)]
    struct FakeVerifier {
        pub ok: bool,
        pub email: String,
    }

    // #[async_trait]
    // impl TokenVerifier for FakeVerifier {
    //     type TokenData = GoogleTokenData;
    //     async fn verify(&self, _id_token: &str) -> Result<GoogleTokenData> {
    //         if self.ok {
    //             Ok(GoogleTokenData { email: self.email.clone() })
    //         } else {
    //             Err(anyhow::anyhow!("Invalid token"))
    //         }
    //     }
    // }
    //
    // Helper to get a test DB pool; set DATABASE_URL=test url in env
    async fn get_pool() -> PgPool {
        let url = std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
        PgPool::connect(&url).await.unwrap()
    }

    #[sqlx::test]
    async fn test_register_and_login_pg(pool: PgPool) {
        let settings = WapSettings::init();
        let svc = AuthService::new(pool.clone(), &settings);

        // Clean slate
        sqlx::query!("DELETE FROM settings")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query!("DELETE FROM users")
            .execute(&pool)
            .await
            .unwrap();

        // 1) register
        let req = RegisterUserRequestSchema {
            email: "Test@Example.Com".into(),
            password: "secret".into(),
        };
        let user = svc.register(&req).await.unwrap();
        assert_eq!(user.email, "test@example.com");

        // 2) login success
        let login_req = LoginUserSchema {
            email: req.email,
            password: req.password,
        };
        let logged = svc.login(login_req).await.unwrap();
        assert_eq!(logged.id, user.id);

        // 3) login invalid password
        let bad = svc
            .login(LoginUserSchema {
                email: "test@example.com".into(),
                password: "wrong".into(),
            })
            .await;
        assert!(bad.is_err());

        // 4) refresh
        let r = svc.refresh(user.id.0).await.unwrap();
        assert_eq!(r.id, user.id);
    }

    // #[tokio::test]
    // async fn test_google_auth_service() {
    //     let pool = get_pool().await;
    //     let settings = WapSettings::init();
    //     let good_verifier = FakeVerifier { ok: true, email: "gg@example.com".into() };
    //     let svc = GoogleAuthService::new(pool.clone(), settings.clone(), good_verifier.clone());
    //
    //     // Clean slate
    //     sqlx::query!("DELETE FROM settings").execute(&pool).await.unwrap();
    //     sqlx::query!("DELETE FROM users").execute(&pool).await.unwrap();
    //
    //     // login (which also registers)
    //     let auth_req = LoginUserSchema { email: "fake-token".into(), password: String::new() };
    //     let user = svc.login(&auth_req).await.unwrap();
    //     assert_eq!(user.email, "gg@example.com");
    //
    //     // invalid token
    //     let bad_verifier = FakeVerifier { ok: false, email: String::new() };
    //     let bad_svc = GoogleAuthService::new(pool.clone(), settings.clone(), bad_verifier);
    //     let res = bad_svc.login(auth_req).await;
    //     assert!(res.is_err());
    // }
}
