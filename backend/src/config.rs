use crate::shared::models::AppStage;
use async_trait::async_trait;
use std::fmt;

#[derive(Debug, Clone)]
pub struct WapSettings {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expires_in: String,
    pub jwt_maxage: i32,
    pub google_oauth_client_id: Option<String>,
    pub google_oauth_client_secret: Option<String>,
    pub google_oauth_redirect_url: Option<String>,
    pub stage: AppStage,
}

#[async_trait]
pub trait WapSettingsImpl {
    async fn is_development(&self) -> bool;
    async fn is_staging(&self) -> bool;
    async fn is_production(&self) -> bool;
}

impl WapSettings {
    pub async fn init() -> WapSettings {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let jwt_expires_in = std::env::var("JWT_EXPIRED_IN").expect("JWT_EXPIRED_IN must be set");
        let jwt_maxage = std::env::var("JWT_MAXAGE").expect("JWT_MAXAGE must be set");
        let google_oauth_client_id =
            std::env::var("GOOGLE_OAUTH_CLIENT_ID").expect("GOOGLE_OAUTH_CLIENT_ID must be set");
        let google_oauth_client_secret = std::env::var("GOOGLE_OAUTH_CLIENT_SECRET")
            .expect("GOOGLE_OAUTH_CLIENT_SECRET must be set");
        let google_oauth_redirect_url = std::env::var("GOOGLE_OAUTH_REDIRECT_URI")
            .expect("GOOGLE_OAUTH_REDIRECT_URI must be set");
        let stage = std::env::var("STAGE").expect("STAGE must be set");
        WapSettings {
            database_url,
            jwt_secret,
            jwt_expires_in,
            jwt_maxage: jwt_maxage.parse::<i32>().unwrap(),
            google_oauth_client_id: Some(google_oauth_client_id),
            google_oauth_client_secret: Some(google_oauth_client_secret),
            google_oauth_redirect_url: Some(google_oauth_redirect_url),
            stage: if stage == "development" {
                AppStage::Development
            } else if stage == "staging" {
                AppStage::Staging
            } else if stage == "production" {
                AppStage::Production
            } else {
                panic!("Invalid STAGE value, put into .env file: STAGE=development|staging|production");
            },
        }
    }
}

#[async_trait]
impl WapSettingsImpl for WapSettings {
    async fn is_development(&self) -> bool {
        matches!(self.stage, AppStage::Development)
    }

    async fn is_staging(&self) -> bool {
        matches!(self.stage, AppStage::Staging)
    }

    async fn is_production(&self) -> bool {
        matches!(self.stage, AppStage::Production)
    }
}

impl fmt::Display for WapSettings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // If you consider jwt_secret sensitive, you might want to avoid printing it, or mask it.
        write!(
            f,
            "Configuration:
  Database URL: {}
  JWT Secret: {}
  JWT Expires In: {}
  JWT MaxAge: {}",
            self.database_url, self.jwt_secret, self.jwt_expires_in, self.jwt_maxage
        )
    }
}
