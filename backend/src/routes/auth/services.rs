use crate::config::WapSettings;
use crate::routes::auth::models;
use crate::routes::auth::models::{
    AuthError, AuthErrorKind, GoogleUser, LoginSuccess, LoginUserSchema, RegisterUserRequestSchema,
    TokenClaims, TokenResponse, UserDb,
};
use crate::routes::auth::utils::hash_password;
use crate::shared::models::DatabaseId;
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

/// Pull all of your JWT logic into a single async helper.
pub async fn create_login_response<S>(user: UserDb, state: &S) -> LoginSuccess
where
    S: JwtConfigImpl, // now jwt_secret is async
{
    // 1) grab the secret and expiries asynchronously
    let secret = state.jwt_secret().await;
    let access_minutes = state.access_expires_minutes().await;
    let refresh_days = state.refresh_expires_days().await;

    // 2) compute timestamps
    let now = chrono::Utc::now();
    let access_exp = (now + chrono::Duration::minutes(access_minutes)).timestamp() as usize;
    let refresh_exp = (now + chrono::Duration::days(refresh_days)).timestamp() as usize;

    // 3) sign tokens
    let access_token = state
        .create_jwt_token(&user.id.0.to_string(), access_exp)
        .await;
    let refresh_token = state
        .create_jwt_token(&user.id.0.to_string(), refresh_exp)
        .await;

    LoginSuccess {
        access_token,
        refresh_token,
    }
}

#[async_trait]
pub trait AuthServiceImpl: Send + Sync + 'static + JwtConfigImpl {
    async fn validate_token(&self, token: &str) -> Result<UserDb, (StatusCode, Json<AuthError>)>;
    async fn token_claim(&self, token: &str) -> Result<TokenClaims, (StatusCode, Json<AuthError>)>;
    async fn register_new_user(
        &self,
        request: &RegisterUserRequestSchema,
    ) -> Result<UserDb, (StatusCode, Json<AuthErrorKind>)>;
    async fn login(&self, request: &LoginUserSchema) -> Result<UserDb>;
    async fn refresh(&self, user_id: DatabaseId) -> Result<UserDb, (StatusCode, Json<AuthError>)>;
    async fn get_user_by_id_or_email(
        &self,
        user_id: &Option<DatabaseId>,
        email: &Option<String>,
    ) -> Result<UserDb, (StatusCode, Json<AuthError>)>;
    async fn change_password(
        &self,
        user_id: DatabaseId,
        current: &str,
        new: &str,
    ) -> Result<(), (StatusCode, Json<AuthError>)>;
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
    async fn validate_token(&self, token: &str) -> Result<UserDb, (StatusCode, Json<AuthError>)> {
        // 1) decode JWT
        let claims = self.token_claim(token).await?;

        // 2) parse sub → user_id
        let user_id: i32 = claims.sub.parse().map_err(|_| {
            let err = AuthError::new("Invalid token subject");
            (StatusCode::UNAUTHORIZED, Json(err))
        })?;

        // 3) lookup user
        let user = sqlx::query_as!(UserDb, "SELECT * FROM users WHERE id = $1", user_id)
            .fetch_optional(&self.db)
            .await
            .map_err(|e| {
                let err = AuthError::new(format!("DB error: {}", e));
                (StatusCode::INTERNAL_SERVER_ERROR, Json(err))
            })?
            .ok_or_else(|| {
                let err = AuthError::new("User no longer exists");
                (StatusCode::UNAUTHORIZED, Json(err))
            })?;

        Ok(user)
    }

    async fn register_new_user(
        &self,
        request: &RegisterUserRequestSchema,
    ) -> Result<UserDb, (StatusCode, Json<AuthErrorKind>)> {
        // 1) check if user already exists
        if let Some(user) = sqlx::query_as!(
        UserDb,
        "SELECT * FROM users WHERE email = $1",
        request.email.to_ascii_lowercase()
    )
            .fetch_optional(&self.db)
            .await
            .map_err(|_| {
                // database error
                (StatusCode::INTERNAL_SERVER_ERROR, Json(AuthErrorKind::DatabaseError))
            })?
        {
            // user exists → just return it
            return Ok(user);
        }

        // 2) otherwise, insert new user
        let hashed = hash_password(&request.password)
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(AuthErrorKind::HashingError)))?
            .to_string();

        let new_user = sqlx::query_as!(
        UserDb,
        "INSERT INTO users (email, password_hash) VALUES ($1, $2) RETURNING *",
        request.email.to_ascii_lowercase(),
        hashed
    )
            .fetch_one(&self.db)
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(AuthErrorKind::DatabaseError)))?;

        Ok(new_user)
    }

    async fn token_claim(&self, token: &str) -> Result<TokenClaims, (StatusCode, Json<AuthError>)> {
        let token_data = decode::<TokenClaims>(
            token,
            &DecodingKey::from_secret(self.settings.jwt_secret.as_ref()),
            &Validation::default(),
        )
            .map_err(|_| {
                let err = AuthError {
                    message: "Invalid token".to_string(),
                };
                (StatusCode::UNAUTHORIZED, Json(err))
            })?;

        Ok(token_data.claims)
    }

    async fn login(&self, request: &LoginUserSchema) -> Result<UserDb> {
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

    async fn get_user_by_id_or_email(
        &self,
        user_id: &Option<DatabaseId>,
        email: &Option<String>,
    ) -> Result<UserDb, (StatusCode, Json<AuthError>)> {
        let user = match (user_id, email) {
            (Some(id), _) => sqlx::query_as!(UserDb, "SELECT * FROM users WHERE id = $1", id.0)
                .fetch_optional(&self.db)
                .await
                .map_err(|e| {
                    let err = AuthError {
                        message: format!("DB error: {}", e),
                    };
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(err))
                })?
                .ok_or_else(|| {
                    let err = AuthError {
                        message: "User not found".to_string(),
                    };
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(err))
                })?,
            (_, Some(email)) => {
                sqlx::query_as!(UserDb, "SELECT * FROM users WHERE email = $1", email)
                    .fetch_optional(&self.db)
                    .await
                    .map_err(|e| {
                        let err = AuthError {
                            message: format!("DB error: {}", e),
                        };
                        (StatusCode::INTERNAL_SERVER_ERROR, Json(err))
                    })?
                    .ok_or_else(|| {
                        let err = AuthError {
                            message: "User not found".to_string(),
                        };
                        (StatusCode::INTERNAL_SERVER_ERROR, Json(err))
                    })?
            }
            (_, _) => {
                let err = AuthError {
                    message: "Either user_id or email must be provided".to_string(),
                };
                return Err((StatusCode::BAD_REQUEST, Json(err)));
            }
        };

        Ok(user)
    }

    async fn refresh(&self, user_id: DatabaseId) -> Result<UserDb, (StatusCode, Json<AuthError>)> {
        let user = sqlx::query_as!(UserDb, "SELECT * FROM users WHERE id = $1", user_id.0)
            .fetch_optional(&self.db)
            .await
            .map_err(|e| {
                let err = AuthError {
                    message: "Database error".to_string(),
                };
                (StatusCode::INTERNAL_SERVER_ERROR, Json(err))
            })?
            .ok_or_else(|| {
                let err = AuthError {
                    message: "User not found".to_string(),
                };
                (StatusCode::INTERNAL_SERVER_ERROR, Json(err))
            })?;

        Ok(user)
    }

    async fn change_password(
        &self,
        user_id: DatabaseId,
        current: &str,
        new: &str,
    ) -> Result<(), (StatusCode, Json<AuthError>)> {
        // 1) Verify current password
        let user = sqlx::query_as!(UserDb, "SELECT * FROM users WHERE id = $1", user_id.0)
            .fetch_one(&self.db)
            .await
            .map_err(|e| {
                let err = AuthError::new(format!("DB error: {}", e));
                (StatusCode::INTERNAL_SERVER_ERROR, Json(err))
            })?;

        let matches = match PasswordHash::new(&user.password_hash) {
            Ok(hash) => Argon2::default().verify_password(current.as_bytes(), &hash).is_ok(),
            Err(_) => false,
        };

        if !matches {
            let err = AuthError::new("Current password is incorrect");
            return Err((StatusCode::BAD_REQUEST, Json(err)));
        }

        // 2) Hash new password
        let new_hash = hash_password(new)
            .await
            .map_err(|e| {
                let err = AuthError::new(format!("Hashing error: {}", e));
                (StatusCode::INTERNAL_SERVER_ERROR, Json(err))
            })?;

        // 3) Update in DB
        sqlx::query!(
            "UPDATE users SET password_hash = $1, updated_at = NOW() WHERE id = $2",
            new_hash.to_string(),
            user_id.0
        )
            .execute(&self.db)
            .await
            .map_err(|e| {
                let err = AuthError::new(format!("DB error: {}", e));
                (StatusCode::INTERNAL_SERVER_ERROR, Json(err))
            })?;

        Ok(())
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
    async fn request_token(&self, code: &str) -> Result<TokenResponse>;
    async fn get_google_user(&self, access_token: &str, id_token: &str) -> Result<GoogleUser>;
    async fn upsert_google_user(&self, google_user: &GoogleUser) -> Result<UserDb>;
}

#[async_trait]
impl GoogleAuthService for AuthService {
    // async fn create_jwt_token(&self, user_id: &str, exp: usize) -> String {
    //     create_jwt_token(user_id, exp, self.settings.jwt_secret.as_str()).await
    // }

    async fn request_token(&self, code: &str) -> Result<TokenResponse> {
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
                (email, password_hash, first_name, last_name, image_url, provider, google_id, created_at, updated_at)
            VALUES
                ($1, '', $2, $3, $4, 'google', $5, NOW(), NOW())
            ON CONFLICT (google_id) DO UPDATE
                SET email        = EXCLUDED.email,
                    image_url    = EXCLUDED.image_url,
                    updated_at   = NOW()
            RETURNING *
            "#,
            google_user.email,
            google_user.given_name,
            google_user.family_name,
            google_user.picture,
            google_user.sub,
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
    use crate::tests::tests::TestApp;
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

    #[sqlx::test]
    async fn test_register_and_login_pg(pool: PgPool) {
        let settings = WapSettings::init().await;
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
        let user = svc.register_new_user(&req).await.unwrap();
        assert_eq!(user.email, "test@example.com");

        // 2) login success
        let login_req = LoginUserSchema {
            email: req.email,
            password: req.password,
        };
        let logged = svc.login(&login_req).await.unwrap();
        assert_eq!(logged.id, user.id);

        // 3) login invalid password
        let bad = svc
            .login(&LoginUserSchema {
                email: "test@example.com".into(),
                password: "wrong".into(),
            })
            .await;
        assert!(bad.is_err());

        // 4) refresh
        let r = svc.refresh(user.id).await.unwrap();
        assert_eq!(r.id, user.id);
    }

    #[sqlx::test]
    async fn test_token_claim_and_refresh(db: PgPool) {
        let test_app = TestApp::new(db).await;
        let svc = AuthService::new(test_app.app.db.clone(), &test_app.app.settings);

        // Clean slate
        sqlx::query!("DELETE FROM settings")
            .execute(&test_app.app.db)
            .await
            .unwrap();
        sqlx::query!("DELETE FROM users")
            .execute(&test_app.app.db)
            .await
            .unwrap();

        // 1) register a user
        let req = RegisterUserRequestSchema {
            email: "foo@bar.com".into(),
            password: "hunter2".into(),
        };
        let user = svc.register_new_user(&req).await.unwrap();

        // 2) issue a JWT for that user
        let exp = (chrono::Utc::now() + chrono::Duration::minutes(5)).timestamp() as usize;
        let jwt = svc.create_jwt_token(&user.id.0.to_string(), exp).await;

        // 3) token_claim should succeed and round‑trip the 'sub'
        let claims = svc.token_claim(&jwt).await.unwrap();
        assert_eq!(claims.sub, user.id.0.to_string());

        // 4) refresh should find the same user
        let refreshed = svc.refresh(user.id).await.unwrap();
        assert_eq!(refreshed.id, user.id);

        // 5) token_claim should error on invalid JWT
        let err = svc.token_claim("not-a-token").await.unwrap_err();
        assert_eq!(err.0, StatusCode::UNAUTHORIZED);

        // 6) refresh should error on missing user
        let err = svc.refresh(DatabaseId { 0: -999 }).await.unwrap_err();
        assert_eq!(err.0, StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[sqlx::test]
    async fn test_google_upsert_roundtrip(pool: PgPool) {
        // for this test we just exercise upsert_google_user on the AuthService
        //   (it uses real HTTP, so we only test that the method exists;
        //    in real code you'd inject a fake HTTP client)
        let test_app = TestApp::new(pool).await;
        let svc = AuthService::new(test_app.app.db.clone(), &test_app.app.settings);

        // emulate a Google profile
        let gu = GoogleUser {
            id: "G123".into(),
            email: "z@z.com".into(),
            verified_email: true,
            name: "Test Z".into(),
            picture: "http://pic".into(),
        };

        // first insert
        let u1 = svc.upsert_google_user(&gu).await.unwrap();
        assert_eq!(u1.email, "z@z.com");

        // update on same google_id should not error
        let u2 = svc.upsert_google_user(&gu).await.unwrap();
        assert_eq!(u2.id, u1.id);
    }

    #[sqlx::test]
    async fn test_change_password_success(pool: PgPool) {
        let test_app = TestApp::new(pool).await;
        let svc = AuthService::new(test_app.app.db.clone(), &test_app.app.settings);

        // Clean slate
        sqlx::query!("DELETE FROM settings").execute(&test_app.app.db).await.unwrap();
        sqlx::query!("DELETE FROM users").execute(&test_app.app.db).await.unwrap();

        // 1) register
        let req = RegisterUserRequestSchema { email: "c@c.com".into(), password: "oldpass".into() };
        let user = svc.register_new_user(&req).await.unwrap();

        // 2) ensure login with old password works
        assert!(svc.login(&LoginUserSchema { email: req.email.clone(), password: req.password.clone() }).await.is_ok());

        // 3) perform password change
        svc.change_password(user.id, "oldpass", "newpass").await.unwrap();

        // 4) old password no longer works, new one does
        assert!(svc.login(&LoginUserSchema { email: req.email.clone(), password: "oldpass".into() }).await.is_err());
        assert!(svc.login(&LoginUserSchema { email: req.email.clone(), password: "newpass".into() }).await.is_ok());
    }

    #[sqlx::test]
    async fn test_change_password_wrong_current(pool: PgPool) {
        let test_app = TestApp::new(pool).await;
        let svc = AuthService::new(test_app.app.db.clone(), &test_app.app.settings);

        // Clean slate
        sqlx::query!("DELETE FROM settings").execute(&test_app.app.db).await.unwrap();
        sqlx::query!("DELETE FROM users").execute(&test_app.app.db).await.unwrap();

        // register
        let req = RegisterUserRequestSchema { email: "d@d.com".into(), password: "pass1".into() };
        let user = svc.register_new_user(&req).await.unwrap();

        // attempt with incorrect current password
        let err = svc.change_password(user.id, "wrongpass", "whatever").await.unwrap_err();
        assert_eq!(err.0, StatusCode::BAD_REQUEST);
    }

    #[sqlx::test]
    async fn test_validate_and_get_user(pool: PgPool) {
        let test_app = TestApp::new(pool).await;
        let svc = AuthService::new(test_app.app.db.clone(), &test_app.app.settings);

        // Clean slate
        sqlx::query!("DELETE FROM settings").execute(&test_app.app.db).await.unwrap();
        sqlx::query!("DELETE FROM users").execute(&test_app.app.db).await.unwrap();

        // register
        let req = RegisterUserRequestSchema { email: "e@e.com".into(), password: "pw".into() };
        let user = svc.register_new_user(&req).await.unwrap();

        // mint a short‑lived JWT
        let exp = (chrono::Utc::now() + chrono::Duration::minutes(1)).timestamp() as usize;
        let token = svc.create_jwt_token(&user.id.0.to_string(), exp).await;

        // validate_token should return the same user
        let validated = svc.validate_token(&token).await.unwrap();
        assert_eq!(validated.id, user.id);

        // get_user_by_id_or_email → by ID
        let by_id = svc.get_user_by_id_or_email(&Some(user.id), &None).await.unwrap();
        assert_eq!(by_id.id, user.id);

        // get_user_by_id_or_email → by email
        let by_email = svc.get_user_by_id_or_email(&None, &Some(req.email.clone())).await.unwrap();
        assert_eq!(by_email.id, user.id);

        // neither ID nor email → BAD_REQUEST
        let bad = svc.get_user_by_id_or_email(&None, &None).await.unwrap_err();
        assert_eq!(bad.0, StatusCode::BAD_REQUEST);
    }

}
