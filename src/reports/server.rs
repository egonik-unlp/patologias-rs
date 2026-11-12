use crate::reports::dto::{FieldsDto, FullStudyDto};
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

#[server]
pub async fn get_suggestions_for_field(field: FieldsDto) -> Result<Vec<String>, ServerFnError> {
    use crate::entrypoint::application_state::AppState;
    let ctx = use_context::<AppState>().ok_or_else(|| ServerFnError::new("getting context"))?;
    ctx.reports_service
        .get_suggestions(field)
        .await
        .map_err(ServerFnError::new)
}
