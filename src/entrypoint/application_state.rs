use axum::extract::FromRef;
use leptos::config::LeptosOptions;

use crate::core::database::connection::DbPool;

#[derive(Debug, Clone, FromRef)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
}

impl AppState {
    pub fn new(pool: DbPool, leptos_options: LeptosOptions) -> Self {
        Self { leptos_options }
    }
}
