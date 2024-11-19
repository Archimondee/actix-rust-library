use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database_url: String,
}

impl Config {
    #[allow(dead_code)]
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();
        Config {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
        }
    }
}
