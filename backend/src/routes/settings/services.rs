use crate::config::WapSettings;
use crate::routes::settings::models::{
    Theme, UserSettingsCreate, UserSettingsDb, UserSettingsServiceSuccess,
    UserSettingsUpdateRequest,
};
use crate::shared::models::DatabaseId;
use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;
use sqlx::PgPool;

/// Core service operations for managing a user’s settings.
///
/// Supports fetching, creating, updating, and deleting the settings
/// record associated with a particular `user_id`.
#[async_trait]
#[cfg_attr(test, automock)]
pub trait SettingsServiceImpl: Send + Sync + 'static {
    /// Retrieve the settings for the given user.
    ///
    /// Returns `Ok(Some(_))` if settings exist, `Ok(None)` if they
    /// have not yet been created, or an error on failure.
    async fn get_settings(
        &self,
        user_id: &DatabaseId,
    ) -> Result<Option<UserSettingsServiceSuccess>>;

    /// Update the existing settings for the given user.
    ///
    /// Fields not provided in `UserSettingsUpdateRequest` will be left unchanged.
    /// Returns `Ok(())` on success or an error on failure.
    async fn update_settings(
        &self,
        user_id: &DatabaseId,
        settings: &UserSettingsUpdateRequest,
    ) -> Result<()>;

    /// Insert a new settings row for the given user.
    ///
    /// Any `None` fields in `UserSettingsCreate` will be replaced by
    /// sensible defaults (`Theme::Dark`, `true` for notifications,
    /// `radius = 10`). Returns the full `UserSettingsDb` record.
    async fn create_settings(&self, setting: &UserSettingsCreate) -> Result<UserSettingsDb>;

    /// Delete the settings row for the given user.
    ///
    /// If no settings row exists, this is a no-op. Returns `Ok(())`
    /// on success or an error on failure.
    async fn delete_settings(&self, user_id: &DatabaseId) -> Result<()>;
}

/// Postgres‐backed implementation of `SettingsServiceImpl`.
///
/// Uses `sqlx::PgPool` for all DB operations and reads default
/// values from the provided `WapSettings` if needed.
#[derive(Clone)]
pub struct SettingsService {
    /// The SQLx Postgres connection pool.
    pub db: PgPool,

    /// Application‐wide defaults (e.g. default radius).
    pub settings: WapSettings,
}

impl SettingsService {
    /// Construct a new `SettingsService` with the given DB pool and app settings.
    pub fn new(db: PgPool, settings: WapSettings) -> Self {
        Self { db, settings }
    }
}

#[async_trait]
impl SettingsServiceImpl for SettingsService {
    async fn get_settings(
        &self,
        user_id: &DatabaseId,
    ) -> Result<Option<UserSettingsServiceSuccess>> {
        tracing::debug!("Getting settings for user_id: {:?}", user_id);
        let settings = sqlx::query_as::<_, UserSettingsServiceSuccess>(
            r#"
            SELECT theme, notifications_enabled, radius, user_id
            FROM settings
            WHERE user_id = $1
            "#,
        )
        .bind(user_id.0)
        .fetch_optional(&self.db)
        .await
        .map_err(|e| {
            tracing::error!("Error fetching settings: {:?}", e);
            anyhow::anyhow!("Error fetching settings")
        })?;

        Ok(settings)
    }

    async fn update_settings(
        &self,
        user_id: &DatabaseId,
        settings: &UserSettingsUpdateRequest,
    ) -> Result<()> {
        sqlx::query!(
            "UPDATE settings SET theme = $1, notifications_enabled = $2, radius = $3, updated_at = NOW() WHERE user_id = $4",
            settings.theme as _,
            settings.notifications_enabled,
            settings.radius,
            user_id.0
        )
            .execute(&self.db)
            .await?;

        Ok(())
    }

    async fn create_settings(&self, setting: &UserSettingsCreate) -> Result<UserSettingsDb> {
        // supply defaults here in Rust:
        let theme = setting
            .theme
            .clone() // now you own an Option<Theme>
            .unwrap_or(Theme::Dark);
        let notifications_enabled = setting.notifications_enabled.unwrap_or(true);
        let radius = setting.radius.unwrap_or(10);

        // let settings = sqlx::query_as!(
        //     UserSettings,
        //     r#"
        //     INSERT INTO settings (user_id, theme, notifications_enabled, radius)
        //     VALUES ($1, $2, $3, $4)
        //     RETURNING id, theme, notifications_enabled, radius, user_id, created_at, updated_at
        //     "#,
        //     // now pass concrete values, not Options:
        //     setting.user_id.0,
        //     theme as _, // i.e. cast your enum into the DB type
        //     notifications_enabled,
        //     radius,
        // )
        // .fetch_one(&self.db)
        // .await?;
        let settings = sqlx::query_as::<_, UserSettingsDb>(
            r#"
        INSERT INTO settings (user_id, theme, notifications_enabled, radius)
        VALUES ($1, $2, $3, $4)
        RETURNING id, theme, notifications_enabled, radius, user_id, created_at, updated_at
        "#,
        )
        .bind(setting.user_id.0) // DatabaseId is transparent newtype over i32
        .bind(theme) // Theme implements sqlx::Type + Encode
        .bind(notifications_enabled) // bool is Copy + Encode
        .bind(radius) // i32 is Copy + Encode
        .fetch_one(&self.db)
        .await?;

        Ok(settings)
    }

    async fn delete_settings(&self, user_id: &DatabaseId) -> Result<()> {
        // Delete the row; if it wasn't there, we still return Ok(())
        sqlx::query!("DELETE FROM settings WHERE user_id = $1", user_id.0)
            .execute(&self.db)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::routes::settings::models::Theme;
    use crate::tests::tests::TestApp;
    use sqlx::PgPool;

    #[sqlx::test]
    async fn test_create_settings(pool: PgPool) {
        let test_app = TestApp::new(pool.clone()).await;
        let service = SettingsService::new(pool.clone(), test_app.app.settings);

        // Test getting settings for non-existent user
        let result = service.get_settings(&DatabaseId { 0: 999 }).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());

        // Create settings for test user
        let user_id: &DatabaseId = &test_app.users[0].user.id;
        service
            .create_settings(&UserSettingsCreate {
                user_id: user_id.clone(),
                ..Default::default()
            })
            .await
            .expect("Failed to create settings");

        // Test getting existing settings
        let settings = service
            .get_settings(user_id)
            .await
            .expect("Failed to get settings");
        assert!(settings.is_some());
        let settings = settings.unwrap();
        match settings.theme {
            Theme::Dark => { /* pass */ }
            other => panic!("expected Dark, got {:?}", other),
        }
        assert!(settings.notifications_enabled);
        assert_eq!(settings.radius, 10);
    }

    #[sqlx::test]
    async fn test_update_settings(pool: PgPool) {
        let test_app = TestApp::new(pool.clone()).await;
        let service = SettingsService::new(pool.clone(), test_app.app.settings);

        let user_id = &test_app.users[0].user.id.clone();
        service
            .create_settings(&UserSettingsCreate {
                user_id: user_id.clone(),
                ..Default::default()
            })
            .await
            .expect("Failed to create settings");

        let update_request = UserSettingsUpdateRequest {
            theme: Theme::Dark,
            notifications_enabled: false,
            radius: 20,
        };

        // Test updating settings
        let result = service.update_settings(user_id, &update_request).await;
        assert!(result.is_ok());

        // Verify the update
        let settings = service
            .get_settings(user_id)
            .await
            .expect("Failed to get settings");
        assert!(settings.is_some());
        let settings = settings.unwrap();
        match settings.theme {
            Theme::Dark => { /* pass */ }
            other => panic!("expected Dark, got {:?}", other),
        }
        assert!(!settings.notifications_enabled);
        assert_eq!(settings.radius, 20);
    }

    #[sqlx::test]
    async fn test_delete_settings(pool: PgPool) {
        let test_app = TestApp::new(pool.clone()).await;
        let service = SettingsService::new(pool.clone(), test_app.app.settings);

        let user_id = &test_app.users[0].user.id;

        // 1) Create settings so there is something to delete
        service
            .create_settings(&UserSettingsCreate {
                user_id: user_id.clone(),
                ..Default::default()
            })
            .await
            .expect("Failed to create settings");

        // 2) Verify it exists
        let before = service
            .get_settings(user_id)
            .await
            .expect("get_settings failed");
        assert!(before.is_some());

        // 3) Delete it
        service
            .delete_settings(user_id)
            .await
            .expect("delete_settings failed");

        // 4) And now get_settings should return None
        let after = service
            .get_settings(user_id)
            .await
            .expect("get_settings after delete failed");
        assert!(after.is_none());
    }
}
