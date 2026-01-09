// Application configuration

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_refresh_secret: String,
    pub api_port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();
        
        Self {
            database_url: std::env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set"),
            jwt_secret: std::env::var("JWT_SECRET")
                .expect("JWT_SECRET must be set"),
            jwt_refresh_secret: std::env::var("JWT_REFRESH_SECRET")
                .expect("JWT_REFRESH_SECRET must be set"),
            api_port: std::env::var("API_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .expect("API_PORT must be a valid number"),
        }
    }
}

