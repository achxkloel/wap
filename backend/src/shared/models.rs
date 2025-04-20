use crate::config::WapSettings;

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
    pub settings: WapSettings,
}

