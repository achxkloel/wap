#[cfg(test)]
pub mod tests {
    use crate::config::WapSettings;
    use crate::routes::auth::models::UserDb;
    use crate::routes::auth::services::{AuthService, JwtConfigImpl};
    use crate::shared::models::AppState;
    use axum::extract::State;
    use axum::http::HeaderMap;
    use sqlx::PgPool;

    #[derive(Clone)]
    pub struct TestTokens {
        pub access_token: String,
        pub refresh_token: String,
    }

    #[derive(Clone)]
    pub struct TestAppUser {
        pub user: UserDb,
        pub header: HeaderMap,
        pub tokens: TestTokens,
    }

    #[derive(Clone)]
    pub struct TestApp {
        pub app: AppState,
        pub users: Vec<TestAppUser>,
    }

    impl TestApp {
        pub async fn new(pool: PgPool) -> Self {
            let app = init_app_state(pool.clone()).await;
            let users = create_test_users(State(pool.clone())).await;
            TestApp { app, users }
        }
    }

    pub async fn init_app_state(pool: PgPool) -> AppState {
        AppState {
            db: pool.clone(),
            settings: WapSettings {
                database_url: "".to_string(),
                jwt_secret: "aaaaa".to_string(),
                jwt_expires_in: "".to_string(),
                jwt_maxage: 0,
                google_oauth_client_id: None,
                google_oauth_client_secret: None,
                google_oauth_redirect_url: None,
                stage: crate::shared::models::AppStage::Testing,
            },
        }
    }

    // Handler: create a test user by registering then logging in, return user + keys with headers
    pub async fn create_test_users(State(pool): State<PgPool>) -> Vec<TestAppUser> {
        let app = init_app_state(pool.clone()).await;

        // Fixed test credentials
        let email = format!("test_1@wap.com");
        use crate::routes::auth::utils::hash_password;
        let hashed_password = hash_password(&"password123".to_string())
            .await
            .expect("hash_password failed");

        // let row = sqlx::query!(
        //     "INSERT INTO users (email, password_hash) VALUES ($1, $2) RETURNING id, email, password_hash, created_at, updated_at",
        //     email,
        //     hashed_password,
        // )
        // .fetch_one(&pool)
        // .await
        //     .unwrap();
        //
        // let user = User {
        //     id: DatabaseId(row.id), // wrap the i32 yourself
        //     email: row.email,
        //     password_hash: Some(row.password_hash),
        //     created_at: row.created_at,
        //     updated_at: row.updated_at,
        // };

        // Reuse registration logic
        // TODO: continue here: fix this error
        let user: UserDb = sqlx::query_as!(
            UserDb,
            r#"INSERT INTO users (email, password_hash)
           VALUES ($1, $2)
           RETURNING *"#,
            email,
            hashed_password
        )
        .fetch_one(&pool)
        .await
        .expect("Failed to insert test user");

        // Reuse login logic (without password hashing here for simplicity)
        let now = chrono::Utc::now();
        let auth_service = AuthService::new(pool, &app.settings);
        let access_token = auth_service
            .create_jwt_token(
                &user.id.0.to_string(),
                (now + chrono::Duration::minutes(60)).timestamp() as usize,
            )
            .await;
        let refresh_token = auth_service
            .create_jwt_token(
                &user.id.0.to_string(),
                (now + chrono::Duration::days(30)).timestamp() as usize,
            )
            .await;

        // Prepare headers
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", access_token).parse().unwrap(),
        );

        // Response body
        let test_user = TestAppUser {
            user: user,
            header: headers,
            tokens: TestTokens {
                access_token: access_token,
                refresh_token: refresh_token,
            },
        };

        // Return test user
        vec![test_user]
    }
    
    pub async fn prepare_tracing() {
        
    }
}
