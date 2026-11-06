use leptos::prelude::*;

#[server]
pub async fn get_reports() -> Result<Vec<String>, ServerFnError> {
    use crate::entrypoint::application_state::AppState;
    let context = expect_context::<AppState>();
    context
        .reports_service
        .get_all_reports()
        .await
        .map_err(ServerFnError::new)
}

