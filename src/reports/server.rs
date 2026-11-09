use crate::reports::dto::FullStudyDto;
use leptos::prelude::*;

#[server]
pub async fn read_all_and_stringify() -> Result<Vec<FullStudyDto>, ServerFnError> {
    use crate::entrypoint::application_state::AppState;

    let ctx = use_context::<AppState>().unwrap();
    ctx.reports_service
        .read_all_studies()
        .await
        .map_err(ServerFnError::new)
        .map(|v| v.into_iter().map(FullStudyDto::from).collect())
}
