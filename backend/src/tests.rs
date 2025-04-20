#[cfg(test)]
mod tests {
    use crate::routes::auth::models::User;
    use argon2::PasswordHasher;
    use axum::{extract::State, http::HeaderMap, Json};
    use sqlx::PgPool;
    use uuid::Uuid;

    // #[derive(Deserialize)]
    // struct CreateUserRequest {
    //     username: String,
    //     password: String,
    // }
    //
    // #[derive(Deserialize)]
    // struct LoginRequest {
    //     username: String,
    //     password: String,
    // }
    //
    // #[derive(Serialize)]
    // struct UserResponse {
    //     id: Uuid,
    //     username: String,
    // }
    //
    // Handler: create a test user by registering then logging in, return user + keys with headers
    async fn create_test_user(
        State(pool): State<PgPool>,
    ) -> (HeaderMap, axum::http::StatusCode, Json<serde_json::Value>) {
        // Fixed test credentials
        let username = format!("test_{}", Uuid::new_v4());
        use crate::routes::auth::utils::hash_password;
        let hashed_password = hash_password(&"password123".to_string())
            .await
            .expect("hash_password failed");

        // Reuse registration logic
        let rec: User = sqlx::query_as!(
            User,
            r#"INSERT INTO users (email, password_hash)
           VALUES ($1, $2)
           RETURNING id, email, password_hash, created_at, updated_at"#,
            username,
            hashed_password
        )
        .fetch_one(&pool)
        .await
        .expect("Failed to insert test user");

        // Reuse login logic (without password hashing here for simplicity)
        let access_key = Uuid::new_v4().to_string();
        let secret_key = Uuid::new_v4().to_string();

        // Prepare headers
        let mut headers = HeaderMap::new();
        headers.insert("x-access-key", access_key.parse().unwrap());
        headers.insert("x-secret-key", secret_key.parse().unwrap());

        // Response body
        let body = Json(serde_json::json!({
            "user": { "id": rec.id, "username": rec.email },
            "access_key": headers["x-access-key"].to_str().unwrap(),
            "secret_key": headers["x-secret-key"].to_str().unwrap(),
        }));

        (headers, axum::http::StatusCode::CREATED, body)
    }
}
