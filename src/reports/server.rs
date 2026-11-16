use crate::{
    frontend::suggestions::Suggestions,
    reports::dto::{FullStudyDto, NewFullStudyDto, ReportFieldsDto, StudyDto},
};
use leptos::prelude::*;

#[server]
pub async fn read_all() -> Result<Vec<FullStudyDto>, ServerFnError> {
    use crate::entrypoint::application_state::AppState;

    let ctx = use_context::<AppState>().unwrap();
    ctx.reports_service
        .read_all_studies()
        .await
        .map_err(ServerFnError::new)
        .map(|v| v.into_iter().map(FullStudyDto::from).collect())
}

#[server]
pub async fn read_report(id: i32) -> Result<FullStudyDto, ServerFnError> {
    use crate::entrypoint::application_state::AppState;

    let ctx = use_context::<AppState>().unwrap();
    ctx.reports_service
        .get_report_by_id(id)
        .await
        .map_err(ServerFnError::new)
}
#[server]
pub async fn read_all_surface() -> Result<Vec<StudyDto>, ServerFnError> {
    use crate::entrypoint::application_state::AppState;

    let ctx = use_context::<AppState>().unwrap();
    ctx.reports_service
        .read_all_shallow()
        .await
        .map_err(ServerFnError::new)
}

#[server]
pub async fn get_suggestions_for_report(
    field: ReportFieldsDto,
) -> Result<Vec<String>, ServerFnError> {
    use crate::entrypoint::application_state::AppState;
    let ctx = use_context::<AppState>().ok_or_else(|| ServerFnError::new("getting context"))?;
    ctx.reports_service
        .get_suggestions(field)
        .await
        .map_err(ServerFnError::new)
}

impl Suggestions for ReportFieldsDto {
    fn get_suggestions(
        &self,
    ) -> impl std::prelude::rust_2024::Future<Output = Result<Vec<String>, ServerFnError>> + Send + 'static
    {
        get_suggestions_for_report(self.clone())
    }
}

#[server]
pub async fn create_study(study: NewFullStudyDto) -> Result<(), ServerFnError> {
    use crate::entrypoint::application_state::AppState;
    use leptos::logging::log;

    log!("Creando estudio: {:?}", study);
    let context =
        use_context::<AppState>().ok_or_else(|| ServerFnError::new("Failed to get state"))?;
    match context
        .reports_service
        .create_study(study)
        .await
        .map_err(ServerFnError::new)
        .map(FullStudyDto::from)
    {
        Ok(study) => leptos_axum::redirect("/create_report"),
        Err(error) => log!("Error => {}", error),
    }
    Ok(())
}
