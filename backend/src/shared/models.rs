use crate::config::WapSettings;

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
    pub settings: WapSettings,
}


#[derive(Clone, sqlx::FromRow, sqlx::Type)]
pub struct Id (pub i32);


