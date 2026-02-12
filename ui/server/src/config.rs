use std::path::PathBuf;

/// Server configuration loaded from environment variables.
#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub jwt_secret: String,
    pub db_path: PathBuf,
    pub cors_origins: Vec<String>,
    pub rate_limit_per_min: u32,
    pub max_concurrent_scans: usize,
    pub template_dir: Option<PathBuf>,
    pub ai_enabled: bool,
}

impl ServerConfig {
    pub fn from_env() -> Self {
        Self {
            host: std::env::var("SWE_HOST").unwrap_or_else(|_| "0.0.0.0".into()),
            port: std::env::var("SWE_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(8080),
            jwt_secret: std::env::var("SWE_JWT_SECRET")
                .unwrap_or_else(|_| "dev-secret-change-in-production".into()),
            db_path: std::env::var("SWE_DB_PATH")
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from("swe-compliance.db")),
            cors_origins: std::env::var("SWE_CORS_ORIGINS")
                .map(|s| s.split(',').map(|o| o.trim().to_string()).collect())
                .unwrap_or_else(|_| vec!["http://localhost:3000".into()]),
            rate_limit_per_min: std::env::var("SWE_RATE_LIMIT")
                .ok()
                .and_then(|r| r.parse().ok())
                .unwrap_or(100),
            max_concurrent_scans: std::env::var("SWE_MAX_CONCURRENT_SCANS")
                .ok()
                .and_then(|r| r.parse().ok())
                .unwrap_or(10),
            template_dir: std::env::var("SWE_TEMPLATE_DIR").ok().map(PathBuf::from),
            ai_enabled: std::env::var("DOC_ENGINE_AI_ENABLED")
                .map(|v| v != "0" && v.to_lowercase() != "false")
                .unwrap_or(false),
        }
    }
}
