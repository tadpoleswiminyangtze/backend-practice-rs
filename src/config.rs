#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_maxage: i64,
    pub port: u16,
}

impl Config {
    pub fn init() -> Self {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_secret = std::env::var("JWT_SECRET_KEY").expect("DATABASE_URL must be set");
        let jwt_maxage = std::env::var("JWT_MAXAGE").expect("DATABASE_URL must be set");

        Self {
            database_url,
            jwt_secret,
            jwt_maxage: jwt_maxage.parse::<i64>().unwrap(),
            port: 8000,
        }
    }
}
