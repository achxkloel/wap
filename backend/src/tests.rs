#[cfg(test)]
pub mod tests {
    use crate::config::WapSettings;
    use crate::routes::auth::models::{User, UserId};
    use crate::routes::auth::utils::create_token;
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
        pub user: User,
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
        //     id: UserId(row.id), // wrap the i32 yourself
        //     email: row.email,
        //     password_hash: Some(row.password_hash),
        //     created_at: row.created_at,
        //     updated_at: row.updated_at,
        // };

        // Reuse registration logic
        // TODO: continue here: fix this error
        let user: User = sqlx::query_as!(
            User,
            r#"INSERT INTO users (email, password_hash)
           VALUES ($1, $2)
           RETURNING id, email, password_hash, created_at, updated_at"#,
            email,
            hashed_password
        )
            .fetch_one(&pool)
            .await
            .expect("Failed to insert test user");

        // Reuse login logic (without password hashing here for simplicity)
        let now = chrono::Utc::now();
        let access_token = create_token(
            &user.id.0.to_string(),
            (now + chrono::Duration::minutes(60)).timestamp() as usize,
            app.settings.jwt_secret.as_ref(),
        );

        let refresh_token = create_token(
            &user.id.0.to_string(),
            (now + chrono::Duration::days(30)).timestamp() as usize,
            app.settings.jwt_secret.as_ref(),
        );

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
}
