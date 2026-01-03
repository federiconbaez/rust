use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub jwt_secret: String,
    pub jwt_expiration_hours: i64,
    pub encryption_key: Vec<u8>,
    pub server_host: String,
    pub server_port: u16,
    pub cors_origin: String,
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, anyhow::Error> {
        dotenvy::dotenv().ok();

        let jwt_secret = env::var("JWT_SECRET")
            .unwrap_or_else(|_| "dev-secret-change-in-production".to_string());
        
        if jwt_secret.len() < 32 {
            tracing::warn!("JWT_SECRET should be at least 32 characters for security");
        }

        let jwt_expiration_hours = env::var("JWT_EXPIRATION_HOURS")
            .unwrap_or_else(|_| "24".to_string())
            .parse()
            .unwrap_or(24);

        let encryption_key_hex = env::var("ENCRYPTION_KEY")
            .unwrap_or_else(|_| {
                tracing::warn!("No ENCRYPTION_KEY set, using development key");
                "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef".to_string()
            });

        let encryption_key = hex::decode(&encryption_key_hex)
            .map_err(|_| anyhow::anyhow!("Invalid ENCRYPTION_KEY hex format"))?;

        if encryption_key.len() != 32 {
            return Err(anyhow::anyhow!("ENCRYPTION_KEY must be 32 bytes (64 hex chars)"));
        }

        let server_host = env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let server_port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .unwrap_or(8080);

        let cors_origin = env::var("CORS_ORIGIN")
            .unwrap_or_else(|_| "http://localhost:3000".to_string());

        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "sqlite:./nexusdb.db".to_string());

        Ok(Config {
            jwt_secret,
            jwt_expiration_hours,
            encryption_key,
            server_host,
            server_port,
            cors_origin,
            database_url,
        })
    }
}
