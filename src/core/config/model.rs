use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub page_length: i64,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().expect("Cannot read .env file. Panicking");
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL missing from .env");
        let page_length = env::var("PAGE_LENGTH")
            .expect("PAGE_LENGTH missing from .env")
            .parse::<i64>()
            .expect("PAGE_LENGTH is not a valid number");
        Self {
            database_url,
            page_length,
        }
    }
}
