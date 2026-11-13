use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().expect("Cannot read .env file. Panicking");
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL missing");
        Self { database_url }
    }
}
